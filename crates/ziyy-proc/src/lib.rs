use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::{LitStr, parse_macro_input};

/// A procedural macro that processes a string literal to apply custom styling.
///
/// This macro takes a string literal as input and processes it using the `ziyy_core::try_style`
/// function. It's designed to handle styled text transformations at compile time.
///
/// # Panics
///
/// Panics if:
/// * The input is not a valid string literal
/// * The `try_style` function returns an error during string processing
///
/// # Example
///
/// ```
/// use ziyy_proc::zstr;
///
/// let styled_text = zstr!("some text with styling");
/// ```
#[proc_macro]
pub fn zstr(tokens: TokenStream) -> TokenStream {
    let source = parse_macro_input!(tokens as LitStr);
    let span = source.span();
    let parsed = match ziyy_core::try_style(&source.value()) {
        Ok(s) => s,
        Err(e) => panic!("{e}"),
    };

    let expanded = quote_spanned! {
        span => #parsed
    };

    TokenStream::from(expanded)
}
