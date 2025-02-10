use std::io::{stdout, Write};

use ziyy_core::scanner::{token::*, Scanner};

fn debug(source: &str, out: &mut impl Write) {
    let mut scanner = Scanner::new(source);
    let mut line = 0;
    loop {
        let token = scanner.scan_token().unwrap();
        let content = if token.custom == 0 {
            token.content
        } else {
            "Unexpected character."
        };
        if token.span[0].0 != line {
            let _ = write!(out, "{:4} ", token.span[0].0);
            line = token.span[0].0;
        } else {
            let _ = write!(out, "   | ");
        }
        let _ = writeln!(out, "{:?} '{}'", token.kind, content);
        if token.kind == TokenKind::Eof {
            break;
        };
    }
}

fn main() {
    let mut out = stdout();
    debug(include_str!("../../.././help.zi"), &mut out)
}
