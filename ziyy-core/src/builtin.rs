use std::{collections::HashMap, sync::LazyLock};

use crate::parser::ansi::{Ansi, AnsiOptions, DuoEffect, Effect};

pub static BUILTIN_STYLES: LazyLock<HashMap<&str, Ansi>> = LazyLock::new(|| {
    [
        (
            "b",
            Ansi::with(AnsiOptions {
                brightness: DuoEffect::A,
                ..Default::default()
            }),
        ),
        (
            "d",
            Ansi::with(AnsiOptions {
                brightness: DuoEffect::B,
                ..Default::default()
            }),
        ),
        (
            "h",
            Ansi::with(AnsiOptions {
                hidden: Effect::Apply,
                ..Default::default()
            }),
        ),
        (
            "i",
            Ansi::with(AnsiOptions {
                italics: Effect::Apply,
                ..Default::default()
            }),
        ),
        (
            "k",
            Ansi::with(AnsiOptions {
                blink: Effect::Apply,
                ..Default::default()
            }),
        ),
        (
            "r",
            Ansi::with(AnsiOptions {
                negetive: Effect::Apply,
                ..Default::default()
            }),
        ),
        (
            "s",
            Ansi::with(AnsiOptions {
                strike: Effect::Apply,
                ..Default::default()
            }),
        ),
        (
            "u",
            Ansi::with(AnsiOptions {
                under: DuoEffect::A,
                ..Default::default()
            }),
        ),
        (
            "uu",
            Ansi::with(AnsiOptions {
                under: DuoEffect::B,
                ..Default::default()
            }),
        ),
    ]
    .into()
});

pub static BUILTIN_TAGS: &[&str] = &[
    "a", "b", "blink", "br", "d", "del", "dim", "div", "em", "h", "i", "input", "ins", "k", "o",
    "over", "p", "r", "s", "script", "span", "strong", "style", "table", "td", "th", "tr", "u",
    "uu", "ziyy",
];
