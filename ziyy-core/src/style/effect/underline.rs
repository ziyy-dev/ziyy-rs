use super::super::convert::FromU8;
use std::ops::{Add, Not, Sub};

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum Underline {
    #[default]
    None,
    Single,
    Double,
    Curly,
    Dotted,
    Dashed,
    Unset,
}

impl Underline {
    #[must_use]
    pub fn as_str(&self) -> &str {
        use Underline::{Curly, Dashed, Dotted, Double, None, Single, Unset};

        match self {
            None => "",
            Single => "\x1b[4m",
            Double => "\x1b[21m",
            Curly => "\x1b[4:3m",
            Dotted => "\x1b[4:4m",
            Dashed => "\x1b[4:5m",
            Unset => "\x1b[24m",
        }
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}

impl FromU8 for Underline {
    fn from_u8(value: u8) -> Self {
        use Underline::{Curly, Dashed, Dotted, Double, None, Single, Unset};

        match value {
            0 => None,
            1 => Single,
            2 => Double,
            3 => Curly,
            4 => Dotted,
            5 => Dashed,
            6 => Unset,
            _ => unreachable!(),
        }
    }
}

impl Add for Underline {
    type Output = Underline;

    fn add(self, rhs: Self) -> Self::Output {
        use Underline::None;

        match (self, rhs) {
            (None, rhs) => rhs,
            (lhs, None) => lhs,
            (_, rhs) => rhs,
        }
    }
}

impl Sub for Underline {
    type Output = Underline;

    fn sub(self, rhs: Self) -> Self::Output {
        use Underline::None;

        match (self, rhs) {
            (None, rhs) => !rhs,
            (lhs, rhs) if lhs == rhs => None,
            (lhs, _) => lhs,
        }
    }
}

impl Not for Underline {
    type Output = Underline;

    fn not(self) -> Self::Output {
        use Underline::{Curly, Dashed, Dotted, Double, None, Single, Unset};

        match self {
            None => None,
            Single => Unset,
            Double => Unset,
            Curly => Unset,
            Dotted => Unset,
            Dashed => Unset,
            Unset => None,
        }
    }
}

/* #[cfg(test)]
mod tests {
    use super::*;
    use super::super::Style;

    #[test]
    fn test_underline() {
        let mut style = Style::default();

        style.set_underline(Underline::Single);
        assert_eq!(Underline::Single, style.underline());

        style.set_underline(Underline::Double);
        assert_eq!(Underline::Double, style.underline());

        style.set_underline(Underline::Curly);
        assert_eq!(Underline::Curly, style.underline());

        style.set_underline(Underline::Dashed);
        assert_eq!(Underline::Dashed, style.underline());

        style.set_underline(Underline::Unset);
        assert_eq!(Underline::Unset, style.underline());
    }
} */
