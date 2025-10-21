#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use arg::{parse_args, Arg::*, Cli};
use std::env;
use std::fs::File;
use std::io::{stdout, BufReader, Read, Write};
use std::path::Path;
use std::process::exit;
use ziyy::{style, Parser};
use ziyy_core::Context;
mod arg {
    #![allow(dead_code)]
    use std::env::Args;
    pub enum Arg {
        ShortFlag(String, String),
        LongFlag(String, String),
        ShortSwitch(String),
        LongSwitch(String),
        Param(String),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Arg {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Arg::ShortFlag(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f,
                        "ShortFlag",
                        __self_0,
                        &__self_1,
                    )
                }
                Arg::LongFlag(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f,
                        "LongFlag",
                        __self_0,
                        &__self_1,
                    )
                }
                Arg::ShortSwitch(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ShortSwitch",
                        &__self_0,
                    )
                }
                Arg::LongSwitch(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "LongSwitch",
                        &__self_0,
                    )
                }
                Arg::Param(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Param",
                        &__self_0,
                    )
                }
            }
        }
    }
    pub enum Error {
        Long(String),
        Short(String),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Error {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Error::Long(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Long",
                        &__self_0,
                    )
                }
                Error::Short(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Short",
                        &__self_0,
                    )
                }
            }
        }
    }
    pub struct Cli<'a> {
        pub short_flags: Vec<&'a str>,
        pub long_flags: Vec<&'a str>,
        pub short_switches: Vec<&'a str>,
        pub long_switches: Vec<&'a str>,
    }
    fn split_args(args0: Args) -> Vec<String> {
        let mut args: Vec<String> = ::alloc::vec::Vec::new();
        for arg in args0 {
            if let Some(ch) = arg.strip_prefix('-') {
                if ch.chars().nth(0) == Some('-') {
                    args.push(arg.clone());
                    continue;
                }
                let _: Vec<_> = ch
                    .chars()
                    .map(|v| {
                        args
                            .push(
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(format_args!("-{0}", v))
                                }),
                            )
                    })
                    .collect();
                continue;
            }
            args.push(arg.clone());
        }
        args
    }
    pub fn parse_args(args0: Args, cli: Cli) -> Result<Vec<Arg>, Error> {
        let _args0 = split_args(args0);
        let mut parts = _args0.split(|x| x == "--");
        let args0 = parts.next().unwrap();
        let mut args = ::alloc::vec::Vec::new();
        let mut i = 0;
        let length = args0.len();
        while i < length {
            let arg = args0[i].clone();
            if let Some(arg) = arg.strip_prefix("--") {
                if arg.contains('=') {
                    let mut split = arg.split('=');
                    let key = split.next().unwrap();
                    let value = split.collect::<Vec<_>>().join("=");
                    if cli.long_flags.contains(&key) {
                        args.push(Arg::LongFlag(key.to_owned(), value));
                    } else {
                        return Err(Error::Long(key.to_owned()));
                    }
                } else {
                    let key = arg;
                    if cli.long_flags.contains(&key) {
                        args.push(Arg::LongFlag(key.to_owned(), args0[i + 1].clone()));
                        i += 1;
                    } else if cli.long_switches.contains(&key) {
                        args.push(Arg::LongSwitch(key.to_owned()));
                    } else {
                        return Err(Error::Long(key.to_owned()));
                    }
                }
            } else if let Some(arg) = arg.strip_prefix("-") {
                if arg.contains('=') {
                    let mut split = arg.split('=');
                    let key = split.next().unwrap();
                    let value = split.collect::<Vec<_>>().join("=");
                    if cli.short_flags.contains(&key) {
                        args.push(Arg::ShortFlag(key.to_owned(), value));
                    } else {
                        return Err(Error::Short(key.to_owned()));
                    }
                } else {
                    let key = arg;
                    if cli.short_flags.contains(&key) {
                        args.push(Arg::ShortFlag(key.to_owned(), args0[i + 1].clone()));
                        i += 1;
                    } else if cli.short_switches.contains(&key) {
                        args.push(Arg::ShortSwitch(key.to_owned()));
                    } else {
                        return Err(Error::Short(key.to_owned()));
                    }
                }
            } else {
                args.push(Arg::Param(arg));
            }
            i += 1;
        }
        args.extend(
            parts
                .collect::<Vec<_>>()
                .join(&String::from("--"))
                .iter()
                .map(|x| Arg::Param(x.clone())),
        );
        Ok(args)
    }
}
pub fn parse<'src>(source: &'src str, out: &mut impl Write) -> ziyy::Result<'src, ()> {
    let mut parser = Parser::new();
    let result = parser.parse_to_bytes(Context::new(source, None))?;
    let _ = out.write(&result);
    Ok(())
}
fn usage() {
    let mut out = stdout().lock();
    let title = "Terminal Markup Language";
    let r = 0;
    let g = 150;
    let b = 75;
    let help = ::alloc::__export::must_use({
        ::alloc::fmt::format(
            format_args!(
                "\u{1b}[m{1} \n\n\u{1b}[4;39mUsage:\u{1b}[m \u{1b}[39m\u{1b}[1mziyy\u{1b}[22m \u{1b}[3m[OPTION]\u{1b}[23m <FILE>\u{1b}[m \n\n\u{1b}[39mOptions:\u{1b}[m \n  \u{1b}[1;39m-V\u{1b}[1;39m, --version\u{1b}[m \n          Print version info and exit \n  \u{1b}[1;39m-h\u{1b}[1;39m, --help\u{1b}[m \n          Print help \n\u{1b}[m",
                0,
                title,
            ),
        )
    });
    let _ = out.write(help.as_bytes());
}
fn main() {
    let mut args0 = env::args();
    let mut out: Vec<u8> = ::alloc::vec::Vec::new();
    let mut stdout = stdout().lock();
    if args0.len() - 1 < 1 {
        usage();
        exit(0);
    }
    args0.next();
    let args = parse_args(
            args0,
            Cli {
                short_flags: ::alloc::vec::Vec::new(),
                long_flags: <[_]>::into_vec(::alloc::boxed::box_new(["mode"])),
                short_switches: <[_]>::into_vec(
                    ::alloc::boxed::box_new(["h", "V", "c", "e", "n"]),
                ),
                long_switches: <[_]>::into_vec(
                    ::alloc::boxed::box_new(["help", "version"]),
                ),
            },
        )
        .unwrap();
    let mut opt = Opt::default();
    let mut params = ::alloc::vec::Vec::new();
    for arg in args {
        match arg {
            LongSwitch(switch) if switch == "help" => {
                usage();
                exit(0);
            }
            ShortSwitch(switch) if switch == "h" => {
                usage();
                exit(0);
            }
            LongSwitch(switch) if switch == "version" => {
                {
                    ::std::io::_print(format_args!("2.0.0\n"));
                };
                exit(0);
            }
            ShortSwitch(switch) if switch == "V" => {
                {
                    ::std::io::_print(format_args!("2.0.0\n"));
                };
                exit(0);
            }
            ShortSwitch(switch) if switch == "c" => {
                opt.cli = true;
            }
            ShortSwitch(switch) if switch == "n" => {
                opt.no_newline = true;
            }
            Param(param) => {
                params.push(param);
            }
            _ => {}
        }
    }
    if opt.cli {
        if let Err(err) = parse(&params.join(" "), &mut out) {
            {
                ::std::io::_print(format_args!("{0}\n", err));
            };
            exit(1)
        }
        if !opt.no_newline {
            let _ = out.write_fmt(format_args!("\n"));
        }
    } else {
        for param in &params {
            if !Path::new(&param).is_file() {
                usage();
                exit(1);
            }
            let f = File::open(param).unwrap();
            let mut reader = BufReader::new(f);
            let mut file = String::new();
            let _ = reader.read_to_string(&mut file);
            if file.starts_with("#!") {
                let mut lines = file.split_inclusive('\n');
                lines.next();
                file = lines.collect::<Vec<_>>().join("\n");
            }
            if let Err(err) = parse(&file, &mut out) {
                {
                    ::std::io::_print(format_args!("{0}\n", err));
                };
                exit(1)
            }
        }
    }
    if params.is_empty() {
        usage();
        exit(1);
    }
    let _ = stdout.write(&out);
}
struct Opt {
    no_newline: bool,
    cli: bool,
}
#[automatically_derived]
impl ::core::default::Default for Opt {
    #[inline]
    fn default() -> Opt {
        Opt {
            no_newline: ::core::default::Default::default(),
            cli: ::core::default::Default::default(),
        }
    }
}
