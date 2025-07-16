use std::{
    borrow::Cow,
    fmt::Display,
    ops::{Deref, DerefMut},
};

use crate::common::Span;

use super::tag_parser::tag::Tag;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChunkData<'a> {
    Tag(Tag),
    WhiteSpace(Cow<'a, str>),
    Word(Cow<'a, str>),
}

impl<'a> ChunkData<'a> {
    pub fn new_tag(tag: Tag) -> Self {
        Self::Tag(tag)
    }

    pub fn new_word(word: Cow<'a, str>) -> Self {
        Self::Word(word)
    }

    pub fn new_ws(ws: Cow<'a, str>) -> Self {
        Self::WhiteSpace(ws)
    }

    pub fn is_tag(&self) -> bool {
        matches!(self, ChunkData::Tag(_))
    }

    pub fn is_tag_and<F>(&self, f: F) -> bool
    where
        F: FnOnce(&Tag) -> bool,
    {
        match self {
            ChunkData::Tag(tag) => f(tag),
            _ => false,
        }
    }

    pub fn is_word(&self) -> bool {
        matches!(self, ChunkData::Word(_))
    }

    pub fn is_ws(&self) -> bool {
        matches!(self, ChunkData::WhiteSpace(_))
    }

    pub fn tag(&self) -> Option<&Tag> {
        if let ChunkData::Tag(tag) = self {
            Some(tag)
        } else {
            None
        }
    }

    pub fn word(&self) -> Option<&Cow<'a, str>> {
        if let ChunkData::Word(word) = self {
            Some(word)
        } else {
            None
        }
    }

    pub fn ws(&self) -> Option<&Cow<'a, str>> {
        if let ChunkData::WhiteSpace(ws) = self {
            Some(ws)
        } else {
            None
        }
    }

    pub fn tag_mut(&mut self) -> Option<&mut Tag> {
        if let ChunkData::Tag(tag) = self {
            Some(tag)
        } else {
            None
        }
    }
}

impl<'a> Display for ChunkData<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            match self {
                ChunkData::Tag(tag) => f.write_fmt(format_args!("{tag:#}")),
                ChunkData::WhiteSpace(ws) => f.write_fmt(format_args!("{ws:?}")),
                ChunkData::Word(word) => f.write_fmt(format_args!("{word:?}")),
            }
        } else {
            match self {
                ChunkData::Tag(tag) => tag.fmt(f),
                ChunkData::WhiteSpace(ws) => ws.fmt(f),
                ChunkData::Word(word) => word.fmt(f),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]

pub struct Chunk<'a> {
    pub data: ChunkData<'a>,
    pub span: Span,
}

impl<'a> Deref for Chunk<'a> {
    type Target = ChunkData<'a>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a> DerefMut for Chunk<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'a> Display for Chunk<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_fmt(format_args!(
                "{:#} \x1b[38;5;59m--> {}\x1b[39m",
                self.data, self.span
            ))
        } else {
            self.data.fmt(f)
        }
    }
}
