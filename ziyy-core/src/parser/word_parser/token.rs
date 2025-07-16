use crate::common::Span;
#[derive(Default, Debug)]
pub struct Token {
    pub literal: char,
    #[allow(dead_code)]
    pub span: Span,
}

impl Token {
    pub fn new(literal: char, span: Span) -> Self {
        Token { literal, span }
    }
}
