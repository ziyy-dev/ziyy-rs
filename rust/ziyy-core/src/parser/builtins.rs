use std::{collections::HashMap, sync::LazyLock};

use crate::{style::Condition, Style};

pub static BUILTIN_TAGS: LazyLock<HashMap<String, Style>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    let builtins = [
        ("a", Style::default()),
        (
            "b",
            Style {
                brightness: Condition::A,
                ..Default::default()
            },
        ),
        ("br", Style::default()),
        ("c", Style::default()),
        (
            "d",
            Style {
                brightness: Condition::B,
                ..Default::default()
            },
        ),
        (
            "h",
            Style {
                hide: true,
                ..Default::default()
            },
        ),
        (
            "k",
            Style {
                blink: true,
                ..Default::default()
            },
        ),
        ("let", Style::default()),
        (
            "r",
            Style {
                invert: true,
                ..Default::default()
            },
        ),
        (
            "i",
            Style {
                italics: true,
                ..Default::default()
            },
        ),
        ("p", Style::default()),
        (
            "s",
            Style {
                strike: true,
                ..Default::default()
            },
        ),
        (
            "u",
            Style {
                under: Condition::A,
                ..Default::default()
            },
        ),
        ("x", Style::default()),
        ("ziyy", Style::default()),
    ];

    for (key, value) in builtins {
        map.insert(key.to_owned(), value);
    }

    map
});
