use std::{borrow::Cow, rc::Rc};
use ziyy_core::Color;
use ziyy_core::{Document, Parser, Resolver, Result, Splitter};

fn try_style<'a>(source: &'a Cow<'a, str>) -> Result<Rc<Document<'a>>> {
    let mut splitter = Splitter::new();
    let frags = splitter.split(source).unwrap();

    let parser = Parser::new(false);
    let chunks = parser.parse(frags);

    let mut resolver = Resolver::new(false);
    resolver.resolve(chunks)
}

fn assert_fg_colors_eq(source: &str, color: Color) {
    let source = Cow::Borrowed(source);
    let styled = try_style(&source);
    let _ = styled.is_ok_and(|doc| {
        let node = doc.get(1);
        let chunk = node.chunk().borrow();
        let other = chunk.tag().unwrap().fg_color();
        assert!(other.eq(&color), "expected {color} and got {other}");
        true
    });
}

fn assert_bg_colors_eq(source: &str, color: Color) {
    let source = Cow::Borrowed(source);
    let styled = try_style(&source);
    let _ = styled.is_ok_and(|doc| {
        let node = doc.get(1);
        let chunk = node.chunk().borrow();
        let other = chunk.tag().unwrap().bg_color();
        assert!(other.eq(&color), "expected {color} and got {other}");
        true
    });
}

/* #[test]
pub fn supported_colors() {
    let test_cases = [
        (Color::four_bit(30), ["<c black>", "<div c='black'>"]),
        (Color::four_bit(31), ["<c red>", "<div c='red'>"]),
        (Color::four_bit(32), ["<c green>", "<div c='green'>"]),
        (Color::four_bit(33), ["<c yellow>", "<div c='yellow'>"]),
        (Color::four_bit(34), ["<c blue>", "<div c='blue'>"]),
        (Color::four_bit(35), ["<c magenta>", "<div c='magenta'>"]),
        (Color::four_bit(36), ["<c cyan>", "<div c='cyan'>"]),
        (Color::four_bit(37), ["<c white>", "<div c='white'>"]),
        (
            Color::fg_rgb(150, 75, 0),
            ["<c rgb='150, 75, 0'>", "<div c='rgb(150, 75, 0)'>"],
        ),
        (
            Color::fg_fixed(200),
            ["<c fixed='200'>", "<div c='fixed(200)'>"],
        ),
        (
            Color::fg_hex("#fff"),
            ["<div c='#fff'>", "<div c='#ffffff'>"],
        ),
        (
            Color::fg_rgb(0, 150, 62),
            ["<c rgb='0, 150, 62'>", "<div c='#00963e'>"],
        ),
    ];

    for (color, cases) in test_cases {
        for case in cases {
            assert_fg_colors_eq(case, color.clone());
        }
    }
} */

fn test_ansi_4_bit(n: u8, i: u8, fg_cases: &[&str], bg_cases: &[&str]) {
    for case in fg_cases {
        assert_fg_colors_eq(case, Color::four_bit(30 + n + i));
    }

    for case in bg_cases {
        assert_bg_colors_eq(case, Color::four_bit(40 + n + i));
    }
}

macro_rules! ansi_4_bit_case {
    ( $n:expr, $color:expr ) => {
        test_ansi_4_bit(
            $n,
            0,
            &[
                format!("<c {}>", $color).as_str(),
                format!("<div c='{}'>", $color).as_str(),
            ],
            &[
                format!("<x {}>", $color).as_str(),
                format!("<div x='{}'>", $color).as_str(),
            ],
        );
        test_ansi_4_bit(
            $n,
            60,
            &[format!("<c {}='light'>", $color).as_str()],
            &[format!("<x {}='light'>", $color).as_str()],
        );
    };
}

#[test]
pub fn it_recognizes_black_color() {
    ansi_4_bit_case!(0, "black");
}

#[test]
pub fn it_recognizes_red_color() {
    ansi_4_bit_case!(1, "red");
}

#[test]
pub fn it_recognizes_green_color() {
    ansi_4_bit_case!(2, "green");
}

#[test]
pub fn it_recognizes_yellow_color() {
    ansi_4_bit_case!(3, "yellow");
}

#[test]
pub fn it_recognizes_blue_color() {
    ansi_4_bit_case!(4, "blue");
}

#[test]
pub fn it_recognizes_magenta_color() {
    ansi_4_bit_case!(5, "magenta");
}

#[test]
pub fn it_recognizes_cyan_color() {
    ansi_4_bit_case!(6, "cyan");
}

#[test]
pub fn it_recognizes_white_color() {
    ansi_4_bit_case!(7, "white");
}

#[test]
pub fn it_recognizes_rgb_colors() {
    let test_cases = [(
        150,
        75,
        0,
        ["<c rgb='150, 75, 0'>", "<div c='rgb(150, 75, 0)'>"],
        ["<x rgb='150, 75, 0'>", "<div x='rgb(150, 75, 0)'>"],
    )];

    for (r, g, b, fg_cases, bg_cases) in test_cases {
        for case in fg_cases {
            assert_fg_colors_eq(case, Color::fg_rgb(r, g, b));
        }

        for case in bg_cases {
            assert_bg_colors_eq(case, Color::bg_rgb(r, g, b));
        }
    }
}

#[test]
pub fn it_recognizes_hex_colors() {
    let test_cases = [(
        "#fff",
        255,
        255,
        255,
        ["<div c='#fff'>", "<div c='#ffffff'>"],
        ["<div x='#fff'>", "<div x='#ffffff'>"],
    )];

    for (hex, r, g, b, fg_cases, bg_cases) in test_cases {
        for case in fg_cases {
            assert_fg_colors_eq(case, Color::fg_rgb(r, g, b));
            assert_fg_colors_eq(case, Color::fg_hex(hex));
        }

        for case in bg_cases {
            assert_bg_colors_eq(case, Color::bg_rgb(r, g, b));
            assert_bg_colors_eq(case, Color::bg_hex(hex));
        }
    }
}

#[test]
pub fn it_recognizes_fixed_colors() {
    let test_cases = [(
        225,
        ["<c fixed='225'>", "<div c='fixed(225)'>"],
        ["<x fixed='225'>", "<div x='fixed(225)'>"],
    )];

    for (n, fg_cases, bg_cases) in test_cases {
        for case in fg_cases {
            assert_fg_colors_eq(case, Color::fg_fixed(n));
        }

        for case in bg_cases {
            assert_bg_colors_eq(case, Color::bg_fixed(n));
        }
    }
}
