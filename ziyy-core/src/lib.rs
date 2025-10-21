#![warn(rustdoc::private_intra_doc_links)]
#![warn(unconditional_panic)]
#![warn(clippy::pedantic)]
#![doc = include_str!("../../../README.md")]
//! # Examples
//! ```
//! use std::collections::HashMap;
//!
//! use ziyy::Parser;
//!
//! let mut parser = Parser::new("This is Some <c magenta u b>Magenta Underlined Bold Text</c>", None);
//! assert!(parser.parse().is_ok());
//!```
//! # Result
//! <pre>This is Some <span style="color: magenta;"><b><u>Magenta Underlined Bold Text</u></b></span></pre>
//!

pub use crate::error::{Error, ErrorKind};
pub use crate::parser::{Context, Parser, Tag, TagKind, TagName};
pub use crate::scanner::position::Position;
pub use crate::scanner::span::Span;
pub use crate::style::*;
pub use crate::ziyy as style;

use num::{str_to_u32, str_to_u8};

mod error;
mod num;
mod parser;
mod scanner;
mod style;

/// Styles the given text using the ziyy parser.
///
/// This function takes a string slice and returns a styled string. It uses the `Parser`
/// to parse the input text and apply the specified styles. If the parsing is successful,
/// it returns the styled string; otherwise, it panics with the error message.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to be styled.
///
/// # Example
///
/// ```
/// use ziyy::ziyy;
///
/// let styled_text = ziyy("This is <b>bold</b> text");
/// assert_eq!(styled_text, "This is <b>bold</b> text");
/// ```
///
/// # Panics
///
/// This function will panic if the parser encounters an error while parsing the input text.
///
/// # Returns
///
/// A `String` containing the styled text.
#[must_use]
pub fn ziyy(text: &str) -> String {
    let mut parser = Parser::new();

    match parser.parse(Context::new(text, None)) {
        Ok(s) => s,
        Err(e) => panic!("{e}"),
    }
}

/// Result
pub type Result<'src, T> = std::result::Result<T, Error<'src>>;
