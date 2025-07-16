use std::borrow::Cow;
use std::mem::take;

use fragment::Fragment;
use fragment::FragmentType::{self, *};

use crate::common::Span;
use crate::{Error, ErrorType, Result};

pub mod fragment;

pub struct Splitter<'a> {
    source: &'a Cow<'a, str>,
    fragments: Vec<Fragment<'a>>,
    start: usize,
    current: usize,
    span: Span,
}

impl<'a> Default for Splitter<'a> {
    fn default() -> Self {
        Self::new()
    }
}

enum Quote {
    Single,
    Double,
    None,
}

static SOURCE: Cow<'_, str> = Cow::Borrowed("");

impl<'a> Splitter<'a> {
    pub fn new() -> Self {
        Self {
            source: &SOURCE,
            fragments: vec![],
            start: 0,
            current: 0,
            span: Span::default(),
        }
    }

    pub fn split(&mut self, source: &'a Cow<'a, str>) -> Result<Vec<Fragment<'a>>> {
        self.source = source;

        macro_rules! consume_word {
            ($c:ident) => {
                loop {
                    if self.is_at_end() {
                        break;
                    }

                    if is_whitespace(self.peek()) {
                        break;
                    }

                    if matches!(self.peek(), b'<') {
                        break;
                    }

                    if matches!($c, b'\\') {
                        self.advance();
                    }

                    self.advance();
                }
            };
        }

        while !self.is_at_end() {
            self.start = self.current;

            let mut c = self.advance();

            match c {
                b' ' | b'\r' | b'\t' | b'\n' => self.whitespace(),
                b'\\' => {
                    c = self.advance();
                    consume_word!(c);
                    self.add_fragment(Word);
                }
                b'<' => self.tag()?,
                _ => {
                    consume_word!(c);
                    self.add_fragment(Word);
                }
            }
        }

        Ok(take(&mut self.fragments))
    }

    fn tag(&mut self) -> Result<()> {
        if self.peek() == b'>' {
            self.advance();
            self.add_fragment(Tag);
            return Ok(());
        }
        let mut quote = Quote::None;

        loop {
            let c = self.advance();
            if self.is_at_end() {
                match quote {
                    Quote::Single | Quote::Double => {
                        return Err(Error::new(
                            ErrorType::UnterminatedString,
                            "Untermitated string literal".into(),
                            self.span,
                        ));
                    }
                    Quote::None => {
                        return Err(Error::new(
                            ErrorType::UnexpectedEof,
                            "Untermitated string literal".into(),
                            self.span,
                        ));
                    }
                }
            }

            let close = matches!(self.peek(), b'>');
            let single = matches!(self.peek(), b'\'');
            let double = matches!(self.peek(), b'"');
            let esc = matches!(c, b'\\');
            match quote {
                Quote::Single => {
                    if single && !esc {
                        quote = Quote::None;
                    }
                }
                Quote::Double => {
                    if double && !esc {
                        quote = Quote::None;
                    }
                }
                Quote::None => {
                    if close {
                        break;
                    } else if single {
                        quote = Quote::Single;
                    } else if double {
                        quote = Quote::Double;
                    }
                }
            }
        }

        self.advance();
        self.add_fragment(Tag);
        Ok(())
    }

    fn whitespace(&mut self) {
        while is_whitespace(self.peek()) {
            self.advance();
        }
        self.add_fragment(Whitespace);
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            b'\0'
        } else {
            self.source.as_bytes()[self.current]
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.span += (0, 1);
        let ch = self.source.as_bytes()[self.current - 1];
        if ch == b'\n' {
            self.span += (1, 0);
        }
        ch
    }

    fn add_fragment(&mut self, r#type: FragmentType) {
        let text = &self.source[self.start..self.current];
        self.fragments
            .push(Fragment::new(r#type, Cow::Borrowed(text), self.span));
        self.span.tie_end();
    }
}

pub fn is_whitespace(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | b'\x0c' | b'\x0d')
}
