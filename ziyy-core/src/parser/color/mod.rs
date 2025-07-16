use crate::common::Span;
use crate::error::{Error, ErrorType};
use crate::scanner::GenericScanner;
pub use number::Number;
use scanner::Scanner;
use std::collections::VecDeque;
use std::fmt::Display;
use std::ops::AddAssign;
use token::TokenType::*;
use token::{Token, TokenType};

mod number;
mod scanner;
mod token;

macro_rules! number {
    ( $token:expr ) => {
        match $token.r#type {
            NUMBER => $token.literal.unwrap().into(),
            PLACE_HOLDER => $token.lexeme.clone().into(),
            _ => {
                return Err(Error::new(
                    ErrorType::InvalidNumber,
                    format!("{:?} is not a valid number", $token.lexeme),
                    $token.span,
                ));
            }
        }
    };
}

macro_rules! hex {
    ( $str:expr ) => {
        u8::from_str_radix($str, 16).unwrap()
    };
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Rgb(pub u8, pub u8, pub u8, pub u8);

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Rgb(r, g, b, n) = self;
        f.write_fmt(format_args!("{n};2;{r};{g};{b};"))
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Ansi256(pub u8, pub u8);

impl Display for Ansi256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Ansi256(i, n) = self;
        f.write_fmt(format_args!("{n};5;{i};"))
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Ansi4Bit(u8);

impl Display for Ansi4Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{};", self.0))
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Color {
    Ansi256(Ansi256),
    Ansi4Bit(Ansi4Bit),
    Rgb(Rgb),
    String(String),
}

impl Default for Color {
    fn default() -> Self {
        Self::new()
    }
}

impl Color {
    pub const fn new() -> Self {
        Color::String(String::new())
    }

    pub fn with(s: String) -> Self {
        Color::String(s)
    }

    fn parse_rgb(mut next: impl FnMut() -> Result<Token, Error>, n: u8) -> Result<Color, Error> {
        let token = next()?;

        expect(&token, LEFT_PAREN, ErrorType::UnexpectedToken)?;

        let token = next()?;
        let r: Number;
        let g: Number;
        let b: Number;

        match token.r#type {
            NUMBER | PLACE_HOLDER => {
                r = number!(token);

                let token = next()?;
                expect(&token, COMMA, ErrorType::UnexpectedToken)?;

                let token = next()?;
                g = number!(token);

                let token = next()?;
                expect(&token, COMMA, ErrorType::UnexpectedToken)?;

                let token = next()?;
                b = number!(token);
            }

            _ => {
                return Err(Error::new(
                    ErrorType::InvalidNumber,
                    format!("{:?} is not a valid number", token.lexeme),
                    token.span,
                ));
            }
        }

        let token = next()?;

        expect(&token, RIGHT_PAREN, ErrorType::UnexpectedToken)?;

        match (&r, &g, &b) {
            (Number::U8(r), Number::U8(g), Number::U8(b)) => Ok(Color::Rgb(Rgb(*r, *g, *b, n))),
            _ => Ok(Color::String(format!("{n};2;{r};{g};{b};"))),
        }
    }

    fn parse_hex(token: &Token, n: u8) -> Color {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        let lexeme = &token.lexeme[1..];
        match lexeme.len() {
            4 => {
                r = hex!(&lexeme[1..2].repeat(2));
                g = hex!(&lexeme[2..3].repeat(2));
                b = hex!(&lexeme[3..4].repeat(2));
            }
            7 => {
                r = hex!(&lexeme[1..3]);
                g = hex!(&lexeme[3..5]);
                b = hex!(&lexeme[5..7]);
            }
            _ => {}
        }
        Color::Rgb(Rgb(r, g, b, n))
    }

    fn parsed_fixed(mut next: impl FnMut() -> Result<Token, Error>, n: u8) -> Result<Color, Error> {
        let token = next()?;
        expect(&token, LEFT_PAREN, ErrorType::UnexpectedToken)?;

        let token = next()?;
        let i: Number = number!(token);

        let token = next()?;
        expect(&token, RIGHT_PAREN, ErrorType::UnexpectedToken)?;

        match &i {
            Number::U8(i) => Ok(Color::Ansi256(Ansi256(*i, n))),
            Number::PlaceHolder(_) => Ok(Color::String(format!("{n};5;{i};"))),
        }
    }

    pub fn four_bit(n: u8) -> Color {
        Color::Ansi4Bit(Ansi4Bit(n))
    }

    pub fn fg_fixed(n: u8) -> Color {
        Color::Ansi256(Ansi256(n, 38))
    }

    pub fn bg_fixed(n: u8) -> Color {
        Color::Ansi256(Ansi256(n, 48))
    }

    pub fn fg_rgb(r: u8, g: u8, b: u8) -> Color {
        Color::Rgb(Rgb(r, g, b, 38))
    }

    pub fn bg_rgb(r: u8, g: u8, b: u8) -> Color {
        Color::Rgb(Rgb(r, g, b, 48))
    }

    pub fn fg_hex(h: &str) -> Color {
        Color::parse_hex(
            &Token {
                r#type: TokenType::NUMBER,
                lexeme: format!("f{h}"),
                literal: None,
                span: Span::inserted(),
            },
            38,
        )
    }

    pub fn bg_hex(h: &str) -> Color {
        Color::parse_hex(
            &Token {
                r#type: TokenType::NUMBER,
                lexeme: format!("b{h}"),
                literal: None,
                span: Span::inserted(),
            },
            48,
        )
    }

    pub fn is_empty(&self) -> bool {
        self.to_string().is_empty()
    }
}

