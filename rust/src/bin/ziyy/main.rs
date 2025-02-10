use arg::{parse_args, Arg::*, Cli};
use std::env;
use std::fs::File;
use std::io::{stdout, BufReader, Read, Write};
use std::path::Path;
use std::process::exit;
use ziyy::{style, Parser};

mod arg;

pub fn parse(source: &str, out: &mut impl Write) -> ziyy::Result<()> {
    let mut parser = Parser::new(source, None);
    let result = parser.parse_to_bytes()?;
    let _ = out.write(&result);
    Ok(())
}

fn usage() {
    let mut out = stdout().lock();
    let title = "Terminal Markup Language";
    let r = 0;
    let g = 150;
    let b = 75;
    let help = style!(
        r#"<ziyy>
            <let name="bold:green" c="rgb({0},{g},{b})" b u />
            <let name="cyan" c="rgb({r},{g},{g})" />

            <p>{1}</p>
            <br />
            <p>
                <u src="bold:green">Usage:</u> <cyan><b>ziyy</b> <i>[OPTION]</i> \<FILE\></cyan>
            </p>
            <br />

            <p src="bold:green">Options:</p>
            <p tab="2" src="cyan" b>-V<e>,</e> --version</p>
            <p tab="10">Print version info and exit</p>
            <p tab="2" src="cyan" b>-h<e>,</e> --help</p>
            <p tab="10">Print help</p>
            <br />
        </ziyy>"#,
        0,
        title
    );

    let _ = out.write(help.as_bytes());
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
                println!("{err}");
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
