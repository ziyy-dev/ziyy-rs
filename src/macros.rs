/// Creates a `String` using interpolation of runtime expressions.
///
/// The first argument `format!` receives is a format string. This must be a string
/// literal. The power of the formatting string is in the `{}`s contained.
/// Additional parameters passed to `format!` replace the `{}`s within the
/// formatting string in the order given unless named or positional parameters
/// are used.
///
/// See [the formatting syntax documentation in `std::fmt`](../std/fmt/index.html)
/// for details.
///
/// A common use for `format!` is concatenation and interpolation of strings.
/// The same convention is used with [`print!`] and [`write!`] macros,
/// depending on the intended destination of the string; all these macros internally use [`format_args!`].
///
/// To convert a single value to a string, use the [`to_string`] method. This
/// will use the [`Display`] formatting trait.
///
/// To concatenate literals into a `&'static str`, use the [`concat!`] macro.
///
/// [`print!`]: ../std/macro.print.html
/// [`write!`]: core::write
/// [`format_args!`]: core::format_args
/// [`to_string`]: crate::string::ToString
/// [`Display`]: core::fmt::Display
/// [`concat!`]: core::concat
///
/// # Panics
///
/// `format!` panics if a formatting trait implementation returns an error.
/// This indicates an incorrect implementation
/// since `fmt::Write for String` never returns an error itself.
///
/// # Examples
///
/// ```
/// # #![allow(unused_must_use)]
/// format!("test");                             // => "test"
/// format!("hello {}", "world!");               // => "hello world!"
/// format!("x = {}, y = {val}", 10, val = 30);  // => "x = 10, y = 30"
/// let (x, y) = (1, 2);
/// format!("{x} + {y} = 3");                    // => "1 + 2 = 3"
/// ```
#[macro_export]
macro_rules! zformat {
    ($s:tt $($arg:tt)*) => {
        ::std::format!($crate::zstr!($s) $($arg)*)
    };
}

/// Prints to the standard output.
///
/// Equivalent to the [`zprintln!`] macro except that a newline is not printed at
/// the end of the message.
///
/// Note that stdout is frequently line-buffered by default so it may be
/// necessary to use [`io::stdout().flush()`][flush] to ensure the output is emitted
/// immediately.
///
/// The `zprint!` macro will lock the standard output on each call. If you call
/// `zprint!` within a hot loop, this behavior may be the bottleneck of the loop.
/// To avoid this, lock stdout with [`io::stdout().lock()`][lock]:
/// ```
/// use std::io::{stdout, Write};
///
/// let mut lock = stdout().lock();
/// zwrite!(lock, "hello world").unwrap();
/// ```
///
/// Use `zprint!` only for the primary output of your program. Use
/// [`zeprint!`] instead to print error and progress messages.
///
/// See the formatting documentation in [`std::fmt`](crate::fmt)
/// for details of the macro argument syntax.
///
/// [flush]: std::io::Write::flush
/// [`zprintln!`]: crate::zprintln
/// [`zeprint!`]: crate::zeprint
/// [lock]: std::io::Stdout
///
/// # Panics
///
/// Panics if writing to `io::stdout()` fails.
///
/// Writing to non-blocking stdout can cause an error, which will lead
/// this macro to panic.
///
/// # Examples
///
/// ```
/// use std::io::{self, Write};
///
/// print!("this ");
/// print!("will ");
/// print!("be ");
/// print!("on ");
/// print!("the ");
/// print!("same ");
/// print!("line ");
///
/// io::stdout().flush().unwrap();
///
/// print!("this string has a newline, why not choose println! instead?\n");
///
/// io::stdout().flush().unwrap();
/// ```
#[macro_export]
macro_rules! zprint {
    ($s:tt $($arg:tt)*) => {
        ::std::print!($crate::zstr!($s) $($arg)*)
    };
}

