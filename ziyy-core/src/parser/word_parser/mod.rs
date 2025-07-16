use std::borrow::Cow;

use super::ansi::{DuoEffect, Effect};
use super::chunk::{Chunk, ChunkData};
use super::color::{Ansi256, Color, Rgb};
use super::tag_parser::tag::{Tag, TagType};
use crate::common::Span;
use crate::error::Error;
use crate::scanner::GenericScanner;
use crate::splitter::fragment::Fragment;
use scanner::Scanner;
use token::Token;
mod scanner;
mod token;

macro_rules! shrink {
    ($num:expr) => {{
        if $num > 255 {
            255u8
        } else if $num < 0 {
            0u8
        } else {
            $num as u8
        }
    }};
}

pub struct WordParser;

impl Default for WordParser {
    fn default() -> Self {
        Self::new()
    }
}

impl WordParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse<'a>(&self, source: Fragment<'a>) -> Vec<Result<Chunk<'a>, Error>> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        let mut chunks = vec![];

        let mut i = 0;
        let len = tokens.len();

        loop {
            if i >= len {
                break;
            }

            let c = tokens[i].literal;

            if c == '\x1b' && tokens[i + 1].literal == '[' {
                let g = i;
                i += 2;
                // Handle escape
                let h = i;

                if !matches!(tokens[i].literal, '\x30'..='\x39' | '\x3b' | '\x40'..='\x7e') {
                    while i < len && tokens[i].literal != '\x1b' {
                        i += 1;
                    }
                    let word = tokens[g..i].to_string();
                    chunks.push(Ok(Chunk {
                        data: ChunkData::Word(Cow::Owned(word)),
                        span: tokens[g..i].to_span(),
                    }));

                    break;
                }

                while i < len && !matches!(tokens[i].literal, '\x40'..='\x7e') {
                    i += 1;
                }

                if tokens[i].literal == 'm' {
                    // Handle escape sequence
                    let escape_sequence = tokens[h..i].to_string();

                    if let Ok(tag) = self.ansi_to_tag(escape_sequence) {
                        chunks.push(Ok(Chunk {
                            data: ChunkData::Tag(tag),
                            span: tokens[g..i].to_span(),
                        }));
                    }
                } else {
                    while i < len && tokens[i].literal != '\x1b' {
                        i += 1;
                    }
                    let word = tokens[h..i].to_string();
                    chunks.push(Ok(Chunk {
                        data: ChunkData::Word(Cow::Owned(word)),
                        span: tokens[h..i].to_span(),
                    }));
                }
                i += 1;

                continue;
            } else {
                let h = i;
                while i < len && tokens[i].literal != '\x1b' {
                    i += 1;
                }
                let word = tokens[h..i].to_string();
                chunks.push(Ok(Chunk {
                    data: ChunkData::Word(Cow::Owned(word)),
                    span: tokens[h..i].to_span(),
                }))
            }
            // Handle normal character
            // i += 1
        }

        chunks
    }

    fn ansi_to_tag(&self, source: String) -> Result<Tag, i8> {
        // Convert ANSI escape codes to tags
        let parts = source.split(';');

        let mut segments = Vec::with_capacity(10);
        for part in parts {
            if part.is_empty() {
                segments.push(0);
            } else {
                segments.push(part.parse::<i32>().map_err(|_| 0)?);
            }
        }

        let mut parts = segments.iter().peekable();

        let mut tag = Tag::default();
        loop {
            let num = parts.next();

            let num = match num {
                Some(n) => *n,
                None => break,
            };

            match num {
                0 => tag.clear_all(),

                1 => {
                    tag.set_brightness(DuoEffect::A);
                }
                2 => {
                    tag.set_brightness(DuoEffect::B);
                }
                22 => {
                    let num = parts.peek();
                    if let Some(num) = num {
                        tag.set_brightness(match num {
                            1 => {
                                parts.next();
                                DuoEffect::BA
                            }
                            2 => {
                                parts.next();
                                DuoEffect::AB
                            }
                            _ => DuoEffect::E,
                        });
                    } else {
                        tag.set_brightness(DuoEffect::E);
                    }
                }

                3 => {
                    tag.set_italics(Effect::Apply);
                }
                23 => {
                    tag.set_italics(Effect::Clear);
                }

                4 => {
                    tag.set_under(DuoEffect::A);
                }
                21 => {
                    tag.set_under(DuoEffect::B);
                }
                24 => {
                    let num = parts.peek();
                    if let Some(num) = num {
                        tag.set_under(match num {
                            4 => {
                                parts.next();
                                DuoEffect::BA
                            }
                            21 => {
                                parts.next();
                                DuoEffect::AB
                            }
                            _ => DuoEffect::E,
                        });
                    } else {
                        tag.set_under(DuoEffect::E);
                    }
                }

                5 => {
                    tag.set_blink(Effect::Apply);
                }
                25 => {
                    tag.set_blink(Effect::Clear);
                }

                7 => {
                    tag.set_negative(Effect::Apply);
                }
                27 => {
                    tag.set_negative(Effect::Clear);
                }

                8 => {
                    tag.set_hidden(Effect::Apply);
                }
                28 => {
                    tag.set_hidden(Effect::Clear);
                }

                9 => {
                    tag.set_strike(Effect::Apply);
                }
                29 => {
                    tag.set_strike(Effect::Clear);
                }

                30..=37 | 39 | 90..=97 => tag.set_fg_color(Color::four_bit(shrink!(num))),
                40..=47 | 49 | 100..=107 => tag.set_bg_color(Color::four_bit(shrink!(num))),

                38 => {
                    let num = *parts.next().ok_or(0)?;
                    if num == 2 {
                        let r = *parts.next().ok_or(0)?;
                        let g = *parts.next().ok_or(0)?;
                        let b = *parts.next().ok_or(0)?;
                        tag.set_fg_color(Color::Rgb(Rgb(38, shrink!(r), shrink!(g), shrink!(b))));
                    }

                    if num == 5 {
                        let fixed = *parts.next().ok_or(0)?;
                        tag.set_fg_color(Color::Ansi256(Ansi256(38, shrink!(fixed))));
                    }
                }

                48 => {
                    let num = *parts.next().ok_or(0)?;
                    if num == 2 {
                        let r = *parts.next().ok_or(0)?;
                        let g = *parts.next().ok_or(0)?;
                        let b = *parts.next().ok_or(0)?;
                        tag.set_fg_color(Color::Rgb(Rgb(48, shrink!(r), shrink!(g), shrink!(b))));
                    }
                    if num == 5 {
                        let fixed = *parts.next().ok_or(0)?;
                        tag.set_fg_color(Color::Ansi256(Ansi256(48, shrink!(fixed))));
                    }
                }
                _ => {}
            }
        }

        tag.set_name("$ansi".to_string());
        tag.r#type = TagType::Open;

        Ok(tag)
    }
}

trait Transform {
    fn to_string(&self) -> String;
    fn to_span(&self) -> Span;
}

impl Transform for [Token] {
    fn to_string(&self) -> String {
        let mut text = String::with_capacity(self.len());

        for token in self {
            text.push(token.literal)
        }

        text
    }

    fn to_span(&self) -> Span {
        let mut span = Span::inserted();
        for token in self {
            if span == Span::inserted() {
                span = token.span;
            } else {
                span += token.span;
            }
        }

        span
    }
}
