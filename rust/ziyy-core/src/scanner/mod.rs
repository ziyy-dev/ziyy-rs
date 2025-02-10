pub mod position;
pub mod span;
pub mod token;

use position::Position;
use span::Span;

use crate::{
    own,
    scanner::token::{Token, TokenKind},
    ErrorKind, Result,
};
use core::str;

fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_valid(c: char) -> bool {
    c == ':' || c == '-' || c == '.'
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_hexdigit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn is_octdigit(c: char) -> bool {
    matches!(c, '0'..'8')
}

fn is_whitespace(c: char) -> bool {
    c.is_ascii_whitespace()
}

/// A struct representing a scanner for tokenizing input strings.
///
/// # Type Parameters
///
/// * `T` - A type that can be referenced as a string slice.
pub struct Scanner<T: AsRef<str>> {
    pub(crate) source: T,
    start: u16,
    current: u16,
    pub(crate) line: u16,
    pub(crate) column: u16,
    pub(crate) text_mode: bool,
    pub(crate) parse_colors: bool,
    pub(crate) parse_styles: bool,
    pub span: Span,
    /// placeholder index
    place_i: u16,
}

impl<T: AsRef<str>> Scanner<T> {
    /// Creates a new `Scanner` instance with the given source string.
    ///
    /// # Arguments
    ///
    /// * `source` - The source string to scan.
    ///
    /// # Returns
    ///
    /// * A new `Scanner` instance.
    pub fn new(source: T) -> Scanner<T> {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 0,
            column: 0,
            text_mode: true,
            parse_colors: false,
            parse_styles: false,
            span: Span::new(),
            place_i: 0,
        }
    }

    /// Checks if the scanner has reached the end of the source string.
    ///
    /// # Returns
    ///
    /// * `true` if the scanner is at the end of the source string, `false` otherwise.
    pub fn is_at_end(&mut self) -> bool {
        self.current as usize + 1 > self.source.as_ref().len()
    }

    /// Advances the scanner by one character and returns the character.
    ///
    /// # Returns
    ///
    /// * The character at the current position before advancing.
    pub fn advance(&mut self) -> char {
        self.current += 1;
        self.span.push(Position(self.line, self.column));
        self.column += 1;
        let ch = self.source.as_ref().as_bytes()[self.current as usize - 1] as char;
        if ch == '\n' {
            self.line += 1;
            self.column = 0;
        }
        ch
    }

    /// Peeks at the character at offset without advancing the scanner.
    ///
    /// # Returns
    ///
    /// * The character at the current position, or `'\0'` if at the end.
    pub fn peek(&mut self, offset: usize) -> char {
        if let Some(c) = self
            .source
            .as_ref()
            .as_bytes()
            .get(self.current as usize + offset)
        {
            *c as char
        } else {
            '\0'
        }
    }

    /// Creates a token of the specified kind.
    ///
    /// # Arguments
    ///
    /// * `kind` - The kind of token to create.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the created token.
    pub fn make_token(&mut self, kind: TokenKind) -> Result<Token<'_>> {
        let s = &self.source.as_ref()[(self.start as usize)..(self.current as usize)];
        let span = Span::from(&self.span[(self.start as usize)..(self.current as usize)]);
        let mut custom = 0;
        if kind == TokenKind::PlaceHolder && s.len() == 2 {
            custom = self.place_i;
            self.place_i += 1;
        } else if matches!(kind, TokenKind::Less | TokenKind::LessSlash) {
            self.text_mode = false
        } else if matches!(kind, TokenKind::Great | TokenKind::SlashGreat) {
            self.text_mode = true
        }

        Ok(Token {
            kind,
            content: s,
            custom,
            span,
        })
    }

    /// Creates an error token with the specified error code.
    ///
    /// # Arguments
    ///
    /// * `code` - The error code for the token.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the created error token.
    pub fn error_token(&self, eof: bool) -> Result<Token<'_>> {
        let s = &self.source.as_ref()[(self.start as usize)..(self.current as usize)];
        let span = Span::from(&self.span[(self.start as usize)..(self.current as usize)]);
        let kind = if eof {
            ErrorKind::UnexpectedEof
        } else {
            ErrorKind::UnknownToken(own!(s))
        };
        Err(crate::Error { kind, span })
    }

    /// Creates a text token.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the created text token.
    pub fn text_token(&mut self) -> Result<Token<'_>> {
        self.make_token(TokenKind::Text)
    }

    /// Skips whitespace characters in the source string.
    pub fn skip_whitespace(&mut self) {
        loop {
            if self.text_mode {
                return;
            }
            let c = self.peek(0);
            if is_whitespace(c) {
                self.advance();
                continue;
            }
            return;
        }
    }

    /// Checks if the current identifier matches a keyword.
    ///
    /// # Arguments
    ///
    /// * `start` - The starting position of the keyword.
    /// * `length` - The length of the keyword.
    /// * `rest` - The rest of the keyword string.
    /// * `kind` - The kind of token to return if the keyword matches.
    ///
    /// # Returns
    ///
    /// * The token kind if the keyword matches, `TokenKind::Identifier` otherwise.
    pub fn check_keyword(
        &mut self,
        start: u16,
        length: u16,
        rest: &str,
        kind: TokenKind,
    ) -> TokenKind {
        let s = &self.source.as_ref()[((self.start + start) as usize)..(self.current as usize)];
        if self.current - self.start == start + length && s == rest {
            kind
        } else {
            TokenKind::Identifier
        }
    }

    /// Determines the kind of identifier token.
    ///
    /// # Returns
    ///
    /// * The kind of identifier token.
    pub fn identifier_kind(&mut self) -> TokenKind {
        use crate::scanner::token::TokenKind;
        use TokenKind::{Identifier, B, C, I, S, T, U, X};
        if self.parse_styles && self.current - self.start == 1 {
            match self.source.as_ref().as_bytes()[self.start as usize] as char {
                'b' => B,
                'c' => C,
                'i' => I,
                's' => S,
                't' => T,
                'u' => U,
                'x' => X,
                _ => Identifier,
            }
        } else if self.parse_colors {
            match self.source.as_ref().as_bytes()[self.start as usize] as char {
                'b' => match self.source.as_ref().as_bytes()[self.start as usize + 1] as char {
                    'l' => match self.source.as_ref().as_bytes()[self.start as usize + 2] as char {
                        'a' => self.check_keyword(3, 2, "ck", TokenKind::Black),
                        'u' => self.check_keyword(3, 1, "e", TokenKind::Blue),
                        _ => Identifier,
                    },
                    'y' => self.check_keyword(2, 2, "te", TokenKind::Byte),
                    _ => Identifier,
                },
                'c' => self.check_keyword(1, 3, "yan", TokenKind::Cyan),
                'g' => self.check_keyword(1, 4, "reen", TokenKind::Green),
                'm' => self.check_keyword(1, 6, "agenta", TokenKind::Magenta),
                'r' => match self.source.as_ref().as_bytes()[self.start as usize + 1] as char {
                    'e' => self.check_keyword(2, 1, "d", TokenKind::Red),
                    'g' => self.check_keyword(2, 1, "b", TokenKind::Rgb),
                    _ => Identifier,
                },
                'w' => self.check_keyword(1, 4, "hite", TokenKind::White),
                'y' => self.check_keyword(1, 5, "ellow", TokenKind::Yellow),
                _ => Identifier,
            }
        } else {
            Identifier
        }
    }

    /// Scans an identifier token.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the scanned identifier token.
    pub fn identifier(&mut self) -> Result<Token<'_>> {
        while is_alpha(self.peek(0)) || is_digit(self.peek(0)) || is_valid(self.peek(0)) {
            self.advance();
        }
        let kind = self.identifier_kind();
        self.make_token(kind)
    }

    /// Scans a hexadecimal token.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the scanned hexadecimal token.
    pub fn hex(&mut self) -> Result<Token<'_>> {
        while is_hexdigit(self.peek(0)) {
            self.advance();
        }
        self.make_token(TokenKind::Hex)
    }

    /// Scans a number token.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the scanned number token.
    pub fn number(&mut self) -> Result<Token<'_>> {
        while is_digit(self.peek(0)) {
            self.advance();
        }
        if self.peek(0) == '.' && is_digit(self.peek(1)) {
            self.advance();
            while is_digit(self.peek(0)) {
                self.advance();
            }
        }
        self.make_token(TokenKind::Number)
    }

    /// Scans a string token.
    ///
    /// # Arguments
    ///
    /// * `ch` - The character used to delimit the string.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the scanned string token.
    pub fn string(&mut self, ch: char) -> Result<Token<'_>> {
        while self.peek(0) != ch && !self.is_at_end() {
            self.advance();
        }
        if self.is_at_end() {
            return self.error_token(true);
        }
        self.advance();
        self.make_token(TokenKind::String)
    }

    /// Scans a placeholder token.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the scanned placeholder token.
    pub fn place_holder(&mut self) -> Result<Token<'_>> {
        loop {
            if self.peek(0) == '}' && self.peek(1) == '}' {
                self.advance();
            } else if self.peek(0) == '}' || self.is_at_end() {
                break;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token(true);
        }
        self.advance();

        self.make_token(TokenKind::PlaceHolder)
    }

    pub fn comment(&mut self) -> Result<Token<'_>> {
        loop {
            if self.is_at_end() {
                break;
            }

            if matches!(self.peek(0), '-')
                && matches!(self.peek(1), '-')
                && matches!(self.peek(2), '>')
            {
                break;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.make_token(TokenKind::Comment);
        }
        self.advance();
        self.advance();
        self.advance();

        self.make_token(TokenKind::Comment)
    }

    /// Scans a whitespace token.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the scanned whitespace token.
    pub fn whitespace(&mut self) -> Result<Token<'_>> {
        while is_whitespace(self.peek(0)) {
            self.advance();
        }
        self.make_token(TokenKind::WhiteSpace)
    }

    /// Scans an escape sequence token.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the scanned escape sequence token.
    pub fn escape(&mut self) -> Result<Token<'_>> {
        let c = self.advance();
        let mut scan_until = |limit: u8, tester: fn(c: char) -> bool| {
            let mut i = 0;
            while i < limit && tester(self.peek(0)) {
                self.advance();
                i += 1;
            }
        };
        let kind = match c {
            'a' => TokenKind::EscA,
            'b' => TokenKind::EscB,
            'e' => TokenKind::EscE,
            'f' => TokenKind::EscF,
            'n' => TokenKind::EscN,
            'r' => TokenKind::EscR,
            't' => TokenKind::EscT,
            'v' => TokenKind::EscV,
            '\\' => TokenKind::EscBackSlash,
            '<' => TokenKind::EscLess,
            '>' => TokenKind::EscGreat,
            '0' => {
                scan_until(3, is_octdigit);
                TokenKind::Esc0
            }
            'x' => {
                scan_until(2, is_hexdigit);
                TokenKind::EscX
            }
            'u' => {
                scan_until(8, is_hexdigit);
                TokenKind::EscU
            }
            _ => {
                return self.text_token();
            }
        };
        self.make_token(kind)
    }

    /// Scans the next token from the source string.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the scanned token.
    pub fn scan_token(&mut self) -> Result<Token<'_>> {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenKind::Eof);
        }

        let c = self.advance();

        if self.text_mode && !matches!(c, '<' | '>') {
            if is_whitespace(c) {
                return self.whitespace();
            }
            if c == '\\' {
                return self.escape();
            }
            if c == '{' {
                return self.place_holder();
            }
            while !self.is_at_end() {
                match self.peek(0) {
                    '<' | '\\' | '{' | '>' => break,
                    ws if is_whitespace(ws) => break,
                    _ => {
                        self.advance();
                    }
                }
            }
            return self.text_token();
        }

        if is_alpha(c) {
            return self.identifier();
        }

        if is_digit(c) {
            return self.number();
        }

        match c {
            '(' => self.make_token(TokenKind::LeftParen),
            ')' => self.make_token(TokenKind::RightParen),
            '{' => match self.peek(0) {
                '{' => self.text_token(),
                _ => self.place_holder(),
            },
            ',' => self.make_token(TokenKind::Comma),
            '.' => self.make_token(TokenKind::Dot),
            '=' => self.make_token(TokenKind::Equal),
            '"' => self.string('"'),
            '\'' => self.string('\''),
            '/' => match self.peek(0) {
                '>' => {
                    self.advance();
                    self.text_mode = true;
                    self.make_token(TokenKind::SlashGreat)
                }
                _ => self.make_token(TokenKind::Slash),
            },
            '>' => self.make_token(TokenKind::Great),
            '<' => match self.peek(0) {
                '/' => {
                    self.advance();
                    self.make_token(TokenKind::LessSlash)
                }
                '!' => match self.peek(1) {
                    '-' => match self.peek(2) {
                        '-' => {
                            self.advance();
                            self.advance();
                            self.advance();
                            self.comment()
                        }
                        _ => self.make_token(TokenKind::Less),
                    },
                    _ => self.make_token(TokenKind::Less),
                },
                _ => self.make_token(TokenKind::Less),
            },
            _ => {
                if self.parse_colors && c == '#' {
                    self.hex()
                } else {
                    self.error_token(false)
                }
            }
        }
    }

    /// Sets a new source string for the scanner.
    ///
    /// # Arguments
    ///
    /// * `source` - The new source string to scan.
    pub(crate) fn set_source(&mut self, source: T) {
        *self = Scanner::new(source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_alpha() {
        assert!(is_alpha('a'));
        assert!(is_alpha('Z'));
        assert!(is_alpha('_'));
        assert!(!is_alpha('1'));
        assert!(!is_alpha('-'));
    }

    #[test]
    fn test_is_digit() {
        assert!(is_digit('0'));
        assert!(is_digit('9'));
        assert!(!is_digit('a'));
        assert!(!is_digit('-'));
    }

    #[test]
    fn test_is_hexdigit() {
        assert!(is_hexdigit('0'));
        assert!(is_hexdigit('9'));
        assert!(is_hexdigit('a'));
        assert!(is_hexdigit('F'));
        assert!(!is_hexdigit('g'));
        assert!(!is_hexdigit('-'));
    }

    #[test]
    fn test_is_octdigit() {
        assert!(is_octdigit('0'));
        assert!(is_octdigit('7'));
        assert!(!is_octdigit('8'));
        assert!(!is_octdigit('9'));
    }

    #[test]
    fn test_is_whitespace() {
        assert!(is_whitespace(' '));
        assert!(is_whitespace('\t'));
        assert!(is_whitespace('\n'));
        assert!(!is_whitespace('a'));
        assert!(!is_whitespace('1'));
    }

    #[test]
    fn test_advance() {
        let mut scanner = Scanner::new("abc");
        assert_eq!(scanner.advance(), 'a');
        assert_eq!(scanner.advance(), 'b');
        assert_eq!(scanner.advance(), 'c');
    }

    #[test]
    fn test_peek() {
        let mut scanner = Scanner::new("abc");
        assert_eq!(scanner.peek(0), 'a');
        scanner.advance();
        assert_eq!(scanner.peek(1), 'c');
    }

    #[test]
    fn test_make_token() {
        let mut scanner = Scanner::new("abc");
        scanner.advance();
        scanner.advance();
        let token = scanner.make_token(TokenKind::Identifier).unwrap();
        assert_eq!(token.content, "ab");
        assert_eq!(token.kind, TokenKind::Identifier);
    }

    #[test]
    fn test_error_token() {
        let scanner = Scanner::new("abc");
        let token = scanner.error_token(true);
        assert!(token.is_err());
    }

    #[test]
    fn test_identifier() {
        let mut scanner = Scanner::new("abc123");
        let token = scanner.identifier().unwrap();
        assert_eq!(token.content, "abc123");
        assert_eq!(token.kind, TokenKind::Identifier);
    }

    #[test]
    fn test_number() {
        let mut scanner = Scanner::new("123.456");
        let token = scanner.number().unwrap();
        assert_eq!(token.content, "123.456");
        assert_eq!(token.kind, TokenKind::Number);
    }

    #[test]
    fn test_string() {
        let mut scanner = Scanner::new("\"hello\"");
        scanner.advance(); // Skip the "
        let token = scanner.string('"').unwrap();
        assert_eq!(token.content, "\"hello\"");
        assert_eq!(token.kind, TokenKind::String);
    }

    #[test]
    fn test_whitespace() {
        let mut scanner = Scanner::new("   ");
        let token = scanner.whitespace().unwrap();
        assert_eq!(token.content, "   ");
        assert_eq!(token.kind, TokenKind::WhiteSpace);
    }

    #[test]
    fn test_escape() {
        let mut scanner = Scanner::new("\\n");
        scanner.advance(); // Skip the backslash
        let token = scanner.escape().unwrap();
        assert_eq!(token.content, "\\n");
        assert_eq!(token.kind, TokenKind::EscN);
    }

    #[test]
    fn test_scan_token() {
        let mut scanner = Scanner::new("<tag>");
        let token = scanner.scan_token().unwrap();
        assert_eq!(token.content, "<");
        assert_eq!(token.kind, TokenKind::Less);

        let token = scanner.scan_token().unwrap();
        assert_eq!(token.content, "tag");
        assert_eq!(token.kind, TokenKind::Identifier);

        let token = scanner.scan_token().unwrap();
        assert_eq!(token.content, ">");
        assert_eq!(token.kind, TokenKind::Great);
    }

    #[test]
    fn test_set_source() {
        let mut scanner = Scanner::new("initial");
        scanner.set_source("new source");
        let s: &str = scanner.source.as_ref();
        assert_eq!(s, "new source");
        assert_eq!(scanner.start, 0);
        assert_eq!(scanner.current, 0);
        assert_eq!(scanner.line, 0);
        assert_eq!(scanner.column, 0);
        assert!(scanner.text_mode);
    }

    #[test]
    fn test_skip_whitespace() {
        let mut scanner = Scanner::new("   abc");
        scanner.text_mode = false;
        scanner.skip_whitespace();
        assert_eq!(scanner.peek(0), 'a');
    }

    #[test]
    fn test_check_keyword() {
        let mut scanner = Scanner::new("blue");
        scanner.advance();
        scanner.advance();
        scanner.advance();
        scanner.advance();
        let kind = scanner.check_keyword(1, 3, "lue", TokenKind::Blue);
        assert_eq!(kind, TokenKind::Blue);
    }

    #[test]
    fn test_identifier_kind() {
        let mut scanner = Scanner::new("blue");
        scanner.advance();
        scanner.advance();
        scanner.advance();
        scanner.advance();
        let kind = scanner.identifier_kind();
        assert_eq!(kind, TokenKind::Identifier);
    }

    #[test]
    fn test_hex() {
        let mut scanner = Scanner::new("#1a2b3c");
        scanner.advance(); // Skip the #
        let token = scanner.hex().unwrap();
        assert_eq!(token.content, "#1a2b3c");
        assert_eq!(token.kind, TokenKind::Hex);
    }

    #[test]
    fn test_place_holder() {
        let mut scanner = Scanner::new("{placeholder}");
        let token = scanner.place_holder().unwrap();
        assert_eq!(token.content, "{placeholder}");
        assert_eq!(token.kind, TokenKind::PlaceHolder);
    }
}
