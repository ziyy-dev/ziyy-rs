#![feature(rustc_private, test)]

extern crate test;

use test::{black_box, Bencher};
use ziyy_core::style;

#[bench]
fn style_help(b: &mut Bencher) {
    b.iter(|| {
        black_box(style(include_str!("help.zy")));
    });
}
