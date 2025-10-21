use crate::scanner::Scanner;
use crate::Style;
use std::{collections::HashMap, sync::LazyLock};

use super::parse_chunk::match_tag_name;
use super::{Tag, TagKind, TagName};

pub static BUILTIN_STYLES: LazyLock<HashMap<&str, Style>> = LazyLock::new(|| {
    [
        ("b", Tag::new(TagName::B, TagKind::Open).style),
        ("d", Tag::new(TagName::D, TagKind::Open).style),
        ("h", Tag::new(TagName::H, TagKind::Open).style),
        ("k", Tag::new(TagName::K, TagKind::Open).style),
        ("r", Tag::new(TagName::R, TagKind::Open).style),
        ("i", Tag::new(TagName::I, TagKind::Open).style),
        ("s", Tag::new(TagName::S, TagKind::Open).style),
        ("u", Tag::new(TagName::U, TagKind::Open).style),
    ]
    .into()
});

#[inline]
pub fn is_builtin_tag(s: &str) -> bool {
    let mut scanner = Scanner::new(s);
    if let Some(token) = scanner.scan_one() {
        if let Ok(name) = match_tag_name(&token) {
            return !matches!(name, TagName::Any(_));
        }
    }

    false
}
