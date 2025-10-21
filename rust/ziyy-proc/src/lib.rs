use core::iter::FromIterator;
use proc_macro::{Literal, TokenStream, TokenTree};

#[proc_macro]
pub fn ziyy(item: TokenStream) -> TokenStream {
    let mut tokens: Vec<_> = item.into_iter().collect();

    if !tokens.is_empty() {
        let token = tokens.get_mut(0).unwrap();

        if let TokenTree::Literal(literal) = token {
            let s: String = literal.to_string();
            let strings: Vec<_> = s.split('"').collect();
            let end = strings.len() - 1;
            let s = strings[1..end].join("\"");
            let parsed = ziyy_core::ziyy(&s);

            let literal = Literal::string(&parsed);
            *token = TokenTree::Literal(literal)
        }
    }

    TokenStream::from_iter(tokens)
}
