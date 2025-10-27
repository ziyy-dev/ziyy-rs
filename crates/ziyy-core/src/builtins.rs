use std::{collections::HashMap, sync::LazyLock};

use crate::parser::{match_tag_name, Tag, TagKind, TagName};
use crate::scanner::Scanner;
use crate::shared::Input;
use crate::style::Style;

pub static BUILTIN_STYLES: LazyLock<HashMap<&[u8], Style>> = LazyLock::new(|| {
    [
        (&b"b"[..], Tag::new(TagName::<str>::B, TagKind::Open).style),
        (b"d", Tag::new(TagName::<str>::D, TagKind::Open).style),
        (b"h", Tag::new(TagName::<str>::H, TagKind::Open).style),
        (b"k", Tag::new(TagName::<str>::K, TagKind::Open).style),
        (b"r", Tag::new(TagName::<str>::R, TagKind::Open).style),
        (b"i", Tag::new(TagName::<str>::I, TagKind::Open).style),
        (b"s", Tag::new(TagName::<str>::S, TagKind::Open).style),
        (b"u", Tag::new(TagName::<str>::U, TagKind::Open).style),
    ]
    .into()
});

#[inline]
pub fn is_builtin_tag<I: ?Sized + Input>(input: &I) -> bool {
    let mut scanner = Scanner::new(input);
    if let Some(token) = scanner.scan_one() {
        if let Ok(name) = match_tag_name(&token) {
            return !matches!(name, TagName::Any(_));
        }
    }

    false
}
