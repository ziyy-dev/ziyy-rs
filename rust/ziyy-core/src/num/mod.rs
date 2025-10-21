use crate::ErrorKind;

pub fn str_to_u32<'src>(s: &'src str, radix: u32) -> Result<u32, ErrorKind<'src>> {
    match u32::from_str_radix(s, radix) {
        Ok(n) => Ok(n),
        Err(_) => Err(ErrorKind::InvalidNumber(s)),
    }
}
