use std::num::Wrapping;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RawToken {
    Increment,
    Decrement,
    ShiftRight,
    ShiftLeft,
    Output,
    Input,
    OpenLoop,
    CloseLoop,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AstToken {
    Decrement(Wrapping<u8>),
    Increment(Wrapping<u8>),
    ShiftRight(usize),
    ShiftLeft(usize),
    Output,
    Input,
    Loop(Vec<AstToken>),
}

pub fn lex<'a>(chars: &'a mut impl Iterator<Item=char>) -> Vec<RawToken> {
    use self::RawToken::*;
    chars.filter_map(|c| match c {
        '+' => Some(Increment),
        '-' => Some(Decrement),
        '<' => Some(ShiftLeft),
        '>' => Some(ShiftRight),
        '.' => Some(Output),
        ',' => Some(Input),
        '[' => Some(OpenLoop),
        ']' => Some(CloseLoop),
        _ => None
    }).collect()
}

pub fn to_ast(tokens: &[RawToken]) -> Vec<AstToken> {
    let mut ast = vec![];

    let mut i = 0;

    // First convert one on one
    while i < tokens.len() {
        let token = tokens[i];

        ast.push(if token == RawToken::OpenLoop {
            // Create loop
            let close = find_close_loop(tokens, i);
            let loop_slice = &tokens[i + 1..close];
            assert!(check_loop_balance(loop_slice));
            let inner = to_ast(loop_slice);
            i = close + 1;
            AstToken::Loop(inner)
        } else {
            i +=  1;
            // Other tokens can be mapped directly else {
            match token {
                RawToken::Increment => AstToken::Increment(Wrapping(1)),
                RawToken::Decrement => AstToken::Decrement(Wrapping(1)),
                RawToken::ShiftRight => AstToken::ShiftRight(1),
                RawToken::ShiftLeft => AstToken::ShiftLeft(1),
                RawToken::Output => AstToken::Output,
                RawToken::Input => AstToken::Input,
                _ => panic!("Invalid syntax: {:?} [{}]{:?}", token, i, tokens)
            }
        });
    }

    ast
}

pub fn find_close_loop(tokens: &[RawToken], from: usize) -> usize {
    let mut inner_open = 0;
    tokens[from..]
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
        .0 + from
}

pub fn check_loop_balance(tokens: &[RawToken]) -> bool {
    tokens.iter().filter(|t| **t == RawToken::OpenLoop).count() ==
        tokens.iter().filter(|t| **t == RawToken::CloseLoop).count()
}