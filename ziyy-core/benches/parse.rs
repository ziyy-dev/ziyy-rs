#![feature(rustc_private, test)]

extern crate test;

use std::{borrow::Cow, rc::Rc};

use test::{Bencher, black_box};
use ziyy_core::{
    Chunk, Document, Fragment, Indexer, Parser, Resolver, Result, Splitter, WordParser, style,
};

const HELP: &str = include_str!("help.zy");

#[bench]
fn style_simple_text(b: &mut Bencher) {
    b.iter(|| {
        black_box(style("This is <b>bold</b> text"));
    });
}

#[bench]
fn style_single_tag(b: &mut Bencher) {
    b.iter(|| {
        black_box(style("<b i s u>"));
    });
}

#[bench]
fn style_empty_string(b: &mut Bencher) {
    b.iter(|| {
        black_box(style(""));
    });
}

fn index() -> Cow<'static, str> {
    let mut indexer = Indexer::new();
    indexer.index(HELP)
}

#[bench]
fn index_help(b: &mut Bencher) {
    b.iter(|| {
        black_box(index());
    });
}

fn split<'a>(source: &'a Cow<'a, str>) -> Vec<Fragment<'a>> {
    let mut splitter = Splitter::new();
    splitter.split(source).unwrap()
}

#[bench]
fn split_help(b: &mut Bencher) {
    b.iter(|| {
        let indexed = index();
        black_box(split(&indexed));
    });
}

fn parse<'a>(source: &'a Cow<'a, str>) -> Vec<Result<Chunk<'a>>> {
    let parser = Parser::new(false);
    parser.parse(split(source))
}

#[bench]
fn parse_help(b: &mut Bencher) {
    b.iter(|| {
        let indexed = index();
        black_box(parse(&indexed));
    });
}

fn resolve<'a>(source: &'a Cow<'a, str>, word_parser: &'a WordParser) -> Rc<Document<'a>> {
    let mut resolver = Resolver::new(false);
    // let word_parser = WordParser::new();
    resolver.resolve(parse(source), word_parser).unwrap()
}

#[bench]
fn resolve_help(b: &mut Bencher) {
    b.iter(|| {
        let indexed = index();
        let word_parser = WordParser::new();
        black_box(resolve(&indexed, &word_parser));
    });
}

fn to_string<'a>(source: &'a Cow<'a, str>, word_parser: &'a WordParser) -> String {
    let output = resolve(source, word_parser);
    let mut buf = String::new();
    output.root().to_string(&mut buf);
    buf
}

#[bench]
fn style_help(b: &mut Bencher) {
    b.iter(|| {
        let indexed = index();
        let word_parser = WordParser::new();
        black_box(to_string(&indexed, &word_parser));
    });
}
