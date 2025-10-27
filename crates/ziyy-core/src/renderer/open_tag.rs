use std::collections::HashMap;

use crate::builtins::BUILTIN_STYLES;
use crate::context::Context;
use crate::error::{Error, ErrorKind, Result};
use crate::num::input_to_u8;
use crate::parser::{Chunk, Parser, Tag, TagKind, TagName};
use crate::shared::{Input, Value};

use super::Renderer;

impl<I: ?Sized + Input, O> Renderer<I, O> {
    #[allow(clippy::too_many_lines)]
    pub(super) fn render_open_tag<'src>(
        &mut self,
        ctx: &mut Context<'src, I>,
        mut tag: Tag<'src, I>,
    ) -> Result<'src, I, ()> {
        match tag.name {
            TagName::A => {
                self.buf.extend_from_slice(b"\x1b]8;;");
                self.buf
                    .extend_from_slice(if let Value::Some(href) = tag.custom {
                        href.as_ref()
                    } else {
                        b""
                    });
                self.buf.extend_from_slice(b"\x1b\\");
                loop {
                    let chunk = Parser::parse(ctx)?;
                    match chunk {
                        Chunk::Comment(_, _) | Chunk::Escape(_, _) => {}
                        Chunk::Tag(tag2) => {
                            if tag2.name == TagName::A && tag2.kind == TagKind::Close {
                                break;
                            }
                        }

                        Chunk::Text(text, _) => {
                            self.buf.extend_from_slice(text.as_ref());
                        }

                        Chunk::WhiteSpace(ws, _) => {
                            self.buf.extend_from_slice(ws.as_ref());
                        }

                        Chunk::Eof(_) => {
                            return Err(Error {
                                kind: ErrorKind::UnexpectedEof,
                                span: tag.span,
                            });
                        }
                    }
                    self.buf.extend_from_slice(b"\x1b]8;;\x1b\\");
                }
            }
            TagName::Any(s) => {
                if let Some(bindings) = &ctx.bindings {
                    let src = bindings.get(s.as_ref());
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
                    for class in s
                        .as_ref()
                        .split(|n| *n == b' ')
                        .filter(|s| !s.is_empty())
                        .rev()
                    {
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

            TagName::Br => loop {
                let chunk = Parser::parse(ctx)?;
                match chunk {
                    Chunk::Tag(tag2) => {
                        if tag2.name == TagName::Br && tag2.kind == TagKind::Close {
                            if let Value::Some(val) = tag.custom {
                                let n: usize = get_num2!(input_to_u8(val, 10), tag) as usize;
                                for _ in 0..n {
                                    self.buf.push(b'\n');
                                }
                            } else {
                                self.buf.push(b'\n');
                            }
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

            #[cfg(feature = "bindings")]
            TagName::Let => loop {
                let chunk = Parser::parse(ctx)?;
                match chunk {
                    Chunk::Tag(tag2) => {
                        if tag2.name == TagName::Let && tag2.kind == TagKind::Close {
                            if ctx.bindings.is_none() {
                                ctx.bindings = Some(HashMap::with_capacity(10));
                            }

                            if let Value::Some(name) = tag.custom {
                                ctx.bindings
                                    .as_mut()
                                    .unwrap()
                                    .insert(name.as_ref(), tag.style);
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
                    self.buf.push(b'\n');
                    self.block_start = true;
                }

                if tag.name == TagName::Pre {
                    self.pre_ws += 1;
                }

                if let Value::Some(s) = tag.class {
                    for class in s
                        .as_ref()
                        .split(|n| *n == b' ')
                        .filter(|s| !s.is_empty())
                        .rev()
                    {
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
                        self.buf.push(b'\t');
                    }

                    Value::Some(val) => {
                        let n: usize = get_num2!(input_to_u8(val, 10), tag) as usize;
                        for _ in 0..n {
                            self.buf.push(b' ');
                        }
                    }

                    Value::None => {}
                }

                self.write_and_save(ctx, &tag.name, tag.style);

                self.skip_ws = true;
            }
            TagName::Root => {}
        }

        Ok(())
    }
}
