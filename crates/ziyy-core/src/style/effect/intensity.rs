use std::ops::{Add, Not, Sub};

use super::super::convert::FromU8;

#[repr(u8)]
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum Intensity {
    #[default]
    None,
    Bold,
    Dim,
    NoBold,
    NoDim,
    Unset,
}

impl Intensity {
    #[inline]
    pub(in crate::style) const fn as_str2(&self, prev: Intensity) -> &str {
        use Intensity::{Bold, Dim, NoBold, NoDim, None, Unset};

        match (prev, self) {
            (Bold, Dim) => "\x1b[22;2m",
            (Dim, Bold) => "\x1b[22;1m",

            (_, None) => "",
            (_, Bold) => "\x1b[1m",
            (_, Dim) => "\x1b[2m",
            (_, Unset) => "\x1b[22m",

            (_, NoBold) => "\x1b[22m",
            (_, NoDim) => "\x1b[22m",
        }
    }

    #[inline]
    pub(in crate::style) const fn _as_bytes(&self, prev: Intensity) -> &[u8] {
        self.as_str2(prev).as_bytes()
    }

    #[must_use]
    #[inline]
    pub const fn as_str(&self) -> &str {
        self.as_str2(Intensity::None)
    }

    #[must_use]
    #[inline]
    pub const fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}

impl FromU8 for Intensity {
    #[inline]
    fn from_u8(value: u8) -> Self {
        use Intensity::{Bold, Dim, NoBold, NoDim, None, Unset};

        match value {
            0 => None,
            1 => Bold,
            2 => Dim,
            3 => NoBold,
            4 => NoDim,
            5 => Unset,
            _ => unreachable!(),
        }
    }
}

impl Add for Intensity {
    type Output = Intensity;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        use Intensity::{Bold, Dim, NoBold, NoDim, None, Unset};

        match (self, rhs) {
            (None, Unset | NoBold | NoDim) => None,
            (None, rhs) => rhs,
            (lhs, None) => lhs,

            (Bold, Dim | NoBold) => rhs,
            (Bold, Bold | NoDim) => Bold,

            (Dim, Bold | NoDim) => rhs,
            (Dim, Dim | NoBold) => Dim,

            (NoBold, Bold | Dim) => rhs,
            (NoBold, _) => NoBold,

            (NoDim, Bold | Dim) => rhs,
            (NoDim, _) => NoDim,

            (Unset, Bold | Dim) => rhs,
            (Unset, NoBold | NoDim) => Unset,

            (_, Unset) => Unset,
        }
    }
}

impl Sub for Intensity {
    type Output = Intensity;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        use Intensity::None;

        match (self, rhs) {
            (None, rhs) => !rhs,
            (lhs, rhs) if lhs == rhs => None,
            (lhs, _) => lhs,
        }
    }
}

impl Not for Intensity {
    type Output = Intensity;

    #[inline]
    fn not(self) -> Self::Output {
        use Intensity::{Bold, Dim, NoBold, NoDim, None};

        match self {
            Bold => NoBold,
            Dim => NoDim,
            _ => None,
        }
    }
}
