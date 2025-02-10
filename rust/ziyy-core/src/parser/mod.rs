use crate::error::ErrorKind;
use crate::scanner::token::Token;
use crate::scanner::token::TokenKind;
use crate::scanner::Scanner;
use crate::style::Condition;
use crate::style::Style;
use crate::Error;
use builtins::BUILTIN_TAGS;
use parse_chunk::Chunk;
use state::State;
use std::collections::HashMap;
use std::io::Write;
pub use tag::{Tag, TagName, TagType};

mod builtins;
mod close_tag;
mod helpers;
mod open_and_close_tag;
mod open_tag;
mod parse_chunk;
mod state;
mod tag;

/// Represents different types of printable elements.
#[derive(PartialEq, Debug, Clone)]
enum Printable {
    /// Represents a text element.
    Text,
    /// Represents a paragraph element.
    Paragraph,
    /// Represents a whitespace element.
    WhiteSpace,
    /// Represents no printable element.
    None,
}

/// A parser for the Ziyy language.
pub struct Parser<T: AsRef<str>> {
    /// The scanner used to tokenize the input source.
    pub(super) scanner: Scanner<T>,
    /// A buffer to store the parsed output.
    pub(crate) buf: Vec<u8>,
    /// Optional bindings for styles.
    pub(crate) bindings: Option<HashMap<String, Style>>,
    /// The current state of the parser.
    state: State,
    /// Flag to indicate whether to skip white space.
    skip_ws: bool,
    /// Flag to indicate whether to style the text exactly as it is.
    exact: bool,
    /// Flag to indicate whether to clear underline.
    clear_under: bool,
    /// The last written printable element.
    last_written: Printable,
    /// The next chunk to be parsed.
    next_chunk: Option<Chunk>,
}

impl<T: AsRef<str>> Parser<T> {
    /// Creates a new Ziyy Parser.
    ///
    /// # Arguments
    ///
    /// * `source` - The source input to be parsed.
    /// * `bindings` - Optional bindings for styles.
    ///
    /// # Returns
    ///
    /// A new instance of `Parser`.
    pub fn new(source: T, bindings: Option<HashMap<String, Style>>) -> Parser<T> {
        Parser {
            scanner: Scanner::new(source),
            buf: vec![],
            bindings,
            state: State::new(),
            skip_ws: true,
            exact: false,
            clear_under: false,

            last_written: Printable::None,
            next_chunk: None,
        }
    }

