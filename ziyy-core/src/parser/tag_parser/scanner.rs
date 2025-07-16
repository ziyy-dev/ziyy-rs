use super::token::{
    Token,
    TokenType::{self, *},
};
use crate::{
    common::Span,
    scanner::{GenericScanner, Source, is_alpha, is_alpha_numeric},
    splitter::fragment::Fragment,
};

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

    fn identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }

        self.add_token(IDENTIFIER);
    }

    fn string(&mut self, c: char) {
        while self.peek() != c && !self.is_at_end() {
            if self.peek() == '\n' {
                self.span += (1, 0);
            }
            self.advance();
        }

        if self.is_at_end() {
            // TODO: error
            self.add_token2(ERROR, Some("Unterminated String.".to_owned()));
            return;
        }

        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token2(STRING, Some(value));
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn add_token(&mut self, r#type: TokenType) {
        self.add_token2(r#type, None);
    }

    fn add_token2(&mut self, r#type: TokenType, literal: Option<String>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(r#type, text, literal, self.span));
    }
}

impl_generic_scanner!(|s: &mut Scanner| {
    macro_rules! match_add {
        ($expected:expr, $first:expr, $second:expr) => {{
            let r#type = if s.matches($expected) {
                $first
            } else {
                $second
            };
            s.add_token(r#type);
        }};
    }

    let c = s.advance();

    match c {
        '!' => s.add_token(BANG),
        '=' => s.add_token(EQUAL),
        '-' => s.add_token(DASH),
        '>' => s.add_token(GREATER),
        '/' => match_add!('>', SLASH_GREATER, SLASH),
        '<' => match_add!('/', LESS_SLASH, LESS),
        ' ' | '\r' | '\t' => {}
        '\n' => {
            s.span += (1, 0);
        }
        '"' => s.string('"'),
        '\'' => s.string('\''),

        c => {
            if is_alpha(c) {
                s.identifier();
            } else {
                // TODO: error
                s.add_token2(ERROR, Some("Unexpected character.".to_owned()));
            }
        }
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
