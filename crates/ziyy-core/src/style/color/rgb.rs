use crate::error::{Error, ErrorKind, Result};
use crate::num::input_to_u8;
use crate::scanner::TokenKind;
use crate::scanner::{Scanner, Token};
use crate::shared::Input;

use super::ColorKind;
use super::expect;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    #[must_use]
    #[inline]
    pub fn to_string(&self, kind: ColorKind) -> String {
        let Rgb(r, g, b) = self;
        format!("\x1b[{};2;{r};{g};{b}m", kind as u8 + 8)
    }

    pub(crate) fn parse<'src, I: ?Sized + Input>(
        scanner: &mut Scanner<'src, I>,
    ) -> Result<'src, I, Self> {
        let token = scanner.scan_token()?;
        let r = number!(token.content, 10, &token);

        let token = scanner.scan_token()?;
        expect(&token, TokenKind::COMMA)?;

        let token = scanner.scan_token()?;
        let g = number!(token.content, 10, &token);

        let token = scanner.scan_token()?;
        expect(&token, TokenKind::COMMA)?;

        let token = scanner.scan_token()?;
        let b = number!(token.content, 10, &token);

        Ok(Rgb(r, g, b))
    }

    pub(crate) fn parse_hex<'src, I: ?Sized + Input>(
        token: &Token<'src, I>,
    ) -> Result<'src, I, Self> {
        let r;
        let g;
        let b;

        match token.content.as_ref().len() {
            4 => {
                r = hex!(token.content[1..2].as_ref().repeat(2).as_slice());
                g = hex!(token.content[2..3].as_ref().repeat(2).as_slice());
                b = hex!(token.content[3..4].as_ref().repeat(2).as_slice());
            }

            7 => {
                r = hex!(&token.content[1..3]);
                g = hex!(&token.content[3..5]);
                b = hex!(&token.content[5..7]);
            }

            _ => {
                return Err(Error {
                    kind: ErrorKind::InvalidColor(token.content),
                    span: token.span,
                });
            }
        }

        Ok(Rgb(r, g, b))
    }
}
