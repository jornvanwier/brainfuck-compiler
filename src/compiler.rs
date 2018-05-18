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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Token {
    Increment,
    Decrement,
    ShiftRight,
    ShiftLeft,
    Output,
    Input,
    OpenLoop,
    CloseLoop,
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

//pub fn to_ast(tokens: &mut impl Iterator<Item=RawToken>) -> impl Iterator<Item=Token> {
//    unimplemented!()
//}