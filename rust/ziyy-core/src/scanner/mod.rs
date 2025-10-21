pub mod position;
pub mod span;
pub mod token;

use position::Position;
use span::Span;

use crate::{
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

pub fn is_whitespace(c: char) -> bool {
    c.is_ascii_whitespace()
}

/// A struct representing a scanner for tokenizing input strings.
///
/// # Type Parameters
///
/// * `T` - A type that can be referenced as a string slice.
#[derive(Clone)]
pub struct Scanner<'src> {
    pub(crate) source: &'src str,
    start: usize,
    current: usize,
    pub(crate) text_mode: bool,
    pub(crate) parse_colors: bool,
    pub start_pos: Position,
    pub current_pos: Position,
}

impl<'src> Scanner<'src> {
    /// Creates a new `Scanner` instance with the given source string.
    ///
    /// # Arguments
    ///
    /// * `source` - The source string to scan.
    ///
    /// # Returns
    ///
    /// * A new `Scanner` instance.
    pub fn new(source: &'src str) -> Self {
        Scanner {
            source,
            start: 0,
            current: 0,
            text_mode: true,
            parse_colors: false,
            start_pos: Position::default(),
            current_pos: Position::default(),
        }
    }

    /// Checks if the scanner has reached the end of the source string.
    ///
    /// # Returns
    ///
    /// * `true` if the scanner is at the end of the source string, `false` otherwise.
    pub fn is_at_end(&mut self) -> bool {
        self.current as usize + 1 > self.source.len()
    }

    /// Advances the scanner by one character and returns the character.
    ///
    /// # Returns
    ///
    /// * The character at the current position before advancing.
    pub fn advance(&mut self) -> char {
        self.current += 1;
        self.current_pos.col += 1;
        let ch = self.source.as_bytes()[self.current as usize - 1] as char;
        if ch == '\n' {
            self.current_pos.row += 1;
            self.current_pos.col = 0;
        }
        ch
    }

    pub fn advance_n(&mut self, n: usize) {
        self.current += n;
        self.current_pos.col += n as u16;
    }

