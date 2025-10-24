pub use ansi256::Ansi256;
pub use ansi_color::AnsiColor;
pub use rgb::Rgb;

use crate::number;
use crate::scanner::span::Span;
use crate::scanner::token::{Token, TokenKind};
use crate::scanner::Scanner;
use crate::style::convert::FromU32;
use crate::{Error, ErrorKind, Result};
use std::ops::{Add, Not, Sub};

mod ansi256;
mod ansi_color;
mod rgb;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum Color {
    #[default]
    None,
    Rgb(Rgb),
    Ansi256(Ansi256),
    AnsiColor(AnsiColor),
    Unset,
}

pub enum ColorKind {
    Foreground = 30,
    Background = 40,
    Underline = 50,
}

impl Color {
    const UNSET: Color = Color::AnsiColor(AnsiColor::Default);

    #[must_use]
    pub fn to_string(&self, kind: ColorKind) -> String {
        match self {
            Color::Rgb(rgb) => rgb.to_string(kind),
            Color::Ansi256(ansi256) => ansi256.to_string(kind),
            Color::AnsiColor(ansi_color) => ansi_color.to_string(kind),
            Color::Unset => format!("\x1b[{}m", kind as u8 + 9),
            Color::None => String::new(),
        }
    }

    #[must_use]
    pub fn to_vec(&self, kind: ColorKind) -> Vec<u8> {
        self.to_string(kind).into_bytes()
    }

    pub(crate) fn parse<'src>(source: &'src str, span: Span) -> Result<'src, Self> {
        let mut scanner = Scanner::new(source);
        scanner.text_mode = false;
        scanner.parse_colors = true;
        scanner.current_pos = span.start;

        let token = scanner.scan_token()?;
        let color = match token.kind {
            TokenKind::BLACK => Color::AnsiColor(AnsiColor::Black),
            TokenKind::RED => Color::AnsiColor(AnsiColor::Red),
            TokenKind::GREEN => Color::AnsiColor(AnsiColor::Green),
            TokenKind::YELLOW => Color::AnsiColor(AnsiColor::Yellow),
            TokenKind::BLUE => Color::AnsiColor(AnsiColor::Blue),
            TokenKind::MAGENTA => Color::AnsiColor(AnsiColor::Magenta),
            TokenKind::CYAN => Color::AnsiColor(AnsiColor::Cyan),
            TokenKind::WHITE => Color::AnsiColor(AnsiColor::White),
            TokenKind::FIXED => {
                let token = scanner.scan_token()?;
                expect(&token, TokenKind::LEFT_PAREN)?;

                let token = scanner.scan_token()?;
                expect(&token, TokenKind::NUMBER)?;
                let n = number!(token.content, 10, &token);

                let token = scanner.scan_token()?;
                expect(&token, TokenKind::RIGHT_PAREN)?;

                Color::Ansi256(Ansi256(n))
            }
            TokenKind::RGB => {
                let token = scanner.scan_token()?;
                expect(&token, TokenKind::LEFT_PAREN)?;

                let rgb = Color::Rgb(Rgb::parse(&mut scanner)?);

                let token = scanner.scan_token()?;
                expect(&token, TokenKind::RIGHT_PAREN)?;

                rgb
            }
            TokenKind::NONE => Color::Unset,
            _ => {
                return Err(Error {
                    kind: ErrorKind::InvalidColor(source),
                    span,
                })
            }
        };

        Ok(color)
    }
}

fn expect<'src>(token: &Token<'src>, tt: TokenKind) -> Result<'src, ()> {
    if token.kind != tt {
        return Err(Error::new(
            ErrorKind::UnexpectedToken {
                expected: tt,
                found: Some(token.content),
            },
            token,
        ));
    }
    Ok(())
}

impl FromU32 for Color {
    fn from_u32(n: u32) -> Self {
        if t!(!n) {
            Color::None
        } else if t!(n & 0b01) {
            Color::Rgb(Rgb(
                ((n >> 1) & 0xFF) as u8,
                ((n >> 9) & 0xFF) as u8,
                ((n >> 17) & 0xFF) as u8,
            ))
        } else if t!(n & 0b10) {
            Color::Ansi256(Ansi256((n >> 2) as u8))
        } else {
            let x = ((n >> 2) - 1) as u8;
            if x == 9 {
                Color::Unset
            } else {
                Color::AnsiColor(match AnsiColor::try_from(x) {
                    Ok(x) => x,
                    Err(_) => unreachable!(),
                })
            }
        }
    }
}

impl From<Color> for u32 {
    fn from(val: Color) -> Self {
        match val {
            Color::None => 0,
            Color::Rgb(Rgb(r, g, b)) => {
                0b1 | (u32::from(r) << 1) | (u32::from(g) << 9) | (u32::from(b) << 17)
            }
            Color::Ansi256(Ansi256(n)) => 0b10 | u32::from(n) << 2,
            Color::AnsiColor(n) => ((n as u32) + 1) << 2,
            Color::Unset => (9 + 1) << 2,
        }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        use Color::{None, Unset};

        match (self, rhs) {
            (None, Color::UNSET | Unset) => None,
            (None, rhs) => rhs,
            (lhs, None) => lhs,
            (_, rhs) => rhs,
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        use Color::None;

        match (self, rhs) {
            (None, _) => None,
            (lhs, rhs) if lhs == rhs => None,
            (lhs, _) => lhs,
        }
    }
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        use Color::{None, Unset};

        match self {
            None => None,
            Color::UNSET | Unset => None,
            _ => Unset,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_conversions() {
        let rgb = Color::Rgb(Rgb(255, 128, 0));
        let ansi = Color::AnsiColor(AnsiColor::Red);
        let ansi256 = Color::Ansi256(Ansi256(123));

        // Test FromU32/Into conversions
        let rgb_u32: u32 = rgb.into();
        assert_eq!(Color::from_u32(rgb_u32), rgb);

        let ansi_u32: u32 = ansi.into();
        assert_eq!(Color::from_u32(ansi_u32), ansi);

        let ansi256_u32: u32 = ansi256.into();
        assert_eq!(Color::from_u32(ansi256_u32), ansi256);
    }

    #[test]
    fn test_color_operations() {
        let c1 = Color::AnsiColor(AnsiColor::Red);
        let c2 = Color::AnsiColor(AnsiColor::Blue);
        let none = Color::None;
        let unset = Color::Unset;

        // Test Add
        assert_eq!(none + c1, c1);
        assert_eq!(c1 + none, c1);
        assert_eq!(c1 + c2, c2);
        assert_eq!(none + unset, none);

        // Test Sub
        assert_eq!(c1 - c2, c1);
        assert_eq!(c1 - c1, none);
        assert_eq!(none - c1, none);

        // Test Not
        assert_eq!(!c1, unset);
        assert_eq!(!none, none);
        assert_eq!(!unset, none);
    }

    #[test]
    fn test_color_parsing() {
        assert_eq!(
            Color::parse("red", Span::default()).unwrap(),
            Color::AnsiColor(AnsiColor::Red)
        );

        assert_eq!(
            Color::parse("rgb(255,0,0)", Span::default()).unwrap(),
            Color::Rgb(Rgb(255, 0, 0))
        );

        assert_eq!(
            Color::parse("fixed(123)", Span::default()).unwrap(),
            Color::Ansi256(Ansi256(123))
        );

        assert_eq!(Color::parse("none", Span::default()).unwrap(), Color::Unset);
    }
}