    /// Parses the source and returns a `Vec<u8>`.
    ///
    /// # Errors
    ///
    /// Returns an [Error] if parsing fails.
    pub fn parse_to_bytes(&mut self) -> Result<Vec<u8>, Error> {
        if !self.exact {
            let _ = write!(self.buf, "\x1b[m");
        }

        loop {
            let parsed = self.parse_chunk()?;
            match parsed {
                Chunk::Comment(_) => {}
                Chunk::Escape(c) => {
                    let _ = self.buf.write(c.to_string().as_bytes());
                    self.skip_ws = false;
                    let is_graphic = match c {
                        ..'!' => false,
                        '!'..='~' => true,
                        '\u{007F}' => false,
                        _ => true,
                    };

                    if is_graphic {
                        self.last_written = Printable::Text;
                    }
                }

                Chunk::Tag(tag) => match tag.r#type {
                    TagType::Open => self.parse_open_tag(tag)?,
                    TagType::Close => self.parse_close_tag(&tag)?,
                    TagType::OpenAndClose => self.parse_open_and_close_tag(tag)?,
                },

                Chunk::Text(text) => {
                    let _ = self.buf.write(text.as_bytes());
                    self.skip_ws = false;
                    self.last_written = Printable::Text;
                }

                Chunk::WhiteSpace(ws) => {
                    let chunk = self.parse_next_chunk()?;
                    if self.exact {
                        if let Chunk::Text(_) = chunk {
                            let _ = self.buf.write(ws.as_bytes());
                        } else {
                            let _ = match self.state.current_style() {
                                Some(style) if style.under.is_some() => {
                                    let _ = self.buf.write(b"\x1b[24m");
                                    let _ = self.buf.write(ws.as_bytes());
                                    match style.under {
                                        Condition::A | Condition::BA => self.buf.write(b"\x1b[4m"),
                                        Condition::B | Condition::AB => self.buf.write(b"\x1b[21m"),
                                        Condition::None => Ok(0),
                                    }
                                }
                                _ => self.buf.write(ws.as_bytes()),
                            };
                        }
                    } else {
                        if !self.skip_ws && self.last_written != Printable::WhiteSpace {
                            if let Chunk::Text(_) = chunk {
                                let _ = self.buf.write(b" ");
                            } else {
                                let _ = match self.state.current_style() {
                                    Some(style) if style.under.is_some() => {
                                        let _ = self.buf.write(b"\x1b[24m");
                                        let _ = self.buf.write(b" ");
                                        match style.under {
                                            Condition::A | Condition::BA => {
                                                self.buf.write(b"\x1b[4m")
                                            }
                                            Condition::B | Condition::AB => {
                                                self.buf.write(b"\x1b[21m")
                                            }
                                            Condition::None => Ok(0),
                                        }
                                    }
                                    _ => self.buf.write(b" "),
                                };
                            }

                            self.last_written = Printable::WhiteSpace;
                        }

                        if let Chunk::Eof = chunk {
                            if ws.contains('\n') {
                                let _ = self.buf.write(b"\n");
                            }
                        }
                    }
                }

                Chunk::Eof => {
                    let _ = write!(self.buf, "\x1b[m");
                    return Ok(self.buf.drain(..).collect::<Vec<_>>());
                }
            }
        }
    }

    /// Parses source and Returns a [String].
    /// # Errors
    ///
    /// Returns an `Error` if parsing fails.
    #[allow(clippy::missing_panics_doc)]
    pub fn parse(&mut self) -> Result<String, Error> {
        let s = String::from_utf8(self.parse_to_bytes()?);
        Ok(s.unwrap())
    }

    /// Checks if the given tag matches the expected tag name.
    ///
    /// # Arguments
    ///
    /// * `tag` - The tag to be checked.
    /// * `to_be` - The expected tag name.
    /// * `err` - The error kind to be returned if the tag does not match.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the tag matches, otherwise returns an `Error`.
    fn expect_tag(tag: &Tag, to_be: &TagName, err: ErrorKind) -> Result<(), Error> {
        if tag.name == *to_be {
            Ok(())
        } else {
            Err(Error {
                kind: err,
                span: tag.span.clone(),
            })
        }
    }

    /// Writes the style to the buffer and saves the current style state.
    ///
    /// # Arguments
    ///
    /// * `tag_name` - The name of the tag.
    /// * `style` - The style to be written and saved.
    fn write_and_save(&mut self, tag_name: &TagName, style: Style) {
        if let Some(prev) = self.state.current_style() {
            let delta = style.sub(prev);
            let _ = self.buf.write(delta.to_string().as_bytes());
            self.state.push(tag_name.clone(), style, delta);
        }
        if *tag_name == TagName::P {
            self.last_written = Printable::Paragraph;
        }
    }
}

/// Checks if the given token matches the expected token kind.
///
/// # Arguments
///
/// * `token` - The token to be checked.
/// * `tt` - The expected token kind.
///
/// # Returns
///
/// Returns `Ok(())` if the token matches, otherwise returns an `Error`.
fn expect_token(token: &Token, tt: TokenKind) -> Result<(), Error> {
    if token.kind != tt {
        return Err(Error::new(
            ErrorKind::UnexpectedToken(token.kind, Some(tt)),
            token,
        ));
    }
    Ok(())
}

/// Inherits boolean style properties from the source style to the destination style.
///
/// # Arguments
///
/// * `$src` - The source style.
/// * `$dst` - The destination style.
/// * `$f` - The boolean field to be inherited.
macro_rules! inherit_style_bool {
    ( $src:expr, $dst:expr, $f:tt ) => {
        if $src.$f && !$dst.$f {
            $dst.$f = true
        }
    };
}

/// Inherits color style properties from the source style to the destination style.
///
/// # Arguments
///
/// * `$src` - The source style.
/// * `$dst` - The destination style.
/// * `$f` - The color field to be inherited.
macro_rules! inherit_style_color {
    ( $src:expr, $dst:expr, $f:tt ) => {
        if $src.$f.is_some() && $dst.$f.is_none() {
            $dst.$f = $src.$f.clone()
        }
    };
}

