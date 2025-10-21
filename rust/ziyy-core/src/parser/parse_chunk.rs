use crate::error::ErrorKind;
use crate::parser::builtins::is_builtin_tag;
use crate::scanner::token::{Token, TokenKind};
use crate::scanner::Scanner;
use crate::{char_from_u32, Error};
use crate::{
    number,
    style::{
        Ansi256, Blink, Color, Delete, FontStyle, Hide, Intensity, Invert, Rgb, Style, Underline,
    },
};
use TokenKind::{
    B, BLACK, BLUE, C, CLASS, CYAN, D, DOUBLE, FIXED, GREAT, GREEN, H, HREF, I, ID, IDENTIFIER,
    INDENT, K, MAGENTA, N, R, RED, RGB, S, U, UU, WHITE, X, YELLOW,
};

use super::{
    expect_token,
    tag::{Tag, TagKind, TagName, Value},
    Context, Parser,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Chunk<'src> {
    Comment(&'src str),
    Eof,
    Escape(char),
    Tag(Tag<'src>),
    Text(&'src str),
    WhiteSpace(&'src str),
}

impl<'src> Parser {
    #[allow(clippy::too_many_lines)]
    pub(super) fn parse_chunk(ctx: &mut Context<'src>) -> Result<Chunk<'src>, Error<'src>> {
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
                    TokenKind::COMMENT => Ok(Chunk::Comment(token.content)),
                    TokenKind::TEXT => Ok(Chunk::Text(token.content)),
                    TokenKind::WHITESPACE => Ok(Chunk::WhiteSpace(token.content)),
                    TokenKind::ESCAPED => {
                        Ok(Chunk::Text(&token.content[3..token.content.len() - 4]))
                    }
                    TokenKind::EOF => Ok(Chunk::Eof),
                    TokenKind::ESC_0 => char_from_u32!(&token.content[2..], 8, &token),
                    TokenKind::ESC_X | TokenKind::ESC_U => {
                        char_from_u32!(&token.content[2..], 16, &token)
                    }
                    TokenKind::ESC_A => Ok(Chunk::Escape(7 as char)),
                    TokenKind::ESC_B => Ok(Chunk::Escape(8 as char)),
                    TokenKind::ESC_T => Ok(Chunk::Escape(9 as char)),
                    TokenKind::ESC_N => Ok(Chunk::Escape(10 as char)),
                    TokenKind::ESC_V => Ok(Chunk::Escape(11 as char)),
                    TokenKind::ESC_F => Ok(Chunk::Escape(12 as char)),
                    TokenKind::ESC_R => Ok(Chunk::Escape(13 as char)),
                    TokenKind::ESC_E => Ok(Chunk::Escape(27 as char)),
                    TokenKind::ESC_BACK_SLASH => Ok(Chunk::Escape('\\')),
                    TokenKind::ESC_LESS => Ok(Chunk::Escape('<')),
                    TokenKind::ESC_GREAT => Ok(Chunk::Escape('>')),

                    TokenKind::ANSI => Ok(Chunk::Tag(Tag::parse_from_ansi(
                        &token.content[2..token.content.len() - 1],
                        token.span,
                    ))),
                    TokenKind::ANSI_ESC => Ok(Chunk::Tag(Tag::parse_from_ansi(
                        &token.content[5..token.content.len() - 1],
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
                return Ok(Chunk::Tag(Tag::new(TagName::None, kind)));
            }

            GREAT if kind == TagKind::Close => {
                return Ok(Chunk::Tag(Tag::new(TagName::None, kind)));
            }
            _ => match_tag_name(&token)?,
        };

        let mut tag = Tag::new(tag_name.clone(), kind);
        tag.span += token.span;

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

                    let end = token.content.len() - 1;
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
                    let end = token.content.len() - 1;
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

                    let end = token.content.len() - 1;
                    let s = &token.content[1..end];

                    let mut scanner = Scanner::new(s);
                    scanner.text_mode = false;
                    scanner.parse_colors = true;
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

                    let end = token.content.len() - 1;
                    let s = &token.content[1..end];

                    let mut scanner = Scanner::new(s);
                    scanner.text_mode = false;
                    scanner.parse_colors = true;
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
                                let end = token.content.len() - 1;
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
            TokenKind::SLASH_GREAT => {
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

    pub(super) fn parse_next_chunk(ctx: &mut Context<'src>) -> Result<Chunk<'src>, Error<'src>> {
        let chunk = Parser::parse_chunk(ctx)?;
        ctx.next_chunk = Some(chunk.clone());
        Ok(chunk)
    }
}

pub(crate) fn match_tag_name<'src>(token: &Token<'src>) -> Result<TagName<'src>, Error<'src>> {
    use TokenKind::{
        A, B, BLACK, BLUE, BR, C, CLASS, CODE, CURLY, CYAN, D, DASHED, DIV, DOTTED, DOUBLE, FIXED,
        GREEN, H, HREF, I, ID, IDENTIFIER, INDENT, K, LET, MAGENTA, N, NONE, P, PRE, R, RED, RGB,
        S, SINGLE, SPAN, U, WHITE, X, YELLOW, ZIYY,
    };

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
            })
        }
    };

    Ok(kind)
}
