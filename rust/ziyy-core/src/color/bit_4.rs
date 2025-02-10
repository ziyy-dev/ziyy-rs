use std::fmt::Display;

#[repr(u8)]
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub enum Bit4 {
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
}

impl Display for Bit4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", *self as u8))
    }
}
