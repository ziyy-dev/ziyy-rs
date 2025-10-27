use arg::{parse_args, Cli};
use std::env;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, Read, Write};
use std::path::Path;
use std::process::exit;
use ziyy::{zprint, Error, Renderer};
use ziyy_core::render_to_doc;

mod arg;

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
            short_flags: &[],
            long_flags: &["mode"],
            short_switches: &["h", "V", "c", "e", "n"],
            long_switches: &[
                "ansi",
                "cli",
                "help",
                "no-newline",
                "strip",
                "version",
                "tree",
            ],
        },
    )
    .unwrap();
    let mut options = Options::default();
    let mut params = vec![];
    //println!("{args:?}");
    for arg in args {
        if arg.is_long_switch_and(|s| s == "help") | arg.is_short_switch_and(|s| s == "h") {
            usage();
            exit(0);
        } else if arg.is_long_switch_and(|s| s == "version") | arg.is_short_switch_and(|s| s == "V")
        {
            println!(env!("CARGO_PKG_VERSION"));
            exit(0);
        } else if arg.is_short_switch_and(|s| s == "c") | arg.is_long_switch_and(|s| s == "cli") {
            options.cli = true;
        } else if arg.is_short_switch_and(|s| s == "e") | arg.is_long_switch_and(|s| s == "ansi") {
            options.escape_only = true;
        } else if arg.is_short_switch_and(|s| s == "n")
            | arg.is_long_switch_and(|s| s == "no-newline")
        {
            options.no_newline = true;
        } else if arg.is_long_switch_and(|s| s == "strip") {
            options.strip = true;
        } else if arg.is_long_switch_and(|s| s == "tree") {
            options.tree = true;
        } else {
            arg.is_params_and(|s| params.push(s))
        }
    }

    if options.cli {
        if params.is_empty() {
            let mut buf = String::new();
            let _ = stdin().read_to_string(&mut buf);
            parse_to_out(&buf, &mut out, options);
        } else {
            parse_to_out(&params.join(" "), &mut out, options);
        }
        if !options.no_newline {
            let _ = writeln!(out);
        }
    } else {
        if params.is_empty() {
            usage();
            exit(1);
        }
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
            parse_to_out(&file, &mut out, options)
        }
    }

    let _ = stdout.write(&out);
}

fn parse_to_out(source: &str, out: &mut impl Write, options: Options) {
    let mut f = || {
        if options.tree {
            let _ = out.write(render_to_doc(source).to_string().as_bytes());
        } else {
            match options.escape_only {
                true => todo!(),
                false => parse(&source, out),
            }?;
        }

        /* if options.strip {
            output.root().strip_styles();
        } */

        /* let mut buf = String::new();
        if options.tree {
            buf = output.to_string();
        } else {
            output.root().to_string(&mut buf);
        } */

        // let _ = out.write(buf.as_bytes());
        Ok::<(), Error<str>>(())
    };
    if let Err(err) = f() {
        println!(
            "{}",
            err.to_string()
                .replace("at :", &format!("\x1b[1;34mat\x1b[22;39m :"))
        );
        exit(1)
    }
}

pub fn parse<'src>(source: &'src str, out: &mut impl Write) -> ziyy::Result<'src, str, ()> {
    let mut renderer = Renderer::new(out);
    renderer.write_str(source)
}

fn usage() {
    zprint!(
        r#"<ziyy>
            <let id="g" b c="rgb(0,150,75)" />
            <let id="cy" c="rgb(0,150,150)" />
            <let id="bc" b c="rgb(0,150,150)" />
            <let id="w" b="false" c="none" />

<pre>Ziyy's compiler.

<g>Usage:</g> <cy><b>{0}</b> [OPTIONS] \<FILE\>\n       <b>{0}</b> [OPTIONS] <b>-c</b> [ARGS]...</cy>

<g>Options:</g>
<bc>  -V, --version</bc>     Print version info and exit
<bc>  -c, --cli</bc>         Read input from cli, defaults to stdin if no arguments
<bc>  -e, --ansi</bc>        Parse escape sequences only
<bc>  -n, --no-newline</bc>  Suppress emiting newline after output. Available only on --cli option
<bc>  -h, --help</bc>        Print help
<bc>      --strip</bc>       Strip styles from output
<bc>      --tree</bc>        Strip styles from output
</pre>
        </ziyy>"#,
        env!("CARGO_BIN_NAME")
    );
}

#[derive(Default, Clone, Copy)]
struct Options {
    cli: bool,
    escape_only: bool,
    no_newline: bool,
    strip: bool,
    tree: bool,
}
