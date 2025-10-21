#![allow(clippy::match_same_arms)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_possible_truncation)]
use convert::{FromU32, FromU8};
pub use effect::*;
use std::{
    fmt::{Debug, Display},
    ops::{Add, Not, Sub},
};

macro_rules! t {
    ( !$x:expr ) => {
        $x == 0
    };

    ( $x:expr ) => {
        $x >= 1
    };
}

mod convert;
mod effect;

const MAX_ONE_BIT: u8 = 0b1;
const MAX_TWO_BITS: u8 = 0b11;
const MAX_THREE_BITS: u8 = 0b111;
const MAX_FOUR_BITS: u8 = 0b1111;

#[repr(transparent)]
#[derive(Default, PartialEq, Clone, Copy, Eq, Hash)]
pub struct Style([u8; 14]);

macro_rules! define {
    (
        fn $set:tt($offset:expr, $offset2:expr, $shift:expr);
        fn $get:tt() -> Color;
    ) => {

        pub fn $set(&mut self, val: Color) {
            static MASK: u8 = !(MAX_ONE_BIT << $shift);

            let n: u32 = val.into();
            let b = (n >> 1).to_le_bytes();
            self.0[$offset] = b[0];
            self.0[$offset + 1] = b[1];
            self.0[$offset + 2] = b[2];

            self.0[$offset2] = (self.0[$offset2] & MASK) | ((n as u8 & MAX_ONE_BIT) << $shift);
        }

        #[must_use]
        pub fn $get(&self) -> Color {
            let mut b = [0; 4];
            b[0] = self.0[$offset];
            b[1] = self.0[$offset + 1];
            b[2] = self.0[$offset + 2];
            let n = (u32::from_le_bytes(b) << 1) | ((self.0[$offset2] >> $shift) & MAX_ONE_BIT) as u32;
            Color::from_u32(n)
        }
    };

    (
        $vis1:vis fn $set:tt($offset:expr, $shift:expr);
        $vis2:vis fn $get:tt() -> bool;
    ) => {
        $vis1 fn $set(&mut self, val: bool) {
            static MASK: u8 = !(MAX_ONE_BIT << $shift);
            self.0[$offset] = (self.0[$offset] & MASK) | ((val as u8) << $shift);
        }

        $vis2 fn $get(&self) -> bool {
            if (self.0[$offset] >> $shift) & MAX_ONE_BIT == 1 {
                true
            } else {
                false
            }
        }
    };

    (
        $vis1:vis fn $set:tt($offset:expr, $shift:expr, $max:expr);
        $vis2:vis fn $get:tt() -> $t:tt;
    ) => {
        $vis1 fn $set(&mut self, val: $t) {
            static MASK: u8 = !($max << $shift);
            self.0[$offset] = (self.0[$offset] & MASK) | ((val as u8) << $shift);
        }

        #[must_use] $vis2 fn $get(&self) -> $t {
            $t::from_u8((self.0[$offset] >> $shift) & $max)
        }
    };
}

impl Style {
    #[must_use]
    pub const fn new() -> Self {
        Style([0; 14])
    }

    /* self.0[0] */
    define! {
        pub fn set_reset(0, 0, MAX_ONE_BIT);
        pub fn reset() -> Reset;
    }

    define! {
        pub(crate) fn set_reserved1(0, 1, MAX_ONE_BIT);
        pub(crate) fn reserved1() -> Reserved1;
    }

    define! {
        pub fn set_intensity(0, 2, MAX_THREE_BITS);
        pub fn intensity() -> Intensity;
    }

    define! {
        pub fn set_font_style(0, 5, MAX_THREE_BITS);
        pub fn font_style() -> FontStyle;
    }

    /* self.0[1] */
    define! {
        pub fn set_invert(1, 0, MAX_TWO_BITS);
        pub fn invert() -> Invert;
    }

    define! {
        pub fn set_underline(1, 2, MAX_THREE_BITS);
        pub fn underline() -> Underline;
    }