/// Inherits style properties from the source style to the destination style.
///
/// # Arguments
///
/// * `src` - The source style.
/// * `dst` - The destination style.
fn inherit(src: &Style, dst: &mut Style) {
    //inherit_style_bool!(src, dst, brightness);
    //inherit_style_bool!(src, dst, dim);
    inherit_style_bool!(src, dst, italics);
    //inherit_style_bool!(src, dst, under);
    inherit_style_bool!(src, dst, blink);
    inherit_style_bool!(src, dst, invert);
    inherit_style_bool!(src, dst, hide);
    inherit_style_bool!(src, dst, strike);
    //inherit_style_bool!(src, dst, double_under);

    inherit_style_color!(src, dst, fg_color);
    inherit_style_color!(src, dst, bg_color);
}

#[cfg(test)]
mod tests {
    use crate::{
        color::{bit_4::Bit4, Color, ColorKind},
        scanner::span::Span,
    };

    use super::*;

    #[test]
    fn test_parser_new() {
        let source = "test source";
        let bindings = None;
        let parser = Parser::new(source, bindings);
        assert_eq!(parser.buf, Vec::new());
        assert!(parser.bindings.is_none());
        assert!(parser.state.is_empty());
        assert!(parser.skip_ws);
        assert!(!parser.exact);
        assert!(!parser.clear_under);
        assert_eq!(parser.last_written, Printable::None);
        assert!(parser.next_chunk.is_none());
    }

    #[test]
    fn test_expect_tag() {
        let tag = Tag {
            name: TagName::P,
            r#type: TagType::Open,
            span: Span::default(),
            custom: tag::Value::None,
            style: Style::new(),
            src: tag::Value::None,
        };
        let result = Parser::<&str>::expect_tag(
            &tag,
            &TagName::P,
            ErrorKind::MisMatchedTags(TagName::P, tag.name.clone()),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_expect_tag_error() {
        let tag = Tag {
            name: TagName::B,
            r#type: TagType::Open,
            span: Span::default(),
            custom: tag::Value::None,
            style: Style::new(),
            src: tag::Value::None,
        };

        let result = Parser::<&str>::expect_tag(
            &tag,
            &TagName::P,
            ErrorKind::MisMatchedTags(TagName::P, tag.name.clone()),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_expect_token() {
        let token = Token {
            kind: TokenKind::Text,
            span: Span::default(),
            content: "",
            custom: 0,
        };
        let result = expect_token(&token, TokenKind::Text);
        assert!(result.is_ok());
    }

    #[test]
    fn test_expect_token_error() {
        let token = Token {
            kind: TokenKind::Text,
            span: Span::default(),
            content: "",
            custom: 0,
        };
        let result = expect_token(&token, TokenKind::B);
        assert!(result.is_err());
    }

    #[test]
    fn test_inherit_style() {
        let src = Style {
            italics: true,
            blink: true,
            invert: true,
            hide: true,
            strike: true,
            fg_color: Some(Color::fg(ColorKind::Bit4(Bit4::Red))),
            bg_color: Some(Color::bg(ColorKind::Bit4(Bit4::Black))),
            ..Default::default()
        };
        let mut dst = Style::default();
        inherit(&src, &mut dst);
        assert!(dst.italics);
        assert!(dst.blink);
        assert!(dst.invert);
        assert!(dst.hide);
        assert!(dst.strike);
        assert_eq!(dst.fg_color, Some(Color::fg(ColorKind::Bit4(Bit4::Red))));
        assert_eq!(dst.bg_color, Some(Color::bg(ColorKind::Bit4(Bit4::Black))));
    }

    #[test]
    fn test_parse_to_bytes() {
        let source = "test source";
        let bindings = None;
        let mut parser = Parser::new(source, bindings);
        let result = parser.parse_to_bytes();
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse() {
        let source = "test source";
        let bindings = None;
        let mut parser = Parser::new(source, bindings);
        let result = parser.parse();
        assert!(result.is_ok());
    }
}
