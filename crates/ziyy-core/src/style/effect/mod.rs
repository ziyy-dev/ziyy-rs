use std::ops::{Add, Not, Sub};

use super::convert::FromU8;

pub use blink::*;
#[cfg(feature = "uncommon")]
pub use font::*;
#[cfg(feature = "uncommon")]
pub use frame::*;
pub use intensity::*;
pub use italics::*;
pub use switch::*;
pub use underline::*;

mod blink;
#[cfg(feature = "uncommon")]
mod font;
#[cfg(feature = "uncommon")]
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
            #[inline]
            pub const fn as_str(&self) -> &str {
                use $name::*;

                match self {
                    None => "",
                    Set => $set,
                    Unset => $unset,
                }
            }

            #[must_use]
            #[inline]
            pub const fn as_bytes(&self) -> &[u8] {
                self.as_str().as_bytes()
            }
        }

        impl FromU8 for $name {
            #[inline]
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

            #[inline]
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

            #[inline]
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

            #[inline]
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

#[cfg(feature = "uncommon")]
define_effect! {
    enum Overline {
        set: "\x1b[53m",
        unset: "\x1b[55m",
    }
}

#[cfg(feature = "uncommon")]
define_effect! {
    enum PropSpace {
        set: "\x1b[26m",
        unset: "\x1b[50m",
    }
}
