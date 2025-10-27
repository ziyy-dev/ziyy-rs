#![allow(missing_docs)]

use std::fmt::{Debug, Display};

use crate::num::input_to_u8;
use crate::shared::{Input, Span, Value};
use crate::style::{
    Ansi256, AnsiColor, Blink, Color, Delete, FontStyle, Hide, Intensity, Invert, Rgb, Style,
    Underline,
};
#[cfg(feature = "uncommon")]
use crate::style::{Font, Frame, Overline, PropSpace, Reserved1, Reserved2};

/// Ziyy Tag.
#[derive(PartialEq)]
pub struct Tag<'src, I: ?Sized + Input> {
    /// Name of Tag.
    pub name: TagName<'src, I>,
    /// Kind of Tag.
    pub kind: TagKind,
    /// Custom information.
    pub custom: Value<'src, I>,
    /// Style information of the Tag.
    pub style: Style,
    /// Class.
    pub class: Value<'src, I>,
    /// Span
    pub span: Span,
}

impl<'src, I: ?Sized + Input> Tag<'src, I> {
    /// Creates new Tag
    #[must_use]
    #[inline]
    pub fn new(name: TagName<'src, I>, kind: TagKind) -> Self {
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

    pub(crate) const fn root() -> Self {
        Tag {
            name: TagName::Root,
            kind: TagKind::Open,
            custom: Value::None,
            style: Style::new(),
            class: Value::None,
            span: Span::inserted(),
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
        #[cfg(feature = "uncommon")]
        inherit!(set_font font);
        #[cfg(feature = "uncommon")]
        inherit!(set_prop_space prop_space);
        inherit!(set_fg_color fg_color);
        inherit!(set_bg_color bg_color);
        #[cfg(feature = "uncommon")]
        inherit!(set_frame frame);
        #[cfg(feature = "uncommon")]
        inherit!(set_overline overline);
        #[cfg(feature = "uncommon")]
        inherit!(set_reserved1 reserved1);
        #[cfg(feature = "uncommon")]
        inherit!(set_reserved2 reserved2);
        inherit!(set_ul_color ul_color);
        // inherit!(set_ideogram ideogram);
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn parse_from_ansi(source: &'src I, span: Span) -> Self {
        let mut parts = source.as_ref().split(|n| *n == b';').peekable();

        let mut style = Style::default();

        loop {
            let part = parts.next();

            let Some(part) = part else { break };

            match part {
                b"" | b"0" => style = Style::default(),
                b"1" => style.set_intensity(Intensity::Bold),
                b"2" => style.set_intensity(Intensity::Dim),
                b"3" => style.set_font_style(FontStyle::Italics),
                b"4" => style.set_underline(Underline::Single),
                b"4:3" => style.set_underline(Underline::Curly),
                b"4:4" => style.set_underline(Underline::Dotted),
                b"4:5" => style.set_underline(Underline::Dashed),
                b"5" => style.set_blink(Blink::Slow),
                b"6" => style.set_blink(Blink::Fast),
                b"7" => style.set_invert(Invert::Set),
                b"8" => style.set_hide(Hide::Set),
                b"9" => style.set_delete(Delete::Set),
                #[cfg(feature = "uncommon")]
                b"10" => style.set_font(Font::Primary),
                #[cfg(feature = "uncommon")]
                b"11" => style.set_font(Font::FirstAlt),
                #[cfg(feature = "uncommon")]
                b"12" => style.set_font(Font::SecondAlt),
                #[cfg(feature = "uncommon")]
                b"13" => style.set_font(Font::ThirdAlt),
                #[cfg(feature = "uncommon")]
                b"14" => style.set_font(Font::FourthAlt),
                #[cfg(feature = "uncommon")]
                b"15" => style.set_font(Font::FifthAlt),
                #[cfg(feature = "uncommon")]
                b"16" => style.set_font(Font::SixthAlt),
                #[cfg(feature = "uncommon")]
                b"17" => style.set_font(Font::SeventhAlt),
                #[cfg(feature = "uncommon")]
                b"18" => style.set_font(Font::EighthAlt),
                #[cfg(feature = "uncommon")]
                b"19" => style.set_font(Font::NinthAlt),
                b"20" => style.set_font_style(FontStyle::Fraktur),
                b"21" => style.set_underline(Underline::Double),
                b"22" => style.set_intensity(Intensity::Unset),
                b"23" => style.set_font_style(FontStyle::Unset),
                b"24" => style.set_underline(Underline::Unset),
                b"25" => style.set_blink(Blink::Unset),
                #[cfg(feature = "uncommon")]
                b"26" => style.set_prop_space(PropSpace::Set),
                b"27" => style.set_invert(Invert::Unset),
                b"28" => style.set_hide(Hide::Unset),
                b"29" => style.set_delete(Delete::Unset),
                b"30" => style.set_fg_color(Color::AnsiColor(AnsiColor::Black)),
                b"31" => style.set_fg_color(Color::AnsiColor(AnsiColor::Red)),
                b"32" => style.set_fg_color(Color::AnsiColor(AnsiColor::Green)),
                b"33" => style.set_fg_color(Color::AnsiColor(AnsiColor::Yellow)),
                b"34" => style.set_fg_color(Color::AnsiColor(AnsiColor::Blue)),
                b"35" => style.set_fg_color(Color::AnsiColor(AnsiColor::Magenta)),
                b"36" => style.set_fg_color(Color::AnsiColor(AnsiColor::Cyan)),
                b"37" => style.set_fg_color(Color::AnsiColor(AnsiColor::White)),
                b"38" => {
                    if part == b"2" {
                        let r = match parts.next() {
                            Some(s) => match input_to_u8(s, 10) {
                                Ok(n) => n,
                                Err(_) => continue,
                            },
                            None => continue,
                        };

                        let g = match parts.next() {
                            Some(s) => match input_to_u8(s, 10) {
                                Ok(n) => n,
                                Err(_) => continue,
                            },
                            None => continue,
                        };

                        let b = match parts.next() {
                            Some(s) => match input_to_u8(s, 10) {
                                Ok(n) => n,
                                Err(_) => continue,
                            },
                            None => continue,
                        };

                        style.set_fg_color(Color::Rgb(Rgb(r, g, b)));
                    }

                    if part == b"5" {
                        let fixed = match parts.next() {
                            Some(s) => match input_to_u8(s, 10) {
                                Ok(n) => n,
                                Err(_) => continue,
                            },
                            None => continue,
                        };

                        style.set_fg_color(Color::Ansi256(Ansi256(fixed)));
                    }
                }
                b"39" => style.set_fg_color(Color::Unset),
                b"40" => style.set_bg_color(Color::AnsiColor(AnsiColor::Black)),
                b"41" => style.set_bg_color(Color::AnsiColor(AnsiColor::Red)),
                b"42" => style.set_bg_color(Color::AnsiColor(AnsiColor::Green)),
                b"43" => style.set_bg_color(Color::AnsiColor(AnsiColor::Yellow)),
                b"44" => style.set_bg_color(Color::AnsiColor(AnsiColor::Blue)),
                b"45" => style.set_bg_color(Color::AnsiColor(AnsiColor::Magenta)),
                b"46" => style.set_bg_color(Color::AnsiColor(AnsiColor::Cyan)),
                b"47" => style.set_bg_color(Color::AnsiColor(AnsiColor::White)),
                b"48" => {
                    if part == b"2" {
                        let r = match parts.next() {
                            Some(s) => match input_to_u8(s, 10) {
                                Ok(n) => n,
                                Err(_) => continue,
                            },
                            None => continue,
                        };

                        let g = match parts.next() {
                            Some(s) => match input_to_u8(s, 10) {
                                Ok(n) => n,
                                Err(_) => continue,
                            },
                            None => continue,
                        };

                        let b = match parts.next() {
                            Some(s) => match input_to_u8(s, 10) {
                                Ok(n) => n,
                                Err(_) => continue,
                            },
                            None => continue,
                        };

                        style.set_fg_color(Color::Rgb(Rgb(r, g, b)));
                    }

                    if part == b"5" {
                        let fixed = match parts.next() {
                            Some(s) => match input_to_u8(s, 10) {
                                Ok(n) => n,
                                Err(_) => continue,
                            },
                            None => continue,
                        };

                        style.set_bg_color(Color::Ansi256(Ansi256(fixed)));
                    }
                }
                b"49" => style.set_bg_color(Color::Unset),
                #[cfg(feature = "uncommon")]
                b"50" => style.set_prop_space(PropSpace::Unset),
                #[cfg(feature = "uncommon")]
                b"51" => style.set_frame(Frame::Framed),
                #[cfg(feature = "uncommon")]
                b"52" => style.set_frame(Frame::Encircled),
                #[cfg(feature = "uncommon")]
                b"53" => style.set_overline(Overline::Set),
                #[cfg(feature = "uncommon")]
                b"54" => style.set_frame(Frame::Unset),
                #[cfg(feature = "uncommon")]
                b"55" => style.set_overline(Overline::Unset),
                #[cfg(feature = "uncommon")]
                b"56" => style.set_reserved1(Reserved1::Yes),
                #[cfg(feature = "uncommon")]
                b"57" => style.set_reserved2(Reserved2::Yes),
                b"58" => {
                    if part == b"2" {
                        let r = match parts.next() {
                            Some(s) => match input_to_u8(s, 10) {
                                Ok(n) => n,
                                Err(_) => continue,
                            },
                            None => continue,
                        };

                        let g = match parts.next() {
                            Some(s) => match input_to_u8(s, 10) {
                                Ok(n) => n,
                                Err(_) => continue,
                            },
                            None => continue,
                        };

                        let b = match parts.next() {
                            Some(s) => match input_to_u8(s, 10) {
                                Ok(n) => n,
                                Err(_) => continue,
                            },
                            None => continue,
                        };

                        style.set_ul_color(Color::Rgb(Rgb(r, g, b)));
                    }

                    if part == b"5" {
                        let fixed = match parts.next() {
                            Some(s) => match input_to_u8(s, 10) {
                                Ok(n) => n,
                                Err(_) => continue,
                            },
                            None => continue,
                        };

                        style.set_ul_color(Color::Ansi256(Ansi256(fixed)));
                    }
                }
                b"59" => style.set_ul_color(Color::Unset),

                b"90" => style.set_fg_color(Color::AnsiColor(AnsiColor::BrightBlack)),
                b"91" => style.set_fg_color(Color::AnsiColor(AnsiColor::BrightRed)),
                b"92" => style.set_fg_color(Color::AnsiColor(AnsiColor::BrightGreen)),
                b"93" => style.set_fg_color(Color::AnsiColor(AnsiColor::BrightYellow)),
                b"94" => style.set_fg_color(Color::AnsiColor(AnsiColor::BrightBlue)),
                b"95" => style.set_fg_color(Color::AnsiColor(AnsiColor::BrightMagenta)),
                b"96" => style.set_fg_color(Color::AnsiColor(AnsiColor::BrightCyan)),
                b"97" => style.set_fg_color(Color::AnsiColor(AnsiColor::BrightWhite)),

                b"100" => style.set_bg_color(Color::AnsiColor(AnsiColor::BrightBlack)),
                b"101" => style.set_bg_color(Color::AnsiColor(AnsiColor::BrightRed)),
                b"102" => style.set_bg_color(Color::AnsiColor(AnsiColor::BrightGreen)),
                b"103" => style.set_bg_color(Color::AnsiColor(AnsiColor::BrightYellow)),
                b"104" => style.set_bg_color(Color::AnsiColor(AnsiColor::BrightBlue)),
                b"105" => style.set_bg_color(Color::AnsiColor(AnsiColor::BrightMagenta)),
                b"106" => style.set_bg_color(Color::AnsiColor(AnsiColor::BrightCyan)),
                b"107" => style.set_bg_color(Color::AnsiColor(AnsiColor::BrightWhite)),
                _ => {}
            }
        }

        Tag {
            name: TagName::Ansi,
            kind: TagKind::Open,
            custom: Value::Some(source),
            style,
            class: Value::None,
            span,
        }
    }
}

impl<I: ?Sized + Debug + Input> Debug for Tag<'_, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tag")
            .field("name", &self.name)
            .field("kind", &self.kind)
            .field("custom", &self.custom)
            .field("style", &self.style)
            .field("class", &self.class)
            .field("span", &self.span)
            .finish()
    }
}

impl<I: ?Sized + Display + Input> Display for Tag<'_, I> {
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

impl<I: ?Sized + Input> Clone for Tag<'_, I> {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            kind: self.kind.clone(),
            custom: self.custom.clone(),
            style: self.style.clone(),
            class: self.class.clone(),
            span: self.span.clone(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TagKind {
    Open,
    Close,
    SelfClose,
}

#[derive(Default, PartialEq)]
pub enum TagName<'src, I: ?Sized + Input> {
    A,
    Any(&'src I),
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
    Root,
    #[default]
    Empty,
}

impl<I: ?Sized + Debug + Input> Debug for TagName<'_, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TagName::A => write!(f, "A"),
            TagName::Any(arg0) => f.debug_tuple("Any").field(arg0).finish(),
            TagName::Ansi => write!(f, "Ansi"),
            TagName::B => write!(f, "B"),
            TagName::Br => write!(f, "Br"),
            TagName::C => write!(f, "C"),
            TagName::Code => write!(f, "Code"),
            TagName::D => write!(f, "D"),
            TagName::Div => write!(f, "Div"),
            TagName::H => write!(f, "H"),
            TagName::K => write!(f, "K"),
            TagName::I => write!(f, "I"),
            TagName::Let => write!(f, "Let"),
            TagName::P => write!(f, "P"),
            TagName::Pre => write!(f, "Pre"),
            TagName::R => write!(f, "R"),
            TagName::S => write!(f, "S"),
            TagName::Span => write!(f, "Span"),
            TagName::U => write!(f, "U"),
            TagName::X => write!(f, "X"),
            TagName::Ziyy => write!(f, "Ziyy"),
            TagName::Root => write!(f, "Root"),
            TagName::Empty => write!(f, "Empty"),
        }
    }
}

