use std::{error, fmt};

use crate::scanner::span::Span;
use crate::scanner::token::{Token, TokenKind};
use crate::TagName;

/// Represents an error with additional context such as its type, message, and location.
#[derive(Debug)]
pub struct Error<'src> {
    /// The type of the error.
    pub(crate) kind: ErrorKind<'src>,
    /// The span in the source where the error occurred.
    pub(crate) span: Span,
}

impl<'src> Error<'src> {
    pub fn kind(&self) -> &ErrorKind<'src> {
        &self.kind
    }
}

impl<'src> error::Error for Error<'src> {}

impl<'src> fmt::Display for Error<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("error: ")?;
        match &self.kind {
            ErrorKind::UnexpectedToken { expected, found } => match found {
                Some(found) => f.write_fmt(format_args!(
                    "unexpected token, expected {expected:?}, found `{found}`"
                )),
                None => f.write_fmt(format_args!("unexpected token, expected {expected:?}")),
            },
            ErrorKind::UnknownToken(tok) => f.write_fmt(format_args!("Unknown token: {tok}")),
            ErrorKind::MisMatchedTags { open, close } => {
                f.write_fmt(format_args!("mismatched tags: <{open}>...</{close}>"))
            }
            ErrorKind::InvalidNumber(number) => {
                f.write_fmt(format_args!("invalid number: `{number}`"))
            }
            ErrorKind::InvalidColor(color) => f.write_fmt(format_args!("invalid color: '{color}'")),
            ErrorKind::InvalidTagName(name) => {
                f.write_fmt(format_args!("invalid tag name: `{name}`"))
            }
            ErrorKind::UnexpectedEof => f.write_str("Unexpected Eof"),
            ErrorKind::UnterminatedString => f.write_str("unterminated string"),
        }?;

        f.write_fmt(format_args!(" at {}", self.span))
    }
}

impl<'src> Error<'src> {
    /// Creates a new `Error` instance.
    ///
    /// # Arguments
    ///
    /// * `kind` - The kind of error.
    /// * `token` - The token associated with the error.
    ///
    /// # Returns
    ///
    /// A new `Error` instance.
    pub(crate) fn new(kind: ErrorKind<'src>, token: &Token) -> Self {
        Self {
            kind,
            span: token.span.clone(),
        }
    }
}

#[non_exhaustive]
#[derive(Debug, PartialEq)]
/// Represents the different kinds of parse errors.
pub enum ErrorKind<'src> {
    /// Indicates an invalid color was encountered.
    InvalidColor(&'src str),
    /// Indicates an invalid number was encountered.
    InvalidNumber(&'src str),
    InvalidTagName(&'src str),
    /// Mismatched opening and closing tags.
    MisMatchedTags {
        open: TagName<'src>,
        close: TagName<'src>,
    },
    /// Indicates the end of input was reached unexpectedly.
    UnexpectedEof,
    /// Indicates an unexpected token was encountered.
    UnexpectedToken {
        expected: TokenKind,
        found: Option<&'src str>,
    },
    /// An unknown token was encountered.
    UnknownToken(&'src str),
    /// Indicates an unterminated string literal.
    UnterminatedString,
}
