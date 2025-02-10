use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use super::position::Position;

#[derive(Debug, Default, Clone, PartialEq)]
#[repr(transparent)]
pub struct Span(pub Vec<Position>);

impl Span {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, rhs: &Self) {
        self.extend_from_slice(rhs);
    }
}

impl Deref for Span {
    type Target = Vec<Position>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Span {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            Ok(())
        } else if self.len() == 1 {
            f.write_fmt(format_args!("{:?}", self.0[0]))
        } else {
            f.write_fmt(format_args!("{:?}", self.0[0]))?;
            let i = self.len() - 1;
            f.write_fmt(format_args!("..{:?}", self.0[i]))
        }
    }
}

impl From<&[Position]> for Span {
    fn from(value: &[Position]) -> Self {
        Span(Vec::from(value))
    }
}
