#![allow(clippy::pedantic)]
#![feature(doc_cfg)]
//! # Ziyy's core library

pub use error::{Error, ErrorType, Result};

#[cfg(feature = "unstable")]
#[doc(cfg(feature = "unstable"))]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub use crate::{
    indexer::Indexer,
    parser::{
        Parser, WordParser,
        ansi::{Ansi, AnsiOptions, DuoEffect, Effect},
        chunk::{Chunk, ChunkData},
        color::{Ansi4Bit, Ansi256, Color, Rgb},
        tag_parser::TagParser,
        tag_parser::tag::{Tag, TagType},
    },
    resolver::{
        Resolver,
        document::{Document, Node},
    },
    splitter::{
        Splitter,
        fragment::{Fragment, FragmentType},
    },
};

#[cfg(not(feature = "unstable"))]
use crate::{
    indexer::Indexer,
    parser::{Parser, WordParser},
    resolver::Resolver,
    splitter::{
        Splitter,
        fragment::{Fragment, FragmentType},
    },
};

pub use common::{Position, Span};

mod builtin;
mod error;
#[macro_use]
mod scanner;
mod common;
mod indexer;
mod parser;
mod resolver;
mod splitter;

// mod ziyy;

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

pub fn style<T: AsRef<str>>(source: T) -> String {
    match try_style(source) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    }
}

/// Styles the given text using ziyy.
pub fn try_style<T: AsRef<str>>(source: T) -> Result<String> {
    if source.as_ref().is_empty() {
        return Ok(String::new());
    }

    let mut indexer = Indexer::new();
    let source = indexer.index(source.as_ref());
    let mut splitter = Splitter::new();
    let frags = splitter.split(&source)?;

    let parser = Parser::new(false);
    let chunks = parser.parse(frags);

    let mut resolver = Resolver::new(false);
    let word_parser = WordParser::new();
    let output = resolver.resolve(chunks, &word_parser)?;

    let mut buf = String::new();
    output.root().to_string(&mut buf);
    Ok(buf)
}
