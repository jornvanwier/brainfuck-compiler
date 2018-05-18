use std::io::{self, Read};

use compiler::RawToken;
use std::num::Wrapping;
use std::io::Write;

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
                        let mut inner_open = 0;
                        self.instruction_pointer = self.tokens[self.instruction_pointer..]
                            .iter()
                            .enumerate()
                            .find(|(_, &t)| {
                                match t {
                                    RawToken::OpenLoop => {
                                        inner_open += 1;
                                        false
                                    }
                                    RawToken::CloseLoop => {
                                        inner_open -= 1;
                                        inner_open == 0
                                    }
                                    _ => false
                                }
                            })
                            .expect("Unclosed loop!")
                            .0 +
                            self.instruction_pointer;
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

            self.instruction_pointer += 1;
        }
    }
}