    define! {
        pub fn set_blink(1, 5, MAX_THREE_BITS);
        pub fn blink() -> Blink;
    }

    /* self.0[2] */
    define! {
        pub fn set_hide(2, 0, MAX_TWO_BITS);
        pub fn hide() -> Hide;
    }

    define! {
        pub fn set_delete(2, 2, MAX_TWO_BITS);
        pub fn delete() -> Delete;
    }

    define! {
        pub fn set_font(2, 4, MAX_FOUR_BITS);
        pub fn font() -> Font;
    }

    /* self.0[3] */
    define! {
        pub fn set_prop_space(3, 0, MAX_TWO_BITS);
        pub fn prop_space() -> PropSpace;
    }

    define! {
        pub fn set_frame(3, 2, MAX_THREE_BITS);
        pub fn frame() -> Frame;
    }

    // define! {
    //     fn set_ideogram(3, 5, MAX_THREE_BITS);
    //     fn ideogram() -> Ideogram;
    // }

    /* self.0[4..7] */
    define! {
        fn set_fg_color(4, 13, 0);
        fn fg_color() -> Color;
    }

    /* self.0[7..10] */
    define! {
        fn set_bg_color(7, 13, 1);
        fn bg_color() -> Color;
    }

    /* self.0[10..13] */
    define! {
        fn set_ul_color(10, 13, 2);
        fn ul_color() -> Color;
    }

    /* self.0[13] */
    define! {
        pub(crate) fn set_reserved2(13, 3, MAX_ONE_BIT);
        pub(crate) fn reserved2() -> Reserved2;
    }

    define! {
        pub fn set_overline(13, 4, MAX_TWO_BITS);
        pub fn overline() -> Overline;
    }

    define! {
        pub(crate) fn set_prev_intensity(13, 6, MAX_TWO_BITS);
        pub(crate) fn prev_intensity() -> Intensity;
    }
}

impl Style {
    #[must_use]
    pub(crate) fn to_string2(&self) -> String {
        let mut s = String::with_capacity(128);
        s.push_str("\x1b[");

        // FIXME: reset all styles due to empty string
        let style = [
            cut(self.intensity().as_str2(self.prev_intensity())),
            cut(self.font_style().as_str()),
            cut(self.underline().as_str()),
            cut(self.blink().as_str()),
            cut(self.invert().as_str()),
            cut(self.hide().as_str()),
            cut(self.delete().as_str()),
            cut(self.font().as_str()),
            cut(self.prop_space().as_str()),
            cut(&self.fg_color().to_string(ColorKind::Foreground)),
            cut(&self.bg_color().to_string(ColorKind::Background)),
            cut(self.frame().as_str()),
            cut(self.overline().as_str()),
            cut(self.reserved1().as_str()),
            cut(self.reserved2().as_str()),
            cut(&self.ul_color().to_string(ColorKind::Underline)),
            // cut(self.ideogram().as_str()),
        ]
        .iter()
        .filter_map(|x| *x)
        .collect::<Vec<_>>()
        .join(";");

        s.push_str(&style);
        s.push('m');

        if s.len() == 3 {
            // clear s if no style was written
            s.clear();
        }

        s
    }
}

impl Add for Style {
    type Output = Style;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.set_prev_intensity(match self.intensity() {
            intensity @ (Intensity::None | Intensity::Bold) => intensity,
            _ => Intensity::None,
        });

