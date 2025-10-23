pub use ziyy_core::{style, Context, Error, ErrorKind, Parser, Result};
pub use ziyy_proc::ziyy;

#[macro_export]
macro_rules! zformat {
    ($s:tt $($arg:tt)*) => {
        ::std::format!($crate::ziyy!($s) $($arg)*)
    };
}

#[macro_export]
macro_rules! zprint {
    ($s:tt $($arg:tt)*) => {
        ::std::print!($crate::ziyy!($s) $($arg)*)
    };
}

#[macro_export]
macro_rules! zprintln {
    ($s:tt $($arg:tt)*) => {
        ::std::println!($crate::ziyy!($s) $($arg)*)
    };
}

#[macro_export]
macro_rules! zeprint {
    ($s:tt $($arg:tt)*) => {
        ::std::eprint!($crate::ziyy!($s) $($arg)*)
    };
}

#[macro_export]
macro_rules! zeprintln {
    ($s:tt $($arg:tt)*) => {
        ::std::eprintln!($crate::ziyy!($s) $($arg)*)
    };
}

#[macro_export]
macro_rules! zwrite {
    ($dst:expr, $s:tt $($arg:tt)*) => {
        ::std::write!($dst, $crate::ziyy!($s) $($arg)*)
    };
}

#[macro_export]
macro_rules! zwriteln {
    ($dst:expr, $s:tt $($arg:tt)*) => {
        ::std::writeln!($dst, $crate::ziyy!($s) $($arg)*)
    };
}
