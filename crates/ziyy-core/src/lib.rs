#![warn(rustdoc::private_intra_doc_links)]
#![warn(unconditional_panic)]
#![warn(clippy::pedantic)]
#![doc = include_str!("../../README.md")]
#[doc(hidden)]
pub use crate::document::{Document, NodeId, NodeMut, NodeRef};
pub use crate::error::{Error, ErrorKind};
pub use crate::parser::{Chunk, Context, Parser, Tag, TagKind, TagName};
pub use crate::scanner::position::Position;
pub use crate::scanner::span::Span;
pub use crate::style::*;
use num::{str_to_u32, str_to_u8};

mod document;
mod error;
mod num;
mod parser;
mod scanner;
mod style;

/// Styles the given text using ziyy.
///
/// # Example
///
/// ```
/// # use ziyy_core as ziyy;
/// use ziyy::style;
///
/// let styled_text = style("This is <b>bold</b> text");
/// ```
/// # Panics
///
/// This function will panic if the parser encounters an error while parsing the input source.
#[must_use]
pub fn style(text: &str) -> String {
    let mut parser = Parser::new();

    match parser.parse(Context::new(text, None)) {
        Ok(s) => s,
        Err(e) => panic!("{e}"),
    }
}

/// Styles the given text using ziyy.
///
/// # Example
///
/// ```
/// # fn main() -> ziyy_core::Result<'static, ()> {
/// # use ziyy_core as ziyy;
/// use ziyy::try_style;
///
/// let styled_text = try_style(r#"
/// <let id="custom" c="blue">
///     This is a custom element.
/// </let>
/// <span class='s custom'>This text is in blue</span>"#)?;
/// # Ok(())
/// # }
pub fn try_style(text: &str) -> Result<'_, String> {
    let mut parser = Parser::new();
    parser.parse(Context::new(text, None))
}

#[doc(hidden)]
#[must_use]
pub fn document(text: &str) -> Document<'_> {
    let mut parser = Parser::new();

    match parser.parse_to_doc(Context::new(text, None)) {
        Ok(s) => s,
        Err(e) => panic!("{e}"),
    }
}

/// Result
pub type Result<'src, T> = std::result::Result<T, Error<'src>>;
