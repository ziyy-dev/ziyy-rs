use crate::error::{Error, ErrorType};
use crate::{scanner::GenericScanner, splitter::fragment::Fragment};
use scanner::Scanner;
use std::collections::VecDeque;
use tag::{Tag, TagType};
use token::{Token, TokenType::*};

use super::ansi::{DuoEffect, Effect};
use super::color::Color;

mod scanner;
pub mod tag;
mod token;

macro_rules! consume_declaration {
    ( $tag:expr, $next:expr, $token:expr ) => {{
        $token = $next()?;
        if $token.r#type == EQUAL {
            $token = $next()?;
            expect(&$token, STRING, ErrorType::InvalidTagAttributeValue)?;
            $token = $next()?;
        }
    }};
}

macro_rules! assign_prop {
    ( $tag:expr, $set_prop:tt, $next:expr, $token:expr ) => {{
        $token = $next()?;
        if $token.r#type == EQUAL {
            $token = $next()?;
            expect(&$token, STRING, ErrorType::InvalidTagAttributeValue)?;
            $tag.$set_prop($token.literal.unwrap());
            $token = $next()?;
        }
    }};
}

macro_rules! assign_prop_color {
    ( $tag:expr, $set_prop:tt, $next:expr, $token:expr, $pre:expr ) => {{
        $token = $next()?;
        if $token.r#type == EQUAL {
            $token = $next()?;
            expect(&$token, STRING, ErrorType::InvalidTagAttributeValue)?;
            let color: Color =
                (format!("{}{}", $pre, $token.literal.unwrap()), $token.span).try_into()?;
            $tag.$set_prop(color.into());
            $token = $next()?;
        }
    }};

    ( $tag:expr, $next:expr, $token:expr, $val:expr ) => {{
        let mut i = $val;
        $token = $next()?;
        if $token.r#type == EQUAL {
            $token = $next()?;
            expect(&$token, STRING, ErrorType::InvalidTagAttributeValue)?;
            let s = $token.literal.unwrap();
            if s == "light" {
                i += 60;
            } else if s == "dark" {
            } else {
                // TODO: Throw error
            }

            $token = $next()?;
        }

        if $tag.name() == "c" {
            $tag.set_fg_color(Color::four_bit(30 + i));
        } else if $tag.name() == "x" {
            $tag.set_bg_color(Color::four_bit(40 + i));
        }
    }};
}

macro_rules! assign_prop_effect {
    ( $tag:expr, $next:expr, $token:expr, $set_prop:tt ) => {{
        $token = $next()?;
        if $token.r#type == EQUAL {
            $token = $next()?;
            expect(&$token, STRING, ErrorType::InvalidTagAttributeValue)?;
            let s = $token.literal.unwrap();
            if s == "false" {
                $tag.$set_prop(Effect::Clear);
            } else if s == "true" {
                $tag.$set_prop(Effect::Apply);
            } else {
                // TODO: Throw error
            }

            $token = $next()?;
        } else {
            $tag.$set_prop(Effect::Apply);
        }
    }};
}

macro_rules! assign_prop_duoeffect {
    ( $tag:expr, $next:expr, $token:expr, $set_prop:tt, $val:expr, $clear:expr ) => {{
        $token = $next()?;
        if $token.r#type == EQUAL {
            $token = $next()?;
            expect(&$token, STRING, ErrorType::InvalidTagAttributeValue)?;
            let s = $token.literal.unwrap();
            if s == "false" {
                $tag.$set_prop($clear);
            } else if s == "true" {
                $tag.$set_prop($val);
            } else {
                // TODO: Throw error
            }

            $token = $next()?;
        } else {
            $tag.$set_prop($val);
        }
    }};
}

pub struct TagParser {
    #[allow(dead_code)]
    parse_placeholders: bool,
}

impl Default for TagParser {
    fn default() -> Self {
        Self::new(true)
    }
}

impl TagParser {
    pub fn new(parse_placeholders: bool) -> Self {
        Self { parse_placeholders }
    }

