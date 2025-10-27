use core::fmt::Debug;
use std::fmt::Display;

#[derive(Clone, Copy, Default)]
pub struct Position {
    pub row: u32,
    pub col: u32,
}

impl Position {
    #[must_use]
    pub fn new(row: u32, col: u32) -> Self {
        Self { row, col }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.row, self.col))
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}:{}", self.row, self.col))
    }
}
