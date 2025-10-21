use arg::{parse_args, Arg::*, Cli};
use std::env;
use std::fs::File;
use std::io::{stdout, BufReader, Read, Write};
use std::path::Path;
use std::process::exit;
use ziyy::{zprint, Parser};
use ziyy_core::Context;

mod arg;

pub fn parse<'src>(source: &'src str, out: &mut impl Write) -> ziyy::Result<'src, ()> {
    let mut parser = Parser::new();
    let result = parser.parse_to_bytes(Context::new(source, None))?;
    let _ = out.write(&result);
    Ok(())
}

fn usage() {
    zprint!(
        r#"<ziyy>
            <let id="g" b c="rgb(0,150,75)" />
            <let id="cy" c="rgb(0,150,150)" />
            <let id="bc" b c="rgb(0,150,150)" />
            <let id="w" b="false" c="none" />

<pre>Ziyy's compiler.

<g>Usage:</g>
<cy><b>{0}</b> [OPTIONS] \<FILE\>\n       <b>{0}</b> [OPTIONS] <b>-c</b> [ARGS]...</cy>

<g>Options:</g>
<bc>  -V, --version</bc>     Print version info and exit
<bc>  -c, --cli</bc>         Read input from cli, defaults to stdin if no arguments
<bc>  -e, --ansi</bc>        Parse escape sequences only
<bc>  -n, --no-newline</bc>  Suppress emiting newline after output. Available only on --cli option
<bc>  -h, --help</bc>        Print help
<bc>      --strip</bc>       Strip styles from output
</pre>
        </ziyy>
        "#,
        env!("CARGO_BIN_NAME")
    );
}

fn main() {
    let mut args0 = env::args();
    let mut out: Vec<u8> = vec![];
    let mut stdout = stdout().lock();
    if args0.len() - 1 < 1 {
        usage();
        exit(0);
    }

    args0.next();
    let args = parse_args(
        args0,
        Cli {
            short_flags: vec![],
            long_flags: vec!["mode"],
            short_switches: vec!["h", "V", "c", "e", "n"],
            long_switches: vec!["help", "version"],
        },
    )
    .unwrap();
    let mut opt = Opt::default();
    let mut params = vec![];
    //println!("{args:?}");
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
                println!("2.0.0"); // TODO: use
                exit(0);
            }
            ShortSwitch(switch) if switch == "V" => {
                println!("2.0.0"); // TODO: use
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
            println!("{err}");
            exit(1)
        }
        if !opt.no_newline {
            let _ = writeln!(out);
        }
    } else {
        for param in &params {
            if !Path::new(&param).is_file() {
                usage();
                exit(1);
            }
            let f = File::open(&param).unwrap();
            let mut reader = BufReader::new(f);
            let mut file = String::new();
            let _ = reader.read_to_string(&mut file);
            if file.starts_with("#!") {
                let mut lines = file.split_inclusive('\n');
                lines.next();
                file = lines.collect::<Vec<_>>().join("\n");
            }
            if let Err(err) = parse(&file, &mut out) {
                println!(
                    "{}",
                    err.to_string()
                        .replace("at :", &format!("\x1b[1;34mat\x1b[22;39m {param}:"))
                );
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

#[derive(Default)]
struct Opt {
    no_newline: bool,
    cli: bool,
}
