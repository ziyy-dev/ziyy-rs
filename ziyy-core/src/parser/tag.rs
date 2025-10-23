#![allow(missing_docs)]

use std::fmt::Display;

use crate::scanner::span::Span;
use crate::style::{
    Ansi256, AnsiColor, Blink, Color, Delete, Font, FontStyle, Frame, Hide, Intensity, Invert,
    Overline, PropSpace, Reserved1, Reserved2, Rgb, Style, Underline,
};

#[derive(PartialEq, Debug, Clone)]
pub enum Value<'src> {
    Bool,
    Some(&'src str),
    None,
}

/// Ziyy Tag.
#[derive(PartialEq, Debug, Clone)]
pub struct Tag<'src> {
    /// Name of Tag.
    pub name: TagName<'src>,
    /// Kind of Tag.
    pub kind: TagKind,
    /// Custom information.
    pub custom: Value<'src>,
    /// Style information of the Tag.
    pub style: Style,
    /// Class.
    pub class: Value<'src>,
    /// Span
    pub span: Span,
}

impl<'src> Tag<'src> {
    /// Creates new Tag
    #[must_use]
    pub fn new(name: TagName<'src>, kind: TagKind) -> Self {
        let mut style = Style::new();

        match name {
            TagName::B => {
                style.set_prev_intensity(style.intensity());
                style.set_intensity(Intensity::Bold);
            }
            TagName::D => {
                style.set_prev_intensity(style.intensity());
                style.set_intensity(Intensity::Dim);
            }
            TagName::H => style.set_hide(Hide::Set),
            TagName::I => style.set_font_style(FontStyle::Italics),
            TagName::K => style.set_blink(Blink::Slow),
            TagName::R => style.set_invert(Invert::Set),
            TagName::S => style.set_delete(Delete::Set),
            TagName::U => style.set_underline(Underline::Single),
            _ => {}
        }

        Self {
            kind,
            name,
            custom: Value::None,
            style,
            class: Value::None,
            span: Span::initial(),
        }
    }

