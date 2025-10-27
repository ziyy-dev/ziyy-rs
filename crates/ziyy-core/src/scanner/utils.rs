pub fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

pub fn is_valid(c: char) -> bool {
    c == ':' || c == '-' || c == '.'
}

pub fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

pub fn is_hexdigit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

pub fn is_octdigit(c: char) -> bool {
    matches!(c, '0'..'8')
}

pub fn is_whitespace(c: char) -> bool {
    c.is_ascii_whitespace()
}
