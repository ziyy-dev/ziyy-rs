use core::iter::FromIterator;
use proc_macro::{Literal, TokenStream, TokenTree};

fn style_with(item: TokenStream, styler: fn(String) -> String) -> TokenStream {
    let mut tokens: Vec<_> = item.into_iter().collect();

    if !tokens.is_empty() {
        let token = tokens.get_mut(0).unwrap();

        if let TokenTree::Literal(literal) = token {
            let s: String = literal.to_string();
            let strings: Vec<_> = s.split('"').collect();
            let end = strings.len() - 1;
            let s = strings[1..end].join("\"");
            let parsed = styler(s);

            let literal = Literal::string(&parsed);
            *token = TokenTree::Literal(literal)
        }
    }

    let token_stream = TokenStream::from_iter(tokens).to_string();
    format!("format!({token_stream})").parse().unwrap()
}

/// Styles text
#[proc_macro]
pub fn style(item: TokenStream) -> TokenStream {
    style_with(item, ziyy_core::style::<String>)
}

/// Styles text without removing excess whitespace.
#[proc_macro]
pub fn exact_style(item: TokenStream) -> TokenStream {
    style_with(item, ziyy_core::style::<String>)
}
