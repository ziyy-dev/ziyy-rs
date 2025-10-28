#![warn(rustdoc::private_intra_doc_links)]
#![warn(unconditional_panic)]
#![warn(clippy::pedantic)]
#![doc = include_str!("../../../README.md")]

pub use context::Context;
pub use error::{Error, ErrorKind, Result};
pub use parser::{Chunk, Tag, TagKind, TagName};
pub use renderer::Renderer;
pub use shared::{Position, Span, Value};
#[cfg(feature = "tree")]
pub use tree::Tree;

#[macro_use]
mod macros;

mod builtins;
mod context;
mod error;
mod num;
pub mod parser;
pub mod renderer;
mod scanner;
mod shared;
pub mod style;
#[cfg(feature = "tree")]
pub mod tree;

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
#[inline]
pub fn style(text: &str) -> std::string::String {
    match try_style(text) {
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
/// <span s c='blue'>This text is in blue"#)?;
/// # Ok(())
/// # }
#[must_use]
#[inline]
pub fn try_style(text: &str) -> Result<'_, str, std::string::String> {
    let renderer = Renderer::new(String::new());
    renderer.render(text)
}

#[must_use]
#[inline]
#[cfg(feature = "tree")]
pub fn render_to_tree(text: &str) -> Tree<'_, str> {
    let renderer = Renderer::new(Tree::new());
    renderer.render(text).unwrap()
}
