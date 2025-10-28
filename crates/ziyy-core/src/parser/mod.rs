use std::marker::PhantomData;

use crate::builtins::is_builtin_tag;
use crate::context::Context;
use crate::error::Error;
use crate::error::ErrorKind;
use crate::error::Result;
use crate::scanner::{Scanner, Token, TokenKind};
use crate::shared::Input;
use crate::shared::Value;
use crate::style::{
    Ansi256, Blink, Color, Delete, FontStyle, Hide, Intensity, Invert, Rgb, Style, Underline,
};

use TokenKind::{
    A, B, BLACK, BLUE, BR, C, CLASS, CODE, CURLY, CYAN, D, DASHED, DIV, DOTTED, DOUBLE, FIXED,
    GREAT, GREEN, H, HREF, I, ID, IDENTIFIER, INDENT, K, LET, MAGENTA, N, NONE, P, PRE, R, RED,
    RGB, S, SINGLE, SPAN, U, UU, WHITE, X, YELLOW, ZIYY,
};
pub use chunk::Chunk;
pub use tag::{Tag, TagKind, TagName};

mod chunk;
#[macro_use]
mod tag;

/// A parser for the Ziyy language.
pub struct Parser<I: ?Sized + Input> {
    phantom: PhantomData<I>,
}

impl<I: ?Sized + Input> Default for Parser<I> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'src, I: ?Sized + Input> Parser<I> {
    /// Creates a new Ziyy Parser.
    #[must_use]
    pub fn new() -> Self {
        Parser {
            phantom: PhantomData,
        }
    }

    #[allow(clippy::too_many_lines)]
    pub fn parse(ctx: &mut Context<'src, I>) -> Result<'src, I, Chunk<'src, I>> {
        if let Some(chunk) = ctx.next_chunk.clone() {
            ctx.next_chunk = None;
            return Ok(chunk);
        }

        let token = ctx.scanner.scan_token()?;
        let kind = match token.kind {
            TokenKind::LESS => TagKind::Open,
            TokenKind::LESS_SLASH => TagKind::Close,

            _ => {
                return match token.kind {
                    TokenKind::COMMENT => Ok(Chunk::Comment(token.content, token.span)),
                    TokenKind::TEXT => Ok(Chunk::Text(token.content, token.span)),
                    TokenKind::WHITESPACE => Ok(Chunk::WhiteSpace(token.content, token.span)),
                    TokenKind::ESCAPED => Ok(Chunk::Text(
                        &token.content[3..token.content.as_ref().len() - 4],
                        token.span,
                    )),
                    TokenKind::EOF => Ok(Chunk::Eof(token.span)),
                    TokenKind::ESC_0 => char_from_u32!(&token.content[2..], 8, &token),
                    TokenKind::ESC_X | TokenKind::ESC_U => {
                        char_from_u32!(&token.content[2..], 16, &token)
                    }
                    TokenKind::ESC_A => Ok(Chunk::Escape(7 as char, token.span)),
                    TokenKind::ESC_B => Ok(Chunk::Escape(8 as char, token.span)),
                    TokenKind::ESC_T => Ok(Chunk::Escape(9 as char, token.span)),
                    TokenKind::ESC_N => Ok(Chunk::Escape(10 as char, token.span)),
                    TokenKind::ESC_V => Ok(Chunk::Escape(11 as char, token.span)),
                    TokenKind::ESC_F => Ok(Chunk::Escape(12 as char, token.span)),
                    TokenKind::ESC_R => Ok(Chunk::Escape(13 as char, token.span)),
                    TokenKind::ESC_E => Ok(Chunk::Escape(27 as char, token.span)),
                    TokenKind::ESC_BACK_SLASH => Ok(Chunk::Escape('\\', token.span)),
                    TokenKind::ESC_LESS => Ok(Chunk::Escape('<', token.span)),
                    TokenKind::ESC_GREAT => Ok(Chunk::Escape('>', token.span)),

                    TokenKind::ANSI => Ok(Chunk::Tag(Tag::parse_from_ansi(
                        &token.content[2..token.content.as_ref().len() - 1],
                        token.span,
                    ))),
                    TokenKind::ANSI_ESC => Ok(Chunk::Tag(Tag::parse_from_ansi(
                        &token.content[5..token.content.as_ref().len() - 1],
                        token.span,
                    ))),

                    _ => Err(Error::new(
                        ErrorKind::UnexpectedToken {
                            expected: token.kind,
                            found: None,
                        },
                        &token,
                    )),
                };
            }
        };

        let token = ctx.scanner.scan_token()?;

        let mut style = Style::new();
        let tag_name = match token.kind {
            GREAT if kind == TagKind::Open => {
                return Ok(Chunk::Tag(Tag::new(TagName::Empty, kind)));
            }

            GREAT if kind == TagKind::Close => {
                return Ok(Chunk::Tag(Tag::new(TagName::Empty, kind)));
            }
            _ => match_tag_name(&token)?,
        };

        let mut tag = Tag::new(tag_name.clone(), kind);
        tag.span = token.span;

        let mut token = ctx.scanner.scan_token()?;
        tag.span += token.span;

        macro_rules! assign_effect {
            ($setter:tt, $v:expr) => {{
                style.$setter($v);

                token = ctx.scanner.scan_token()?;
                tag.span += token.span;
                if token.kind == TokenKind::EQUAL {
                    token = ctx.scanner.scan_token()?;
                    tag.span += token.span;
                    expect_token(&token, TokenKind::STRING)?;
                    token = ctx.scanner.scan_token()?;
                    tag.span += token.span;
                }
            }};
        }

        macro_rules! assign_color {
            ($setter:tt) => {{
                token = ctx.scanner.scan_token()?;
                tag.span += token.span;
                if token.kind == TokenKind::EQUAL {
                    token = ctx.scanner.scan_token()?;
                    tag.span += token.span;
                    expect_token(&token, TokenKind::STRING)?;

                    let end = token.content.as_ref().len() - 1;
                    let color = Color::parse(&token.content[1..end], token.span)?;
                    style.$setter(color);
                    token = ctx.scanner.scan_token()?;
                }
            }};
        }

        macro_rules! assign_prop_value {
            ( $prop:tt ) => {{
                tag.$prop = Value::Bool;

                token = ctx.scanner.scan_token()?;
                tag.span += token.span;
                if token.kind == TokenKind::EQUAL {
                    token = ctx.scanner.scan_token()?;
                    tag.span += token.span;
                    expect_token(&token, TokenKind::STRING)?;
                    let end = token.content.as_ref().len() - 1;
                    tag.$prop = Value::Some(&token.content[1..end]);
                    token = ctx.scanner.scan_token()?;
                }
            }};
        }

        macro_rules! consume_declaration {
            () => {{
                token = ctx.scanner.scan_token()?;
                tag.span += token.span;
                if token.kind == TokenKind::EQUAL {
                    token = ctx.scanner.scan_token()?;
                    tag.span += token.span;
                    expect_token(&token, TokenKind::STRING)?;
                    token = ctx.scanner.scan_token()?;
                }
            }};
        }
        loop {
            match token.kind {
                // styles
                B => {
                    assign_effect!(set_intensity, Intensity::Bold);
                }
                D => {
                    assign_effect!(set_intensity, Intensity::Dim);
                }
                I => {
                    assign_effect!(set_font_style, FontStyle::Italics);
                }
                U => {
                    assign_effect!(set_underline, Underline::Single);
                }
                K => {
                    assign_effect!(set_blink, Blink::Slow);
                }
                R => {
                    assign_effect!(set_invert, Invert::Set);
                }
                H => {
                    assign_effect!(set_hide, Hide::Set);
                }
                S => {
                    assign_effect!(set_delete, Delete::Set);
                }
                UU => {
                    assign_effect!(set_underline, Underline::Double);
                }
                DOUBLE => {
                    if tag_name == TagName::U {
                        assign_effect!(set_underline, Underline::Double);
                    } else {
                        consume_declaration!();
                    }
                }

                // colors
                C => {
                    assign_color!(set_fg_color);
                }
                X => {
                    assign_color!(set_bg_color);
                }
                BLACK | BLUE | CYAN | GREEN | MAGENTA | RED | WHITE | YELLOW => {
                    let color = Color::parse(token.content, token.span)?;
                    if tag_name == TagName::C {
                        style.set_fg_color(color);
                    } else if tag_name == TagName::X {
                        style.set_bg_color(color);
                    }

                    token = ctx.scanner.scan_token()?;
                    tag.span += token.span;
                    if token.kind == TokenKind::EQUAL {
                        token = ctx.scanner.scan_token()?;
                        tag.span += token.span;
                        expect_token(&token, TokenKind::STRING)?;
                    }
                }
                FIXED => {
                    token = ctx.scanner.scan_token()?;
                    tag.span += token.span;
                    if token.kind == TokenKind::EQUAL {
                        token = ctx.scanner.scan_token()?;
                        tag.span += token.span;
                        expect_token(&token, TokenKind::STRING)?;
                    }

                    let end = token.content.as_ref().len() - 1;
                    let s = &token.content[1..end];

                    let mut scanner = Scanner::new(s);
                    scanner.text_mode = false;
                    scanner.parse_hex = true;
                    scanner.current_pos = token.span.start; // FIXME: add 1 to start position

                    let tok = scanner.scan_token()?;
                    let color = Color::Ansi256(Ansi256(number!(tok.content, 10, &tok)));

                    if tag_name == TagName::C {
                        style.set_fg_color(color);
                    } else if tag_name == TagName::X {
                        style.set_bg_color(color);
                    }
                    token = ctx.scanner.scan_token()?;
                }
                RGB => {
                    token = ctx.scanner.scan_token()?;
                    tag.span += token.span;
                    if token.kind == TokenKind::EQUAL {
                        token = ctx.scanner.scan_token()?;
                        tag.span += token.span;
                        expect_token(&token, TokenKind::STRING)?;
                    }

                    let end = token.content.as_ref().len() - 1;
                    let s = &token.content[1..end];

                    let mut scanner = Scanner::new(s);
                    scanner.text_mode = false;
                    scanner.parse_hex = true;
                    scanner.current_pos = token.span.start; // TODO: add 1 to start position

                    let color = Color::Rgb(Rgb::parse(&mut scanner)?);

                    if tag_name == TagName::C {
                        style.set_fg_color(color);
                    } else if tag_name == TagName::X {
                        style.set_bg_color(color);
                    }
                    token = ctx.scanner.scan_token()?;
                }

                // custom
                N => {
                    // number of newlines to insert
                    if tag_name == TagName::Br {
                        assign_prop_value!(custom);
                    } else {
                        consume_declaration!();
                    }
                }
                HREF => {
                    // url of link
                    if tag_name == TagName::A {
                        assign_prop_value!(custom);
                    } else {
                        consume_declaration!();
                    }
                }
                ID => {
                    // name of binding to declare
                    if tag_name == TagName::Let {
                        {
                            tag.custom = Value::Bool;
                            token = ctx.scanner.scan_token()?;
                            tag.span += token.span;
                            if token.kind == TokenKind::EQUAL {
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                                expect_token(&token, TokenKind::STRING)?;
                                let end = token.content.as_ref().len() - 1;
                                let s = &token.content[1..end];
                                if is_builtin_tag(s) {
                                    return Err(Error {
                                        kind: ErrorKind::BuiltinTagOverwrite(s),
                                        span: token.span,
                                    });
                                }
                                tag.custom = Value::Some(s);
                                token = ctx.scanner.scan_token()?;
                            }
                        };
                    } else {
                        consume_declaration!();
                    }
                }
                INDENT => {
                    // number of spaces to insert before a paragraph/ a tab if Value::Bool
                    if tag_name == TagName::P {
                        assign_prop_value!(custom);
                    } else {
                        consume_declaration!();
                    }
                }

                // inherit properties from binding with name
                CLASS => assign_prop_value!(class),

                // ignore unknown properties
                IDENTIFIER => {
                    token = ctx.scanner.scan_token()?;
                    tag.span += token.span;
                    if token.kind == TokenKind::EQUAL {
                        token = ctx.scanner.scan_token()?;
                        tag.span += token.span;
                        expect_token(&token, TokenKind::STRING)?;

                        token = ctx.scanner.scan_token()?;
                        tag.span += token.span;
                    }
                }

                _ => break,
            }
        }

        tag.style = style;

        match token.kind {
            TokenKind::GREAT => {}
            TokenKind::SLASH_GREAT if tag.kind == TagKind::Open => {
                tag.kind = TagKind::SelfClose;
            }

            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedToken {
                        expected: token.kind,
                        found: None,
                    },
                    &token,
                ));
            }
        }

        Ok(Chunk::Tag(tag))
    }

    pub fn parse_next(ctx: &mut Context<'src, I>) -> Result<'src, I, Chunk<'src, I>> {
        let chunk = Parser::parse(ctx)?;
        ctx.next_chunk = Some(chunk.clone());
        Ok(chunk)
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
fn expect_token<'src, I: ?Sized + Input>(
    token: &Token<'src, I>,
    tt: TokenKind,
) -> Result<'src, I, ()> {
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

