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

pub static WORD_PARSER: WordParser = WordParser;

macro_rules! shrink {
    ($num:expr) => {{
        if $num > 255.0 {
            255u8
        } else if $num < 0.0 {
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
        let mut parts = source
            .split(';')
            .map(|x| x.replace("4:", "4.1"))
            .map(|x| {
                if x.is_empty() {
                    0.0
                } else {
                    x.parse::<f64>().unwrap_or(-1.0)
                }
            })
            .peekable();

        let mut tag = Tag::default();
        loop {
            let num = parts.next();

            let num = match num {
                Some(n) => n,
                None => break,
            };

            match num {
                0.0 => tag.clear_all(),

                1.0 => {
                    tag.set_brightness(DuoEffect::A);
                }
                2.0 => {
                    tag.set_brightness(DuoEffect::B);
                }
                22.0 => {
                    let num = parts.peek();
                    if let Some(num) = num {
                        tag.set_brightness(match num {
                            1.0 => {
                                parts.next();
                                DuoEffect::BA
                            }
                            2.0 => {
                                parts.next();
                                DuoEffect::AB
                            }
                            _ => DuoEffect::E,
                        });
                    } else {
                        tag.set_brightness(DuoEffect::E);
                    }
                }

                3.0 => {
                    tag.set_italics(Effect::Apply);
                }
                23.0 => {
                    tag.set_italics(Effect::Clear);
                }

                4.0 => {
                    tag.set_under(DuoEffect::A);
                }
                21.0 => {
                    tag.set_under(DuoEffect::B);
                }
                24.0 => {
                    let num = parts.peek();
                    if let Some(num) = num {
                        tag.set_under(match num {
                            4.0 => {
                                parts.next();
                                DuoEffect::BA
                            }
                            21.0 => {
                                parts.next();
                                DuoEffect::AB
                            }
                            _ => DuoEffect::E,
                        });
                    } else {
                        tag.set_under(DuoEffect::E);
                    }
                }

                5.0 => {
                    tag.set_blink(Effect::Apply);
                }
                25.0 => {
                    tag.set_blink(Effect::Clear);
                }

                7.0 => {
                    tag.set_negative(Effect::Apply);
                }
                27.0 => {
                    tag.set_negative(Effect::Clear);
                }

                8.0 => {
                    tag.set_hidden(Effect::Apply);
                }
                28.0 => {
                    tag.set_hidden(Effect::Clear);
                }

                9.0 => {
                    tag.set_strike(Effect::Apply);
                }
                29.0 => {
                    tag.set_strike(Effect::Clear);
                }

                30.0..=37.0 | 39.0 | 90.0..=97.0 => tag.set_fg_color(Color::four_bit(shrink!(num))),
                40.0..=47.0 | 49.0 | 100.0..=107.0 => {
                    tag.set_bg_color(Color::four_bit(shrink!(num)))
                }

                38.0 => {
                    let num = parts.next().ok_or(0)?;
                    if num == 2.0 {
                        let r = parts.next().ok_or(0)?;
                        let g = parts.next().ok_or(0)?;
                        let b = parts.next().ok_or(0)?;
                        tag.set_fg_color(Color::Rgb(Rgb(38, shrink!(r), shrink!(g), shrink!(b))));
                    }

                    if num == 5.0 {
                        let fixed = parts.next().ok_or(0)?;
                        tag.set_fg_color(Color::Ansi256(Ansi256(38, shrink!(fixed))));
                    }
                }

                48.0 => {
                    let num = parts.next().ok_or(0)?;
                    if num == 2.0 {
                        let r = parts.next().ok_or(0)?;
                        let g = parts.next().ok_or(0)?;
                        let b = parts.next().ok_or(0)?;
                        tag.set_fg_color(Color::Rgb(Rgb(48, shrink!(r), shrink!(g), shrink!(b))));
                    }
                    if num == 5.0 {
                        let fixed = parts.next().ok_or(0)?;
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
