use crate::common::Span;

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
/// TokenType represents the different types of tokens that can be recognized by the parser.
/// It is used to categorize the tokens during the parsing process.
/// Each variant corresponds to a specific type of token that can be encountered in the input.
/// The variants are named in uppercase to follow the convention of naming constants.
/// The variants are:
/// - LESS: Represents the '<' character.
/// - GREATER: Represents the '>' character.
/// - LESS_SLASH: Represents the '</' character.
/// - SLASH: Represents the '/' character.
/// - SLASH_GREATER: Represents the '/>' character.
/// - IDENTIFIER: Represents an identifier token.
/// - STRING: Represents a string token.
/// - DASH: Represents a '-' character.
/// - BANG: Represents a '!' character.
/// - ERROR: Represents an error token.
///
/// This enum is used in the `Token` struct to specify the type of token being represented.
/// It is also used in the `Scanner` and `TagParser` modules to categorize tokens during the scanning and parsing process.
pub enum TokenType {
    LESS,
    GREATER,
    LESS_SLASH,
    SLASH,
    SLASH_GREATER,
    IDENTIFIER,
    EQUAL,
    STRING,
    DASH,
    BANG,
    ERROR,
}

#[derive(Debug, Clone)]
/// The `Token` struct represents a token in the parsing process.
/// It contains information about the token's type, lexeme, literal value, and the line number
/// where the token was found in the source code.
/// The `Token` struct is used to represent a token in the parsing process.
/// It is created by the `Scanner` module when scanning the input source code.
/// The `Token` struct is used in the `TagParser` module to represent tokens during the parsing process.
/// The `Token` struct is also used in the `Stage3` module to represent tokens during the parsing process.
/// The `Token` struct is used to represent a token in the parsing process.
/// The `Token` struct is used to represent a token in the parsing process.
/// The `Token` struct is used to represent a token in the parsing process.
/// The `Token` struct is used to represent a token in the parsing process.
pub struct Token {
    pub r#type: TokenType,
    pub lexeme: String,
    pub literal: Option<String>,
    pub span: Span,
}

impl Token {
    pub fn new(r#type: TokenType, lexeme: String, literal: Option<String>, span: Span) -> Self {
        Token {
            r#type,
            literal,
            lexeme,
            span,
        }
    }
}
