use ziyy_core::Chunk;
use ziyy_core::render_to_tree;
use ziyy_core::style::*;

fn assert_fg_colors_eq(source: &str, color: Color) {
    let doc = render_to_tree(source);
    let node = doc.root().first_child().unwrap();
    let chunk = node.value();
    let Chunk::Tag(tag) = chunk else { panic!() };
    let other = tag.style.fg_color();

    assert!(other.eq(&color), "expected {color:?} and got {other:?}");
}

fn assert_bg_colors_eq(source: &str, color: Color) {
    let doc = render_to_tree(source);
    let node = doc.root().first_child().unwrap();
    let chunk = node.value();
    let Chunk::Tag(tag) = chunk else { panic!() };
    let other = tag.style.bg_color();

    assert!(other.eq(&color), "expected {color:?} and got {other:?}");
}

fn test_ansi_color(n: u8, i: u8, fg_cases: &[&str], bg_cases: &[&str]) {
    for case in fg_cases {
        assert_fg_colors_eq(case, Color::AnsiColor(AnsiColor::try_from(n + i).unwrap()));
    }

    for case in bg_cases {
        assert_bg_colors_eq(case, Color::AnsiColor(AnsiColor::try_from(n + i).unwrap()));
    }
}

macro_rules! ansi_color_case {
    ( $n:expr, $color:expr ) => {
        test_ansi_color(
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
        test_ansi_color(
            $n,
            60,
            &[format!("<c {}='light'>", $color).as_str()],
            &[format!("<x {}='light'>", $color).as_str()],
        );
    };
}

#[test]
pub fn it_recognizes_black_color() {
    ansi_color_case!(0, "black");
}

#[test]
pub fn it_recognizes_red_color() {
    ansi_color_case!(1, "red");
}

#[test]
pub fn it_recognizes_green_color() {
    ansi_color_case!(2, "green");
}

#[test]
pub fn it_recognizes_yellow_color() {
    ansi_color_case!(3, "yellow");
}

#[test]
pub fn it_recognizes_blue_color() {
    ansi_color_case!(4, "blue");
}

#[test]
pub fn it_recognizes_magenta_color() {
    ansi_color_case!(5, "magenta");
}

#[test]
pub fn it_recognizes_cyan_color() {
    ansi_color_case!(6, "cyan");
}

#[test]
pub fn it_recognizes_white_color() {
    ansi_color_case!(7, "white");
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
            assert_fg_colors_eq(case, Color::Rgb(Rgb(r, g, b)));
        }

        for case in bg_cases {
            assert_bg_colors_eq(case, Color::Rgb(Rgb(r, g, b)));
        }
    }
}

#[test]
pub fn it_recognizes_hex_colors() {
    let test_cases = [(
        255,
        255,
        255,
        ["<div c='#fff'>", "<div c='#ffffff'>"],
        ["<div x='#fff'>", "<div x='#ffffff'>"],
    )];

    for (r, g, b, fg_cases, bg_cases) in test_cases {
        for case in fg_cases {
            assert_fg_colors_eq(case, Color::Rgb(Rgb(r, g, b)));
        }

        for case in bg_cases {
            assert_bg_colors_eq(case, Color::Rgb(Rgb(r, g, b)));
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
            assert_fg_colors_eq(case, Color::Ansi256(Ansi256(n)));
        }

        for case in bg_cases {
            assert_bg_colors_eq(case, Color::Ansi256(Ansi256(n)));
        }
    }
}
