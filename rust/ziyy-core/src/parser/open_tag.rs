use super::parse_chunk::Chunk;
use super::tag::Value;
use super::{inherit, Parser, Tag, TagName, TagType, BUILTIN_TAGS};
use crate::num::str_to_u32;
use crate::{get_num2, Error, ErrorKind};
use std::io::Write;

impl<T: AsRef<str>> Parser<T> {
    pub(crate) fn parse_open_tag(&mut self, mut tag: Tag) -> Result<(), Error> {
        match tag.name {
            TagName::A => {
                let _ = self.buf.write(b"\x1b]8;;");
                let _ = self.buf.write(if let Value::Some(ref href) = tag.custom {
                    href.as_bytes()
                } else {
                    "".as_bytes()
                });
                let _ = self.buf.write(b"\x1b\\");
                loop {
                    let chunk = self.parse_chunk()?;
                    match chunk {
                        Chunk::Comment(_) => {}
                        Chunk::Escape(_) => {}
                        Chunk::Tag(tag2) => {
                            if tag2.name == TagName::A && tag2.r#type == TagType::Close {
                                break;
                            }
                        }

                        Chunk::Text(text) => {
                            let _ = self.buf.write(text.as_bytes());
                        }

                        Chunk::WhiteSpace(ws) => {
                            let _ = self.buf.write(ws.as_bytes());
                        }

                        Chunk::Eof => {
                            return Err(Error {
                                kind: ErrorKind::UnexpectedEof,
                                span: tag.span,
                            });
                        }
                    }
                    let _ = self.buf.write(b"\x1b]8;;\x1b\\");
                }
            }

            TagName::Any(ref s) => {
                let src = self.bindings.as_ref().unwrap().get(s);
                if let Some(btag) = src {
                    inherit(btag, &mut tag.style);
                }

                self.write_and_save(&tag.name, tag.style);
            }

            TagName::B
            | TagName::D
            | TagName::I
            | TagName::U
            | TagName::K
            | TagName::R
            | TagName::H
            | TagName::S
            | TagName::C
            | TagName::X
            | TagName::Ziyy => {
                if tag.name == TagName::U {
                    self.clear_under = true;
                }

                if let Value::Some(ref s) = tag.src {
                    let src = BUILTIN_TAGS.get(s);
                    if let Some(btag) = src {
                        inherit(btag, &mut tag.style);
                    } else {
                        let src = self.bindings.as_ref().unwrap().get(s);
                        if let Some(btag) = src {
                            inherit(btag, &mut tag.style);
                        }
                    }
                }
                self.write_and_save(&tag.name, tag.style);
            }

            TagName::Br => {
                if let Value::Some(val) = tag.custom {
                    let n: usize = get_num2!(str_to_u32(&val, 10), tag) as usize;
                    let _ = self.buf.write("\n".repeat(n).as_bytes());
                } else {
                    let _ = self.buf.write("\n".as_bytes());
                }
            }

            TagName::E => {
                // TODO: correct implementation
                if let Some(style) = self.state.previous_style() {
                    let _ = self.buf.write(&style.to_string().into_bytes());
                }

                self.state.push(tag.name, tag.style.clone(), tag.style);
            }

            TagName::Let => {} // TODO:

            TagName::P => {
                if self.skip_ws {
                    self.skip_ws = false;
                } else {
                    let _ = self.buf.write("\n".as_bytes());
                }

                if let Value::Some(ref s) = tag.src {
                    let src = self.bindings.as_ref().unwrap().get(s);
                    if let Some(btag) = src {
                        inherit(btag, &mut tag.style);
                    }
                }

                match tag.custom {
                    Value::Bool => {
                        let _ = self.buf.write("\t".as_bytes());
                    }

                    Value::Some(val) => {
                        let n: usize = get_num2!(str_to_u32(&val, 10), tag) as usize;
                        let _ = self.buf.write(" ".repeat(n).as_bytes());
                    }

                    Value::None => {}
                }

                self.write_and_save(&tag.name, tag.style);

                self.skip_ws = true;
            }
        }

        Ok(())
    }
}
