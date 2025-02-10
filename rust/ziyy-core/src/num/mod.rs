use crate::ErrorKind;

fn char_to_digit(ch: char) -> Result<u32, ErrorKind> {
    Ok(match ch {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' | 'A' => 10,
        'b' | 'B' => 11,
        'c' | 'C' => 12,
        'd' | 'D' => 13,
        'e' | 'E' => 14,
        'f' | 'F' => 15,
        _ => return Err(ErrorKind::InvalidNumber),
    })
}

fn parse_num(s: &str, radix: u32) -> Result<u32, ErrorKind> {
    let mut n = s.len() as u32;
    let mut num = 0;
    for ch in s.as_bytes() {
        n -= 1;
        let ch = *ch as char;

        let digit = char_to_digit(ch)?;

        if digit >= radix {
            return Err(ErrorKind::InvalidNumber);
        }

        num += digit * radix.pow(n);
    }

    Ok(num)
}

fn parse_decimal(s: &str, radix: u32) -> Result<f64, ErrorKind> {
    let mut n = 1;
    let mut float = 0.0;
    for ch in s.as_bytes() {
        let ch = *ch as char;

        let digit = char_to_digit(ch)?;

        if digit >= radix {
            return Err(ErrorKind::InvalidNumber);
        }

        float += f64::from(digit) / f64::from(radix).powi(n);

        n += 1;
    }

    Ok(float)
}

/// # Panics
/// Does not panic
pub fn str_to_u32(s: &str, radix: u32) -> Result<u32, ErrorKind> {
    let mut parts = s.split('.');

    let mut num = parse_num(parts.next().unwrap(), radix)?;

    if let Some(decimal) = parts.next() {
        num += parse_decimal(decimal, radix)?.round() as u32;
    }

    if parts.next().is_some() {
        return Err(ErrorKind::InvalidNumber);
    }

    Ok(num)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_str_to_u32() {
        assert_eq!(str_to_u32("13", 10), Ok(13));
        assert_eq!(str_to_u32("33", 8), Ok(27));
        assert_eq!(str_to_u32("ff", 16), Ok(255));
        assert_eq!(str_to_u32("ff.ff", 16), Ok(256));
    }

    #[test]
    fn test_parse_num() {
        assert_eq!(parse_num("13", 10), Ok(13));
        assert_eq!(parse_num("33", 8), Ok(27));
        assert_eq!(parse_num("ff", 16), Ok(255));
    }

    #[test]
    fn test_parse_decimal() {
        assert_eq!(parse_decimal("13", 10), Ok(0.13));
        assert_eq!(parse_decimal("33", 8), Ok(0.421875));
        assert_eq!(parse_decimal("ff", 16), Ok(0.99609375));
    }

    #[test]
    fn test_char_to_digit() {
        assert_eq!(char_to_digit('0'), Ok(0));
        assert_eq!(char_to_digit('9'), Ok(9));
        assert_eq!(char_to_digit('a'), Ok(10));
        assert_eq!(char_to_digit('F'), Ok(15));
        assert!(char_to_digit('g').is_err());
    }

    #[test]
    fn test_str_to_u32_with_invalid_input() {
        assert!(str_to_u32("g", 16).is_err());
        assert!(str_to_u32("1.1.1", 10).is_err());
        assert!(str_to_u32("12", 2).is_err());
    }

    #[test]
    fn test_parse_num_with_invalid_input() {
        assert!(parse_num("g", 16).is_err());
        assert!(parse_num("12", 2).is_err());
    }

    #[test]
    fn test_parse_decimal_with_invalid_input() {
        assert!(parse_decimal("g", 16).is_err());
        assert!(parse_decimal("12", 2).is_err());
    }
}