pub(crate) fn match_tag_name<'src, I: ?Sized + Input>(
    token: &Token<'src, I>,
) -> Result<'src, I, TagName<'src, I>> {
    let kind = match token.kind {
        A => TagName::A,
        B => TagName::B,
        BR => TagName::Br,
        C => TagName::C,
        CODE => TagName::Code,
        D => TagName::D,
        DIV => TagName::Div,
        H => TagName::H,
        I => TagName::I,
        K => TagName::K,
        LET => TagName::Let,
        P => TagName::P,
        PRE => TagName::Pre,
        R => TagName::R,
        S => TagName::S,
        SPAN => TagName::Span,
        U => TagName::U,
        X => TagName::X,
        ZIYY => TagName::Ziyy,

        IDENTIFIER | BLACK | BLUE | CYAN | GREEN | MAGENTA | RED | WHITE | YELLOW | FIXED | RGB
        | CLASS | CURLY | DASHED | DOUBLE | DOTTED | ID | INDENT | HREF | N | NONE | SINGLE => {
            TagName::Any(token.content)
        }
        _ => {
            return Err(Error {
                kind: ErrorKind::InvalidTagName(token.content),
                span: token.span,
            });
        }
    };

    Ok(kind)
}

/* #[cfg(test)]
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
 */
