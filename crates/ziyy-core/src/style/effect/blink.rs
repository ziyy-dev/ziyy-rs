use std::ops::{Add, Not, Sub};

use super::super::convert::FromU8;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum Blink {
    #[default]
    None,
    Slow,
    Fast,
    Unset,
}

impl Blink {
    #[must_use]
    #[inline]
    pub const fn as_str(&self) -> &str {
        use Blink::{Fast, None, Slow, Unset};

        match self {
            None => "",
            Slow => "\x1b[5m",
            Fast => "\x1b[6m",
            Unset => "\x1b[25m",
        }
    }

    #[must_use]
    #[inline]
    pub const fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}

impl FromU8 for Blink {
    #[inline]
    fn from_u8(value: u8) -> Self {
        use Blink::{Fast, None, Slow, Unset};

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

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        use Blink::{None, Unset};

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

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        use Blink::None;

        match (self, rhs) {
            (None, rhs) => !rhs,
            (lhs, rhs) if lhs == rhs => None,
            (lhs, _) => lhs,
        }
    }
}

impl Not for Blink {
    type Output = Blink;

    #[inline]
    fn not(self) -> Self::Output {
        use Blink::{Fast, None, Slow, Unset};

        match self {
            Slow => Unset,
            Fast => Unset,
            _ => None,
        }
    }
}