    /// Inherits style properties from the source style.
    pub fn inherit(&mut self, src: &Style) {
        macro_rules! inherit {
            ( $set_x:tt $x:tt ) => {
                if self.style.$x().is_unset() & src.$x().is_set() {
                    self.style.$set_x(src.$x());
                }
            };
        }

        inherit!(set_intensity intensity);
        inherit!(set_font_style font_style);
        inherit!(set_underline underline);
        inherit!(set_blink blink);
        inherit!(set_invert invert);
        inherit!(set_hide hide);
        inherit!(set_delete delete);
        inherit!(set_font font);
        inherit!(set_prop_space prop_space);
        inherit!(set_fg_color fg_color);
        inherit!(set_bg_color bg_color);
        inherit!(set_frame frame);
        inherit!(set_overline overline);
        inherit!(set_reserved1 reserved1);
        inherit!(set_reserved2 reserved2);
        inherit!(set_ul_color ul_color);
        // inherit!(set_ideogram ideogram);
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn parse_from_ansi(source: &str, span: Span) -> Self {
        let mut parts = source.split(';').peekable();

        let mut style = Style::default();

        loop {
            let part = parts.next();

            let Some(part) = part else { break };

            match part {
                "" | "0" => style = Style::default(),
                "1" => style.set_intensity(Intensity::Bold),
                "2" => style.set_intensity(Intensity::Dim),
                "3" => style.set_font_style(FontStyle::Italics),
                "4" => style.set_underline(Underline::Single),
                "4:3" => style.set_underline(Underline::Curly),
                "4:4" => style.set_underline(Underline::Dotted),
                "4:5" => style.set_underline(Underline::Dashed),
                "5" => style.set_blink(Blink::Slow),
                "6" => style.set_blink(Blink::Fast),
                "7" => style.set_invert(Invert::Set),
                "8" => style.set_hide(Hide::Set),
                "9" => style.set_delete(Delete::Set),
                "10" => style.set_font(Font::Primary),
                "11" => style.set_font(Font::FirstAlt),
                "12" => style.set_font(Font::SecondAlt),
                "13" => style.set_font(Font::ThirdAlt),
                "14" => style.set_font(Font::FourthAlt),
                "15" => style.set_font(Font::FifthAlt),
                "16" => style.set_font(Font::SixthAlt),
                "17" => style.set_font(Font::SeventhAlt),
                "18" => style.set_font(Font::EighthAlt),
                "19" => style.set_font(Font::NinthAlt),
                "20" => style.set_font_style(FontStyle::Fraktur),
                "21" => style.set_underline(Underline::Double),
                "22" => style.set_intensity(Intensity::Unset),
                "23" => style.set_font_style(FontStyle::Unset),
                "24" => style.set_underline(Underline::Unset),
                "25" => style.set_blink(Blink::Unset),
                "26" => style.set_prop_space(PropSpace::Set),
                "27" => style.set_invert(Invert::Unset),
                "28" => style.set_hide(Hide::Unset),
                "29" => style.set_delete(Delete::Unset),
                "30" => style.set_fg_color(Color::AnsiColor(AnsiColor::Black)),
                "31" => style.set_fg_color(Color::AnsiColor(AnsiColor::Red)),
                "32" => style.set_fg_color(Color::AnsiColor(AnsiColor::Green)),
                "33" => style.set_fg_color(Color::AnsiColor(AnsiColor::Yellow)),
                "34" => style.set_fg_color(Color::AnsiColor(AnsiColor::Blue)),
                "35" => style.set_fg_color(Color::AnsiColor(AnsiColor::Magenta)),
                "36" => style.set_fg_color(Color::AnsiColor(AnsiColor::Cyan)),
                "37" => style.set_fg_color(Color::AnsiColor(AnsiColor::White)),
                "38" => {
                    if part == "2" {
                        let r = match parts.next() {
                            Some(s) => match s.parse::<u8>() {
                                Ok(n) => n,
                                Err(_) => continue,
                            },
                            None => continue,
                        };

                        let g = match parts.next() {
                            Some(s) => match s.parse::<u8>() {
                                Ok(n) => n,
                                Err(_) => continue,
                            },
                            None => continue,
                        };

                        let b = match parts.next() {
                            Some(s) => match s.parse::<u8>() {
                                Ok(n) => n,
                                Err(_) => continue,
                            },
                            None => continue,
                        };

                        style.set_fg_color(Color::Rgb(Rgb(r, g, b)));
                    }

                    if part == "5" {
                        let fixed = match parts.next() {
                            Some(s) => match s.parse::<u8>() {
                                Ok(n) => n,
                                Err(_) => continue,
                            },
                            None => continue,
                        };

                        style.set_fg_color(Color::Ansi256(Ansi256(fixed)));
                    }
                }
                "39" => style.set_fg_color(Color::Unset),
                "40" => style.set_bg_color(Color::AnsiColor(AnsiColor::Black)),
                "41" => style.set_bg_color(Color::AnsiColor(AnsiColor::Red)),
                "42" => style.set_bg_color(Color::AnsiColor(AnsiColor::Green)),
                "43" => style.set_bg_color(Color::AnsiColor(AnsiColor::Yellow)),
                "44" => style.set_bg_color(Color::AnsiColor(AnsiColor::Blue)),
                "45" => style.set_bg_color(Color::AnsiColor(AnsiColor::Magenta)),
                "46" => style.set_bg_color(Color::AnsiColor(AnsiColor::Cyan)),
                "47" => style.set_bg_color(Color::AnsiColor(AnsiColor::White)),
                "48" => {
                    if part == "2" {
                        let r = match parts.next() {
                            Some(s) => match s.parse::<u8>() {
                                Ok(n) => n,
                                Err(_) => continue,
                            },
                            None => continue,
                        };

                        let g = match parts.next() {
                            Some(s) => match s.parse::<u8>() {
                                Ok(n) => n,
                                Err(_) => continue,
                            },
                            None => continue,
                        };

                        let b = match parts.next() {
                            Some(s) => match s.parse::<u8>() {
                                Ok(n) => n,
                                Err(_) => continue,
                            },
                            None => continue,
                        };

                        style.set_fg_color(Color::Rgb(Rgb(r, g, b)));
                    }

                    if part == "5" {
                        let fixed = match parts.next() {
                            Some(s) => match s.parse::<u8>() {
                                Ok(n) => n,
                                Err(_) => continue,
                            },
                            None => continue,
                        };

                        style.set_bg_color(Color::Ansi256(Ansi256(fixed)));
                    }
                }
                "49" => style.set_bg_color(Color::Unset),
                "50" => style.set_prop_space(PropSpace::Unset),
                "51" => style.set_frame(Frame::Framed),
                "52" => style.set_frame(Frame::Encircled),
                "53" => style.set_overline(Overline::Set),
                "54" => style.set_frame(Frame::Unset),
                "55" => style.set_overline(Overline::Unset),
                "56" => style.set_reserved1(Reserved1::Yes),
                "57" => style.set_reserved2(Reserved2::Yes),
                "58" => {
                    if part == "2" {
                        let r = match parts.next() {
                            Some(s) => match s.parse::<u8>() {
                                Ok(n) => n,
                                Err(_) => continue,
                            },
                            None => continue,
                        };

                        let g = match parts.next() {
                            Some(s) => match s.parse::<u8>() {
                                Ok(n) => n,
                                Err(_) => continue,
                            },
                            None => continue,
                        };

                        let b = match parts.next() {
                            Some(s) => match s.parse::<u8>() {
                                Ok(n) => n,
                                Err(_) => continue,
                            },
                            None => continue,
                        };

                        style.set_ul_color(Color::Rgb(Rgb(r, g, b)));
                    }

                    if part == "5" {
                        let fixed = match parts.next() {
                            Some(s) => match s.parse::<u8>() {
                                Ok(n) => n,
                                Err(_) => continue,
                            },
                            None => continue,
                        };

                        style.set_ul_color(Color::Ansi256(Ansi256(fixed)));
                    }
                }
                "59" => style.set_ul_color(Color::Unset),

                "90" => style.set_fg_color(Color::AnsiColor(AnsiColor::BrightBlack)),
                "91" => style.set_fg_color(Color::AnsiColor(AnsiColor::BrightRed)),
                "92" => style.set_fg_color(Color::AnsiColor(AnsiColor::BrightGreen)),
                "93" => style.set_fg_color(Color::AnsiColor(AnsiColor::BrightYellow)),
                "94" => style.set_fg_color(Color::AnsiColor(AnsiColor::BrightBlue)),
                "95" => style.set_fg_color(Color::AnsiColor(AnsiColor::BrightMagenta)),
                "96" => style.set_fg_color(Color::AnsiColor(AnsiColor::BrightCyan)),
                "97" => style.set_fg_color(Color::AnsiColor(AnsiColor::BrightWhite)),

                "100" => style.set_bg_color(Color::AnsiColor(AnsiColor::BrightBlack)),
                "101" => style.set_bg_color(Color::AnsiColor(AnsiColor::BrightRed)),
                "102" => style.set_bg_color(Color::AnsiColor(AnsiColor::BrightGreen)),
                "103" => style.set_bg_color(Color::AnsiColor(AnsiColor::BrightYellow)),
                "104" => style.set_bg_color(Color::AnsiColor(AnsiColor::BrightBlue)),
                "105" => style.set_bg_color(Color::AnsiColor(AnsiColor::BrightMagenta)),
                "106" => style.set_bg_color(Color::AnsiColor(AnsiColor::BrightCyan)),
                "107" => style.set_bg_color(Color::AnsiColor(AnsiColor::BrightWhite)),
                _ => {}
            }
        }

        Tag {
            name: TagName::Ansi,
            kind: TagKind::Open,
            custom: Value::None,
            style,
            class: Value::None,
            span,
        }
    }
}

impl Display for Tag<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            TagKind::Open | TagKind::SelfClose => f.write_str("<"),
            TagKind::Close => f.write_str("</"),
        }?;

        self.name.fmt(f)?;

        match self.kind {
            TagKind::Open | TagKind::Close => f.write_str(">"),
            TagKind::SelfClose => f.write_str("/>"),
        }?;

        Ok(())
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TagKind {
    Open,
    Close,
    SelfClose,
}

#[derive(Default, PartialEq, Debug, Clone)]
pub enum TagName<'src> {
    A,
    Any(&'src str),
    Ansi,
    B,
    Br,
    C,
    Code,
    D,
    Div,
    H,
    K,
    I,
    Let,
    P,
    Pre,
    R,
    S,
    Span,
    U,
    X,
    Ziyy,
    Empty,
    #[default]
    None,
}

impl Display for TagName<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TagName::A => "a",
            TagName::Any(any) => any,
            TagName::Ansi => "$ansi",
            TagName::B => "b",
            TagName::Br => "br",
            TagName::C => "c",
            TagName::Code => "code",
            TagName::D => "d",
            TagName::Div => "div",
            TagName::H => "h",
            TagName::I => "i",
            TagName::K => "k",
            TagName::Let => "let",
            TagName::P => "p",
            TagName::Pre => "pre",
            TagName::R => "r",
            TagName::S => "s",
            TagName::Span => "span",
            TagName::U => "u",
            TagName::X => "x",
            TagName::Ziyy => "ziyy",
            TagName::Empty | TagName::None => "",
        })
    }
}
