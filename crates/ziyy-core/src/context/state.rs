use smallvec::{SmallVec, smallvec};

use crate::error::{Error, ErrorKind, Result};
use crate::parser::{Tag, TagName};
use crate::shared::Input;
use crate::style::Style;

struct Entry<'src, I: ?Sized + Input> {
    name: TagName<'src, I>,
    accum: Style,
    diff: Style,
}

pub struct State<'src, I: ?Sized + Input> {
    stack: SmallVec<[Entry<'src, I>; 21]>,
}

impl<'src, I: ?Sized + Input> State<'src, I> {
    pub fn new() -> Self {
        State {
            stack: smallvec![Entry {
                name: TagName::Root,
                accum: Style::new(),
                diff: Style::new()
            }],
        }
    }

    pub fn push(&mut self, name: TagName<'src, I>, style: Style) -> Style {
        let prev_accum = match self.stack.last() {
            Some(entry) => entry.accum,
            None => unreachable!(),
        };

        let accum = prev_accum + style;
        let diff = style - prev_accum;
        if name == TagName::Ansi {
            match self.stack.last_mut() {
                Some(entry) => {
                    entry.accum = accum;
                    entry.diff = diff;
                }
                // the stack must contain at least the root element
                None => unreachable!(),
            }
        } else {
            self.stack.push(Entry { name, accum, diff });
        }
        diff
    }

    pub fn pop_tag(&mut self, tag: &Tag<'src, I>) -> Result<'src, I, Style> {
        let lname = match self.stack.last() {
            Some(entry) => &entry.name,
            None => unreachable!(),
        };

        if tag.name == *lname || tag.name == TagName::Empty {
            match self.stack.pop() {
                Some(entry) => Ok(entry.diff),
                None => unreachable!(),
            }
        } else {
            Err(Error {
                kind: ErrorKind::MisMatchedTags {
                    open: lname.clone(),
                    close: tag.name.clone(),
                },
                span: tag.span,
            })
        }
    }

    pub fn pop(&mut self) -> Option<Style> {
        match self.stack.pop() {
            Some(entry) => Some(entry.diff),
            None => None,
        }
    }
}
