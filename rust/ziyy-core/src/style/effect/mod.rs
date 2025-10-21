use super::convert::FromU8;
pub use blink::*;
pub use color::*;
pub use font::*;
pub use frame::*;
pub use intensity::*;
pub use italics::*;
use std::ops::{Add, Not, Sub};
pub use switch::*;
pub use underline::*;

mod blink;
mod color;
mod font;
mod frame;
mod intensity;
mod italics;
mod switch;
mod underline;

macro_rules! define_effect {
    (
        enum $name:tt {
            set: $set:literal,
            unset: $unset:literal $(,)?
        }
    ) => {
        #[derive(Default, Debug, PartialEq)]
        pub enum $name {
            #[default]
            None,
            Set,
            Unset,
        }

        impl $name {
            #[must_use]
            pub fn as_str(&self) -> &str {
                use $name::*;

                match self {
                    None => "",
                    Set => $set,
                    Unset => $unset,
                }
            }

            #[must_use]
            pub fn as_bytes(&self) -> &[u8] {
                self.as_str().as_bytes()
            }
        }

        impl FromU8 for $name {
            fn from_u8(value: u8) -> Self {
                use $name::*;

                match value {
                    0 => None,
                    1 => Set,
                    2 => Unset,
                    _ => unreachable!(),
                }
            }
        }

        impl Add for $name {
            type Output = $name;

            fn add(self, rhs: Self) -> Self::Output {
                use $name::*;

                match (self, rhs) {
                    (None, None) => None,
                    (None, Set) => Set,
                    (None, Unset) => None,

                    (Set, None) => Set,
                    (Set, Set) => Set,
                    (Set, Unset) => Unset,

                    (Unset, None) => Unset,
                    (Unset, Set) => Set,
                    (Unset, Unset) => Unset,
                }
            }
        }

        impl Sub for $name {
            type Output = $name;

            fn sub(self, rhs: Self) -> Self::Output {
                use $name::*;

                match (self, rhs) {
                    (None, rhs) => !rhs,
                    (lhs, rhs) if lhs == rhs => None,
                    (lhs, _) => lhs,
                }
            }
        }

        impl Not for $name {
            type Output = $name;

            fn not(self) -> Self::Output {
                use $name::*;

                match self {
                    Set => Unset,
                    _ => None,
                }
            }
        }
    };
}

define_effect! {
    enum Invert {
        set: "\x1b[7m",
        unset: "\x1b[27m",
    }
}

define_effect! {
    enum Hide {
        set: "\x1b[8m",
        unset: "\x1b[28m",
    }
}

define_effect! {
    enum Delete {
        set: "\x1b[9m",
        unset: "\x1b[29m",
    }
}

define_effect! {
    enum Overline {
        set: "\x1b[53m",
        unset: "\x1b[55m",
    }
}

define_effect! {
    enum PropSpace {
        set: "\x1b[26m",
        unset: "\x1b[50m",
    }
}

macro_rules! impl_set_unset {
    ($($name:tt)*) => {
        $(
            impl $name {
                #[must_use] pub fn is_set(&self) -> bool {
                    use $name::*;

                    match self {
                        None => false,
                        _ => true,
                    }
                }

                #[must_use] pub fn is_unset(&self) -> bool {
                    !self.is_set()
                }
            }
        )*
    };
}

impl_set_unset! {
    Blink Color Delete Font FontStyle Frame Hide Intensity Invert Overline PropSpace Underline
}
