use super::token::Token;
use crate::common::Span;
use crate::scanner::{GenericScanner, Source};
use crate::splitter::fragment::Fragment;

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    span: Span,
}

impl Scanner {
    pub fn new(mut source: Fragment) -> Self {
        source.span.tie_start();

        Self {
            source: source.lexeme.chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            span: source.span,
        }
    }

    fn escape(&mut self) {
        if self.is_at_end() {
            return;
        }
        let c = self.advance();

        let mut scan_until = |limit: u8, tester: fn(c: char) -> bool| {
            let mut i = 0;
            while i < limit && tester(self.peek()) {
                self.advance();
                i += 1;
            }
        };

        fn is_hexdigit(c: char) -> bool {
            c.is_ascii_hexdigit()
        }

        fn is_octdigit(c: char) -> bool {
            matches!(c, '0'..'8')
        }

        match c {
            'a' => self.add_token('\x07'),
            'b' => self.add_token('\x08'),
            'e' => self.add_token('\x1b'),
            'f' => self.add_token('\x0c'),
            'n' => self.add_token('\x0a'),
            'r' => self.add_token('\x0d'),
            't' => self.add_token('\t'),
            'v' => self.add_token('\x0b'),
            '\\' => self.add_token('\\'),
            '<' => self.add_token('<'),
            '>' => self.add_token('>'),
            '0' => {
                scan_until(3, is_octdigit);
                let num = u32::from_str_radix(&self.text()[2..], 8).unwrap();
                self.add_token(char::from_u32(num).unwrap_or(char::REPLACEMENT_CHARACTER));
            }
            'x' => {
                scan_until(2, is_hexdigit);
                let num = u32::from_str_radix(&self.text()[2..], 16).unwrap();
                self.add_token(char::from_u32(num).unwrap_or(char::REPLACEMENT_CHARACTER));
            }
            'u' => {
                scan_until(4, is_hexdigit);
                let num = u32::from_str_radix(&self.text()[2..], 16).unwrap();
                self.add_token(char::from_u32(num).unwrap_or(char::REPLACEMENT_CHARACTER));
            }
            'U' => {
                scan_until(8, is_hexdigit);
                let num = u32::from_str_radix(&self.text()[2..], 16).unwrap();
                self.add_token(char::from_u32(num).unwrap_or(char::REPLACEMENT_CHARACTER));
            }
            _ => {
                self.add_token('\\');
                self.add_token(c);
            }
        };
    }

    fn text(&self) -> String {
        self.source[self.start..self.current].to_string()
    }

    fn add_token(&mut self, literal: char) {
        self.tokens.push(Token::new(literal, self.span));
        self.span.tie_end();
    }
}

impl_generic_scanner!(|s: &mut Scanner| {
    let c = s.advance();
    match c {
        '\\' => s.escape(),
        '\x1b' => {
            s.add_token('\x1b');
        }
        _ => s.add_token(c),
    }
});

trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for [char] {
    fn to_string(&self) -> String {
        let mut text = String::with_capacity(self.len());

        for ch in self {
            text.push(*ch)
        }

        text
    }
}
