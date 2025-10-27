use crate::error::ErrorKind;
use crate::shared::Input;

#[inline]
pub fn input_to_u8<I: ?Sized + Input>(input: &I, radix: u32) -> Result<u8, ErrorKind<'_, I>> {
    // SAFETY: input contains only bytes in ASCII range as this function
    // is only called for tokens with TokenKind::NUMBER
    unsafe {
        match u8::from_str_radix(str::from_utf8_unchecked(input.as_ref()), radix) {
            Ok(n) => Ok(n),
            Err(_) => Err(ErrorKind::InvalidNumber(input)),
        }
    }
}

#[inline]
pub fn input_to_u32<I: ?Sized + Input>(input: &I, radix: u32) -> Result<u32, ErrorKind<'_, I>> {
    // SAFETY: input contains only bytes in ASCII range as this function
    // is only called for tokens with TokenKind::NUMBER
    unsafe {
        match u32::from_str_radix(str::from_utf8_unchecked(input.as_ref()), radix) {
            Ok(n) => Ok(n),
            Err(_) => Err(ErrorKind::InvalidNumber(input)),
        }
    }
}
