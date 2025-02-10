use super::span::Span;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    // Single-character tokens.
    LeftParen,
    RightParen,
    Equal,
    Comma,
    Less,
    LessSlash,
    Great,
    SlashGreat,
    //Plus,
    Dot,
    Slash,

    PlaceHolder,

    // C-Escapes
    EscA,
    EscB,
    EscT,
    EscN,
    EscV,
    EscF,
    EscR,
    EscE,
    EscBackSlash,
    EscLess,
    EscGreat,

    Esc0, // Octal Escape \0XXX
    EscX, // Hex Escape \xHHH
    EscU, // Unicode Escape \uHHHH

    // Literals.
    Identifier,
    String,
    Number,
    WhiteSpace,
    Text,

    // Builtin Variables.
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Rgb,
    Hex, /* #HHHHHH | #HHH */
    Byte,
    B,
    C,
    I,
    S,
    T,
    U,
    X,
    Eof,
    Error,
    Comment,
}

impl TokenKind {
    #[must_use]
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub content: &'a str,
    pub custom: u16,
    pub span: Span,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, content: &'a str, span: Span) -> Self {
        Token {
            kind,
            content,
            span,
            custom: 0,
        }
    }
}
