use std::{
    fmt::{Debug, Display},
    ops::{Add, Deref, DerefMut, Not, Sub},
};

use crate::parser::ansi::Ansi;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TagType {
    #[default]
    Open,
    Close,
    SelfClose,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Tag {
    pub r#type: TagType,
    pub ansi: Ansi,
    data: [String; 3],
}

impl Default for Tag {
    fn default() -> Self {
        Self {
            r#type: TagType::SelfClose,
            ansi: Ansi::new(),
            data: [const { String::new() }; 3],
        }
    }
}

impl Tag {
    pub fn with_name(name: &str) -> Self {
        let mut tag = Tag::default();
        tag.set_name(name.to_string());
        tag
    }

    pub fn close(&self) -> Self {
        assert!(self.r#type == TagType::Open);
        let mut tag = Tag {
            r#type: TagType::Close,
            ..Default::default()
        };
        tag.set_name(self.name().clone());
        tag
    }

    pub fn inherit(&mut self, src: &Ansi) {
        macro_rules! inherit {
            ( 1 $set_y:tt $y:tt ) => {
                if !src.$y().is_empty() && self.$y().is_empty() {
                    self.$set_y(src.$y().clone());
                }
            };

            ( 2 $set_x:tt $x:tt ) => {
                if self.$x().is_unset() & src.$x().is_set() {
                    self.$set_x(src.$x());
                }
            };
        }

        inherit!(1 set_fg_color fg_color);
        inherit!(1 set_bg_color bg_color);
        inherit!(2 set_brightness brightness);
        inherit!(2 set_under under);
        inherit!(2 set_blink blink);
        inherit!(2 set_hidden hidden);
        inherit!(2 set_italics italics);
        inherit!(2 set_negative negative);
        inherit!(2 set_strike hidden);
    }

    pub fn reset_styles(&mut self) {
        self.ansi = Ansi::new();
    }
}

macro_rules! impl_tag {
    ($(($j:expr, $set_y:tt, $y:tt ) ),+ $(,)? ) => {
        impl Tag {

        $(
            pub fn $set_y(&mut self, value: String) {
                self.data[$j] = value;
            }

            pub fn $y(&self) -> &String {
                &self.data[$j]
            }
        )*
        }
    };
}

impl_tag![
    (0, set_name, name),
    (1, set_custom, custom),
    (2, set_class, class)
];

impl Add for Tag {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Tag {
            r#type: rhs.r#type,
            ansi: self.ansi + rhs.ansi,
            data: rhs.data,
        }
    }
}

impl Sub for Tag {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Tag {
            r#type: self.r#type,
            ansi: self.ansi - rhs.ansi,
            data: self.data,
        }
    }
}

impl Not for Tag {
    type Output = Self;

    fn not(self) -> Self::Output {
        let mut tag = self;

        tag.r#type = match tag.r#type {
            TagType::Open => TagType::Close,
            TagType::Close => TagType::Open,
            TagType::SelfClose => TagType::SelfClose,
        };

        tag.ansi = !tag.ansi;

        tag
    }
}

impl Deref for Tag {
    type Target = Ansi;

    fn deref(&self) -> &Self::Target {
        &self.ansi
    }
}

impl DerefMut for Tag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ansi
    }
}

impl Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tag")
            .field("name", &self.name())
            .field("src", &self.class())
            .field("custom", &self.custom())
            .field("type", &self.r#type)
            .field("ansi", &self.ansi)
            .finish()
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            match self.r#type {
                TagType::Open | TagType::SelfClose => f.write_str("<"),
                TagType::Close => f.write_str("</"),
            }?;

            f.write_str(self.name())?;

            match self.r#type {
                TagType::Open | TagType::Close => f.write_str(">"),
                TagType::SelfClose => f.write_str("/>"),
            }?;

            return Ok(());
        }

        if self.name() == "br" {
            return if !self.custom().is_empty() {
                f.write_fmt(format_args!(
                    "{}",
                    "\n".repeat(self.custom().parse::<usize>().unwrap_or(0))
                ))
            } else {
                f.write_str("\n")
            };
        } else if matches!(self.name().as_str(), "p") && !self.custom().is_empty() {
            f.write_fmt(format_args!(
                "{}",
                " ".repeat(self.custom().parse::<usize>().unwrap_or(0))
            ))?;
        }

        Display::fmt(&self.ansi, f)?;

        if self.name() == "a" {
            match self.r#type {
                TagType::Open => {
                    f.write_str("\x1b]8;;")?;
                    f.write_str(self.custom())?;
                    f.write_str("\x1b\\")?;
                }
                TagType::Close => {
                    f.write_str("\x1b]8;;\x1b\\")?;
                }
                TagType::SelfClose => {}
            }
        }

        Ok(())
    }
}
