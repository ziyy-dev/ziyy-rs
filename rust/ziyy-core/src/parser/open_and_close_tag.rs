use std::{collections::HashMap, io::Write};

use crate::{get_num2, num::str_to_u32, Error};

use super::{tag::Value, Parser, Tag, TagName};

impl<T: AsRef<str>> Parser<T> {
    pub(crate) fn parse_open_and_close_tag(&mut self, tag: Tag) -> Result<(), Error> {
        match tag.name {
            TagName::Br => {
                if let Value::Some(val) = tag.custom {
                    let n: usize = get_num2!(str_to_u32(&val, 10), tag) as usize;
                    let _ = self.buf.write("\n".repeat(n).as_bytes());
                } else {
                    let _ = self.buf.write("\n".as_bytes());
                }
            }

            TagName::Let => {
                if self.bindings.is_none() {
                    self.bindings = Some(HashMap::with_capacity(8));
                }

                if let Value::Some(ref name) = tag.custom {
                    self.bindings
                        .as_mut()
                        .unwrap()
                        .insert(name.clone(), tag.style);
                }
            }
            _ => {}
        }
        Ok(())
    }
}
