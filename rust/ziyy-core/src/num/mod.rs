use crate::ErrorKind;

pub fn str_to_u8(s: &str, radix: u32) -> Result<u8, ErrorKind<'_>> {
    match u8::from_str_radix(s, radix) {
        Ok(n) => Ok(n),
        Err(_) => Err(ErrorKind::InvalidNumber(s)),
    }
}

pub fn str_to_u32(s: &str, radix: u32) -> Result<u32, ErrorKind<'_>> {
    match u32::from_str_radix(s, radix) {
        Ok(n) => Ok(n),
        Err(_) => Err(ErrorKind::InvalidNumber(s)),
    }
}
