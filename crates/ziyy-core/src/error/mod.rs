use std::fmt::{Debug, Display};
use std::{error, fmt, io};

use crate::parser::TagName;
use crate::scanner::{Token, TokenKind};
use crate::shared::{Input, Span};

/// Result
pub type Result<'src, I, T> = std::result::Result<T, Error<'src, I>>;

/// Represents an error with additional context such as its type, message, and location.
pub struct Error<'src, I: ?Sized + Input> {
    /// The type of the error.
    pub(crate) kind: ErrorKind<'src, I>,
    /// The span in the source where the error occurred.
    pub(crate) span: Span,
}

impl<'src, I: ?Sized + Input> Error<'src, I> {
    #[must_use]
    pub fn kind(&self) -> &ErrorKind<'src, I> {
        &self.kind
    }
}

impl<I: ?Sized + Debug + Input> Debug for Error<'_, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Error")
            .field("kind", &self.kind)
            .field("span", &self.span)
            .finish()
    }
}

impl<I: ?Sized + Display + Input> fmt::Display for Error<'_, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("error: ")?;
        self.kind.fmt(f)?;
        if self.span == Span::inserted() {
            Ok(())
        } else {
            f.write_fmt(format_args!(" at {}", self.span))
        }
    }
}

impl<I: ?Sized + Debug + Display + Input> error::Error for Error<'_, I> {}

impl<'src, I: ?Sized + Input> From<io::Error> for Error<'src, I> {
    fn from(value: io::Error) -> Self {
        Error {
            kind: ErrorKind::IoError(value),
            span: Span::inserted(),
        }
    }
}

impl<'src, I: ?Sized + Input> From<fmt::Error> for Error<'src, I> {
    fn from(_: fmt::Error) -> Self {
        Error {
            kind: ErrorKind::FmtError,
            span: Span::inserted(),
        }
    }
}

impl<'src, I: ?Sized + Input> Error<'src, I> {
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
    pub(crate) fn new(kind: ErrorKind<'src, I>, token: &Token<I>) -> Self {
        Self {
            kind,
            span: token.span,
        }
    }
}

#[non_exhaustive]
/// Represents the different kinds of parse errors.
pub enum ErrorKind<'src, I: ?Sized + Input> {
    BuiltinTagOverwrite(&'src I),
    /// Indicates an invalid color was encountered.
    FmtError,
    IoError(io::Error),
    InvalidColor(&'src I),
    /// Indicates an invalid number was encountered.
    InvalidNumber(&'src I),
    InvalidTagName(&'src I),
    /// Mismatched opening and closing tags.
    MisMatchedTags {
        open: TagName<'src, I>,
        close: TagName<'src, I>,
    },
    /// Indicates the end of input was reached unexpectedly.
    UnexpectedEof,
    /// Indicates an unexpected token was encountered.
    UnexpectedToken {
        expected: TokenKind,
        found: Option<&'src I>,
    },
    /// An unknown token was encountered.
    UnknownToken(&'src I),
    /// Indicates an unterminated string literal.
    UnterminatedString,
}

impl<'src, I: ?Sized + Debug + Input> Debug for ErrorKind<'src, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::BuiltinTagOverwrite(arg0) => {
                f.debug_tuple("BuiltinTagOverwrite").field(arg0).finish()
            }
            ErrorKind::FmtError => write!(f, "FmtError"),
            ErrorKind::IoError(arg0) => f.debug_tuple("IoError").field(arg0).finish(),
            ErrorKind::InvalidColor(arg0) => f.debug_tuple("InvalidColor").field(arg0).finish(),
            ErrorKind::InvalidNumber(arg0) => f.debug_tuple("InvalidNumber").field(arg0).finish(),
            ErrorKind::InvalidTagName(arg0) => f.debug_tuple("InvalidTagName").field(arg0).finish(),
            ErrorKind::MisMatchedTags { open, close } => f
                .debug_struct("MisMatchedTags")
                .field("open", open)
                .field("close", close)
                .finish(),
            ErrorKind::UnexpectedEof => write!(f, "UnexpectedEof"),
            ErrorKind::UnexpectedToken { expected, found } => f
                .debug_struct("UnexpectedToken")
                .field("expected", expected)
                .field("found", found)
                .finish(),
            ErrorKind::UnknownToken(arg0) => f.debug_tuple("UnknownToken").field(arg0).finish(),
            ErrorKind::UnterminatedString => write!(f, "UnterminatedString"),
        }
    }
}

impl<'src, I: ?Sized + Display + Input> Display for ErrorKind<'src, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::BuiltinTagOverwrite(name) => {
                f.write_fmt(format_args!("attempt to overwrite builtin tag: `{name}`"))
            }
            ErrorKind::FmtError => f.write_str("format error"),
            ErrorKind::IoError(error) => Display::fmt(&error.kind(), f),
            ErrorKind::InvalidNumber(number) => {
                f.write_fmt(format_args!("invalid number: `{number}`"))
            }
            ErrorKind::InvalidColor(color) => f.write_fmt(format_args!("invalid color: '{color}'")),
            ErrorKind::InvalidTagName(name) => {
                f.write_fmt(format_args!("invalid tag name: `{name}`"))
            }
            ErrorKind::MisMatchedTags { open, close } => {
                f.write_fmt(format_args!("mismatched tags: <{open}>...</{close}>"))
            }
            ErrorKind::UnexpectedEof => f.write_str("Unexpected Eof"),
            ErrorKind::UnexpectedToken { expected, found } => match found {
                Some(found) => f.write_fmt(format_args!(
                    "unexpected token, expected {expected:?} but found `{found}`"
                )),
                None => f.write_fmt(format_args!("unexpected token, expected {expected:?}")),
            },
            ErrorKind::UnknownToken(tok) => f.write_fmt(format_args!("Unknown token: {tok}")),
            ErrorKind::UnterminatedString => f.write_str("unterminated string"),
        }
    }
}
