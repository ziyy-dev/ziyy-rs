#![feature(rustc_private, test)]

extern crate test;

use test::{Bencher, black_box};
use ziyy_core::style;

#[bench]
fn style_help(b: &mut Bencher) {
    b.iter(|| {
        black_box(style(include_str!("help.zy")));
    });
}