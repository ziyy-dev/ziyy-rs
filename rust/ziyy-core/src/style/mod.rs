#![allow(dead_code)]
use crate::color::Color;
pub use builder::StyleBuilder;
use close::StyleClose;
pub use condition::Condition;
use std::fmt::{Display, Write};
use std::io::Write as _;

mod builder;
mod close;
mod condition;

#[derive(Clone, Default, PartialEq, Debug)]
/// Style Information.
pub struct Style {
    pub(crate) brightness: Condition, // 1,22 \ 2,22
    pub(crate) italics: bool,         // 3,23
    pub(crate) under: Condition,      // 4,24 \ 21,24
    pub(crate) blink: bool,           // 5,25
    pub(crate) invert: bool,          // 7,27
    pub(crate) hide: bool,            // 8,28
    pub(crate) strike: bool,          // 9,29

    pub(crate) fg_color: Option<Color>,
    pub(crate) bg_color: Option<Color>,
}

impl Style {
    pub(crate) fn new() -> Self {
        Style::default()
    }

    /// Get close for tag.
    #[must_use]
    pub fn close(&self) -> StyleClose {
        StyleClose(self.clone())
    }

    pub(crate) fn add(&mut self, rhs: Self) {
        macro_rules! add {
            (bool, $lhs:expr, $rhs:expr, $f:tt) => {
                if $rhs.$f {
                    $lhs.$f = $rhs.$f
                }
            };

            (bool, $lhs:expr, $rhs:expr, $f:tt, $g:tt) => {
                if $rhs.$f {
                    $lhs.$g = false;
                    $lhs.$f = $rhs.$f
                }
            };

            (Option, $lhs:expr, $rhs:expr, $f:tt) => {
                if $rhs.$f.is_some() {
                    $lhs.$f = $rhs.$f
                }
            };
        }

        self.brightness += rhs.brightness;
        self.under += rhs.under;
        //add!(bool, self, rhs, dim, brightness);
        add!(bool, self, rhs, italics);
        //add!(bool, self, rhs, under, double_under);
        add!(bool, self, rhs, blink);
        add!(bool, self, rhs, invert);
        add!(bool, self, rhs, hide);
        add!(bool, self, rhs, strike);
        //add!(bool, self, rhs, double_under, under);

        add!(Option, self, rhs, fg_color);
        add!(Option, self, rhs, bg_color);
    }

    pub(crate) fn sub(&self, rhs: &Self) -> Self {
        macro_rules! sub {
            (bool, $lhs:expr, $rhs:expr, $f:tt) => {
                if $rhs.$f {
                    $lhs.$f = false;
                }
            };

            (bool, $lhs:expr, $rhs:expr, $f:tt, $g:tt) => {
                if $rhs.$f {
                    $lhs.$f = $rhs.$f;
                }
            };

            (Option, $lhs:expr, $rhs:expr, $f:tt) => {
                if $lhs.$f.is_some() && $lhs.$f == $rhs.$f {
                    $lhs.$f = None;
                }
            };
        }

        let mut lhs = self.clone();

        lhs.brightness -= rhs.brightness.clone();
        lhs.under -= rhs.under.clone();
        sub!(bool, lhs, rhs, italics);
        //sub!(bool, lhs, rhs, under);
        sub!(bool, lhs, rhs, blink);
        sub!(bool, lhs, rhs, invert);
        sub!(bool, lhs, rhs, hide);
        sub!(bool, lhs, rhs, strike);
        //sub!(bool, lhs, rhs, double_under);

        sub!(Option, lhs, rhs, fg_color);
        sub!(Option, lhs, rhs, bg_color);

        lhs
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        macro_rules! checked_write_bool {
            ($buf:expr, $bytes:expr, $current:expr, $f:tt) => {
                if $current.$f {
                    let _ = $buf.write($bytes);
                }
            };
        }

        macro_rules! checked_write_color {
            ($buf:expr, $current:expr, $f:tt) => {
                if let Some(color) = $current.$f.clone() {
                    let _ = $buf.write(color.to_string().as_bytes());
                }
            };
        }

        let mut buf = vec![];
        if *self == Self::default() {
            return Ok(());
        }
        let _ = buf.write(b"\x1b[");

        let _ = match self.brightness {
            Condition::A => buf.write(b"1;"),
            Condition::B | Condition::AB => buf.write(b"2;"),
            Condition::BA => buf.write(b"22;1;"),
            Condition::None => Ok(0),
        };

        if self.under.ends_with_a() {
            let _ = buf.write(b"4;");
        } else if self.under.ends_with_b() {
            let _ = buf.write(b"21;");
        }

        checked_write_bool!(buf, b"3;", self, italics);
        checked_write_bool!(buf, b"5;", self, blink);
        checked_write_bool!(buf, b"7;", self, invert);
        checked_write_bool!(buf, b"8;", self, hide);
        checked_write_bool!(buf, b"9;", self, strike);

        checked_write_color!(buf, self, fg_color);
        checked_write_color!(buf, self, bg_color);

        if buf[buf.len() - 1] == b';' {
            buf.pop();
        }

        buf.push(b'm');

        for ch in buf {
            f.write_char(ch as char)?; // all in ASCII range
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::color::{bit_4::Bit4, ColorKind};

    use super::*;

    #[test]
    fn test_style_default() {
        let style = Style::default();
        assert_eq!(style.brightness, Condition::None);
        assert!(!style.italics);
        assert_eq!(style.under, Condition::None);
        assert!(!style.blink);
        assert!(!style.invert);
        assert!(!style.hide);
        assert!(!style.strike);
        assert!(style.fg_color.is_none());
        assert!(style.bg_color.is_none());
    }

    #[test]
    fn test_style_add() {
        let mut style1 = Style::default();
        style1.italics = true;
        style1.fg_color = Some(Color::fg(ColorKind::Bit4(Bit4::Red)));

        let mut style2 = Style::default();
        style2.blink = true;
        style2.bg_color = Some(Color::fg(ColorKind::Bit4(Bit4::Blue)));

        style1.add(style2);

        assert!(style1.italics);
        assert!(style1.blink);
        assert_eq!(style1.fg_color, Some(Color::fg(ColorKind::Bit4(Bit4::Red))));
        assert_eq!(
            style1.bg_color,
            Some(Color::fg(ColorKind::Bit4(Bit4::Blue)))
        );
    }

    #[test]
    fn test_style_sub() {
        let mut style1 = Style::default();
        style1.italics = true;
        style1.fg_color = Some(Color::fg(ColorKind::Bit4(Bit4::Red)));

        let mut style2 = Style::default();
        style2.italics = true;
        style2.fg_color = Some(Color::fg(ColorKind::Bit4(Bit4::Red)));

        let result = style1.sub(&style2);

        assert!(!result.italics);
        assert!(result.fg_color.is_none());
    }

    #[test]
    fn test_style_display() {
        let mut style = Style::default();
        style.italics = true;
        style.fg_color = Some(Color::fg(ColorKind::Bit4(Bit4::Red)));

        let display = format!("{}", style);
        assert_eq!(display, "\x1b[3;31m");
    }
}
