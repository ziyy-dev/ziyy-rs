use std::collections::HashMap;

use crate::{get_num2, num::str_to_u32, Error};

use super::Context;
use super::{tag::Value, Parser, Tag, TagName};

impl<'src> Parser {
    pub(crate) fn parse_open_and_close_tag(
        &mut self,
        ctx: &mut Context<'src>,
        tag: Tag<'src>,
    ) -> Result<(), Error<'src>> {
        match tag.name {
            TagName::Br => {
                if let Value::Some(val) = tag.custom {
                    let n: usize = get_num2!(str_to_u32(&val, 10), tag) as usize;
                    let _ = self.buf.push_str(&"\n".repeat(n));
                } else {
                    let _ = self.buf.push('\n');
                }
            }

            TagName::Let => {
                if ctx.bindings.is_none() {
                    ctx.bindings = Some(HashMap::with_capacity(10));
                }

                if let Value::Some(ref name) = tag.custom {
                    ctx.bindings.as_mut().unwrap().insert(*name, tag.style);
                }
                self.skip_ws = true
            }
            _ => {}
        }
        Ok(())
    }
}
