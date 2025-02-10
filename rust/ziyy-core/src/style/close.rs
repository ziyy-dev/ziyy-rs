use std::{
    fmt::{Display, Write as _},
    io::Write,
};

use crate::style::Condition;

use super::Style;

#[repr(transparent)]
#[derive(Debug)]
pub struct StyleClose(pub(crate) Style);

macro_rules! checked_write_bool {
    ($buf:expr, $bytes:expr, $current:expr, $f:tt) => {
        if $current.0.$f {
            let _ = $buf.write($bytes);
        }
    };
}

macro_rules! checked_write_color {
    ($buf:expr, $bytes:expr, $current:expr, $f:tt) => {
        if let Some(color) = $current.0.$f.clone() {
            let _ = $buf.write(color.to_string().as_bytes());
        }
    };
}

impl StyleClose {}

impl Display for StyleClose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = vec![];
        if self.0 == Style::default() {
            return Ok(());
        }
        let _ = buf.write(b"\x1b[");

        let _ = match self.0.brightness {
            Condition::A | Condition::B => buf.write(b"22;"),
            Condition::AB => buf.write(b"22;1;"),
            Condition::BA => buf.write(b"2;"),
            Condition::None => Ok(0),
        };

        let _ = match self.0.under {
            Condition::A | Condition::B => buf.write(b"24;"),
            Condition::AB => buf.write(b"4;"),
            Condition::BA => buf.write(b"21;"),
            Condition::None => Ok(0),
        };

        checked_write_bool!(buf, b"23;", self, italics);
        //checked_write_bool!(buf, b"24;", self, under);
        checked_write_bool!(buf, b"25;", self, blink);
        checked_write_bool!(buf, b"27;", self, invert);
        checked_write_bool!(buf, b"28;", self, hide);
        checked_write_bool!(buf, b"29;", self, strike);
        //checked_write_bool!(buf, b"24;", self, double_under);

        checked_write_color!(buf, b"39;", self, fg_color);
        checked_write_color!(buf, b"49;", self, bg_color);

        if buf[buf.len() - 1] == b';' {
            buf.pop();
        }

        buf.push(b'm');

        for ch in buf {
            f.write_char(ch as char)?; // all in ASCII range
        }

        Ok(())
    }
}
