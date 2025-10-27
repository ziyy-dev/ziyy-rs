#[cfg(feature = "bindings")]
use std::ops::RangeFrom;
use std::ops::{Index, Range};

#[cfg(feature = "bindings")]
pub trait Input
where
    Self: AsRef<[u8]>
        + Index<Range<usize>, Output = Self>
        + Index<RangeFrom<usize>, Output = Self>
        + PartialEq<Self>,
{
}

impl<
        T: ?Sized
            + AsRef<[u8]>
            + Index<Range<usize>, Output = Self>
            + Index<RangeFrom<usize>, Output = Self>
            + PartialEq<Self>,
    > Input for T
{
}
