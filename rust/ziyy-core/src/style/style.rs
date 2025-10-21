use super::*;

const MAX_ONE_BIT: u8 = 0b1;
const MAX_TWO_BITS: u8 = 0b11;
const MAX_THREE_BITS: u8 = 0b111;
const MAX_FOUR_BITS: u8 = 0b1111;

macro_rules! define {
    ( 
        fn $set:tt($offset:expr, $shl:expr, $max:expr);
        fn $get:tt() -> $t:tt;
    ) => {
        pub fn $set(&mut self, val: $t) {
            static MASK: u8 = !($max << $shl);
            self.0[$offset] = (self.0[$offset] & MASK) | ((val as u8) << $shl);
        }

        pub fn $get(&self) -> $t {
            // (self.0[$offset] >> $shl) & $max
            todo!()
        }
    };
}

#[repr(transparent)]
pub struct Style([u8; 14]);

impl Style {
    /* self.0[0] */
    define! {
        fn set_reset(0, 0, MAX_ONE_BIT);
        fn get_reset() -> Reset;
    }

    define! {
        fn set_reserved1(0, 1, MAX_ONE_BIT);
        fn get_reserved1() -> Reserved1;
    }

    define! {
        fn set_intensity(0, 2, MAX_THREE_BITS);
        fn get_intensity() -> Intensity;
    }

    define! {
        fn set_font_style(0, 5, MAX_THREE_BITS);
        fn get_font_style() -> FontStyle;
    }

    /* self.0[1] */
    define! {
        fn set_invert(1, 0, MAX_TWO_BITS);
        fn get_invert() -> Invert;
    }

    define! {
        fn set_underline(1, 2, MAX_THREE_BITS);
        fn get_underline() -> Underline;
    }

    define! {
        fn set_blink(1, 5, MAX_THREE_BITS);
        fn get_blink() -> Blink;
    }

    /* self.0[2] */
    define! {
        fn set_hide(2, 0, MAX_TWO_BITS);
        fn get_hide() -> Hide;
    }

    define! {
        fn set_delete(2, 2, MAX_TWO_BITS);
        fn get_delete() -> Delete;
    }

    define! {
        fn set_font(2, 4, MAX_FOUR_BITS);
        fn get_font() -> Font;
    }

    /* self.0[3] */
    define! {
        fn set_prop_space(3, 0, MAX_TWO_BITS);
        fn get_prop_space() -> PropSpace;
    }

    define! {
        fn set_frame(3, 2, MAX_THREE_BITS);
        fn get_frame() -> Frame;
    }

    // define! {
    //     fn set_frame(3, 5, MAX_THREE_BITS);
    //     fn get_frame() -> Ideogram;
    // }

    pub fn set_fg_color(&mut self, val: Color) {
        // let l = 
    }

    /* self.0[13] */
    define! {
        fn set_reserved2(13, 3, MAX_ONE_BIT);
        fn get_reserved2() -> Frame;
    }

    define! {
        fn set_overline(13, 4, MAX_TWO_BITS);
        fn get_overline() -> Overline;
    }
}
