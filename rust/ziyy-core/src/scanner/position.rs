use core::fmt::Debug;
use std::fmt::Display;

#[derive(Clone, Copy, Default)]
pub struct Position {
    pub row: u16,
    pub col: u16,
}

impl Position {
    pub fn new(row: u16, col: u16) -> Self {
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
        f.write_fmt(format_args!("({},{})", self.row + 1, self.col + 1))
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}:{}", self.row + 1, self.col + 1))
    }
}