/// Prints to the standard output, with a newline.
///
/// On all platforms, the newline is the LINE FEED character (`\n`/`U+000A`) alone
/// (no additional CARRIAGE RETURN (`\r`/`U+000D`)).
///
/// This macro uses the same syntax as [`zformat!`], but writes to the standard output instead.
/// See [`std::fmt`] for more information.
///
/// The `println!` macro will lock the standard output on each call. If you call
/// `println!` within a hot loop, this behavior may be the bottleneck of the loop.
/// To avoid this, lock stdout with [`io::stdout().lock()`][lock]:
/// ```
/// use std::io::{stdout, Write};
///
/// let mut lock = stdout().lock();
/// writeln!(lock, "hello world").unwrap();
/// ```
///
/// Use `println!` only for the primary output of your program. Use
/// [`zeprintln!`] instead to print error and progress messages.
///
/// See the formatting documentation in [`std::fmt`](std::fmt)
/// for details of the macro argument syntax.
///
/// [`std::fmt`]: std::fmt
/// [`zeprintln!`]: crate::zeprintln
/// [lock]: std::io::Stdout
///
/// # Panics
///
/// Panics if writing to [`io::stdout`] fails.
///
/// Writing to non-blocking stdout can cause an error, which will lead
/// this macro to panic.
///
/// [`io::stdout`]: std::io::stdout
///
/// # Examples
///
/// ```
/// zprintln!(); // prints just a newline
/// zprintln!("hello there!");
/// zprintln!("format {} arguments", "some");
/// let local_variable = "some";
/// zprintln!("format {local_variable} arguments");
/// ```
#[macro_export]
macro_rules! zprintln {
    () => {
        ::std::println()
    };

    ($s:tt $($arg:tt)*) => {
        ::std::println!($crate::zstr!($s) $($arg)*)
    };
}

/// Prints to the standard error.
///
/// Equivalent to the [`zprint!`] macro, except that output goes to
/// [`io::stderr`] instead of [`io::stdout`]. See [`zprint!`] for
/// example usage.
///
/// Use `eprint!` only for error and progress messages. Use `zprint!`
/// instead for the primary output of your program.
///
/// [`io::stderr`]: std::io::stderr
/// [`io::stdout`]: std::io::stdout
///
/// See the formatting documentation in [`std::fmt`](std::fmt)
/// for details of the macro argument syntax.
///
/// # Panics
///
/// Panics if writing to `io::stderr` fails.
///
/// Writing to non-blocking stderr can cause an error, which will lead
/// this macro to panic.
///
/// # Examples
///
/// ```
/// zeprint!("<b><c red>Error</c>: Could not complete task");
/// ```
#[macro_export]
macro_rules! zeprint {
    ($s:tt $($arg:tt)*) => {
        ::std::eprint!($crate::zstr!($s) $($arg)*)
    };
}

/// Prints to the standard error, with a newline.
///
/// Equivalent to the [`println!`] macro, except that output goes to
/// [`io::stderr`] instead of [`io::stdout`]. See [`println!`] for
/// example usage.
///
/// Use `eprintln!` only for error and progress messages. Use `println!`
/// instead for the primary output of your program.
///
/// See the formatting documentation in [`std::fmt`](crate::fmt)
/// for details of the macro argument syntax.
///
/// [`io::stderr`]: crate::io::stderr
/// [`io::stdout`]: crate::io::stdout
/// [`println!`]: crate::println
///
/// # Panics
///
/// Panics if writing to `io::stderr` fails.
///
/// Writing to non-blocking stderr can cause an error, which will lead
/// this macro to panic.
///
/// # Examples
///
/// ```
/// eprintln!("Error: Could not complete task");
/// ```
#[macro_export]
macro_rules! zeprintln {
    () => {
        ::std::eprintln()
    };

    ($s:tt $($arg:tt)*) => {
        ::std::eprintln!($crate::zstr!($s) $($arg)*)
    };
}