        self.set_reset(self.reset() + rhs.reset());
        self.set_intensity(self.intensity() + rhs.intensity());
        self.set_font_style(self.font_style() + rhs.font_style());
        self.set_underline(self.underline() + rhs.underline());
        self.set_blink(self.blink() + rhs.blink());
        self.set_invert(self.invert() + rhs.invert());
        self.set_hide(self.hide() + rhs.hide());
        self.set_delete(self.delete() + rhs.delete());
        self.set_font(self.font() + rhs.font());
        self.set_prop_space(self.prop_space() + rhs.prop_space());
        self.set_fg_color(self.fg_color() + rhs.fg_color());
        self.set_bg_color(self.bg_color() + rhs.bg_color());
        self.set_frame(self.frame() + rhs.frame());
        self.set_overline(self.overline() + rhs.overline());
        self.set_reserved1(self.reserved1() + rhs.reserved1());
        self.set_reserved2(self.reserved2() + rhs.reserved2());
        self.set_ul_color(self.ul_color() + rhs.ul_color());
        // self.set_ideogram(self.ideogram() + rhs.ideogram());

        self
    }
}

impl Sub for Style {
    type Output = Style;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.set_prev_intensity(match rhs.intensity() {
            intensity @ (Intensity::None | Intensity::Bold) => intensity,
            _ => Intensity::None,
        });

        self.set_reset(self.reset() - rhs.reset());
        self.set_intensity(self.intensity() - rhs.intensity());
        self.set_font_style(self.font_style() - rhs.font_style());
        self.set_underline(self.underline() - rhs.underline());
        self.set_blink(self.blink() - rhs.blink());
        self.set_invert(self.invert() - rhs.invert());
        self.set_hide(self.hide() - rhs.hide());
        self.set_delete(self.delete() - rhs.delete());
        self.set_font(self.font() - rhs.font());
        self.set_prop_space(self.prop_space() - rhs.prop_space());
        self.set_fg_color(self.fg_color() - rhs.fg_color());
        self.set_bg_color(self.bg_color() - rhs.bg_color());
        self.set_frame(self.frame() - rhs.frame());
        self.set_overline(self.overline() - rhs.overline());
        self.set_reserved1(self.reserved1() - rhs.reserved1());
        self.set_reserved2(self.reserved2() - rhs.reserved2());
        self.set_ul_color(self.ul_color() - rhs.ul_color());
        // self.set_ideogram(self.ideogram() - rhs.ideogram());

        self
    }
}

impl Not for Style {
    type Output = Style;

    fn not(mut self) -> Self::Output {
        self.set_prev_intensity(Intensity::None);

        self.set_reset(!self.reset());
        self.set_intensity(!self.intensity());
        self.set_font_style(!self.font_style());
        self.set_underline(!self.underline());
        self.set_blink(!self.blink());
        self.set_invert(!self.invert());
        self.set_hide(!self.hide());
        self.set_delete(!self.delete());
        self.set_font(!self.font());
        self.set_prop_space(!self.prop_space());
        self.set_fg_color(!self.fg_color());
        self.set_bg_color(!self.bg_color());
        self.set_frame(!self.frame());
        self.set_overline(!self.overline());
        self.set_reserved1(!self.reserved1());
        self.set_reserved2(!self.reserved2());
        self.set_ul_color(!self.ul_color());
        // self.set_ideogram(!self.ideogram());

        self
    }
}

impl Debug for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.debug_struct("Style")
                .field("reset", &self.reset())
                .field("intensity", &self.intensity())
                .field("font_style", &self.font_style())
                .field("underline", &self.underline())
                .field("blink", &self.blink())
                .field("invert", &self.invert())
                .field("delete", &self.delete())
                .field("font", &self.font())
                .field("prop_space", &self.prop_space())
                .field("fg_color", &self.fg_color())
                .field("bg_color", &self.bg_color())
                .field("frame", &self.frame())
                .field("overline", &self.overline())
                .field("reserved1", &self.reserved1())
                .field("reserved2", &self.reserved2())
                .field("ul_color", &self.ul_color())
                // .field("ideogram", &self.ideogram())
                .finish()
        } else {
            f.debug_tuple("Style").field(&self.0).finish()
        }
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_str(&self.not().to_string2())
        } else {
            f.write_str(&self.to_string2())
        }
    }
}

fn cut(s: &str) -> Option<&str> {
    if s.is_empty() {
        None
    } else {
        Some(&s[2..s.len() - 1])
    }
}
