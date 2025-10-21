use super::super::convert::FromU8;
use std::ops::{Add, Not, Sub};

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum Blink {
    #[default]
    None,
    Slow,
    Fast,
    Unset,
}

impl Blink {
    pub fn as_str(&self) -> &str {
        use Blink::*;

        match self {
            None => "",
            Slow => "\x1b[5m",
            Fast => "\x1b[6m",
            Unset => "\x1b[25m",
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}

impl FromU8 for Blink {
    fn from_u8(value: u8) -> Self {
        use Blink::*;

        match value {
            0 => None,
            1 => Slow,
            2 => Fast,
            3 => Unset,
            _ => unreachable!(),
        }
    }
}

impl Add for Blink {
    type Output = Blink;

    fn add(self, rhs: Self) -> Self::Output {
        use Blink::*;

        match (self, rhs) {
            (None, Unset) => None,
            (None, rhs) => rhs,
            (lhs, None) => lhs,
            (_, rhs) => rhs,
        }
    }
}

impl Sub for Blink {
    type Output = Blink;

    fn sub(self, rhs: Self) -> Self::Output {
        use Blink::*;

        match (self, rhs) {
            (None, rhs) => !rhs,
            (lhs, rhs) if lhs == rhs => None,
            (lhs, _) => lhs,
        }
    }
}

impl Not for Blink {
    type Output = Blink;

    fn not(self) -> Self::Output {
        use Blink::*;

        match self {
            Slow => Unset,
            Fast => Unset,
            _ => None,
        }
    }
}
