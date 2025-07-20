use std::borrow::Cow;

use crate::common::Span;

#[derive(Debug, Clone)]
pub enum FragmentType {
    Tag,
    Whitespace,
    Word,
}

#[derive(Debug, Clone)]

pub struct Fragment<'a> {
    pub r#type: FragmentType,
    pub lexeme: Cow<'a, str>,
    pub span: Span,
}

impl<'a> Fragment<'a> {
    pub fn new(r#type: FragmentType, lexeme: Cow<'a, str>, span: Span) -> Self {
        Fragment {
            r#type,
            lexeme,
            span,
        }
    }
}