/// Writes formatted data into a buffer.
///
/// This macro accepts a 'writer', a format string, and a list of arguments. Arguments will be
/// formatted according to the specified format string and the result will be passed to the writer.
/// The writer may be any value with a `write_fmt` method; generally this comes from an
/// implementation of either the [`fmt::Write`] or the [`io::Write`] trait. The macro
/// returns whatever the `write_fmt` method returns; commonly a [`fmt::Result`], or an
/// [`io::Result`].
///
/// See [`std::fmt`] for more information on the format string syntax.
///
/// [`std::fmt`]: ../std/fmt/index.html
/// [`fmt::Write`]: crate::fmt::Write
/// [`io::Write`]: ../std/io/trait.Write.html
/// [`fmt::Result`]: crate::fmt::Result
/// [`io::Result`]: ../std/io/type.Result.html
///
/// # Examples
///
/// ```
/// use std::io::Write;
///
/// fn main() -> std::io::Result<()> {
///     let mut w = Vec::new();
///     write!(&mut w, "test")?;
///     write!(&mut w, "formatted {}", "arguments")?;
///
///     assert_eq!(w, b"testformatted arguments");
///     Ok(())
/// }
/// ```
///
/// A module can import both `std::fmt::Write` and `std::io::Write` and call `write!` on objects
/// implementing either, as objects do not typically implement both. However, the module must
/// avoid conflict between the trait names, such as by importing them as `_` or otherwise renaming
/// them:
///
/// ```
/// use std::fmt::Write as _;
/// use std::io::Write as _;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut s = String::new();
///     let mut v = Vec::new();
///
///     write!(&mut s, "{} {}", "abc", 123)?; // uses fmt::Write::write_fmt
///     write!(&mut v, "s = {:?}", s)?; // uses io::Write::write_fmt
///     assert_eq!(v, b"s = \"abc 123\"");
///     Ok(())
/// }
/// ```
///
/// If you also need the trait names themselves, such as to implement one or both on your types,
/// import the containing module and then name them with a prefix:
///
/// ```
/// # #![allow(unused_imports)]
/// use std::fmt::{self, Write as _};
/// use std::io::{self, Write as _};
///
/// struct Example;
///
/// impl fmt::Write for Example {
///     fn write_str(&mut self, _s: &str) -> core::fmt::Result {
///          unimplemented!();
///     }
/// }
/// ```
///
/// Note: This macro can be used in `no_std` setups as well.
/// In a `no_std` setup you are responsible for the implementation details of the components.
///
/// ```no_run
/// use core::fmt::Write;
///
/// struct Example;
///
/// impl Write for Example {
///     fn write_str(&mut self, _s: &str) -> core::fmt::Result {
///          unimplemented!();
///     }
/// }
///
/// let mut m = Example{};
/// write!(&mut m, "Hello World").expect("Not written");
/// ```
#[macro_export]
macro_rules! zwrite {
    ($dst:expr, $s:tt $($arg:tt)*) => {
        ::std::write!($dst, $crate::zstr!($s) $($arg)*)
    };
}

/// Writes formatted data into a buffer, with a newline appended.
///
/// On all platforms, the newline is the LINE FEED character (`\n`/`U+000A`) alone
/// (no additional CARRIAGE RETURN (`\r`/`U+000D`).
///
/// For more information, see [`write!`]. For information on the format string syntax, see
/// [`std::fmt`].
///
/// [`std::fmt`]: ../std/fmt/index.html
///
/// # Examples
///
/// ```
/// use std::io::{Write, Result};
///
/// fn main() -> Result<()> {
///     let mut w = Vec::new();
///     writeln!(&mut w)?;
///     writeln!(&mut w, "test")?;
///     writeln!(&mut w, "formatted {}", "arguments")?;
///
///     assert_eq!(&w[..], "\ntest\nformatted arguments\n".as_bytes());
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! zwriteln {
    ($dst:expr $(,)?) => {
        ::std::writeln!($dst)
    };

    ($dst:expr, $s:tt $($arg:tt)*) => {
        ::std::writeln!($dst, $crate::zstr!($s) $($arg)*)
    };
}
