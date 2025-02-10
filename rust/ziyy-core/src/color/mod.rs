use std::fmt::Display;

use bit_4::Bit4;
use channel::Channel;
use number::Number;
use rgb::Rgb;

use crate::{
    error::ErrorKind,
    get_num,
    num::str_to_u32,
    number, own,
    scanner::{
        span::Span,
        token::{Token, TokenKind},
        Scanner,
    },
    Error,
};

pub mod bit_4;
pub mod channel;
pub mod number;
pub mod rgb;

#[derive(PartialEq, Debug, Clone)]
pub enum ColorKind {
    Bit4(Bit4),
    Byte(Number),
    Rgb(Rgb),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Color {
    kind: ColorKind,
    channel: Channel,
}

impl Color {
    pub fn new(kind: ColorKind, channel: Channel) -> Self {
        Self { kind, channel }
    }

    #[cfg(test)]
    pub fn fg(kind: ColorKind) -> Self {
        Self {
            kind,
            channel: Channel::Foreground,
        }
    }

    #[cfg(test)]
    pub fn bg(kind: ColorKind) -> Self {
        Self {
            kind,
            channel: Channel::Background,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ColorKind::Bit4(bit4) => f.write_fmt(format_args!("{}{bit4};", self.channel)),
            ColorKind::Byte(byte) => f.write_fmt(format_args!("{}8;5;{byte};", self.channel)),
            ColorKind::Rgb(rgb) => f.write_fmt(format_args!("{}8;2;{rgb};", self.channel)),
        }
    }
}

impl TryFrom<(&str, Channel, Span)> for Color {
    type Error = Error;

    fn try_from(value: (&str, Channel, Span)) -> Result<Self, Self::Error> {
        let mut scanner = Scanner::new(value.0);
        scanner.text_mode = false;
        scanner.parse_colors = true;
        scanner.span = value.2.clone();

        let token = scanner.scan_token()?;
        let kind = match token.kind {
            TokenKind::Black => ColorKind::Bit4(Bit4::Black),
            TokenKind::Red => ColorKind::Bit4(Bit4::Red),
            TokenKind::Green => ColorKind::Bit4(Bit4::Green),
            TokenKind::Yellow => ColorKind::Bit4(Bit4::Yellow),
            TokenKind::Blue => ColorKind::Bit4(Bit4::Blue),
            TokenKind::Magenta => ColorKind::Bit4(Bit4::Magenta),
            TokenKind::Cyan => ColorKind::Bit4(Bit4::Cyan),
            TokenKind::White => ColorKind::Bit4(Bit4::White),
            TokenKind::Byte => {
                let token = scanner.scan_token()?;
                expect(&token, TokenKind::LeftParen)?;

                let token = scanner.scan_token()?;
                expect(&token, TokenKind::Number)?;
                let i = number!(token.content, 10, &token);

                let token = scanner.scan_token()?;
                expect(&token, TokenKind::RightParen)?;

                ColorKind::Byte(i)
            }
            TokenKind::Rgb => {
                let mut rgb_string = String::with_capacity(16);

                let token = scanner.scan_token()?;
                expect(&token, TokenKind::LeftParen)?;

                let mut token = scanner.scan_token()?;
                while token.kind != TokenKind::RightParen {
                    if token.kind == TokenKind::Eof {
                        return Err(Error::new(ErrorKind::UnexpectedEof, &token));
                    }
                    rgb_string.push_str(token.content);

                    token = scanner.scan_token()?;
                }

                ColorKind::Rgb(Rgb::try_from((rgb_string.as_str(), value.2))?)
            }
            _ => ColorKind::Bit4(Bit4::default()),
        };

        Ok(Color::new(kind, value.1))
    }
}

impl From<(Rgb, Channel)> for Color {
    fn from(value: (Rgb, Channel)) -> Self {
        Self {
            kind: ColorKind::Rgb(value.0),
            channel: value.1,
        }
    }
}

fn expect(token: &Token, tt: TokenKind) -> Result<(), Error> {
    if token.kind != tt {
        return Err(Error::new(
            ErrorKind::UnexpectedToken(tt, Some(token.kind)),
            token,
        ));
    }
    Ok(())
}

#[test]
fn test_color_from_str() {
    let color = Color::try_from(("red", Channel::Foreground, Span::default()));
    assert!(color.is_ok());
    assert_eq!(
        color.unwrap(),
        Color::new(ColorKind::Bit4(Bit4::Red), Channel::Foreground)
    );

    let color = Color::try_from(("rgb(0,10,100)", Channel::Background, Span::default()));
    assert!(color.is_ok());
    /* assert_eq!(
        color.unwrap(),
        Color::new(ColorKind::Rgb(Rgb(0, 10, 100)), Channel::Background)
    ); */

    let color = Color::try_from(("rgb(#B0B0B0)", Channel::Background, Span::default()));
    assert!(color.is_ok());
    /* assert_eq!(
        color.unwrap(),
        Color::new(ColorKind::Rgb(Rgb(176, 176, 176)), Channel::Background)
    ); */

    let color = Color::try_from(("byte(1)", Channel::Background, Span::default()));
    assert!(color.is_ok());
    /* assert_eq!(
        color.unwrap(),
        Color::new(ColorKind::Byte(1), Channel::Background)
    ); */
}
