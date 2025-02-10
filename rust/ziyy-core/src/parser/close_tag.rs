use std::io::Write;

use crate::{error::ErrorKind, Error, Style};

use super::{Parser, Tag};

impl<T: AsRef<str>> Parser<T> {
    pub(crate) fn parse_close_tag(&mut self, tag: &Tag) -> Result<(), Error> {
        let ctag = self.state.current_tag_name();
        //if ctag.is_none() {}
        Self::expect_tag(
            tag,
            ctag.unwrap(),
            ErrorKind::MisMatchedTags(ctag.unwrap().clone(), tag.name.clone()),
        )?;

        if let Some((_, _, style)) = self.state.pop() {
            if let Some(prev) = self.state.current_style() {
                if *prev == Style::default() && style != Style::default() {
                    let _ = self.buf.write(b"\x1b[m");
                } else {
                    let _ = self.buf.write(style.close().to_string().as_bytes());
                }
            }
        }

        Ok(())
    }
}
