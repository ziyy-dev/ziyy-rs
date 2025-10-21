use crate::number;
use crate::scanner::token::TokenKind;
use crate::scanner::Scanner;
use crate::Error;
use crate::ErrorKind;
use crate::Result;

use super::expect;
use super::ColorKind;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    #[must_use]
    pub fn to_string(&self, kind: ColorKind) -> String {
        let Rgb(r, g, b) = self;
        format!("\x1b[{};2;{r};{g};{b}m", kind as u8 + 8)
    }

    pub(crate) fn parse<'src>(scanner: &mut Scanner<'src>) -> Result<'src, Self> {
        let token = scanner.scan_token()?;
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        match token.kind {
            TokenKind::NUMBER => {
                r = number!(token.content, 10, &token);

                let token = scanner.scan_token()?;
                expect(&token, TokenKind::COMMA)?;

                let token = scanner.scan_token()?;
                g = number!(token.content, 10, &token);

                let token = scanner.scan_token()?;
                expect(&token, TokenKind::COMMA)?;

                let token = scanner.scan_token()?;
                b = number!(token.content, 10, &token);
            }

            TokenKind::HEX => match token.content.len() {
                4 => {
                    /* r = number!(&token.content[1..2].repeat(2), 16, &token);
                    g = number!(&token.content[2..3].repeat(2), 16, &token);
                    b = number!(&token.content[3..4].repeat(2), 16, &token); */
                }

                7 => {
                    r = number!(&token.content[1..3], 16, &token);
                    g = number!(&token.content[3..5], 16, &token);
                    b = number!(&token.content[5..7], 16, &token);
                }

                _ => {}
            },

            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedToken {
                        expected: TokenKind::NUMBER,
                        found: Some(token.content),
                    },
                    &token,
                ))
            }
        }

        Ok(Rgb(r, g, b))
    }
}
