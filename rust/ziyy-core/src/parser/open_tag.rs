use super::parse_chunk::Chunk;
use super::tag::Value;
use super::{Context, Parser, Tag, TagKind, TagName, BUILTIN_STYLES};
use crate::num::str_to_u32;
use crate::{get_num2, Error, ErrorKind};

impl<'src> Parser {
    pub(crate) fn parse_open_tag(
        &mut self,
        ctx: &mut Context<'src>,
        mut tag: Tag<'src>,
    ) -> Result<(), Error<'src>> {
        match tag.name {
            TagName::A => {
                let _ = self.buf.push_str("\x1b]8;;");
                let _ = self
                    .buf
                    .push_str(if let Value::Some(ref href) = tag.custom {
                        href
                    } else {
                        ""
                    });
                let _ = self.buf.push_str("\x1b\\");
                loop {
                    let chunk = self.parse_chunk(ctx)?;
                    match chunk {
                        Chunk::Comment(_) => {}
                        Chunk::Escape(_) => {}
                        Chunk::Tag(tag2) => {
                            if tag2.name == TagName::A && tag2.kind == TagKind::Close {
                                break;
                            }
                        }

                        Chunk::Text(text) => {
                            let _ = self.buf.push_str(text);
                        }

                        Chunk::WhiteSpace(ws) => {
                            let _ = self.buf.push_str(ws);
                        }

                        Chunk::Eof => {
                            return Err(Error {
                                kind: ErrorKind::UnexpectedEof,
                                span: tag.span,
                            });
                        }
                    }
                    let _ = self.buf.push_str("\x1b]8;;\x1b\\");
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

                if let Value::Some(ref s) = tag.class {
                    for class in s.split(|ch| ch == ' ').filter(|s| !s.is_empty()).rev() {
                        if let Some(btag) = BUILTIN_STYLES.get(class) {
                            tag.inherit(btag);
                        } else {
                            if let Some(bindings) = &ctx.bindings {
                                if let Some(btag) = bindings.get(class) {
                                    tag.inherit(btag);
                                }
                            }
                        }
                    }
                }
                self.write_and_save(ctx, &tag.name, tag.style);
            }
            TagName::Br => {
                if let Value::Some(val) = tag.custom {
                    let n: usize = get_num2!(str_to_u32(&val, 10), tag) as usize;
                    let _ = self.buf.push_str(&"\n".repeat(n));
                } else {
                    let _ = self.buf.push('\n');
                }
            }
            TagName::Let => {}
            TagName::Div | TagName::P | TagName::Pre => {
                if !self.block_start {
                    let _ = self.buf.push('\n');
                    self.block_start = true;
                }

                if tag.name == TagName::Pre {
                    self.pre_ws += 1;
                }

                if let Value::Some(ref s) = tag.class {
                    for class in s.split(|ch| ch == ' ').filter(|s| !s.is_empty()).rev() {
                        if let Some(btag) = BUILTIN_STYLES.get(class) {
                            tag.inherit(btag);
                        } else {
                            if let Some(bindings) = &ctx.bindings {
                                if let Some(btag) = bindings.get(class) {
                                    tag.inherit(btag);
                                }
                            }
                        }
                    }
                }

                match tag.custom {
                    Value::Bool => {
                        let _ = self.buf.push('\t');
                    }

                    Value::Some(val) => {
                        let n: usize = get_num2!(str_to_u32(&val, 10), tag) as usize;
                        let _ = self.buf.push_str(&" ".repeat(n));
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