impl TryFrom<(String, Span)> for Color {
    type Error = crate::error::Error;

    fn try_from(source: (String, Span)) -> Result<Self, Self::Error> {
        if source.0.is_empty() {
            return Ok(Color::String(source.0));
        }
        let mut scanner = Scanner::new(source.0, source.1);
        let mut tokens: VecDeque<_> = scanner.scan_tokens().into();
        //println!("{:?}", tokens);
        //let line = tokens[0].line;
        let mut next = || {
            if tokens.is_empty() {
                return Err(Error::new(
                    ErrorType::UnexpectedEof,
                    "Unexpected end of input".to_string(),
                    Span::default(), // TODO: sapn
                ));
            }

            Ok(tokens.pop_front().unwrap())
        };

        let token = next()?;
        let color = match token.r#type {
            token::TokenType::FG_BLACK => Color::four_bit(30),
            token::TokenType::FG_RED => Color::four_bit(31),
            token::TokenType::FG_GREEN => Color::four_bit(32),
            token::TokenType::FG_YELLOW => Color::four_bit(33),
            token::TokenType::FG_BLUE => Color::four_bit(34),
            token::TokenType::FG_MAGENTA => Color::four_bit(35),
            token::TokenType::FG_CYAN => Color::four_bit(36),
            token::TokenType::FG_WHITE => Color::four_bit(37),
            token::TokenType::FG_RGB => Color::parse_rgb(next, 38)?,
            token::TokenType::FG_HEX => Color::parse_hex(&token, 38),
            token::TokenType::FG_FIXED => Color::parsed_fixed(next, 38)?,
            token::TokenType::FG_DEFAULT => Color::four_bit(39),

            token::TokenType::BG_BLACK => Color::four_bit(40),
            token::TokenType::BG_RED => Color::four_bit(41),
            token::TokenType::BG_GREEN => Color::four_bit(42),
            token::TokenType::BG_YELLOW => Color::four_bit(43),
            token::TokenType::BG_BLUE => Color::four_bit(44),
            token::TokenType::BG_MAGENTA => Color::four_bit(45),
            token::TokenType::BG_CYAN => Color::four_bit(46),
            token::TokenType::BG_WHITE => Color::four_bit(47),
            token::TokenType::BG_RGB => Color::parse_rgb(next, 48)?,
            token::TokenType::BG_HEX => Color::parse_hex(&token, 48),
            token::TokenType::BG_FIXED => Color::parsed_fixed(next, 38)?,
            token::TokenType::BG_DEFAULT => Color::four_bit(49),
            _ => {
                return Err(Error::new(
                    ErrorType::InvalidColor,
                    format!("{:?} is not a valid color", token.lexeme),
                    token.span,
                ));
            }
        };

        Ok(color)
    }
}

impl From<Color> for String {
    fn from(color: Color) -> Self {
        color.to_string()
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        if !rhs.is_empty() {
            *self = rhs
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Ansi256(ansi256) => ansi256.fmt(f),
            Color::Ansi4Bit(ansi4_bit) => ansi4_bit.fmt(f),
            Color::Rgb(rgb) => rgb.fmt(f),
            Color::String(s) => s.fmt(f),
        }
    }
}

fn expect(token: &Token, expected: token::TokenType, error: ErrorType) -> Result<(), Error> {
    if token.r#type == expected {
        Ok(())
    } else {
        Err(Error::new(
            error,
            format!("Expected {:?}, but found {:?}", expected, token.r#type),
            token.span,
        ))
    }
}
