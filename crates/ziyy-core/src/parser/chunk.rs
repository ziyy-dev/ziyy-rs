use std::fmt::{Debug, Display};

use crate::shared::{Input, Span};
use crate::TagName;

use super::Tag;

#[derive(PartialEq)]
pub enum Chunk<'src, I: ?Sized + Input> {
    Comment(&'src I, Span),
    Eof(Span),
    Escape(char, Span),
    Tag(Tag<'src, I>),
    Text(&'src I, Span),
    WhiteSpace(&'src I, Span),
}

impl<I: ?Sized + Input> Chunk<'_, I> {
    pub fn span(&self) -> Span {
        match self {
            Chunk::Comment(_, span) => *span,
            Chunk::Eof(span) => *span,
            Chunk::Escape(_, span) => *span,
            Chunk::Tag(tag) => tag.span,
            Chunk::Text(_, span) => *span,
            Chunk::WhiteSpace(_, span) => *span,
        }
    }
}

impl<I: ?Sized + Debug + Display + Input> Display for Chunk<'_, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Chunk::Comment(s, _) => Debug::fmt(s, f),
            Chunk::Eof(span) => Debug::fmt(span, f),
            Chunk::Escape(s, _) => Debug::fmt(s, f),
            Chunk::Tag(tag) => match tag.name {
                TagName::Ansi => {
                    f.write_fmt(format_args!("<{} =\"\\x1b[{}m\">", tag.name, tag.custom))
                }
                _ => Display::fmt(tag, f),
            },
            Chunk::Text(s, _) => Debug::fmt(s, f),
            Chunk::WhiteSpace(s, _) => Debug::fmt(s, f),
        }?;

        f.write_fmt(format_args!(" \x1b[38;5;59m--> {}\x1b[39m", self.span()))
    }
}

impl<'src, I: ?Sized + Input> Clone for Chunk<'src, I> {
    fn clone(&self) -> Self {
        match self {
            Self::Comment(arg0, arg1) => Self::Comment(*arg0, arg1.clone()),
            Self::Eof(arg0) => Self::Eof(arg0.clone()),
            Self::Escape(arg0, arg1) => Self::Escape(arg0.clone(), arg1.clone()),
            Self::Tag(arg0) => Self::Tag(arg0.clone()),
            Self::Text(arg0, arg1) => Self::Text(*arg0, arg1.clone()),
            Self::WhiteSpace(arg0, arg1) => Self::WhiteSpace(*arg0, arg1.clone()),
        }
    }
}
