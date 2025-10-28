use std::collections::HashMap;

use crate::context::Context;
use crate::error::Result;
use crate::num::input_to_u8;
use crate::parser::{Tag, TagName};
use crate::shared::{Input, Value};

use super::Renderer;

impl<O> Renderer<O> {
    pub(super) fn render_self_close_tag<'src, I: ?Sized + Input>(
        &mut self,
        ctx: &mut Context<'src, I>,
        tag: &Tag<'src, I>,
    ) -> Result<'src, I, ()> {
        match tag.name {
            TagName::Br => {
                if let Value::Some(val) = tag.custom {
                    let n: usize = get_num2!(input_to_u8(val, 10), tag) as usize;
                    for _ in 0..n {
                        self.buf.push(b'\n');
                    }
                } else {
                    self.buf.push(b'\n');
                }
            }

            #[cfg(feature = "bindings")]
            TagName::Let => {
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
            }
            _ => {}
        }
        Ok(())
    }
}
