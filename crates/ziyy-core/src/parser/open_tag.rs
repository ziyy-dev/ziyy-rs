use std::collections::HashMap;

use super::parse_chunk::Chunk;
use super::tag::Value;
use super::{Context, Parser, Tag, TagKind, TagName, BUILTIN_STYLES};
use crate::num::str_to_u8;
use crate::{get_num2, Error, ErrorKind};

impl<'src> Parser {
    #[allow(clippy::too_many_lines)]
    pub(crate) fn parse_open_tag(
        &mut self,
        ctx: &mut Context<'src>,
        mut tag: Tag<'src>,
    ) -> Result<(), Error<'src>> {
        match tag.name {
            TagName::A => {
                self.buf.push_str("\x1b]8;;");
                self.buf.push_str(if let Value::Some(href) = tag.custom {
                    href
                } else {
                    ""
                });
                self.buf.push_str("\x1b\\");
                loop {
                    let chunk = Parser::parse_chunk(ctx)?;
                    match chunk {
                        Chunk::Comment(_, _) | Chunk::Escape(_, _) => {}
                        Chunk::Tag(tag2) => {
                            if tag2.name == TagName::A && tag2.kind == TagKind::Close {
                                break;
                            }
                        }

                        Chunk::Text(text, _) => {
                            self.buf.push_str(text);
                        }

                        Chunk::WhiteSpace(ws, _) => {
                            self.buf.push_str(ws);
                        }

                        Chunk::Eof(_) => {
                            return Err(Error {
                                kind: ErrorKind::UnexpectedEof,
                                span: tag.span,
                            });
                        }
                    }
                    self.buf.push_str("\x1b]8;;\x1b\\");
                }
            }
            TagName::Any(s) => {
                if let Some(bindings) = &ctx.bindings {
                    let src = bindings.get(s);
                    if let Some(btag) = src {
                        tag.inherit(btag);
                    }

                    self.write_and_save(ctx, &tag.name, tag.style);
                }
            }
            TagName::Ansi
            | TagName::B
            | TagName::C
            | TagName::Code
            | TagName::D
            | TagName::Empty
            | TagName::H
            | TagName::I
            | TagName::K
            | TagName::R
            | TagName::S
            | TagName::Span
            | TagName::U
            | TagName::X
            | TagName::Ziyy => {
                if tag.name == TagName::Ziyy {
                    self.pre_ws -= 1;
                }

                if let Value::Some(s) = tag.class {
                    for class in s.split(' ').filter(|s| !s.is_empty()).rev() {
                        if let Some(btag) = BUILTIN_STYLES.get(class) {
                            tag.inherit(btag);
                        } else if let Some(bindings) = &ctx.bindings {
                            if let Some(btag) = bindings.get(class) {
                                tag.inherit(btag);
                            }
                        }
                    }
                }
                self.write_and_save(ctx, &tag.name, tag.style);
            }
            TagName::Br => {
                if let Value::Some(val) = tag.custom {
                    let n: usize = get_num2!(str_to_u8(val, 10), tag) as usize;
                    self.buf.push_str(&"\n".repeat(n));
                } else {
                    self.buf.push('\n');
                }
            }
            TagName::Let => loop {
                let chunk = Parser::parse_chunk(ctx)?;
                match chunk {
                    Chunk::Tag(tag2) => {
                        if tag2.name == TagName::Let && tag2.kind == TagKind::Close {
                            if ctx.bindings.is_none() {
                                ctx.bindings = Some(HashMap::with_capacity(10));
                            }

                            if let Value::Some(name) = tag.custom {
                                ctx.bindings.as_mut().unwrap().insert(name, tag.style);
                            }
                            self.skip_ws = true;
                            break;
                        }
                    }

                    Chunk::Eof(_) => {
                        return Err(Error {
                            kind: ErrorKind::UnexpectedEof,
                            span: tag.span,
                        });
                    }

                    _ => {}
                }
            },
            TagName::Div | TagName::P | TagName::Pre => {
                if !self.block_start {
                    self.buf.push('\n');
                    self.block_start = true;
                }

                if tag.name == TagName::Pre {
                    self.pre_ws += 1;
                }

                if let Value::Some(s) = tag.class {
                    for class in s.split(' ').filter(|s| !s.is_empty()).rev() {
                        if let Some(btag) = BUILTIN_STYLES.get(class) {
                            tag.inherit(btag);
                        } else if let Some(bindings) = &ctx.bindings {
                            if let Some(btag) = bindings.get(class) {
                                tag.inherit(btag);
                            }
                        }
                    }
                }

                match tag.custom {
                    Value::Bool => {
                        self.buf.push('\t');
                    }

                    Value::Some(val) => {
                        let n: usize = get_num2!(str_to_u8(val, 10), tag) as usize;
                        self.buf.push_str(&" ".repeat(n));
                    }

                    Value::None => {}
                }

                self.write_and_save(ctx, &tag.name, tag.style);

                self.skip_ws = true;
            }
            TagName::None => {}
        }

        Ok(())
    }
}
