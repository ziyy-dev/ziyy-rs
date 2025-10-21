use super::super::convert::FromU8;
use std::ops::{Add, Not, Sub};

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum Frame {
    #[default]
    None,
    Framed,
    Encircled,
    Unset,
}

impl Frame {
    pub fn as_str(&self) -> &str {
        use Frame::*;

        match self {
            None => "",
            Framed => "\x1b[51m",
            Encircled => "\x1b[52m",
            Unset => "\x1b[54m",
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}

impl FromU8 for Frame {
    fn from_u8(value: u8) -> Self {
        use Frame::*;

        match value {
            0 => None,
            1 => Framed,
            2 => Encircled,
            3 => Unset,
            _ => unreachable!(),
        }
    }
}

impl Add for Frame {
    type Output = Frame;

    fn add(self, rhs: Self) -> Self::Output {
        use Frame::*;

        match (self, rhs) {
            (None, Unset) => None,
            (None, rhs) => rhs,
            (lhs, None) => lhs,
            (_, rhs) => rhs,
        }
    }
}

impl Sub for Frame {
    type Output = Frame;

    fn sub(self, rhs: Self) -> Self::Output {
        use Frame::*;

        match (self, rhs) {
            (None, rhs) => !rhs,
            (lhs, rhs) if lhs == rhs => None,
            (lhs, _) => lhs,
        }
    }
}

impl Not for Frame {
    type Output = Frame;

    fn not(self) -> Self::Output {
        use Frame::*;

        match self {
            Framed => Unset,
            Encircled => Unset,
            _ => None,
        }
    }
}
