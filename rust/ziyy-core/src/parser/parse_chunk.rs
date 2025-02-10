use crate::{
    assign_prop_bool, assign_prop_color, assign_prop_cond, assign_prop_value, char_from_u32,
    color::{channel::Channel, number::Number, Color},
    error::ErrorKind,
    get_num,
    num::str_to_u32,
    own,
    scanner::token::TokenKind,
    style::{Condition, Style},
    Error,
};

use super::{
    expect_token,
    tag::{Tag, TagName, TagType, Value},
    Parser,
};

/* const KEYWORDS: &[&str] = &[
    "a", "b", "blink", "br", "c", "dim", "e", "h", "i", "invert", "let", "p", "u", "uu", "x",
    "ziyy",
]; */

#[derive(Debug, PartialEq, Clone)]
pub enum Chunk {
    Comment(String),
    Eof,
    Escape(char),
    Tag(Tag),
    Text(String),
    WhiteSpace(String),
}

impl<T: AsRef<str>> Parser<T> {
    #[allow(clippy::too_many_lines)]
    pub(super) fn parse_chunk(&mut self) -> Result<Chunk, Error> {
        if let Some(chunk) = self.next_chunk.clone() {
            self.next_chunk = None;
            return Ok(chunk);
        }

        let token = self.scanner.scan_token()?;
        let r#type = match token.kind {
            TokenKind::Less => TagType::Open,
            TokenKind::LessSlash => TagType::Close,

            _ => {
                return match token.kind {
                    TokenKind::Comment => Ok(Chunk::Comment(own!(token.content))),
                    TokenKind::Text => Ok(Chunk::Text(own!(token.content))),
                    TokenKind::PlaceHolder => Ok(Chunk::Text(
                        Number::PlaceHolder(own!(token.content), token.custom).to_string(),
                    )),
                    TokenKind::WhiteSpace => Ok(Chunk::WhiteSpace(own!(token.content))),
                    TokenKind::Eof => Ok(Chunk::Eof),
                    TokenKind::Esc0 => char_from_u32!(&token.content[2..], 8, &token),
                    TokenKind::EscX | TokenKind::EscU => {
                        char_from_u32!(&token.content[2..], 16, &token)
                    }
                    TokenKind::EscA => Ok(Chunk::Escape(7 as char)),
                    TokenKind::EscB => Ok(Chunk::Escape(8 as char)),
                    TokenKind::EscT => Ok(Chunk::Escape(9 as char)),
                    TokenKind::EscN => Ok(Chunk::Escape(10 as char)),
                    TokenKind::EscV => Ok(Chunk::Escape(11 as char)),
                    TokenKind::EscF => Ok(Chunk::Escape(12 as char)),
                    TokenKind::EscR => Ok(Chunk::Escape(13 as char)),
                    TokenKind::EscE => Ok(Chunk::Escape(27 as char)),
                    TokenKind::EscBackSlash => Ok(Chunk::Escape('\\')),
                    TokenKind::EscLess => Ok(Chunk::Escape('<')),
                    TokenKind::EscGreat => Ok(Chunk::Escape('>')),

                    _ => Err(Error::new(
                        ErrorKind::UnexpectedToken(token.kind, None),
                        &token,
                    )),
                };
            }
        };

        let token = self.scanner.scan_token()?;
        expect_token(&token, TokenKind::Identifier)?;
        let mut style = Style::new();
        let tag_name = match_tag_name(token.content, &mut style);

