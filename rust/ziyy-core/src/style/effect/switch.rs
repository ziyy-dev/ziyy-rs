use super::super::convert::FromU8;
use std::ops::{Add, Not, Sub};

macro_rules! define_switch {
    (
        enum $name:tt {
            on: $on:literal $(,)?
        }
    ) => {
        #[derive(Default, Debug, PartialEq, Clone, Copy)]
        pub enum $name {
            #[default]
            No,
            Yes,
        }

        impl $name {
            pub fn as_str(&self) -> &str {
                use $name::*;

                match self {
                    No => "",
                    Yes => $on,
                }
            }

            pub fn as_bytes(&self) -> &[u8] {
                self.as_str().as_bytes()
            }

            pub fn is_set(&self) -> bool {
                use $name::*;

                match self {
                    No => false,
                    Yes => true,
                }
            }

            pub fn is_unset(&self) -> bool {
                !self.is_set()
            }
        }

        impl FromU8 for $name {
            fn from_u8(value: u8) -> Self {
                use $name::*;

                match value {
                    0 => No,
                    1 => Yes,
                    _ => unreachable!(),
                }
            }
        }

        impl Add for $name {
            type Output = $name;

            fn add(self, rhs: Self) -> Self::Output {
                use $name::*;

                match (self, rhs) {
                    (No, No) => No,
                    (_, _) => Yes,
                }
            }
        }

        impl Sub for $name {
            type Output = $name;

            fn sub(self, rhs: Self) -> Self::Output {
                use $name::*;

                match (self, rhs) {
                    (Yes, No) => Yes,
                    (_, _) => No,
                }
            }
        }

        impl Not for $name {
            type Output = $name;

            fn not(self) -> Self::Output {
                use $name::*;

                match self {
                    No => No,
                    Yes => No,
                }
            }
        }
    };
}

define_switch! {
    enum Reset {
        on: "\x1b[0m"
    }
}

define_switch! {
    enum Reserved1 {
        on: "\x1b[56m"
    }
}

define_switch! {
    enum Reserved2 {
        on: "\x1b[57m"
    }
}
