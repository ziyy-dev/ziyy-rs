use std::fmt::Display;
use std::ops::{Add, AddAssign};

use super::position::Position;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    #[must_use]
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    pub(crate) const fn initial() -> Self {
        Self {
            start: Position { row: 1, col: 1 },
            end: Position { row: 1, col: 1 },
        }
    }

    pub(crate) const fn inserted() -> Self {
        Self {
            start: Position { row: 0, col: 0 },
            end: Position { row: 0, col: 0 },
        }
    }
}

impl Add for Span {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.start, rhs.end)
    }
}

impl AddAssign for Span {
    fn add_assign(&mut self, rhs: Self) {
        self.end = rhs.end;
    }
}

impl AddAssign<Position> for Span {
    fn add_assign(&mut self, rhs: Position) {
        self.end = rhs;
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == Span::inserted() {
            f.write_str("inserted")
        } else {
            f.write_fmt(format_args!(":{}", self.start))
        }
    }
}
