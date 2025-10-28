use getopts::{Matches, Options, ParsingStyle};
use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write, stdin, stdout};
use std::path::Path;
use std::process::exit;
use ziyy::{Error, Renderer, zprint};
#[cfg(feature = "tree")]
use ziyy_core::render_to_tree;

fn main() {
    let args = env::args().skip(1);
    let mut out = stdout().lock();
    if args.len() < 1 {
        print_usage();
        return;
    }

    let mut opts = Options::new();
    opts.optflag("c", "cli", "");
    opts.optflag("e", "ansi", "");
    opts.optflag("n", "no-newline", "");
    opts.optflag("", "strip", "");
    opts.optflag("", "tree", "");
    opts.optflag("h", "help", "");
    opts.optflag("V", "version", "");
    opts.parsing_style(ParsingStyle::FloatingFrees);

    let matches = match opts.parse(args) {
        Ok(m) => m,
        Err(f) => {
            eprint!("{}", f.to_string());
            exit(1);
        }
    };

    if matches.opt_present("h") {
        print_usage();
        return;
    }

    if matches.opt_present("V") {
        println!("{} {}", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"));
        return;
    }

    if matches.opt_present("c") {
        if matches.free.is_empty() {
            let mut buf = String::new();
            let _ = stdin().read_to_string(&mut buf);
            parse_to_out(&buf, &mut out, &matches);
        } else {
            parse_to_out(&matches.free.join(" "), &mut out, &matches);
        }
        if !matches.opt_present("n") {
            let _ = writeln!(out);
        }
    } else {
        if matches.free.is_empty() {
            print_usage();
            exit(1);
        }
        for free in &matches.free {
            if !Path::new(&free).is_file() {
                print_usage();
                exit(1);
            }
            let f = File::open(free).unwrap();
            let mut reader = BufReader::new(f);
            let mut file = String::new();
            let _ = reader.read_to_string(&mut file);
            if file.starts_with("#!") {
                let mut lines = file.split_inclusive('\n');
                lines.next();
                file = lines.collect::<Vec<_>>().join("\n");
            }
            parse_to_out(&file, &mut out, &matches)
        }
    }
}

fn parse_to_out(source: &str, out: &mut impl Write, matches: &Matches) {
    let mut f = || {
        #[cfg(feature = "tree")]
        if matches.opt_present("tree") {
            let _ = out.write(render_to_tree(source).to_string().as_bytes());
            return Ok(());
        }

        match matches.opt_present("e") {
            true => todo!(),
            false => parse(&source, out),
        }?;

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

fn print_usage() {
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
