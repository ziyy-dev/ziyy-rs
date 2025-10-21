use super::span::Span;

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TokenKind {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    EQUAL,
    COMMA,
    LESS,
    LESS_SLASH,
    GREAT,
    SLASH_GREAT,
    //Plus,
    DOT,
    SLASH,

    // C-Escapes
    ESC_A,
    ESC_B,
    ESC_T,
    ESC_N,
    ESC_V,
    ESC_F,
    ESC_R,
    ESC_E,
    ESC_BACK_SLASH,
    ESC_LESS,
    ESC_GREAT,

    ESC_0, // Octal Escape \0XXX
    ESC_X, // Hex Escape \xHHH
    ESC_U, // Unicode Escape \uHHHH

    // Literals.
    ANSI,
    ANSI_ESC,
    ESCAPED,
    IDENTIFIER,
    STRING,
    NUMBER,
    WHITESPACE,
    TEXT,
    HEX, /* #HHHHHH | #HHH */

    // Colors.
    BLACK,
    RED,
    GREEN,
    YELLOW,
    BLUE,
    MAGENTA,
    CYAN,
    WHITE,
    FIXED,
    RGB,

    // Tag Names
    A,
    B,
    BR,
    C,
    CODE,
    D,
    DIV,
    H,
    I,
    K,
    LET,
    O,
    P,
    PRE,
    R,
    S,
    SPAN,
    U,
    UU,
    X,
    ZIYY,

    // Others
    CLASS,
    CURLY,
    BLOCK,
    DASHED,
    DOUBLE,
    DOTTED,
    ID,
    INDENT,
    HREF,
    N,
    NONE,
    SINGLE,

    // Special
    COMMENT,
    EOF,
}

impl TokenKind {
    #[must_use]
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug, Clone)]
pub struct Token<'src> {
    pub kind: TokenKind,
    pub content: &'src str,
    pub custom: u16,
    pub span: Span,
}

impl<'src> Token<'src> {
    pub fn new(kind: TokenKind, content: &'src str, span: Span) -> Self {
        Token {
            kind,
            content,
            span,
            custom: 0,
        }
    }
}
