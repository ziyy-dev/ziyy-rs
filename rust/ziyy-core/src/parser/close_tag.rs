use super::{Context, Parser, Tag, TagName};
use crate::{error::ErrorKind, Error};
use std::ops::Not;

impl<'src> Parser {
    pub(crate) fn parse_close_tag(
        &mut self,
        ctx: &mut Context<'src>,
        tag: &Tag<'src>,
    ) -> Result<(), Error<'src>> {
        let ctag = ctx.state.previous_tag_name();
        Self::expect_tag(
            tag,
            ctag.unwrap(),
            ErrorKind::MisMatchedTags {
                open: ctag.unwrap().clone(),
                close: tag.name.clone(),
            },
        )?;

        if tag.name == TagName::Pre {
            self.pre_ws -= 1;
        } else if tag.name == TagName::Ziyy {
            self.pre_ws += 1;
        }

        let current_tag = ctx.state.pop().unwrap_or_default();
        self.buf.push_str(&current_tag.2.not().to_string());

        Ok(())
    }
}