    /// Peeks at the character at offset without advancing the scanner.
    ///
    /// # Returns
    ///
    /// * The character at the current position, or `'\0'` if at the end.
    pub fn peek(&mut self, offset: usize) -> char {
        if let Some(c) = self.source.as_bytes().get(self.current as usize + offset) {
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
    pub fn make_token(&mut self, kind: TokenKind) -> Result<'src, Token<'src>> {
        let s = &self.source[(self.start)..(self.current as usize)];
        let span = Span::new(self.start_pos, self.current_pos);

        if matches!(kind, TokenKind::LESS | TokenKind::LESS_SLASH) {
            self.text_mode = false
        } else if matches!(kind, TokenKind::GREAT | TokenKind::SLASH_GREAT) {
            self.text_mode = true
        }

        Ok(Token {
            kind,
            content: s,
            custom: 0,
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
    pub fn error_token(&self, eof: bool) -> Result<'src, Token<'src>> {
        let s = &self.source[(self.start)..(self.current as usize)];
        let span = Span::new(self.start_pos, self.current_pos);
        let kind = if eof {
            ErrorKind::UnexpectedEof
        } else {
            ErrorKind::UnknownToken(s)
        };
        Err(crate::Error { kind, span })
    }

    /// Creates a text token.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the created text token.
    pub fn text_token(&mut self, c: char) -> Result<'src, Token<'src>> {
        if is_whitespace(c) {
            return self.whitespace();
        }
        if c == '\\' {
            return self.escape();
        }
        while !self.is_at_end() {
            match self.peek(0) {
                '<' | '\\' | '{' | '>' => break,
                ws if is_whitespace(ws) => break,
                _ => {
                    self.advance_n(1);
                }
            }
        }

        self.make_token(TokenKind::TEXT)
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
    pub fn check_keyword(&mut self, start: usize, rest: &str, kind: TokenKind) -> TokenKind {
        let s = &self.source[(self.start + start)..self.current];
        if self.current - self.start == start + rest.len() && s == rest {
            kind
        } else {
            TokenKind::IDENTIFIER
        }
    }

    /// Determines the kind of identifier token.
    ///
    /// # Returns
    ///
    /// * The kind of identifier token.
    pub fn identifier_kind(&mut self) -> TokenKind {
        use token::TokenKind::*;

        macro_rules! get {
            ($idx:literal, $kind:expr) => {
                if self.current - self.start > $idx {
                    self.source.as_bytes()[self.start + $idx] as char
                } else {
                    return $kind;
                }
            };

            ($idx:literal) => {
                if self.current - self.start > $idx {
                    self.source.as_bytes()[self.start + $idx] as char
                } else {
                    return IDENTIFIER;
                }
            };
        }

        macro_rules! check {
            ($start:expr, $rest:expr, $kind:expr) => {
                return self.check_keyword($start, $rest, $kind)
            };
        }

        match get!(0) {
            'a' => check!(1, "", A),
            'b' => match get!(1, B) {
                'g' => check!(2, "", X),
                'l' => match get!(2) {
                    'a' => check!(3, "ck", BLACK),
                    'i' => check!(3, "nk", K),
                    'o' => check!(3, "ck", BLOCK),
                    'u' => check!(3, "e", BLUE),
                    _ => {}
                },
                'r' => check!(2, "", BR),
                _ => {}
            },
            'c' => match get!(1, C) {
                'l' => check!(2, "ass", CLASS),
                'u' => check!(2, "rly", CURLY),
                'y' => check!(2, "an", CYAN),
                _ => {}
            },
            'd' => match get!(1, D) {
                'a' => check!(2, "shed", CYAN),
                'i' => match get!(2) {
                    'm' => check!(3, "", D),
                    'v' => check!(3, "", DIV),
                    _ => {}
                },
                'o' => match get!(2) {
                    't' => check!(3, "ted", DOTTED),
                    'u' => match get!(3) {
                        'b' => match get!(4) {
                            'l' => match get!(5) {
                                'e' => match get!(6, DOUBLE) {
                                    '-' => match get!(7) {
                                        'u' => match get!(8) {
                                            'n' => match get!(9) {
                                                'd' => match get!(10) {
                                                    'e' => match get!(11) {
                                                        'r' => match get!(12, U) {
                                                            'l' => check!(3, "ine", UU),
                                                            _ => {}
                                                        },
                                                        _ => {}
                                                    },
                                                    _ => {}
                                                },
                                                _ => {}
                                            },
                                            _ => {}
                                        },
                                        _ => {}
                                    },
                                    _ => {}
                                },
                                _ => {}
                            },
                            _ => {}
                        },
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            'e' => check!(1, "m", I),
            'f' => match get!(1) {
                'i' => check!(2, "xed", FIXED),
                'g' => check!(2, "", C),
                _ => {}
            },
            'g' => check!(1, "reen", GREEN),
            'h' => match get!(1, H) {
                'i' => match get!(2) {
                    'd' => match get!(3) {
                        'd' => check!(4, "en", H),
                        'e' => check!(4, "", H),
                        _ => {}
                    },
                    _ => {}
                },
                'r' => check!(2, "ef", HREF),
                _ => {}
            },
            'i' => match get!(1, I) {
                'd' => check!(2, "", ID),
                'n' => match get!(2) {
                    'd' => check!(3, "ent", INDENT),
                    's' => check!(3, "", U),
                    'v' => match get!(3) {
                        'e' => check!(4, "rt", R),
                        'i' => check!(4, "sible", H),
                        _ => {}
                    },
                    _ => {}
                },
                't' => check!(2, "alics", I),
                _ => {}
            },
            'k' => check!(1, "", K),
            'l' => check!(1, "et", LET),
            'm' => check!(1, "agenta", MAGENTA),
            'n' => match get!(1, N) {
                'e' => check!(2, "gative", R),
                'o' => check!(2, "ne", NONE),
                _ => {}
            },
            'p' => match get!(1, P) {
                'r' => check!(2, "e", PRE),
                _ => {}
            },
            'r' => match get!(1, R) {
                'e' => match get!(2) {
                    'd' => check!(3, "", RED),
                    'v' => check!(3, "erse", R),
                    _ => {}
                },
                'g' => check!(2, "b", RGB),
                _ => {}
            },
            's' => match get!(1, S) {
                'i' => check!(2, "ngle", SINGLE),
                'p' => check!(2, "an", SPAN),
                't' => match get!(2) {
                    'r' => match get!(3) {
                        'i' => match get!(4) {
                            'k' => match get!(5) {
                                'e' => match get!(6, S) {
                                    '-' => check!(7, "through", S),
                                    _ => {}
                                },
                                _ => {}
                            },
                            _ => {}
                        },
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            'u' => match get!(1, U) {
                'n' => match get!(2) {
                    'd' => match get!(3) {
                        'e' => match get!(4) {
                            'r' => match get!(5, U) {
                                'l' => check!(6, "ine", U),
                                _ => {}
                            },
                            _ => {}
                        },
                        _ => {}
                    },
                    _ => {}
                },
                'u' => check!(2, "", UU),
                _ => {}
            },
            'w' => check!(1, "hite", WHITE),
            'x' => check!(1, "", X),
            'y' => check!(1, "ellow", YELLOW),
            'z' => check!(1, "iyy", ZIYY),
            _ => {}
        }

        IDENTIFIER
    }

    /// Scans an identifier token.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the scanned identifier token.
    pub fn identifier(&mut self) -> Result<'src, Token<'src>> {
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
    pub fn hex(&mut self) -> Result<'src, Token<'src>> {
        while is_hexdigit(self.peek(0)) {
            self.advance();
        }
        self.make_token(TokenKind::HEX)
    }

    /// Scans a number token.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the scanned number token.
    pub fn number(&mut self) -> Result<'src, Token<'src>> {
        while is_digit(self.peek(0)) {
            self.advance();
        }

        self.make_token(TokenKind::NUMBER)
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
    pub fn string(&mut self, ch: char) -> Result<'src, Token<'src>> {
        while self.peek(0) != ch && !self.is_at_end() {
            self.advance();
        }
        if self.is_at_end() {
            return self.error_token(true);
        }
        self.advance_n(1);
        self.make_token(TokenKind::STRING)
    }

    pub fn comment(&mut self) -> Result<'src, Token<'src>> {
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
            return self.make_token(TokenKind::COMMENT);
        }
        self.advance_n(3);

        self.make_token(TokenKind::COMMENT)
    }

    pub fn ansi_sgr(&mut self, esc: bool) -> Result<'src, Token<'src>> {
        if !matches!(self.peek(0), '\x30'..='\x39' | '\x3b' | '\x40'..='\x7e') {
            let c = self.peek(0);
            return self.text_token(c);
        }

        while !self.is_at_end() && !matches!(self.peek(0), '\x40'..='\x7e') {
            self.advance_n(1);
        }

        if self.peek(0) == 'm' {
            self.advance_n(1);
            self.make_token(if esc {
                TokenKind::ANSI_ESC
            } else {
                TokenKind::ANSI
            })
        } else {
            let c = self.peek(0);
            return self.text_token(c);
        }
    }

    /// Scans a whitespace token.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the scanned whitespace token.
    pub fn whitespace(&mut self) -> Result<'src, Token<'src>> {
        while is_whitespace(self.peek(0)) {
            self.advance();
        }
        self.make_token(TokenKind::WHITESPACE)
    }

    /// Scans an escape sequence token.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the scanned escape sequence token.
    pub fn escape(&mut self) -> Result<'src, Token<'src>> {
        let c = self.advance();

        macro_rules! scan_until {
            ($limit:expr, $tester:tt) => {
                let mut i = 0;
                while i < $limit && $tester(self.peek(0)) {
                    self.advance();
                    i += 1;
                }
            };
        }
        let kind = match c {
            'a' => TokenKind::ESC_A,
            'b' => TokenKind::ESC_B,
            'e' => TokenKind::ESC_R,
            'f' => TokenKind::ESC_F,
            'n' => TokenKind::ESC_N,
            'r' => TokenKind::ESC_R,
            't' => TokenKind::ESC_T,
            'v' => TokenKind::ESC_V,
            '\\' => TokenKind::ESC_BACK_SLASH,
            '<' => TokenKind::ESC_LESS,
            '>' => TokenKind::ESC_GREAT,
            '0' => {
                scan_until!(3, is_octdigit);
                TokenKind::ESC_0
            }
            'x' => {
                if self.peek(0) == '1' && matches!(self.peek(1), 'b' | 'B') && self.peek(2) == '[' {
                    self.advance_n(3);
                    return self.ansi_sgr(true);
                }
                scan_until!(2, is_hexdigit);
                TokenKind::ESC_X
            }
            'u' => {
                scan_until!(4, is_hexdigit);
                TokenKind::ESC_U
            }
            'U' => {
                scan_until!(8, is_hexdigit);
                TokenKind::ESC_U
            }
            _ => {
                return self.text_token(c);
            }
        };
        self.make_token(kind)
    }

    /// Scans the next token from the source string.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the scanned token.
    pub fn scan_token(&mut self) -> Result<'src, Token<'src>> {
        self.skip_whitespace();
        self.start = self.current;
        self.start_pos = self.current_pos;

        if self.is_at_end() {
            return self.make_token(TokenKind::EOF);
        }

        let c = self.advance();

        if self.text_mode && !matches!(c, '<' | '>') {
            return self.text_token(c);
        }

        if is_alpha(c) {
            return self.identifier();
        }

        if is_digit(c) {
            return self.number();
        }

        match c {
            '(' => self.make_token(TokenKind::LEFT_PAREN),
            ')' => self.make_token(TokenKind::RIGHT_PAREN),
            ',' => self.make_token(TokenKind::COMMA),
            '.' => self.make_token(TokenKind::DOT),
            '=' => self.make_token(TokenKind::EQUAL),
            '"' => self.string('"'),
            '\'' => self.string('\''),
            '\x1b' if self.peek(0) == '[' => {
                self.advance_n(1);
                self.ansi_sgr(false)
            }
            '/' => match self.peek(0) {
                '>' => {
                    self.advance();
                    self.text_mode = true;
                    self.make_token(TokenKind::SLASH_GREAT)
                }
                _ => self.make_token(TokenKind::SLASH),
            },
            '>' => self.make_token(TokenKind::GREAT),
            '<' => match self.peek(0) {
                'e' => match self.peek(1) {
                    '>' => {
                        self.advance_n(2);
                        while !self.is_at_end() {
                            if self.peek(0) == '<'
                                && self.peek(1) == '/'
                                && self.peek(2) == 'e'
                                && self.peek(3) == '>'
                            {
                                self.advance_n(4);
                                break;
                            } else {
                                self.advance_n(1);
                            }
                        }

                        self.make_token(TokenKind::ESCAPED)
                    }
                    _ => self.make_token(TokenKind::LESS),
                },
                '/' => {
                    self.advance();
                    self.make_token(TokenKind::LESS_SLASH)
                }
                '!' => match self.peek(1) {
                    '-' => match self.peek(2) {
                        '-' => {
                            self.advance_n(3);
                            self.comment()
                        }
                        _ => self.make_token(TokenKind::LESS),
                    },
                    _ => self.make_token(TokenKind::LESS),
                },
                _ => self.make_token(TokenKind::LESS),
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

    pub fn scan_one(&mut self) -> Option<Token<'src>> {
        let token = self.scan_token().ok()?;
        let eof = self.scan_token().ok()?;
        if eof.kind == TokenKind::EOF {
            Some(token)
        } else {
            None
        }
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
        let token = scanner.make_token(TokenKind::IDENTIFIER).unwrap();
        assert_eq!(token.content, "ab");
        assert_eq!(token.kind, TokenKind::IDENTIFIER);
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
        assert_eq!(token.kind, TokenKind::IDENTIFIER);
    }

    #[test]
    fn test_number() {
        let mut scanner = Scanner::new("123.456");
        let token = scanner.number().unwrap();
        assert_eq!(token.content, "123.456");
        assert_eq!(token.kind, TokenKind::NUMBER);
    }

    #[test]
    fn test_string() {
        let mut scanner = Scanner::new("\"hello\"");
        scanner.advance(); // Skip the "
        let token = scanner.string('"').unwrap();
        assert_eq!(token.content, "\"hello\"");
        assert_eq!(token.kind, TokenKind::STRING);
    }

    #[test]
    fn test_whitespace() {
        let mut scanner = Scanner::new("   ");
        let token = scanner.whitespace().unwrap();
        assert_eq!(token.content, "   ");
        assert_eq!(token.kind, TokenKind::WHITESPACE);
    }

    #[test]
    fn test_escape() {
        let mut scanner = Scanner::new("\\n");
        scanner.advance(); // Skip the backslash
        let token = scanner.escape().unwrap();
        assert_eq!(token.content, "\\n");
        assert_eq!(token.kind, TokenKind::ESC_N);
    }

    #[test]
    fn test_scan_token() {
        let mut scanner = Scanner::new("<tag>");
        let token = scanner.scan_token().unwrap();
        assert_eq!(token.content, "<");
        assert_eq!(token.kind, TokenKind::LESS);

        let token = scanner.scan_token().unwrap();
        assert_eq!(token.content, "tag");
        assert_eq!(token.kind, TokenKind::IDENTIFIER);

        let token = scanner.scan_token().unwrap();
        assert_eq!(token.content, ">");
        assert_eq!(token.kind, TokenKind::GREAT);
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
        let kind = scanner.check_keyword(1, "lue", TokenKind::BLUE);
        assert_eq!(kind, TokenKind::BLUE);
    }

    #[test]
    fn test_identifier_kind() {
        let mut scanner = Scanner::new("blue");
        scanner.advance();
        scanner.advance();
        scanner.advance();
        scanner.advance();
        let kind = scanner.identifier_kind();
        assert_eq!(kind, TokenKind::BLUE);
    }

    #[test]
    fn test_hex() {
        let mut scanner = Scanner::new("#1a2b3c");
        scanner.advance(); // Skip the #
        let token = scanner.hex().unwrap();
        assert_eq!(token.content, "#1a2b3c");
        assert_eq!(token.kind, TokenKind::HEX);
    }
}
