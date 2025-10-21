pub trait FromU8: Sized {
    fn from_u8(value: u8) -> Self;
}

pub trait FromU32: Sized {
    fn from_u32(value: u32) -> Self;
}
