#![allow(clippy::match_same_arms)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_possible_truncation)]

pub use color::*;
use convert::{FromU32, FromU8};
pub use effect::*;
use std::fmt::{Debug, Display};
use std::ops::{Add, Not, Sub};

mod color;
mod convert;
mod effect;

const MAX_ONE_BIT: u8 = 0b1;
const MAX_TWO_BITS: u8 = 0b11;
const MAX_THREE_BITS: u8 = 0b111;
#[cfg(feature = "uncommon")]
const MAX_FOUR_BITS: u8 = 0b1111;

#[repr(transparent)]
#[derive(Default, PartialEq, Clone, Copy, Eq, Hash)]
#[cfg(feature = "uncommon")]
pub struct Style([u8; 14]);

#[repr(transparent)]
#[derive(Default, PartialEq, Clone, Copy, Eq, Hash)]
#[cfg(not(feature = "uncommon"))]
pub struct Style([u8; 12]);

macro_rules! define {
    (
        fn $set:tt($offset:expr, $offset2:expr, $shift:expr);
        fn $get:tt() -> Color;
    ) => {

        #[inline]
        pub fn $set(&mut self, val: Color) {
            static MASK: u8 = !(MAX_ONE_BIT << $shift);

            let n: u32 = val.into();
            let b = (n >> 1).to_le_bytes();
            self.0[$offset] = b[0];
            self.0[$offset + 1] = b[1];
            self.0[$offset + 2] = b[2];

            self.0[$offset2] = (self.0[$offset2] & MASK) | ((n as u8 & MAX_ONE_BIT) << $shift);
        }

        #[inline]
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
        $vis1:vis fn $set:tt($offset:expr, $shift:expr, $max:expr);
        $vis2:vis fn $get:tt() -> $t:tt;
    ) => {
        #[inline]
        $vis1 fn $set(&mut self, val: $t) {
            static MASK: u8 = !($max << $shift);
            self.0[$offset] = (self.0[$offset] & MASK) | ((val as u8) << $shift);
        }

        #[must_use]
        #[inline]
        $vis2 fn $get(&self) -> $t {
            $t::from_u8((self.0[$offset] >> $shift) & $max)
        }
    };
}

impl Style {
    #[must_use]
    #[inline]

    pub const fn new() -> Self {
        #[cfg(feature = "uncommon")]
        {
            Style([0; 14])
        }
        #[cfg(not(feature = "uncommon"))]
        Style([0; 12])
    }

    /* self.0[0] */
    define! {
        pub(crate) fn set_prev_intensity(0, 0, MAX_TWO_BITS);
        pub(crate) fn prev_intensity() -> Intensity;
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
        pub fn set_reset(2, 3, MAX_ONE_BIT);
        pub fn reset() -> Reset;
    }

    define! {
        pub fn set_hide(2, 4, MAX_TWO_BITS);
        pub fn hide() -> Hide;
    }

    define! {
        pub fn set_delete(2, 6, MAX_TWO_BITS);
        pub fn delete() -> Delete;
    }

    /* self.0[3..6] */
    define! {
        fn set_fg_color(3, 2, 0);
        fn fg_color() -> Color;
    }

    /* self.0[6..9] */
    define! {
        fn set_bg_color(6, 2, 1);
        fn bg_color() -> Color;
    }

    /* self.0[9..12] */
    define! {
        fn set_ul_color(9, 2, 2);
        fn ul_color() -> Color;
    }

    /* self.0[12] */
    #[cfg(feature = "uncommon")]
    define! {
        pub fn set_prop_space(12, 0, MAX_TWO_BITS);
        pub fn prop_space() -> PropSpace;
    }

    #[cfg(feature = "uncommon")]
    define! {
        pub fn set_frame(12, 2, MAX_THREE_BITS);
        pub fn frame() -> Frame;
    }

    // #[cfg(feature = "uncommon")]
    // define! {
    //     fn set_ideogram(12, 5, MAX_THREE_BITS);
    //     fn ideogram() -> Ideogram;
    // }

    /* self.0[13] */
    #[cfg(feature = "uncommon")]
    define! {
        pub(crate) fn set_reserved1(13, 0, MAX_ONE_BIT);
        pub(crate) fn reserved1() -> Reserved1;
    }

    #[cfg(feature = "uncommon")]
    define! {
        pub(crate) fn set_reserved2(13, 1, MAX_ONE_BIT);
        pub(crate) fn reserved2() -> Reserved2;
    }

    #[cfg(feature = "uncommon")]
    define! {
        pub fn set_overline(13, 2, MAX_TWO_BITS);
        pub fn overline() -> Overline;
    }

    #[cfg(feature = "uncommon")]
    define! {
        pub fn set_font(13, 4, MAX_FOUR_BITS);
        pub fn font() -> Font;
    }
}