    pub fn parse(&mut self, source: Fragment) -> Result<Tag, Error> {
        let mut scanner = Scanner::new(source.clone());
        let tokens = scanner.scan_tokens();
        let open = &tokens[0].r#type;
        let close = &tokens[tokens.len() - 1].r#type;

        let tag_type = match (open, close) {
            (LESS, GREATER) => TagType::Open,
            (LESS, SLASH_GREATER) => TagType::SelfClose,
            (LESS_SLASH, GREATER) => TagType::Close,
            _ => {
                return Err(Error::new(
                    ErrorType::InvalidTag,
                    format!("Invalid tag: {open:?} {close:?}"),
                    tokens[0].span,
                ));
            }
        };
        let span = tokens[0].span;
        let mut tokens: VecDeque<_> = tokens[1..tokens.len()].to_vec().into();

        let mut next = || {
            if tokens.is_empty() {
                return Err(Error::new(
                    ErrorType::InvalidTag,
                    "Unexpected end of input".to_string(),
                    span,
                ));
            }

            Ok(tokens.pop_front().unwrap())
        };

        let token = next()?;
        expect(&token, IDENTIFIER, ErrorType::InvalidTagName)?;

        let mut tag = Tag::default();
        tag.r#type = tag_type;
        tag.set_name(token.lexeme.clone());

        match tag.name().to_lowercase().as_str() {
            "b" | "strong" => {
                tag.set_brightness(DuoEffect::A);
            }
            "d" | "dim" => {
                tag.set_brightness(DuoEffect::B);
            }
            "i" | "em" => {
                tag.set_italics(Effect::Apply);
            }
            "u" | "ins" => {
                tag.set_under(DuoEffect::A);
            }
            "uu" => {
                tag.set_under(DuoEffect::B);
            }
            "k" | "blink" => {
                tag.set_blink(Effect::Apply);
            }
            "r" => {
                tag.set_negative(Effect::Apply);
            }
            "h" => {
                tag.set_hidden(Effect::Apply);
            }
            "s" | "del" => {
                tag.set_strike(Effect::Apply);
            }
            _ => {}
        }

        let mut token = next()?;
        while token.r#type == IDENTIFIER {
            match token.lexeme.as_str() {
                "b" | "bold" => {
                    assign_prop_duoeffect!(
                        tag,
                        next,
                        token,
                        set_brightness,
                        DuoEffect::A,
                        DuoEffect::AE
                    )
                }
                "d" | "dim" => {
                    assign_prop_duoeffect!(
                        tag,
                        next,
                        token,
                        set_brightness,
                        DuoEffect::B,
                        DuoEffect::BE
                    )
                }
                "k" | "blink" => assign_prop_effect!(tag, next, token, set_blink),
                "h" | "hidden" | "hide" | "invisible" => {
                    assign_prop_effect!(tag, next, token, set_hidden)
                }
                "s" | "strike" | "strike-through" => {
                    assign_prop_effect!(tag, next, token, set_strike)
                }
                "i" | "italics" => {
                    assign_prop_effect!(tag, next, token, set_italics)
                }
                "r" | "invert" | "reverse" | "negative" => {
                    assign_prop_effect!(tag, next, token, set_negative)
                }
                "u" | "under" | "underline" => {
                    assign_prop_duoeffect!(tag, next, token, set_under, DuoEffect::A, DuoEffect::AE)
                }
                "uu" | "double-under" | "double-underline" => {
                    assign_prop_duoeffect!(tag, next, token, set_under, DuoEffect::B, DuoEffect::BE)
                }

                "c" | "fg" => assign_prop_color!(tag, set_fg_color, next, token, "f"),

                "x" | "bg" => assign_prop_color!(tag, set_bg_color, next, token, "b"),
                "black" => assign_prop_color!(tag, next, token, 0),
                "red" => assign_prop_color!(tag, next, token, 1),
                "green" => assign_prop_color!(tag, next, token, 2),
                "yellow" => assign_prop_color!(tag, next, token, 3),
                "blue" => assign_prop_color!(tag, next, token, 4),
                "magenta" => assign_prop_color!(tag, next, token, 5),
                "cyan" => assign_prop_color!(tag, next, token, 6),
                "white" => assign_prop_color!(tag, next, token, 7),
                "none" => assign_prop_color!(tag, next, token, 9),
                "fixed" => {
                    token = next()?;
                    let token2: Token;
                    if token.r#type == EQUAL {
                        token = next()?;
                        expect(&token, STRING, ErrorType::InvalidTagAttributeValue)?;
                        token2 = token;
                        token = next()?;
                    } else {
                        continue;
                    }

                    let color = |pre: &str| -> Result<_, _> {
                        let c: Color = (
                            format!("{pre}fixed({})", token2.literal.unwrap()),
                            // move start of span back by 7 columns due to inserted text ffixed( or bfixed( to preserve span of color in string
                            // fixed = "..."
                            // ^^^^^ ^ ^
                            // ||||| | |
                            // 12345 6 7
                            token2.span.unquote() - (0, 7),
                        )
                            .try_into()?;
                        Ok(c)
                    };

                    if tag.name() == "c" {
                        tag.set_fg_color(color("f")?);
                    } else if tag.name() == "x" {
                        tag.set_bg_color(color("b")?);
                    }
                }
                "rgb" => {
                    token = next()?;
                    let token2: Token;
                    if token.r#type == EQUAL {
                        token = next()?;
                        expect(&token, STRING, ErrorType::InvalidTagAttributeValue)?;
                        token2 = token;
                        token = next()?;
                    } else {
                        continue;
                    }

                    let color = |pre: &str| -> Result<_, _> {
                        let c: Color = (
                            format!("{pre}rgb({})", token2.literal.unwrap()),
                            // move start of span back by 5 columns due to inserted text frgb( or brgb( to preserve span of color in string
                            // rgb = "..."
                            // ^^^ ^ ^
                            // ||| | |
                            // 123 4 5
                            token2.span.unquote() - (0, 5),
                        )
                            .try_into()?;
                        Ok(c)
                    };

                    if tag.name() == "c" {
                        tag.set_fg_color(color("f")?);
                    } else if tag.name() == "x" {
                        tag.set_bg_color(color("b")?);
                    }
                }

