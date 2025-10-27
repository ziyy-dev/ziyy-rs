use std::fmt;
use std::io;
use std::marker::PhantomData;

use smallvec::{smallvec, SmallVec};

use crate::context::Context;
#[cfg(feature = "tree")]
use crate::tree::Document;
use crate::error::Result;
use crate::parser::{Chunk, Parser, TagKind, TagName};
use crate::scanner::is_whitespace;
use crate::shared::Input;
use crate::style::Style;

mod close_tag;
mod open_tag;
mod self_close_tag;

pub struct Renderer<I: ?Sized + Input, O> {
    /// A buffer to store the parsed output.
    buf: SmallVec<[u8; 1024]>,
    /// Output writter
    output: O,
    /// Flag to indicate whether to skip white space.
    skip_ws: bool,
    /// Flag to indicate whether to preserve white space.
    pre_ws: i16,
    /// The last written printable element.
    block_start: bool,
    phantom: PhantomData<I>,
}

impl<I: ?Sized + Input, O> Renderer<I, O> {
    pub fn new(output: O) -> Self {
        Self {
            buf: smallvec![],
            output,
            skip_ws: true,
            pre_ws: 1,
            block_start: true,
            phantom: PhantomData,
        }
    }

    fn write_input<'src>(&mut self, input: &'src I) -> Result<'src, I, ()> {
        let mut ctx = Context::new(input, None);
        loop {
            let parsed = Parser::parse(&mut ctx)?;
            match parsed {
                Chunk::Comment(_, _) => {}
                Chunk::Escape(ch, _) => {
                    match ch.len_utf8() {
                        1 => self.buf.push(ch as u8),
                        n => {
                            let mut buf = [0; 4];
                            ch.encode_utf8(&mut buf);
                            self.buf.extend_from_slice(&buf[0..n]);
                        }
                    }

                    self.skip_ws = is_whitespace(ch);
                    self.block_start = self.skip_ws;
                }

                Chunk::Tag(tag) => match tag.kind {
                    TagKind::Open => self.render_open_tag(&mut ctx, tag)?,
                    TagKind::Close => self.render_close_tag(&mut ctx, &tag)?,
                    TagKind::SelfClose => self.render_self_close_tag(&mut ctx, &tag)?,
                },

                Chunk::Text(text, _) => {
                    self.buf.extend_from_slice(text.as_ref());
                    self.skip_ws = false;
                    self.block_start = false;
                }

                Chunk::WhiteSpace(ws, _) => {
                    let chunk = Parser::parse_next(&mut ctx)?;
                    if self.pre_ws > 0 {
                        self.buf.extend_from_slice(ws.as_ref());
                    } else if let Chunk::Eof(_) = chunk {
                        if ws.as_ref().contains(&b'\n') {
                            self.buf.push(b'\n');
                        }
                    } else if !self.skip_ws {
                        self.buf.push(b' ');
                        self.skip_ws = true;
                    }
                }

                Chunk::Eof(_) => {
                    return Ok(());
                }
            }
        }
    }

    #[cfg(feature = "tree")]
    pub fn render_to_doc<'src>(&mut self, input: &'src I) -> Result<'src, I, Document<'src, I>> {
        let mut ctx = Context::new(input, None);
        let mut doc = Document::new();
        let mut id = doc.root().id();

        loop {
            let parsed = Parser::parse(&mut ctx)?;
            match &parsed {
                chunk @ Chunk::Tag(tag) => match tag.kind {
                    TagKind::Open if tag.name == TagName::Ansi => {
                        let mut node = doc.get_mut(id).unwrap();
                        node.append(chunk.clone());
                    }

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

    /// Writes the style to the buffer and saves the current style state.
    fn write_and_save<'src>(
        &mut self,
        ctx: &mut Context<'src, I>,
        name: &TagName<'src, I>,
        style: Style,
    ) {
        let diff = ctx.state.push(name.clone(), style);
        self.buf.extend_from_slice(&diff.to_string2().as_ref());
    }
}

impl<O> Renderer<str, O> {
    pub fn render<'src>(&mut self, input: &'src str) -> Result<'src, str, std::string::String> {
        self.write_input(input)?;
        let mut s = String::with_capacity(self.buf.len());

        // SAFETY: all bytes written to buffer are valid utf-8
        // since input is a string and all ansi escapes are byte strings
        unsafe {
            s.push_str(str::from_utf8_unchecked(&self.buf));
        }

        Ok(s)
    }
}

impl<I: ?Sized + Input, O: io::Write> Renderer<I, O> {
    pub fn write_str<'src>(&mut self, s: &'src I) -> Result<'src, I, ()> {
        self.write_input(s)?;
        self.output.write_all(&self.buf)?;
        self.buf.clear();

        Ok(())
    }
}

impl<O: io::Write> io::Write for Renderer<[u8], O> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.write_input(buf) {
            Ok(_) => {}
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, format!("{e:?}"))),
        }
        self.output.write_all(&self.buf)?;
        self.buf.clear();

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }
}

impl<O: fmt::Write> fmt::Write for Renderer<str, O> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        match self.write_input(s) {
            Ok(_) => {}
            Err(_) => return Err(fmt::Error),
        }

        // SAFETY: all bytes written to buffer are valid utf-8
        // since input is a string and all ansi escapes are byte strings
        unsafe {
            self.output.write_str(str::from_utf8_unchecked(&self.buf))?;
        }

        self.buf.clear();

        Ok(())
    }
}
