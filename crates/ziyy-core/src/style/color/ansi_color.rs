use super::ColorKind;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum AnsiColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,

    #[default]
    Default = 9,

    BrightBlack = 60,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl AnsiColor {
    #[must_use]
    #[inline]
    pub fn to_string(&self, kind: ColorKind) -> String {
        format!("\x1b[{}m", kind as u8 + *self as u8)
    }
}

impl TryFrom<u8> for AnsiColor {
    type Error = u8;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use AnsiColor::{
            Black, Blue, BrightBlack, BrightBlue, BrightCyan, BrightGreen, BrightMagenta,
            BrightRed, BrightWhite, BrightYellow, Cyan, Default, Green, Magenta, Red, White,
            Yellow,
        };

        match value {
            0 => Ok(Black),
            1 => Ok(Red),
            2 => Ok(Green),
            3 => Ok(Yellow),
            4 => Ok(Blue),
            5 => Ok(Magenta),
            6 => Ok(Cyan),
            7 => Ok(White),
            9 => Ok(Default),
            60 => Ok(BrightBlack),
            61 => Ok(BrightRed),
            62 => Ok(BrightGreen),
            63 => Ok(BrightYellow),
            64 => Ok(BrightBlue),
            65 => Ok(BrightMagenta),
            66 => Ok(BrightCyan),
            67 => Ok(BrightWhite),
            n => Err(n),
        }
    }
}
