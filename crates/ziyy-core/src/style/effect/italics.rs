use std::ops::{Add, Not, Sub};

use super::super::convert::FromU8;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum FontStyle {
    #[default]
    None,
    Italics,
    Fraktur,
    Unset,
}

impl FontStyle {
    #[must_use]
    #[inline]
    pub const fn as_str(&self) -> &str {
        use FontStyle::{Fraktur, Italics, None, Unset};

        match self {
            None => "",
            Italics => "\x1b[3m",
            Fraktur => "\x1b[20m",
            Unset => "\x1b[23m",
        }
    }

    #[must_use]
    #[inline]
    pub const fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}

impl FromU8 for FontStyle {
    #[inline]
    fn from_u8(value: u8) -> Self {
        use FontStyle::{Fraktur, Italics, None, Unset};

        match value {
            0 => None,
            1 => Italics,
            2 => Fraktur,
            3 => Unset,
            _ => unreachable!(),
        }
    }
}

impl Add for FontStyle {
    type Output = FontStyle;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        use FontStyle::{None, Unset};

        match (self, rhs) {
            (None, Unset) => None,
            (None, rhs) => rhs,
            (lhs, None) => lhs,
            (_, rhs) => rhs,
        }
    }
}

impl Sub for FontStyle {
    type Output = FontStyle;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        use FontStyle::None;

        match (self, rhs) {
            (None, rhs) => !rhs,
            (lhs, rhs) if lhs == rhs => None,
            (lhs, _) => lhs,
        }
    }
}

impl Not for FontStyle {
    type Output = FontStyle;

    #[inline]
    fn not(self) -> Self::Output {
        use FontStyle::{Fraktur, Italics, None, Unset};

        match self {
            Italics | Fraktur => Unset,
            _ => None,
        }
    }
}
