pub use super::types::*;

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct RawStyle(pub(super) [u8; 16]);

macro_rules! impl_raw_style {
    (
        $( $idx:expr =>
            {
                fn $set:tt();
                fn $get:tt() -> $t:tt;
            } )*
    ) => {
        impl RawStyle {
            $(
                pub fn $set(&mut self, v: $t) {
                    static MASK: u128 = !(($t::MAX.0 as u128) << $idx);
                    let n = u128::from_ne_bytes(self.0);
                    self.0 = ((n & MASK) | ((v.0 as u128) << $idx)).to_ne_bytes();
                }

                pub fn $get(&self) -> $t {
                    ((u128::from_ne_bytes(self.0) >> $idx) & ($t::MAX.0 as u128)).into()
                }
            )*
        }
    };
}

impl_raw_style! {
    0 => {
        fn set_tagged();
        fn tagged() -> U1;
    }

    1 => {
        fn set_reset();
        fn reset() -> U1;
    }

    2 => {
        fn set_intensity();
        fn intensity() -> U3;
    }

    5 => {
        fn set_italics();
        fn italics() -> U3;
    }

    8 => {
       fn set_underline();
       fn underline() -> U3;
    }

    11 => {
        fn set_blink();
        fn blink() -> U3;
    }

    14 => {
        fn set_invert();
        fn invert() -> U2;
    }

    16 => {
        fn set_hide();
        fn hide() -> U2;
    }

    18 => {
        fn set_delete();
        fn delete() -> U2;
    }

    20 => {
        fn set_font();
        fn font() -> U4;
    }

    24 => {
        fn set_pspace();
        fn pspace() -> U2;
    }

    26 => {
        fn set_fg_color();
        fn fg_color() -> U26;
    }

    52 => {
        fn set_bg_color();
        fn bg_color() -> U26;
    }

    78 => {
        fn set_frame();
        fn frame() -> U3;
    }

    81 => {
        fn set_overline();
        fn overline() -> U2;
    }

    83 => {
        fn set_unknown1();
        fn unknown1() -> U1;
    }

    84 => {
        fn set_unknown2();
        fn unknown2() -> U1;
    }

    85 => {
        fn set_ul_color();
        fn ul_color() -> U26;
    }

    111 => {
        fn set_ideogram();
        fn ideogram() -> U3;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let mut style = RawStyle::default();
        style.set_reset(U1(1));
        style.set_blink(U3(3));

        assert_eq!(style.reset().0, 1);
        assert_eq!(style.blink().0, 3);
    }
}