                "double" => {
                    if matches!(tag.name().as_str(), "u" | "ins") {
                        tag.set_under(DuoEffect::B);

                        token = next()?;
                        if token.r#type == EQUAL {
                            token = next()?;
                            expect(&token, STRING, ErrorType::InvalidTagAttributeValue)?;
                            let s = token.literal.unwrap();
                            if s == "false" {
                                tag.set_under(DuoEffect::A);
                            } else if s == "true" {
                            } else {
                                // TODO: Throw error
                            }

                            token = next()?;
                        }
                    } else {
                        consume_declaration!(tag, next, token);
                    }
                }

                "n" => {
                    if tag.name() == "br" {
                        assign_prop!(tag, set_custom, next, token);
                    } else {
                        consume_declaration!(tag, next, token);
                    }
                }
                "href" => {
                    if tag.name() == "a" {
                        assign_prop!(tag, set_custom, next, token);
                    } else {
                        consume_declaration!(tag, next, token);
                    }
                }
                "id" => {
                    if tag.name() == "let" {
                        assign_prop!(tag, set_custom, next, token);
                    } else {
                        consume_declaration!(tag, next, token);
                    }
                }
                "indent" => {
                    if matches!(tag.name().as_str(), "p" | "table") {
                        assign_prop!(tag, set_custom, next, token);
                    } else {
                        consume_declaration!(tag, next, token);
                    }
                }

                "class" => assign_prop!(tag, set_class, next, token),

                _ => {
                    consume_declaration!(tag, next, token);
                }
            }
        }

        Ok(tag)
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