        let mut tag = Tag::new(tag_name.clone(), r#type);
        tag.span.add(&token.span);

        let mut token = self.scanner.scan_token()?;
        tag.span.add(&token.span);
        while token.kind == TokenKind::Identifier {
            match token.content {
                // styles
                "b" | "bold" => {
                    assign_prop_cond!(style, brightness, Condition::A, self.scanner, token);
                }
                "d" | "dim" => {
                    assign_prop_cond!(style, brightness, Condition::B, self.scanner, token);
                }
                "i" | "italics" => assign_prop_bool!(style, italics, self.scanner, token),
                "u" | "underline" => {
                    assign_prop_cond!(style, under, Condition::A, self.scanner, token);
                }
                "k" | "blink" => assign_prop_bool!(style, blink, self.scanner, token),
                "r" | "invert" | "reverse" => {
                    assign_prop_bool!(style, invert, self.scanner, token);
                }
                "h" | "hidden" | "hide" | "invisible" => {
                    assign_prop_bool!(style, hide, self.scanner, token);
                }
                "s" | "strike-through" => assign_prop_bool!(style, strike, self.scanner, token),
                "uu" | "double-underline" => {
                    assign_prop_cond!(style, under, Condition::B, self.scanner, token);
                }
                "double" => {
                    if tag_name == TagName::U {
                        assign_prop_cond!(style, under, Condition::B, self.scanner, token);
                    }
                    // TODO: consume equals to and string if any
                }

                // colors
                "c" | "fg" => {
                    assign_prop_color!(tag, style, fg_color, Foreground, self.scanner, token);
                }
                "x" | "bg" => {
                    assign_prop_color!(tag, style, bg_color, Background, self.scanner, token);
                }
                "black" | "blue" | "cyan" | "green" | "magenta" | "red" | "white" | "yellow" => {
                    if tag_name == TagName::C {
                        style.fg_color = Some(Color::try_from((
                            token.content,
                            Channel::Foreground,
                            token.span.clone(),
                        ))?);
                    } else if tag_name == TagName::X {
                        style.bg_color = Some(Color::try_from((
                            token.content,
                            Channel::Background,
                            token.span.clone(),
                        ))?);
                    }

                    token = self.scanner.scan_token()?;
                    tag.span.add(&token.span);
                    if token.kind == TokenKind::Equal {
                        token = self.scanner.scan_token()?;
                        tag.span.add(&token.span);
                        expect_token(&token, TokenKind::String)?;
                    }
                }
                "byte" => {
                    token = self.scanner.scan_token()?;
                    tag.span.add(&token.span);
                    if token.kind == TokenKind::Equal {
                        token = self.scanner.scan_token()?;
                        tag.span.add(&token.span);
                        expect_token(&token, TokenKind::String)?;
                    }

                    let end = token.content.len() - 1;
                    let s = format!("byte({})", &token.content[1..end]);
                    if tag_name == TagName::C {
                        style.fg_color = Some(Color::try_from((
                            s.as_str(),
                            Channel::Foreground,
                            token.span.clone(),
                        ))?);
                    } else if tag_name == TagName::X {
                        style.bg_color = Some(Color::try_from((
                            s.as_str(),
                            Channel::Background,
                            token.span.clone(),
                        ))?);
                    }
                    token = self.scanner.scan_token()?;
                }
                "rgb" => {
                    token = self.scanner.scan_token()?;
                    tag.span.add(&token.span);
                    if token.kind == TokenKind::Equal {
                        token = self.scanner.scan_token()?;
                        tag.span.add(&token.span);
                        expect_token(&token, TokenKind::String)?;
                    }

                    let end = token.content.len() - 1;
                    let s = format!("rgb({})", &token.content[1..end]);
                    if tag_name == TagName::C {
                        style.fg_color = Some(Color::try_from((
                            s.as_str(),
                            Channel::Foreground,
                            token.span.clone(),
                        ))?);
                    } else if tag_name == TagName::X {
                        style.bg_color = Some(Color::try_from((
                            s.as_str(),
                            Channel::Background,
                            token.span.clone(),
                        ))?);
                    }
                    token = self.scanner.scan_token()?;
                }

                // custom
                "n" => {
                    // number of newlines to insert
                    if tag_name == TagName::Br {
                        assign_prop_value!(tag, custom, self.scanner, token);
                    }
                }
                "href" => {
                    // url of link
                    if tag_name == TagName::A {
                        assign_prop_value!(tag, custom, self.scanner, token);
                    }
                }
                "name" => {
                    // name of binding to declare
                    if tag_name == TagName::Let {
                        assign_prop_value!(tag, custom, self.scanner, token);
                    }
                }
                "tab" => {
                    // number of spaces to insert before a paragraph/ a tab if Value::Bool
                    if tag_name == TagName::P {
                        assign_prop_value!(tag, custom, self.scanner, token);
                    }
                }

                // inherit properties from binding with name
                "src" => assign_prop_value!(tag, src, self.scanner, token),

                // ignore unknown properties
                _ => {
                    token = self.scanner.scan_token()?;
                    tag.span.add(&token.span);
                    if token.kind == TokenKind::Equal {
                        token = self.scanner.scan_token()?;
                        tag.span.add(&token.span);
                        expect_token(&token, TokenKind::String)?;

                        token = self.scanner.scan_token()?;
                        tag.span.add(&token.span);
                    }
                }
            }
        }

        tag.style = style;

        match token.kind {
            TokenKind::Great => {}
            TokenKind::SlashGreat => {
                tag.r#type = TagType::OpenAndClose;
            }

            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedToken(token.kind, None),
                    &token,
                ));
            }
        }

        Ok(Chunk::Tag(tag))
    }

    pub(super) fn parse_next_chunk(&mut self) -> Result<Chunk, Error> {
        let chunk = self.parse_chunk()?;
        self.next_chunk = Some(chunk.clone());
        Ok(chunk)
    }
}

fn match_tag_name(text: &str, style: &mut Style) -> TagName {
    match text {
        // styles
        "b" => {
            style.brightness = Condition::A;
            TagName::B
        }
        "d" => {
            style.brightness = Condition::B;
            TagName::D
        }
        "i" => {
            style.italics = true;
            TagName::I
        }
        "u" => {
            style.under = Condition::A;
            TagName::U
        }
        "k" => {
            style.blink = true;
            TagName::K
        }
        "r" => {
            style.invert = true;
            TagName::R
        }
        "h" => {
            style.hide = true;
            TagName::H
        }
        "s" => {
            style.strike = true;
            TagName::S
        }

        "a" => TagName::A,
        "br" => TagName::Br,
        "c" => TagName::C,
        "e" => TagName::E,
        "let" => TagName::Let,
        "p" => TagName::P,
        "x" => TagName::X,
        "ziyy" => TagName::Ziyy,
        _ => TagName::Any(String::from(text)),
    }
}
