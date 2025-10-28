use std::ops::Not;

use crate::context::Context;
use crate::error::Result;
use crate::parser::{Tag, TagName};
use crate::shared::Input;

use super::Renderer;

impl<O> Renderer<O> {
    pub(super) fn render_close_tag<'src, I: ?Sized + Input>(
        &mut self,
        ctx: &mut Context<'src, I>,
        tag: &Tag<'src, I>,
    ) -> Result<'src, I, ()> {
        let diff = ctx.state.pop_tag(tag)?;

        self.buf
            .extend_from_slice(&diff.not().to_string2().as_bytes());

        if tag.name == TagName::Pre {
            self.pre_ws -= 1;
        } else if tag.name == TagName::Ziyy {
            self.pre_ws += 1;
        }

        Ok(())
    }
}