impl Style {
    #[must_use]
    #[inline]
    pub(crate) fn to_string2(&self) -> String {
        let mut s = String::with_capacity(128);
        s.push_str("\x1b[");

        let style = [
            cut(self.intensity().as_str2(self.prev_intensity())),
            cut(self.font_style().as_str()),
            cut(self.underline().as_str()),
            cut(self.blink().as_str()),
            cut(self.invert().as_str()),
            cut(self.hide().as_str()),
            cut(self.delete().as_str()),
            cut(&self.fg_color().to_string(ColorKind::Foreground)),
            cut(&self.bg_color().to_string(ColorKind::Background)),
            cut(&self.ul_color().to_string(ColorKind::Underline)),
            #[cfg(feature = "uncommon")]
            cut(self.font().as_str()),
            #[cfg(feature = "uncommon")]
            cut(self.prop_space().as_str()),
            #[cfg(feature = "uncommon")]
            cut(self.frame().as_str()),
            #[cfg(feature = "uncommon")]
            cut(self.overline().as_str()),
            #[cfg(feature = "uncommon")]
            cut(self.reserved1().as_str()),
            #[cfg(feature = "uncommon")]
            cut(self.reserved2().as_str()),
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

    #[inline]
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
        self.set_fg_color(self.fg_color() + rhs.fg_color());
        self.set_bg_color(self.bg_color() + rhs.bg_color());
        self.set_ul_color(self.ul_color() + rhs.ul_color());

        #[cfg(feature = "uncommon")]
        {
            self.set_font(self.font() + rhs.font());
            self.set_prop_space(self.prop_space() + rhs.prop_space());
            self.set_frame(self.frame() + rhs.frame());
            self.set_overline(self.overline() + rhs.overline());
            self.set_reserved1(self.reserved1() + rhs.reserved1());
            self.set_reserved2(self.reserved2() + rhs.reserved2());
            // self.set_ideogram(self.ideogram() + rhs.ideogram());
        }

        self
    }
}

impl Sub for Style {
    type Output = Style;

    #[inline]
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
        self.set_fg_color(self.fg_color() - rhs.fg_color());
        self.set_bg_color(self.bg_color() - rhs.bg_color());
        self.set_ul_color(self.ul_color() - rhs.ul_color());

        #[cfg(feature = "uncommon")]
        {
            self.set_font(self.font() - rhs.font());
            self.set_prop_space(self.prop_space() - rhs.prop_space());
            self.set_frame(self.frame() - rhs.frame());
            self.set_overline(self.overline() - rhs.overline());
            self.set_reserved1(self.reserved1() - rhs.reserved1());
            self.set_reserved2(self.reserved2() - rhs.reserved2());
            // self.set_ideogram(self.ideogram() - rhs.ideogram());
        }

        self
    }
}

impl Not for Style {
    type Output = Style;

    #[inline]
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
        self.set_fg_color(!self.fg_color());
        self.set_bg_color(!self.bg_color());
        self.set_ul_color(!self.ul_color());

        #[cfg(feature = "uncommon")]
        {
            self.set_font(!self.font());
            self.set_prop_space(!self.prop_space());
            self.set_frame(!self.frame());
            self.set_overline(!self.overline());
            self.set_reserved1(!self.reserved1());
            self.set_reserved2(!self.reserved2());
            // self.set_ideogram(!self.ideogram());
        }

        self
    }
}

impl Debug for Style {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            let mut d = f.debug_struct("Style");

            d.field("reset", &self.reset())
                .field("intensity", &self.intensity())
                .field("font_style", &self.font_style())
                .field("underline", &self.underline())
                .field("blink", &self.blink())
                .field("invert", &self.invert())
                .field("delete", &self.delete())
                .field("fg_color", &self.fg_color())
                .field("bg_color", &self.bg_color())
                .field("ul_color", &self.ul_color());

            #[cfg(feature = "uncommon")]
            d.field("font", &self.font())
                .field("prop_space", &self.prop_space())
                .field("frame", &self.frame())
                .field("overline", &self.overline())
                .field("reserved1", &self.reserved1())
                .field("reserved2", &self.reserved2());
            // .field("ideogram", &self.ideogram())

            d.finish()
        } else {
            f.debug_tuple("Style").field(&self.0).finish()
        }
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_str(&self.not().to_string2())
        } else {
            f.write_str(&self.to_string2())
        }
    }
}

#[inline]
fn cut(s: &str) -> Option<&str> {
    if s.is_empty() {
        None
    } else {
        Some(&s[2..s.len() - 1])
    }
}

macro_rules! impl_set_unset {
    ($($name:tt)*) => {
        $(
            impl $name {
                #[must_use]
                #[inline]
                pub const fn is_set(&self) -> bool {
                    use $name::*;

                    match self {
                        None => false,
                        _ => true,
                    }
                }

                #[must_use]
                #[inline]
                pub const fn is_unset(&self) -> bool {
                    !self.is_set()
                }
            }
        )*
    };
}

impl_set_unset! {
    Blink Color Delete FontStyle  Hide Intensity Invert Underline
}

#[cfg(feature = "uncommon")]
impl_set_unset! {
    Font Frame Overline PropSpace
}
