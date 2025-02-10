#![allow(missing_docs)]

use crate::{scanner::span::Span, style::Style};

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Bool,
    Some(String),
    None,
}

/// Ziyy Tag.
#[derive(PartialEq, Debug, Clone)]
pub struct Tag {
    /// Name of Tag.
    pub name: TagName,
    /// Type of Tag.
    pub r#type: TagType,
    /// Custom information.
    pub custom: Value,
    /// Style information of the Tag.
    pub style: Style,
    /// Inherit from Tag with name.
    pub src: Value,

    pub(crate) span: Span,
}

impl Tag {
    /// Creates new Tag
    #[must_use]
    pub fn new(name: TagName, r#type: TagType) -> Self {
        use Value::None;
        Self {
            r#type,
            name,
            custom: None,
            style: Style::default(),
            src: None,

            span: Span::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TagType {
    Open,
    Close,
    OpenAndClose,
}

#[derive(PartialEq, Debug, Clone)]
pub enum TagName {
    A,
    Any(String),
    B,
    Br,
    C,
    E,
    I,
    Let,
    P,
    S,
    U,
    X,
    Ziyy,
    D,
    H,
    K,
    R,
}
