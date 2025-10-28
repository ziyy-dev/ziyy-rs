macro_rules! get_num {
    ( $kind:expr, $token:expr ) => {
        $kind.map_err(|k| $crate::error::Error::<'src>::new(k, $token))?
    };
}

macro_rules! get_num2 {
    ( $kind:expr, $tag:expr ) => {
        $kind.map_err(|k| $crate::error::Error {
            kind: k,
            span: $tag.span,
        })?
    };
}

macro_rules! char_from_u32 {
    ( $text:expr, $radix:expr, $token:expr ) => {{
        let num = get_num!($crate::num::input_to_u32($text, $radix), $token);
        let unicode = char::from_u32(num);
        if let Some(ch) = unicode {
            Ok($crate::parser::Chunk::Escape(ch, $token.span))
        } else {
            Ok($crate::parser::Chunk::Escape(
                char::REPLACEMENT_CHARACTER,
                $token.span,
            ))
        }
    }};
}

macro_rules! number {
    ( $text:expr, $radix:expr, $token:expr ) => {
        match $token.kind {
            $crate::scanner::TokenKind::NUMBER => {
                get_num!($crate::num::input_to_u8($text, $radix), $token) as u8
            }
            _ => {
                return Err($crate::error::Error::new(
                    $crate::error::ErrorKind::UnexpectedToken {
                        expected: $crate::scanner::TokenKind::NUMBER,
                        found: Some($token.content),
                    },
                    $token,
                ));
            }
        }
    };
}

macro_rules! hex {
    ($expression:expr) => {
        match input_to_u8($expression, 16) {
            Ok(n) => n,
            Err(_) => unreachable!(),
        }
    };
}

macro_rules! t {
    ( !$x:expr ) => {
        $x == 0
    };

    ( $x:expr ) => {
        $x >= 1
    };
}

/* #[cfg(test)]
mod tests {
    use crate::color::number::NUMBER;
    use crate::color::Color;
    use crate::error::Error;
    use crate::parser::parse_chunk::Chunk;
    use crate::parser::Tag;
    use crate::scanner::span::Span;
    use crate::scanner::token::{Token, TokenKind};
    use crate::scanner::Scanner;
    use crate::style::Style;
    use crate::{char_from_u32, get_num, number, own, ErrorKind};

    #[test]
    /* fn test_assign_prop_value() {
        let mut tag = Tag {
            prop: Value::None,
            span: Span::new(),
        };
        let mut scanner = Scanner::new("= \"value\"");
        let mut token = scanner.scan_token().unwrap();

        assign_prop_value!(tag, prop, scanner, token);

        assert_eq!(tag.prop, Value::Some("value".to_owned()));
    } */

    /* #[test]
    fn test_assign_prop_color() {
        let mut tag = Tag { span: Span::new() };
        let mut style = Style { color: None };
        let mut scanner = Scanner::new("= \"#ff0000\"");
        let mut token = scanner.scan_token().unwrap();

        assign_prop_color!(tag, style, color, Red, scanner, token);

        assert_eq!(
            style.color,
            Some(Color::try_from(("#ff0000", Channel::Red, token.span)).unwrap())
        );
    } */

    /* #[test]
    fn test_assign_prop_bool() {
        let mut tag = Tag { prop: false };
        let mut scanner = Scanner::new("= \"true\"");
        let mut token = scanner.scan_token().unwrap();

        assign_prop_bool!(tag, prop, scanner, token);

        assert!(tag.prop);
    }

    #[test]
    fn test_assign_prop_cond() {
        let mut tag = Tag {
            name: todo!(),
            r#type: todo!(),
            custom: todo!(),
            style: Style::new(),
            src: todo!(),
            span: Span::new(),
        };
        let mut scanner = Scanner::new("= \"true\"");
        let mut token = scanner.scan_token().unwrap();

        assign_prop_cond!(tag, prop, true, scanner, token);

        assert!(tag.prop);
    } */
    #[test]
    fn test_own() {
        let text = "example";
        let owned_text = own!(text);

        assert_eq!(owned_text, "example".to_owned());
    }

    #[test]
    fn test_get_num() {
        let kind = Ok(42);
        let token = Token::new(TokenKind::NUMBER, "42", Span::new());

        let num = get_num!(kind, &token);

        assert_eq!(num, 42);
    }

    #[test]
    fn test_get_num2() {
        let kind = Ok(42);
        let tag = Tag {
            span: Span::new(),
            name: todo!(),
            r#type: todo!(),
            custom: todo!(),
            style: Style::new(),
            src: todo!(),
        };

        let num = get_num2!(kind, tag);

        assert_eq!(num, 42);
    }

    #[test]
    fn test_char_from_u32() {
        let text = "41";
        let radix = 16;
        let token = Token::new(TokenKind::NUMBER, "41", Span::new());

        let chunk = char_from_u32!(text, radix, &token).unwrap();

        assert_eq!(chunk, Chunk::Escape('A'));
    }

    #[test]
    fn test_number() {
        let text = "42";
        let radix = 10;
        let token = Token::new(TokenKind::NUMBER, "42", Span::new());

        let number = number!(text, radix, &token);

        assert_eq!(number, NUMBER::U8(42));
    }
}
 */
