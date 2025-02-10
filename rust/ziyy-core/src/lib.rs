#![warn(missing_docs)]
#![warn(rustdoc::private_intra_doc_links)]
#![warn(unconditional_panic)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
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
pub use crate::parser::{Parser, Tag, TagName, TagType};
pub use crate::scanner::token::TokenKind;
pub use crate::style::{Style, StyleBuilder};

mod color;
mod error;
mod num;
mod parser;
#[doc(hidden)]
pub mod scanner;
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
/// use ziyy::style;
///
/// let styled_text = style("This is <b>bold</b> text");
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
pub fn style<T: AsRef<str>>(text: T) -> String {
    let mut parser = Parser::new(text.as_ref(), None);
    match parser.parse() {
        Ok(s) => s,
        Err(e) => panic!("{e}"),
    }
}

/// Creates a new Template for styling text.
///
/// It takes in styling information and returns a
/// Closure that can be used to style text using
/// the styling information.
///
/// # Example
/// ```
/// use ziyy::prepare;
/// let bred = prepare("<b><c red>");
/// let text = bred("Bold Red Text");
/// assert!(text.is_ok());
/// println!("{}", text.unwrap());
/// ```
/// # Output
/// <pre style="color: red;"><b>Bold Red Text</b></pre>
///
pub fn prepare<T: AsRef<str>>(save: T) -> impl for<'a> FnMut(T) -> String {
    move |text: T| -> String { style(format!("{}{}", save.as_ref(), text.as_ref())) }
}

/// Result
pub type Result<T> = std::result::Result<T, Error>;