impl<I: ?Sized + Display + Input> Display for TagName<'_, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TagName::A => "a",
            TagName::Any(any) => return any.fmt(f),
            TagName::Ansi => "[ansi]",
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
            TagName::Root => "[root]",
            TagName::Empty => "",
        })
    }
}

impl<I: ?Sized + Input> Clone for TagName<'_, I> {
    fn clone(&self) -> Self {
        match self {
            TagName::A => TagName::A,
            TagName::Any(arg0) => TagName::Any(*arg0),
            TagName::Ansi => TagName::Ansi,
            TagName::B => TagName::B,
            TagName::Br => TagName::Br,
            TagName::C => TagName::C,
            TagName::Code => TagName::Code,
            TagName::D => TagName::D,
            TagName::Div => TagName::Div,
            TagName::H => TagName::H,
            TagName::K => TagName::K,
            TagName::I => TagName::I,
            TagName::Let => TagName::Let,
            TagName::P => TagName::P,
            TagName::Pre => TagName::Pre,
            TagName::R => TagName::R,
            TagName::S => TagName::S,
            TagName::Span => TagName::Span,
            TagName::U => TagName::U,
            TagName::X => TagName::X,
            TagName::Ziyy => TagName::Ziyy,
            TagName::Root => TagName::Root,
            TagName::Empty => TagName::Empty,
        }
    }
}
