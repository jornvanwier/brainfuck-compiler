use std::io::{self, Read};

use compiler::RawToken;
use std::num::Wrapping;
use std::io::Write;
use compiler::find_close_loop;
use compiler::AstToken;

const TAPE_SIZE: usize = 2 << 16;
const WRAPPING_ONE: Wrapping<u8> = Wrapping(1);

pub struct VM<'a, Output: 'a + Write> {
    data_pointer: usize,
    instruction_pointer: usize,
    tokens: Vec<RawToken>,
    tape: Box<[Wrapping<u8>]>,
    open_loops: Vec<usize>,
    output: &'a mut Output,
}

impl<'a, Output: Write> VM<'a, Output> {
    pub fn new(tokens: Vec<RawToken>, output: &'a mut Output) -> Self {
        Self {
            data_pointer: 0,
            instruction_pointer: 0,
            tokens,
            tape: vec![Wrapping(0); TAPE_SIZE].into_boxed_slice(),
            open_loops: Vec::new(),
            output,
        }
    }

    pub fn execute(&mut self) {
        // todo change output?
        while self.instruction_pointer < self.tokens.len() {
//            println!("{} {}", self.instruction_pointer, self.tape[self.data_pointer]);
            match self.tokens[self.instruction_pointer] {
                RawToken::Increment => self.tape[self.data_pointer] += WRAPPING_ONE,
                RawToken::Decrement => self.tape[self.data_pointer] -= WRAPPING_ONE,
                RawToken::ShiftRight => self.data_pointer += 1,
                RawToken::ShiftLeft => self.data_pointer -= 1,
                RawToken::Output => { let _ = self.output.write(format!("{}", self.tape[self.data_pointer].0 as char).as_bytes()); }
                RawToken::Input => {
                    self.tape[self.data_pointer] = Wrapping(io::stdin().bytes().next().unwrap().expect(
                        "Couldn't read from stdin",
                    ))
                }
                RawToken::OpenLoop => {
                    if self.tape[self.data_pointer].0 == 0 {
                        self.instruction_pointer = find_close_loop(&self.tokens, self.instruction_pointer);
                    } else {
                        self.open_loops.push(self.instruction_pointer);
                    }
                }
                RawToken::CloseLoop => {
                    if self.tape[self.data_pointer].0 != 0 {
                        self.instruction_pointer =
                            *self.open_loops.last().expect("Unexpected CloseLoop token");
                    } else {
                        self.open_loops.pop();
                    }
                }
            }

            println!("{:?}", &self.tape[..20]);
            self.instruction_pointer += 1;
        }
    }
}

pub fn execute_ast<Output: Write>(tokens: &[AstToken], output: &mut Output) {
    let mut data_pointer = 0;
    let mut tape = vec![Wrapping(0); TAPE_SIZE].into_boxed_slice();

    execute_ast_slice(tokens, &mut data_pointer, &mut tape, output);
}

pub fn execute_ast_slice<Output: Write>(tokens: &[AstToken], data_pointer: &mut usize, tape: &mut [Wrapping<u8>], output: &mut Output) {
    let mut instruction_pointer = 0;
    while instruction_pointer < tokens.len() {
        match tokens[instruction_pointer] {
            AstToken::Increment(v) => tape[*data_pointer] += v,
            AstToken::Decrement(v) => tape[*data_pointer] -= v,
            AstToken::ShiftRight(v) => *data_pointer += v,
            AstToken::ShiftLeft(v) => *data_pointer -= v,
            AstToken::Output => { let _ = output.write(format!("{}", tape[*data_pointer].0 as char).as_bytes()); }
            AstToken::Input => tape[*data_pointer] = Wrapping(io::stdin().bytes().next().unwrap().expect(
                "Couldn't read from stdin",
            )),
            AstToken::Loop(ref loop_tokens) => {
                while tape[*data_pointer].0 != 0 {
                    execute_ast_slice(loop_tokens, data_pointer, tape, output);
                }
            }
        }

        instruction_pointer += 1;
    }
}