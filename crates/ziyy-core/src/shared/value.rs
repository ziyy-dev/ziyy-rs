use std::fmt::Display;

use super::Input;

#[derive(PartialEq, Debug)]
pub enum Value<'src, I: ?Sized + Input> {
    Bool,
    Some(&'src I),
    None,
}

impl<I: ?Sized + Display + Input> Display for Value<'_, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool => write!(f, "true"),
            Self::Some(arg0) => Display::fmt(arg0, f),
            Self::None => write!(f, "none"),
        }
    }
}

impl<I: ?Sized + Input> Clone for Value<'_, I> {
    fn clone(&self) -> Self {
        match self {
            Self::Bool => Self::Bool,
            Self::Some(arg0) => Self::Some(*arg0),
            Self::None => Self::None,
        }
    }
}
