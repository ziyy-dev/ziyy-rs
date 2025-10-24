use crate::document::Document;
use crate::error::ErrorKind;
use crate::scanner::is_whitespace;
use crate::scanner::token::Token;
use crate::scanner::token::TokenKind;
use crate::scanner::Scanner;
use crate::style::Style;
use crate::Error;
use builtins::BUILTIN_STYLES;
pub use parse_chunk::Chunk;
use state::State;
use std::collections::HashMap;
use std::mem::take;
pub use tag::{Tag, TagKind, TagName};

mod builtins;
mod close_tag;
mod helpers;
mod open_and_close_tag;
mod open_tag;
mod parse_chunk;
mod state;
mod tag;

pub struct Context<'src> {
    /// The scanner used to tokenize the input source.
    pub(crate) scanner: Scanner<'src>,
    /// Optional bindings for styles.
    pub(crate) bindings: Option<HashMap<&'src str, Style>>,
    /// The current state of the parser.
    pub(crate) state: State<'src>,
    /// The next chunk to be parsed.
    pub(crate) next_chunk: Option<Chunk<'src>>,
}

impl<'src> Context<'src> {
    #[must_use]
    pub fn new(source: &'src str, bindings: Option<HashMap<&'src str, Style>>) -> Self {
        Self {
            scanner: Scanner::new(source),
            bindings,
            state: State::new(),
            next_chunk: None,
        }
    }
}

/// A parser for the Ziyy language.
pub struct Parser {
    /// A buffer to store the parsed output.
    pub(crate) buf: String,
    /// Flag to indicate whether to skip white space.
    pub(crate) skip_ws: bool,
    /// Flag to indicate whether to style the text exactly as it is.
    pub(crate) pre_ws: i16,
    /// The last written printable element.
    pub(crate) block_start: bool,
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl<'src> Parser {
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
    #[must_use]
    pub fn new() -> Parser {
        Parser {
            buf: String::new(),
            skip_ws: true,
            pre_ws: 1,
            block_start: true,
        }
    }

    /// Parses source and Returns a [String].
    /// # Errors
    ///
    /// Returns an `Error` if parsing fails.
    pub fn parse(&mut self, mut ctx: Context<'src>) -> Result<String, Error<'src>> {
        loop {
            let parsed = Parser::parse_chunk(&mut ctx)?;
            match parsed {
                Chunk::Comment(_, _) => {}
                Chunk::Escape(ch, _) => {
                    self.buf.push(ch);
                    self.skip_ws = is_whitespace(ch);
                    self.block_start = self.skip_ws;
                }

                Chunk::Tag(tag) => match tag.kind {
                    TagKind::Open => self.parse_open_tag(&mut ctx, tag)?,
                    TagKind::Close => self.parse_close_tag(&mut ctx, &tag)?,
                    TagKind::SelfClose => self.parse_open_and_close_tag(&mut ctx, &tag)?,
                },

                Chunk::Text(text, _) => {
                    self.buf.push_str(text);
                    self.skip_ws = false;
                    self.block_start = false;
                }

                Chunk::WhiteSpace(ws, _) => {
                    let chunk = Parser::parse_next_chunk(&mut ctx)?;
                    if self.pre_ws > 0 {
                        self.buf.push_str(ws);
                    } else if let Chunk::Eof(_) = chunk {
                        if ws.contains('\n') {
                            self.buf.push('\n');
                        }
                    } else if !self.skip_ws {
                        self.buf.push(' ');
                        self.skip_ws = true;
                    }
                }

                Chunk::Eof(_) => {
                    return Ok(take(&mut self.buf));
                }
            }
        }
    }

    /// Parses the source and returns a `Vec<u8>`.
    ///
    /// # Errors
    ///
    /// Returns an [Error] if parsing fails.
    pub fn parse_to_bytes(&mut self, ctx: Context<'src>) -> Result<Vec<u8>, Error<'src>> {
        match self.parse(ctx) {
            Ok(res) => Ok(res.into_bytes()),
            Err(err) => Err(err),
        }
    }

    pub fn parse_to_doc(&mut self, mut ctx: Context<'src>) -> Result<Document<'src>, Error<'src>> {
        let mut doc = Document::new();
        let mut id = doc.root().id();

        loop {
            let parsed = Parser::parse_chunk(&mut ctx)?;
            match &parsed {
                chunk @ Chunk::Tag(tag) => match tag.kind {
                    TagKind::Open => {
                        let mut node = doc.get_mut(id).unwrap();
                        let child = node.append(chunk.clone());
                        id = child.id();
                    }
                    TagKind::Close => {
                        let mut node = doc.get_mut(id).unwrap();
                        node.append(chunk.clone());
                        id = node.parent().unwrap().id();
                    }
                    TagKind::SelfClose => {
                        let mut node = doc.get_mut(id).unwrap();
                        node.append(chunk.clone());
                    }
                },

                Chunk::Eof(_) => return Ok(doc),

                chunk => {
                    let mut node = doc.get_mut(id).unwrap();
                    node.append(chunk.clone());
                }
            }
        }
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
    fn expect_tag(
        tag: &Tag<'src>,
        to_be: &TagName<'src>,
        err: ErrorKind<'src>,
    ) -> Result<(), Error<'src>> {
        if tag.name == *to_be {
            Ok(())
        } else {
            Err(Error {
                kind: err,
                span: tag.span,
            })
        }
    }

    /// Writes the style to the buffer and saves the current style state.
    ///
    /// # Arguments
    ///
    /// * `tag_name` - The name of the tag.
    /// * `style` - The style to be written and saved.
    fn write_and_save(&mut self, ctx: &mut Context<'src>, tag_name: &TagName<'src>, style: Style) {
        let prev = ctx.state.previous_style();
        let new = prev + style;
        let delta = style - prev;
        self.buf.push_str(&delta.to_string2());
        ctx.state.push(tag_name.clone(), new, delta);
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
fn expect_token<'src>(token: &Token<'src>, tt: TokenKind) -> Result<(), Error<'src>> {
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

#[cfg(test)]
mod tests {
    use crate::scanner::span::Span;

    use super::*;

    /* #[test]
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
    } */

    #[test]
    fn test_expect_tag() {
        let tag = Tag {
            name: TagName::P,
            kind: TagKind::Open,
            span: Span::default(),
            custom: tag::Value::None,
            style: Style::new(),
            class: tag::Value::None,
        };
        let result = Parser::expect_tag(
            &tag,
            &TagName::P,
            ErrorKind::MisMatchedTags {
                open: TagName::P,
                close: tag.name.clone(),
            },
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_expect_tag_error() {
        let tag = Tag {
            name: TagName::B,
            kind: TagKind::Open,
            span: Span::default(),
            custom: tag::Value::None,
            style: Style::new(),
            class: tag::Value::None,
        };

        let result = Parser::expect_tag(
            &tag,
            &TagName::P,
            ErrorKind::MisMatchedTags {
                open: TagName::P,
                close: tag.name.clone(),
            },
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_expect_token() {
        let token = Token {
            kind: TokenKind::TEXT,
            span: Span::default(),
            content: "",
        };
        let result = expect_token(&token, TokenKind::TEXT);
        assert!(result.is_ok());
    }

    #[test]
    fn test_expect_token_error() {
        let token = Token {
            kind: TokenKind::TEXT,
            span: Span::default(),
            content: "",
        };
        let result = expect_token(&token, TokenKind::B);
        assert!(result.is_err());
    }
}
