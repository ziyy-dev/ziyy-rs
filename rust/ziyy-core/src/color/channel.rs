use std::fmt::Display;

#[repr(u8)]
#[derive(PartialEq, Debug, Clone)]
pub enum Channel {
    Foreground = 3,
    Background = 4,
}

impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.clone() as u8))
    }
}
