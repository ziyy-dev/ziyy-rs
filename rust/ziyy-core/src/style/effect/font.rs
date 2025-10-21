use crate::style::convert::FromU8;
use std::ops::{Add, Not, Sub};

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum Font {
    #[default]
    None,
    Primary,
    FirstAlt,
    SecondAlt,
    ThirdAlt,
    FourthAlt,
    FifthAlt,
    SixthAlt,
    SeventhAlt,
    EighthAlt,
    NinthAlt,
}

impl Font {
    pub fn as_str(&self) -> &str {
        use Font::*;

        match self {
            None => "",
            Primary => "\x1b[10m",
            FirstAlt => "\x1b[11m",
            SecondAlt => "\x1b[12m",
            ThirdAlt => "\x1b[13m",
            FourthAlt => "\x1b[14m",
            FifthAlt => "\x1b[15m",
            SixthAlt => "\x1b[16m",
            SeventhAlt => "\x1b[17m",
            EighthAlt => "\x1b[18m",
            NinthAlt => "\x1b[19m",
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}

impl FromU8 for Font {
    fn from_u8(value: u8) -> Self {
        use Font::*;

        match value {
            0 => None,
            1 => Primary,
            2 => FirstAlt,
            3 => SecondAlt,
            4 => ThirdAlt,
            5 => FourthAlt,
            6 => FifthAlt,
            7 => SixthAlt,
            8 => SeventhAlt,
            9 => EighthAlt,
            10 => NinthAlt,
            _ => unreachable!(),
        }
    }
}

impl Add for Font {
    type Output = Font;

    fn add(self, rhs: Self) -> Self::Output {
        use Font::*;

        match (self, rhs) {
            (None, Primary) => None,
            (None, rhs) => rhs,
            (lhs, None) => lhs,
            (_, rhs) => rhs,
        }
    }
}

impl Sub for Font {
    type Output = Font;

    fn sub(self, rhs: Self) -> Self::Output {
        use Font::*;

        match (self, rhs) {
            (None, rhs) => !rhs,
            (lhs, rhs) if lhs == rhs => None,
            (lhs, _) => lhs,
        }
    }
}

impl Not for Font {
    type Output = Font;

    fn not(self) -> Self::Output {
        use Font::*;

        match self {
            None | Primary => None,
            _ => Primary,
        }
    }
}
