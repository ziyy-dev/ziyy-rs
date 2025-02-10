use std::fmt::Display;

#[derive(PartialEq, Debug, Clone)]
pub enum Number {
    U8(u8),
    PlaceHolder(String, u16),
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::U8(u) => f.write_fmt(format_args!("{u}")),
            Number::PlaceHolder(p, i) => {
                if p.len() == 2 {
                    f.write_fmt(format_args!("{{{i}}}"))
                } else {
                    f.write_str(p)
                }
            }
        }
    }
}
