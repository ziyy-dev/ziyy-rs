use std::ops::Not;

use crate::context::Context;
use crate::error::Result;
use crate::parser::{Tag, TagName};
use crate::shared::Input;

use super::Renderer;

impl<I: ?Sized + Input, O> Renderer<I, O> {
    pub(super) fn render_close_tag<'src>(
        &mut self,
        ctx: &mut Context<'src, I>,
        tag: &Tag<'src, I>,
    ) -> Result<'src, I, ()> {
        #[cfg(not(feature = "terminfo"))]
        {
            let diff = ctx.state.pop(tag)?;

            self.buf
                .extend_from_slice(&diff.not().to_string2().as_bytes());
        }

        #[cfg(feature = "terminfo")]
        self.buf
            .extend_from_slice(tag.style.to_string2().as_bytes());

        if tag.name == TagName::Pre {
            self.pre_ws -= 1;
        } else if tag.name == TagName::Ziyy {
            self.pre_ws += 1;
        }

        Ok(())
    }
}
