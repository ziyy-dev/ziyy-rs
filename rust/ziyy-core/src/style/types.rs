use std::ops::{Deref, DerefMut};

macro_rules! define_subtype {
    ( $t:ty => { $( struct $name:tt($max:expr) $(;)? );* } ) => {
    $(
        #[repr(transparent)]
        #[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
        pub struct $name(pub(super) $t);

        impl $name {
            pub const MIN: $name = $name(0);
            pub const MAX: $name = $name($max);

            pub fn new(n: $t) -> Option<$name> {
                if n <= $max { Some($name(n)) } else { None }
            }
        }

        impl Deref for $name {
            type Target = $t;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl<T> AsRef<T> for $name
        where
            T: ?Sized,
            <$name as Deref>::Target: AsRef<T>,
        {
            fn as_ref(&self) -> &T {
                self.deref().as_ref()
            }
        }

        impl<T> AsMut<T> for $name
        where
            <$name as Deref>::Target: AsMut<T>,
        {
            fn as_mut(&mut self) -> &mut T {
                self.deref_mut().as_mut()
            }
        }

        impl From<$t> for $name {
            fn from(value: $t) -> Self {
                assert!(value <= $name::MAX.0);
                $name(value)
            }
        }

        impl Into<$t> for $name {
            fn into(self) -> $t {
                self.0
            }
        }

        impl From<u128> for $name {
            fn from(value: u128) -> Self {
                assert!(value as $t <= $name::MAX.0);
                $name(value as $t)
            }
        }
    )*
    };
}

define_subtype! {
    u8 => {
        struct U1(1);
        struct U2(3);
        struct U3(7);
        struct U4(15);
    }
}

define_subtype! {
    u32 => {
        struct U24(16_777_215);
        struct U26(67_108_863);
    }
}
