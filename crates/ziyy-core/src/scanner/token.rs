use crate::shared::{Input, Span};

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
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

#[derive(Debug, Clone)]
pub struct Token<'src, I: ?Sized + Input> {
    pub kind: TokenKind,
    pub content: &'src I,
    pub span: Span,
}
