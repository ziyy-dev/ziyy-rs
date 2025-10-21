#![feature(prelude_import)]
#![warn(rustdoc::private_intra_doc_links)]
#![warn(unconditional_panic)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
/*!# Ziyy - Terminal Markup Language.

<p align="center">
  <img src='logo.svg' width='250' alt='Ziyy Logo'>
</p>

## Overview
Style your Terminal using HTML like tags `<b>..</b>`.

## Example
```html
<b u i c="rgb(5, 165, 104)">This is a Bold Green Underlined Text in Italics</b>
```
### Output
<pre>
<b style="color:rgb(5, 165, 104);"><i><u>This is a Bold Green Underlined Text in Italics</u></i></b>
</pre>

## Tags
### Text Effects
| Tag | Effect | Properties |
| --------| ----------- | --- |
| `<b>` | Bold | Effects, Colors and Inheritance Properties. |
| `<d>` | Dim | Same as above. |
| `<h>` | Hide | Same as above. |
| `<k>` | Blink | Same as above. |
| `<r>` | Reverse (foreground and background) | Same as above. |
| `<i>` | Italics | Same as above. |
| `<s>` | Strike-Through | Same as above. |
| `<u>` | Underline | Same as above. |

### Text Color
| Tag | Color | Properties |
| --- | ------ | -- |
| `<c>` | Foreground Color | Effects, Colors and Inheritance Properties.` |
| `<x>` | Background Color | Same as above. |

### Others
| Tag | Description | Properties |
| --- | ------ | -- |
| `<a>` | Insert a link. | `href`: url of the link. |
| `<p>` | Insert new Paragraph | `tab`: indent the paragraph with either tab if no assigned value or  *n* spaces. |
| `<br>` | Insert a line break. | `n`: no of line breaks to insert. default is 1. |
| `<let>` | Declares new custom tag.  | `name`: Name of tag. Supports only ASCII character set.  |
| `<ziyy>` | The root of other tags. | Effects, Colors and Inheritance Properties. |

## Properties
### Effects
| Property | Description | Type |
| --- | ------ | --- |
| `b \| bold` | Bold | `"true" \| "false" or unassigned (true` |
| `d \| dim` | Dim | Same as above. |
| `double` | Double Underline (`<u>` tag only) | Same as above. |
| `h \| hidden \| hide \| invisible` | Hide | Same as above. |
| `k \| blink` | Blink | Same as above. |
| `r \| invert \| reverse` | Reverse | Same as above. |
| `i \| italics` | Italics | Same as above. |
| `s \| strike-through` | Strike-Through | Same as above. |
| `u \| underline` | Underline | Same as above. |
| `uu \| double-underline` | Double Underline | Same as above. |

### Colors
| Property | Description | Type |
| --- | ------ | --- |
| `c \| fg`   | Foreground color | `"black" \| "red" \| "green" \| "yellow" \| "blue" \| "magenta" \| "cyan" \| "white" \| "byte(#)" \| "rgb(#, #, #)"` |
| `x \| bg`   | Background color | Same as above. | ANSI 4 bit colors (`<c>` and `<x>` tags only) | `unassinged` |
| `byte` | ANSI 256 color (`<c>` and `<x>` tags only) | `"#"` |
| `rgb` | Rgb colors (`<c>` and `<x>` tags only) | `"#, #, #"` |
> `#` is a number within `0..255`

### Inheritance
| Property | Description | Type |
| --- | ------ | --- |
| `src` | Inherit properties from a tag. | "..." |

*/
//! # Examples
//! ```
//! use std::collections::HashMap;
//!
//! use ziyy::Parser;
//!
//! let mut parser = Parser::new("This is Some <c magenta u b>Magenta Underlined Bold Text</c>", None);
//! assert!(parser.parse().is_ok());
//!```
//! # Result
//! <pre>This is Some <span style="color: magenta;"><b><u>Magenta Underlined Bold Text</u></b></span></pre>
//!
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub use crate::error::{Error, ErrorKind};
pub use crate::parser::{Context, Parser, Tag, TagKind, TagName};
pub use crate::scanner::position::Position;
pub use crate::scanner::span::Span;
pub use crate::style::*;
use num::str_to_u32;
mod error {
    use std::{error, fmt};
    use crate::scanner::span::Span;
    use crate::scanner::token::{Token, TokenKind};
    use crate::TagName;
    /// Represents an error with additional context such as its type, message, and location.
    pub struct Error<'src> {
        /// The type of the error.
        pub(crate) kind: ErrorKind<'src>,
        /// The span in the source where the error occurred.
        pub(crate) span: Span,
    }
    #[automatically_derived]
    impl<'src> ::core::fmt::Debug for Error<'src> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Error",
                "kind",
                &self.kind,
                "span",
                &&self.span,
            )
        }
    }
    impl<'src> Error<'src> {
        pub fn kind(&self) -> &ErrorKind<'src> {
            &self.kind
        }
    }
    impl<'src> error::Error for Error<'src> {}
    impl<'src> fmt::Display for Error<'src> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("error: ")?;
            match &self.kind {
                ErrorKind::UnexpectedToken { expected, found } => {
                    match found {
                        Some(found) => {
                            f.write_fmt(
                                format_args!(
                                    "unexpected token, expected {0:?}, found `{1}`",
                                    expected,
                                    found,
                                ),
                            )
                        }
                        None => {
                            f.write_fmt(
                                format_args!("unexpected token, expected {0:?}", expected),
                            )
                        }
                    }
                }
                ErrorKind::UnknownToken(tok) => {
                    f.write_fmt(format_args!("Unknown token: {0}", tok))
                }
                ErrorKind::MisMatchedTags { open, close } => {
                    f.write_fmt(
                        format_args!("mismatched tags: <{0}>...</{1}>", open, close),
                    )
                }
                ErrorKind::InvalidNumber(number) => {
                    f.write_fmt(format_args!("invalid number: `{0}`", number))
                }
                ErrorKind::InvalidColor(color) => {
                    f.write_fmt(format_args!("invalid color: \'{0}\'", color))
                }
                ErrorKind::InvalidTagName(name) => {
                    f.write_fmt(format_args!("invalid tag name: `{0}`", name))
                }
                ErrorKind::UnexpectedEof => f.write_str("Unexpected Eof"),
                ErrorKind::UnterminatedString => f.write_str("unterminated string"),
            }?;
            f.write_fmt(format_args!(" at {0}", self.span))
        }
    }
    impl<'src> Error<'src> {
        /// Creates a new `Error` instance.
        ///
        /// # Arguments
        ///
        /// * `kind` - The kind of error.
        /// * `token` - The token associated with the error.
        ///
        /// # Returns
        ///
        /// A new `Error` instance.
        pub(crate) fn new(kind: ErrorKind<'src>, token: &Token) -> Self {
            Self {
                kind,
                span: token.span.clone(),
            }
        }
    }
    #[non_exhaustive]
    /// Represents the different kinds of parse errors.
    pub enum ErrorKind<'src> {
        /// Indicates an invalid color was encountered.
        InvalidColor(&'src str),
        /// Indicates an invalid number was encountered.
        InvalidNumber(&'src str),
        InvalidTagName(&'src str),
        /// Mismatched opening and closing tags.
        MisMatchedTags { open: TagName<'src>, close: TagName<'src> },
        /// Indicates the end of input was reached unexpectedly.
        UnexpectedEof,
        /// Indicates an unexpected token was encountered.
        UnexpectedToken { expected: TokenKind, found: Option<&'src str> },
        /// An unknown token was encountered.
        UnknownToken(&'src str),
        /// Indicates an unterminated string literal.
        UnterminatedString,
    }
    #[automatically_derived]
    impl<'src> ::core::fmt::Debug for ErrorKind<'src> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ErrorKind::InvalidColor(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "InvalidColor",
                        &__self_0,
                    )
                }
                ErrorKind::InvalidNumber(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "InvalidNumber",
                        &__self_0,
                    )
                }
                ErrorKind::InvalidTagName(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "InvalidTagName",
                        &__self_0,
                    )
                }
                ErrorKind::MisMatchedTags { open: __self_0, close: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "MisMatchedTags",
                        "open",
                        __self_0,
                        "close",
                        &__self_1,
                    )
                }
                ErrorKind::UnexpectedEof => {
                    ::core::fmt::Formatter::write_str(f, "UnexpectedEof")
                }
                ErrorKind::UnexpectedToken { expected: __self_0, found: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "UnexpectedToken",
                        "expected",
                        __self_0,
                        "found",
                        &__self_1,
                    )
                }
                ErrorKind::UnknownToken(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "UnknownToken",
                        &__self_0,
                    )
                }
                ErrorKind::UnterminatedString => {
                    ::core::fmt::Formatter::write_str(f, "UnterminatedString")
                }
            }
        }
    }
    #[automatically_derived]
    impl<'src> ::core::marker::StructuralPartialEq for ErrorKind<'src> {}
    #[automatically_derived]
    impl<'src> ::core::cmp::PartialEq for ErrorKind<'src> {
        #[inline]
        fn eq(&self, other: &ErrorKind<'src>) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (
                        ErrorKind::InvalidColor(__self_0),
                        ErrorKind::InvalidColor(__arg1_0),
                    ) => __self_0 == __arg1_0,
                    (
                        ErrorKind::InvalidNumber(__self_0),
                        ErrorKind::InvalidNumber(__arg1_0),
                    ) => __self_0 == __arg1_0,
                    (
                        ErrorKind::InvalidTagName(__self_0),
                        ErrorKind::InvalidTagName(__arg1_0),
                    ) => __self_0 == __arg1_0,
                    (
                        ErrorKind::MisMatchedTags { open: __self_0, close: __self_1 },
                        ErrorKind::MisMatchedTags { open: __arg1_0, close: __arg1_1 },
                    ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                    (
                        ErrorKind::UnexpectedToken {
                            expected: __self_0,
                            found: __self_1,
                        },
                        ErrorKind::UnexpectedToken {
                            expected: __arg1_0,
                            found: __arg1_1,
                        },
                    ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                    (
                        ErrorKind::UnknownToken(__self_0),
                        ErrorKind::UnknownToken(__arg1_0),
                    ) => __self_0 == __arg1_0,
                    _ => true,
                }
        }
    }
}
mod num {
    use crate::ErrorKind;
    pub fn str_to_u32<'src>(s: &'src str, radix: u32) -> Result<u32, ErrorKind<'src>> {
        match u32::from_str_radix(s, radix) {
            Ok(n) => Ok(n),
            Err(_) => Err(ErrorKind::InvalidNumber(s)),
        }
    }
}
mod parser {
    use crate::error::ErrorKind;
    use crate::scanner::is_whitespace;
    use crate::scanner::token::Token;
    use crate::scanner::token::TokenKind;
    use crate::scanner::Scanner;
    use crate::style::Style;
    use crate::Error;
    use builtins::BUILTIN_STYLES;
    use parse_chunk::Chunk;
    use state::State;
    use std::collections::HashMap;
    use std::mem::take;
    pub use tag::{Tag, TagKind, TagName};
    mod builtins {
        use crate::scanner::Scanner;
        use crate::Style;
        use std::{collections::HashMap, sync::LazyLock};
        use super::parse_chunk::match_tag_name;
        use super::{Tag, TagKind, TagName};
        pub static BUILTIN_STYLES: LazyLock<HashMap<&str, Style>> = LazyLock::new(|| {
            [
                ("b", Tag::new(TagName::B, TagKind::Open).style),
                ("d", Tag::new(TagName::D, TagKind::Open).style),
                ("h", Tag::new(TagName::H, TagKind::Open).style),
                ("k", Tag::new(TagName::K, TagKind::Open).style),
                ("r", Tag::new(TagName::R, TagKind::Open).style),
                ("i", Tag::new(TagName::I, TagKind::Open).style),
                ("s", Tag::new(TagName::S, TagKind::Open).style),
                ("u", Tag::new(TagName::U, TagKind::Open).style),
            ]
                .into()
        });
        #[inline]
        pub fn is_builtin_tag(s: &str) -> bool {
            let mut scanner = Scanner::new(s);
            match scanner.scan_one() {
                Some(token) => {
                    match match_tag_name(&token).ok() {
                        Some(name) => {
                            return !match name {
                                TagName::Any(_) => true,
                                _ => false,
                            };
                        }
                        None => {}
                    }
                }
                None => {}
            }
            false
        }
    }
    mod close_tag {
        use super::{Context, Parser, Tag, TagName};
        use crate::{error::ErrorKind, Error};
        use std::ops::Not;
        impl<'src> Parser {
            pub(crate) fn parse_close_tag(
                &mut self,
                ctx: &mut Context<'src>,
                tag: &Tag<'src>,
            ) -> Result<(), Error<'src>> {
                let ctag = ctx.state.previous_tag_name();
                Self::expect_tag(
                    tag,
                    ctag.unwrap(),
                    ErrorKind::MisMatchedTags {
                        open: ctag.unwrap().clone(),
                        close: tag.name.clone(),
                    },
                )?;
                if tag.name == TagName::Pre {
                    self.pre_ws -= 1;
                } else if tag.name == TagName::Ziyy {
                    self.pre_ws += 1;
                }
                let current_tag = ctx.state.pop().unwrap_or_default();
                self.buf.push_str(&current_tag.2.not().to_string());
                Ok(())
            }
        }
    }
    mod helpers {}
    mod open_and_close_tag {
        use std::collections::HashMap;
        use crate::{get_num2, num::str_to_u32, Error};
        use super::Context;
        use super::{tag::Value, Parser, Tag, TagName};
        impl<'src> Parser {
            pub(crate) fn parse_open_and_close_tag(
                &mut self,
                ctx: &mut Context<'src>,
                tag: Tag<'src>,
            ) -> Result<(), Error<'src>> {
                match tag.name {
                    TagName::Br => {
                        if let Value::Some(val) = tag.custom {
                            let n: usize = str_to_u32(&val, 10)
                                .map_err(|k| Error { kind: k, span: tag.span })? as usize;
                            let _ = self.buf.push_str(&"\n".repeat(n));
                        } else {
                            let _ = self.buf.push('\n');
                        }
                    }
                    TagName::Let => {
                        if ctx.bindings.is_none() {
                            ctx.bindings = Some(HashMap::with_capacity(10));
                        }
                        if let Value::Some(ref name) = tag.custom {
                            ctx.bindings.as_mut().unwrap().insert(*name, tag.style);
                        }
                        self.skip_ws = true;
                    }
                    _ => {}
                }
                Ok(())
            }
        }
    }
    mod open_tag {
        use super::parse_chunk::Chunk;
        use super::tag::Value;
        use super::{Context, Parser, Tag, TagKind, TagName, BUILTIN_STYLES};
        use crate::num::str_to_u32;
        use crate::{get_num2, Error, ErrorKind};
        impl<'src> Parser {
            pub(crate) fn parse_open_tag(
                &mut self,
                ctx: &mut Context<'src>,
                mut tag: Tag<'src>,
            ) -> Result<(), Error<'src>> {
                match tag.name {
                    TagName::A => {
                        let _ = self.buf.push_str("\x1b]8;;");
                        let _ = self
                            .buf
                            .push_str(
                                if let Value::Some(ref href) = tag.custom {
                                    href
                                } else {
                                    ""
                                },
                            );
                        let _ = self.buf.push_str("\x1b\\");
                        loop {
                            let chunk = self.parse_chunk(ctx)?;
                            match chunk {
                                Chunk::Comment(_) => {}
                                Chunk::Escape(_) => {}
                                Chunk::Tag(tag2) => {
                                    if tag2.name == TagName::A && tag2.kind == TagKind::Close {
                                        break;
                                    }
                                }
                                Chunk::Text(text) => {
                                    let _ = self.buf.push_str(text);
                                }
                                Chunk::WhiteSpace(ws) => {
                                    let _ = self.buf.push_str(ws);
                                }
                                Chunk::Eof => {
                                    return Err(Error {
                                        kind: ErrorKind::UnexpectedEof,
                                        span: tag.span,
                                    });
                                }
                            }
                            let _ = self.buf.push_str("\x1b]8;;\x1b\\");
                        }
                    }
                    TagName::Any(s) => {
                        if let Some(bindings) = &ctx.bindings {
                            let src = bindings.get(s);
                            if let Some(btag) = src {
                                tag.inherit(btag);
                            }
                            self.write_and_save(ctx, &tag.name, tag.style);
                        }
                    }
                    TagName::Ansi
                    | TagName::B
                    | TagName::C
                    | TagName::Code
                    | TagName::D
                    | TagName::Empty
                    | TagName::H
                    | TagName::I
                    | TagName::K
                    | TagName::R
                    | TagName::S
                    | TagName::Span
                    | TagName::U
                    | TagName::X
                    | TagName::Ziyy => {
                        if tag.name == TagName::Ziyy {
                            self.pre_ws -= 1;
                        }
                        if let Value::Some(ref s) = tag.class {
                            for class in s
                                .split(|ch| ch == ' ')
                                .filter(|s| !s.is_empty())
                                .rev()
                            {
                                if let Some(btag) = BUILTIN_STYLES.get(class) {
                                    tag.inherit(btag);
                                } else {
                                    if let Some(bindings) = &ctx.bindings {
                                        if let Some(btag) = bindings.get(class) {
                                            tag.inherit(btag);
                                        }
                                    }
                                }
                            }
                        }
                        self.write_and_save(ctx, &tag.name, tag.style);
                    }
                    TagName::Br => {
                        if let Value::Some(val) = tag.custom {
                            let n: usize = str_to_u32(&val, 10)
                                .map_err(|k| Error { kind: k, span: tag.span })? as usize;
                            let _ = self.buf.push_str(&"\n".repeat(n));
                        } else {
                            let _ = self.buf.push('\n');
                        }
                    }
                    TagName::Let => {}
                    TagName::Div | TagName::P | TagName::Pre => {
                        if !self.block_start {
                            let _ = self.buf.push('\n');
                            self.block_start = true;
                        }
                        if tag.name == TagName::Pre {
                            self.pre_ws += 1;
                        }
                        if let Value::Some(ref s) = tag.class {
                            for class in s
                                .split(|ch| ch == ' ')
                                .filter(|s| !s.is_empty())
                                .rev()
                            {
                                if let Some(btag) = BUILTIN_STYLES.get(class) {
                                    tag.inherit(btag);
                                } else {
                                    if let Some(bindings) = &ctx.bindings {
                                        if let Some(btag) = bindings.get(class) {
                                            tag.inherit(btag);
                                        }
                                    }
                                }
                            }
                        }
                        match tag.custom {
                            Value::Bool => {
                                let _ = self.buf.push('\t');
                            }
                            Value::Some(val) => {
                                let n: usize = str_to_u32(&val, 10)
                                    .map_err(|k| Error { kind: k, span: tag.span })? as usize;
                                let _ = self.buf.push_str(&" ".repeat(n));
                            }
                            Value::None => {}
                        }
                        self.write_and_save(ctx, &tag.name, tag.style);
                        self.skip_ws = true;
                    }
                    TagName::None => {}
                }
                Ok(())
            }
        }
    }
    mod parse_chunk {
        use crate::error::ErrorKind;
        use crate::scanner::token::{Token, TokenKind};
        use crate::scanner::Scanner;
        use crate::{char_from_u32, Error};
        use crate::{number, style::*};
        use TokenKind::*;
        use super::{
            expect_token, tag::{Tag, TagKind, TagName, Value},
            Context, Parser,
        };
        pub enum Chunk<'src> {
            Comment(&'src str),
            Eof,
            Escape(char),
            Tag(Tag<'src>),
            Text(&'src str),
            WhiteSpace(&'src str),
        }
        #[automatically_derived]
        impl<'src> ::core::fmt::Debug for Chunk<'src> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Chunk::Comment(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Comment",
                            &__self_0,
                        )
                    }
                    Chunk::Eof => ::core::fmt::Formatter::write_str(f, "Eof"),
                    Chunk::Escape(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Escape",
                            &__self_0,
                        )
                    }
                    Chunk::Tag(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Tag",
                            &__self_0,
                        )
                    }
                    Chunk::Text(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Text",
                            &__self_0,
                        )
                    }
                    Chunk::WhiteSpace(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "WhiteSpace",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl<'src> ::core::marker::StructuralPartialEq for Chunk<'src> {}
        #[automatically_derived]
        impl<'src> ::core::cmp::PartialEq for Chunk<'src> {
            #[inline]
            fn eq(&self, other: &Chunk<'src>) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (Chunk::Comment(__self_0), Chunk::Comment(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (Chunk::Escape(__self_0), Chunk::Escape(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (Chunk::Tag(__self_0), Chunk::Tag(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (Chunk::Text(__self_0), Chunk::Text(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (Chunk::WhiteSpace(__self_0), Chunk::WhiteSpace(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl<'src> ::core::clone::Clone for Chunk<'src> {
            #[inline]
            fn clone(&self) -> Chunk<'src> {
                match self {
                    Chunk::Comment(__self_0) => {
                        Chunk::Comment(::core::clone::Clone::clone(__self_0))
                    }
                    Chunk::Eof => Chunk::Eof,
                    Chunk::Escape(__self_0) => {
                        Chunk::Escape(::core::clone::Clone::clone(__self_0))
                    }
                    Chunk::Tag(__self_0) => {
                        Chunk::Tag(::core::clone::Clone::clone(__self_0))
                    }
                    Chunk::Text(__self_0) => {
                        Chunk::Text(::core::clone::Clone::clone(__self_0))
                    }
                    Chunk::WhiteSpace(__self_0) => {
                        Chunk::WhiteSpace(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        impl<'src> Parser {
            #[allow(clippy::too_many_lines)]
            pub(super) fn parse_chunk(
                &mut self,
                ctx: &mut Context<'src>,
            ) -> Result<Chunk<'src>, Error<'src>> {
                if let Some(chunk) = ctx.next_chunk.clone() {
                    ctx.next_chunk = None;
                    return Ok(chunk);
                }
                let token = ctx.scanner.scan_token()?;
                let kind = match token.kind {
                    TokenKind::LESS => TagKind::Open,
                    TokenKind::LESS_SLASH => TagKind::Close,
                    _ => {
                        return match token.kind {
                            TokenKind::COMMENT => Ok(Chunk::Comment(token.content)),
                            TokenKind::TEXT => Ok(Chunk::Text(token.content)),
                            TokenKind::WHITESPACE => Ok(Chunk::WhiteSpace(token.content)),
                            TokenKind::ESCAPED => {
                                Ok(Chunk::Text(&token.content[3..token.content.len() - 4]))
                            }
                            TokenKind::EOF => Ok(Chunk::Eof),
                            TokenKind::ESC_0 => {
                                let num = crate::str_to_u32(&token.content[2..], 8)
                                    .map_err(|k| Error::<'src>::new(k, &token))?;
                                let unicode = char::from_u32(num);
                                if let Some(ch) = unicode {
                                    Ok(Chunk::Escape(ch))
                                } else {
                                    Ok(Chunk::Escape(char::REPLACEMENT_CHARACTER))
                                }
                            }
                            TokenKind::ESC_X | TokenKind::ESC_U => {
                                let num = crate::str_to_u32(&token.content[2..], 16)
                                    .map_err(|k| Error::<'src>::new(k, &token))?;
                                let unicode = char::from_u32(num);
                                if let Some(ch) = unicode {
                                    Ok(Chunk::Escape(ch))
                                } else {
                                    Ok(Chunk::Escape(char::REPLACEMENT_CHARACTER))
                                }
                            }
                            TokenKind::ESC_A => Ok(Chunk::Escape(7 as char)),
                            TokenKind::ESC_B => Ok(Chunk::Escape(8 as char)),
                            TokenKind::ESC_T => Ok(Chunk::Escape(9 as char)),
                            TokenKind::ESC_N => Ok(Chunk::Escape(10 as char)),
                            TokenKind::ESC_V => Ok(Chunk::Escape(11 as char)),
                            TokenKind::ESC_F => Ok(Chunk::Escape(12 as char)),
                            TokenKind::ESC_R => Ok(Chunk::Escape(13 as char)),
                            TokenKind::ESC_E => Ok(Chunk::Escape(27 as char)),
                            TokenKind::ESC_BACK_SLASH => Ok(Chunk::Escape('\\')),
                            TokenKind::ESC_LESS => Ok(Chunk::Escape('<')),
                            TokenKind::ESC_GREAT => Ok(Chunk::Escape('>')),
                            TokenKind::ANSI => {
                                Ok(
                                    Chunk::Tag(
                                        Tag::parse_from_ansi(
                                            &token.content[2..token.content.len() - 1],
                                            token.span,
                                        ),
                                    ),
                                )
                            }
                            TokenKind::ANSI_ESC => {
                                Ok(
                                    Chunk::Tag(
                                        Tag::parse_from_ansi(
                                            &token.content[5..token.content.len() - 1],
                                            token.span,
                                        ),
                                    ),
                                )
                            }
                            _ => {
                                Err(
                                    Error::new(
                                        ErrorKind::UnexpectedToken {
                                            expected: token.kind,
                                            found: None,
                                        },
                                        &token,
                                    ),
                                )
                            }
                        };
                    }
                };
                let token = ctx.scanner.scan_token()?;
                let mut style = Style::new();
                let tag_name = match token.kind {
                    GREAT if kind == TagKind::Open => {
                        return Ok(Chunk::Tag(Tag::new(TagName::None, kind)));
                    }
                    GREAT if kind == TagKind::Close => {
                        return Ok(Chunk::Tag(Tag::new(TagName::None, kind)));
                    }
                    _ => match_tag_name(&token)?,
                };
                let mut tag = Tag::new(tag_name.clone(), kind);
                tag.span += token.span;
                let mut token = ctx.scanner.scan_token()?;
                tag.span += token.span;
                loop {
                    match token.kind {
                        B => {
                            style.set_intensity(Intensity::Bold);
                            token = ctx.scanner.scan_token()?;
                            tag.span += token.span;
                            if token.kind == TokenKind::EQUAL {
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                                expect_token(&token, TokenKind::STRING)?;
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                            }
                        }
                        D => {
                            style.set_intensity(Intensity::Dim);
                            token = ctx.scanner.scan_token()?;
                            tag.span += token.span;
                            if token.kind == TokenKind::EQUAL {
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                                expect_token(&token, TokenKind::STRING)?;
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                            }
                        }
                        I => {
                            style.set_font_style(FontStyle::Italics);
                            token = ctx.scanner.scan_token()?;
                            tag.span += token.span;
                            if token.kind == TokenKind::EQUAL {
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                                expect_token(&token, TokenKind::STRING)?;
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                            }
                        }
                        U => {
                            style.set_underline(Underline::Single);
                            token = ctx.scanner.scan_token()?;
                            tag.span += token.span;
                            if token.kind == TokenKind::EQUAL {
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                                expect_token(&token, TokenKind::STRING)?;
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                            }
                        }
                        K => {
                            style.set_blink(Blink::Slow);
                            token = ctx.scanner.scan_token()?;
                            tag.span += token.span;
                            if token.kind == TokenKind::EQUAL {
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                                expect_token(&token, TokenKind::STRING)?;
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                            }
                        }
                        R => {
                            style.set_invert(Invert::Set);
                            token = ctx.scanner.scan_token()?;
                            tag.span += token.span;
                            if token.kind == TokenKind::EQUAL {
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                                expect_token(&token, TokenKind::STRING)?;
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                            }
                        }
                        H => {
                            style.set_hide(Hide::Set);
                            token = ctx.scanner.scan_token()?;
                            tag.span += token.span;
                            if token.kind == TokenKind::EQUAL {
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                                expect_token(&token, TokenKind::STRING)?;
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                            }
                        }
                        S => {
                            style.set_delete(Delete::Set);
                            token = ctx.scanner.scan_token()?;
                            tag.span += token.span;
                            if token.kind == TokenKind::EQUAL {
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                                expect_token(&token, TokenKind::STRING)?;
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                            }
                        }
                        UU => {
                            style.set_underline(Underline::Double);
                            token = ctx.scanner.scan_token()?;
                            tag.span += token.span;
                            if token.kind == TokenKind::EQUAL {
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                                expect_token(&token, TokenKind::STRING)?;
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                            }
                        }
                        DOUBLE => {
                            if tag_name == TagName::U {
                                {
                                    style.set_underline(Underline::Double);
                                    token = ctx.scanner.scan_token()?;
                                    tag.span += token.span;
                                    if token.kind == TokenKind::EQUAL {
                                        token = ctx.scanner.scan_token()?;
                                        tag.span += token.span;
                                        expect_token(&token, TokenKind::STRING)?;
                                        token = ctx.scanner.scan_token()?;
                                        tag.span += token.span;
                                    }
                                }
                            } else {
                                {
                                    token = ctx.scanner.scan_token()?;
                                    tag.span += token.span;
                                    if token.kind == TokenKind::EQUAL {
                                        token = ctx.scanner.scan_token()?;
                                        tag.span += token.span;
                                        expect_token(&token, TokenKind::STRING)?;
                                        token = ctx.scanner.scan_token()?;
                                    }
                                }
                            }
                        }
                        C => {
                            token = ctx.scanner.scan_token()?;
                            tag.span += token.span;
                            if token.kind == TokenKind::EQUAL {
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                                expect_token(&token, TokenKind::STRING)?;
                                let end = token.content.len() - 1;
                                let color = Color::parse(
                                    &token.content[1..end],
                                    token.span,
                                )?;
                                style.set_fg_color(color);
                                token = ctx.scanner.scan_token()?;
                            }
                        }
                        X => {
                            token = ctx.scanner.scan_token()?;
                            tag.span += token.span;
                            if token.kind == TokenKind::EQUAL {
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                                expect_token(&token, TokenKind::STRING)?;
                                let end = token.content.len() - 1;
                                let color = Color::parse(
                                    &token.content[1..end],
                                    token.span,
                                )?;
                                style.set_bg_color(color);
                                token = ctx.scanner.scan_token()?;
                            }
                        }
                        BLACK | BLUE | CYAN | GREEN | MAGENTA | RED | WHITE | YELLOW => {
                            let color = Color::parse(token.content, token.span)?;
                            if tag_name == TagName::C {
                                style.set_fg_color(color);
                            } else if tag_name == TagName::X {
                                style.set_bg_color(color);
                            }
                            token = ctx.scanner.scan_token()?;
                            tag.span += token.span;
                            if token.kind == TokenKind::EQUAL {
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                                expect_token(&token, TokenKind::STRING)?;
                            }
                        }
                        FIXED => {
                            token = ctx.scanner.scan_token()?;
                            tag.span += token.span;
                            if token.kind == TokenKind::EQUAL {
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                                expect_token(&token, TokenKind::STRING)?;
                            }
                            let end = token.content.len() - 1;
                            let s = &token.content[1..end];
                            let mut scanner = Scanner::new(s);
                            scanner.text_mode = false;
                            scanner.parse_colors = true;
                            scanner.current_pos = token.span.start;
                            let tok = scanner.scan_token()?;
                            let color = Color::Ansi256(
                                Ansi256(
                                    match (&tok).kind {
                                        TokenKind::NUMBER => {
                                            crate::str_to_u32(tok.content, 10)
                                                .map_err(|k| Error::<'src>::new(k, &tok))? as u8
                                        }
                                        _ => {
                                            return Err(
                                                Error::new(
                                                    ErrorKind::UnexpectedToken {
                                                        expected: TokenKind::NUMBER,
                                                        found: Some((&tok).content),
                                                    },
                                                    &tok,
                                                ),
                                            );
                                        }
                                    },
                                ),
                            );
                            if tag_name == TagName::C {
                                style.set_fg_color(color);
                            } else if tag_name == TagName::X {
                                style.set_bg_color(color);
                            }
                            token = ctx.scanner.scan_token()?;
                        }
                        RGB => {
                            token = ctx.scanner.scan_token()?;
                            tag.span += token.span;
                            if token.kind == TokenKind::EQUAL {
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                                expect_token(&token, TokenKind::STRING)?;
                            }
                            let end = token.content.len() - 1;
                            let s = &token.content[1..end];
                            let mut scanner = Scanner::new(s);
                            scanner.text_mode = false;
                            scanner.parse_colors = true;
                            scanner.current_pos = token.span.start;
                            let color = Color::Rgb(Rgb::parse(&mut scanner)?);
                            if tag_name == TagName::C {
                                style.set_fg_color(color);
                            } else if tag_name == TagName::X {
                                style.set_bg_color(color);
                            }
                            token = ctx.scanner.scan_token()?;
                        }
                        N => {
                            if tag_name == TagName::Br {
                                {
                                    tag.custom = Value::Bool;
                                    token = ctx.scanner.scan_token()?;
                                    tag.span += token.span;
                                    if token.kind == TokenKind::EQUAL {
                                        token = ctx.scanner.scan_token()?;
                                        tag.span += token.span;
                                        expect_token(&token, TokenKind::STRING)?;
                                        let end = token.content.len() - 1;
                                        tag.custom = Value::Some(&token.content[1..end]);
                                        token = ctx.scanner.scan_token()?;
                                    }
                                };
                            } else {
                                {
                                    token = ctx.scanner.scan_token()?;
                                    tag.span += token.span;
                                    if token.kind == TokenKind::EQUAL {
                                        token = ctx.scanner.scan_token()?;
                                        tag.span += token.span;
                                        expect_token(&token, TokenKind::STRING)?;
                                        token = ctx.scanner.scan_token()?;
                                    }
                                }
                            }
                        }
                        HREF => {
                            if tag_name == TagName::A {
                                {
                                    tag.custom = Value::Bool;
                                    token = ctx.scanner.scan_token()?;
                                    tag.span += token.span;
                                    if token.kind == TokenKind::EQUAL {
                                        token = ctx.scanner.scan_token()?;
                                        tag.span += token.span;
                                        expect_token(&token, TokenKind::STRING)?;
                                        let end = token.content.len() - 1;
                                        tag.custom = Value::Some(&token.content[1..end]);
                                        token = ctx.scanner.scan_token()?;
                                    }
                                };
                            } else {
                                {
                                    token = ctx.scanner.scan_token()?;
                                    tag.span += token.span;
                                    if token.kind == TokenKind::EQUAL {
                                        token = ctx.scanner.scan_token()?;
                                        tag.span += token.span;
                                        expect_token(&token, TokenKind::STRING)?;
                                        token = ctx.scanner.scan_token()?;
                                    }
                                }
                            }
                        }
                        ID => {
                            if tag_name == TagName::Let {
                                {
                                    tag.custom = Value::Bool;
                                    token = ctx.scanner.scan_token()?;
                                    tag.span += token.span;
                                    if token.kind == TokenKind::EQUAL {
                                        token = ctx.scanner.scan_token()?;
                                        tag.span += token.span;
                                        expect_token(&token, TokenKind::STRING)?;
                                        let end = token.content.len() - 1;
                                        tag.custom = Value::Some(&token.content[1..end]);
                                        token = ctx.scanner.scan_token()?;
                                    }
                                };
                            } else {
                                {
                                    token = ctx.scanner.scan_token()?;
                                    tag.span += token.span;
                                    if token.kind == TokenKind::EQUAL {
                                        token = ctx.scanner.scan_token()?;
                                        tag.span += token.span;
                                        expect_token(&token, TokenKind::STRING)?;
                                        token = ctx.scanner.scan_token()?;
                                    }
                                }
                            }
                        }
                        INDENT => {
                            if tag_name == TagName::P {
                                {
                                    tag.custom = Value::Bool;
                                    token = ctx.scanner.scan_token()?;
                                    tag.span += token.span;
                                    if token.kind == TokenKind::EQUAL {
                                        token = ctx.scanner.scan_token()?;
                                        tag.span += token.span;
                                        expect_token(&token, TokenKind::STRING)?;
                                        let end = token.content.len() - 1;
                                        tag.custom = Value::Some(&token.content[1..end]);
                                        token = ctx.scanner.scan_token()?;
                                    }
                                };
                            } else {
                                {
                                    token = ctx.scanner.scan_token()?;
                                    tag.span += token.span;
                                    if token.kind == TokenKind::EQUAL {
                                        token = ctx.scanner.scan_token()?;
                                        tag.span += token.span;
                                        expect_token(&token, TokenKind::STRING)?;
                                        token = ctx.scanner.scan_token()?;
                                    }
                                }
                            }
                        }
                        CLASS => {
                            tag.class = Value::Bool;
                            token = ctx.scanner.scan_token()?;
                            tag.span += token.span;
                            if token.kind == TokenKind::EQUAL {
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                                expect_token(&token, TokenKind::STRING)?;
                                let end = token.content.len() - 1;
                                tag.class = Value::Some(&token.content[1..end]);
                                token = ctx.scanner.scan_token()?;
                            }
                        }
                        IDENTIFIER => {
                            token = ctx.scanner.scan_token()?;
                            tag.span += token.span;
                            if token.kind == TokenKind::EQUAL {
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                                expect_token(&token, TokenKind::STRING)?;
                                token = ctx.scanner.scan_token()?;
                                tag.span += token.span;
                            }
                        }
                        _ => break,
                    }
                }
                tag.style = style;
                match token.kind {
                    TokenKind::GREAT => {}
                    TokenKind::SLASH_GREAT => {
                        tag.kind = TagKind::SelfClose;
                    }
                    _ => {
                        return Err(
                            Error::new(
                                ErrorKind::UnexpectedToken {
                                    expected: token.kind,
                                    found: None,
                                },
                                &token,
                            ),
                        );
                    }
                }
                Ok(Chunk::Tag(tag))
            }
            pub(super) fn parse_next_chunk(
                &mut self,
                ctx: &mut Context<'src>,
            ) -> Result<Chunk<'src>, Error<'src>> {
                let chunk = self.parse_chunk(ctx)?;
                ctx.next_chunk = Some(chunk.clone());
                Ok(chunk)
            }
        }
        pub(crate) fn match_tag_name<'src>(
            token: &Token<'src>,
        ) -> Result<TagName<'src>, Error<'src>> {
            use TokenKind::*;
            let kind = match token.kind {
                A => TagName::A,
                B => TagName::B,
                BR => TagName::Br,
                C => TagName::C,
                CODE => TagName::Code,
                D => TagName::D,
                DIV => TagName::Div,
                H => TagName::H,
                I => TagName::I,
                K => TagName::K,
                LET => TagName::Let,
                P => TagName::P,
                PRE => TagName::Pre,
                R => TagName::R,
                S => TagName::S,
                SPAN => TagName::Span,
                U => TagName::U,
                X => TagName::X,
                ZIYY => TagName::Ziyy,
                IDENTIFIER
                | BLACK
                | BLUE
                | CYAN
                | GREEN
                | MAGENTA
                | RED
                | WHITE
                | YELLOW
                | FIXED
                | RGB
                | CLASS
                | CURLY
                | DASHED
                | DOUBLE
                | DOTTED
                | ID
                | INDENT
                | HREF
                | N
                | NONE
                | SINGLE => TagName::Any(token.content),
                _ => {
                    return Err(Error {
                        kind: ErrorKind::InvalidTagName(token.content),
                        span: token.span,
                    });
                }
            };
            Ok(kind)
        }
    }
    mod state {
        use crate::style::Style;
        use super::tag::TagName;
        pub struct State<'src>(pub(crate) Vec<(TagName<'src>, Style, Style)>);
        impl<'src> State<'src> {
            pub fn new() -> Self {
                State(::alloc::vec::Vec::new())
            }
            pub fn push(&mut self, tag_name: TagName<'src>, style: Style, delta: Style) {
                self.0.push((tag_name, style, delta));
            }
            pub fn pop(&mut self) -> Option<(TagName<'src>, Style, Style)> {
                self.0.pop()
            }
            pub fn previous_tag_name(&self) -> Option<&TagName<'src>> {
                let i = self.0.len() - 1;
                self.0.get(i).map(|x| &x.0)
            }
            pub fn previous_style(&self) -> Style {
                match self.0.last() {
                    Some(v) => v.1,
                    None => Style::new(),
                }
            }
        }
    }
    mod tag {
        #![allow(missing_docs)]
        use std::fmt::Display;
        use crate::scanner::span::Span;
        use crate::style::*;
        pub enum Value<'src> {
            Bool,
            Some(&'src str),
            None,
        }
        #[automatically_derived]
        impl<'src> ::core::marker::StructuralPartialEq for Value<'src> {}
        #[automatically_derived]
        impl<'src> ::core::cmp::PartialEq for Value<'src> {
            #[inline]
            fn eq(&self, other: &Value<'src>) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (Value::Some(__self_0), Value::Some(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl<'src> ::core::fmt::Debug for Value<'src> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Value::Bool => ::core::fmt::Formatter::write_str(f, "Bool"),
                    Value::Some(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Some",
                            &__self_0,
                        )
                    }
                    Value::None => ::core::fmt::Formatter::write_str(f, "None"),
                }
            }
        }
        #[automatically_derived]
        impl<'src> ::core::clone::Clone for Value<'src> {
            #[inline]
            fn clone(&self) -> Value<'src> {
                match self {
                    Value::Bool => Value::Bool,
                    Value::Some(__self_0) => {
                        Value::Some(::core::clone::Clone::clone(__self_0))
                    }
                    Value::None => Value::None,
                }
            }
        }
        /// Ziyy Tag.
        pub struct Tag<'src> {
            /// Name of Tag.
            pub name: TagName<'src>,
            /// Kind of Tag.
            pub kind: TagKind,
            /// Custom information.
            pub custom: Value<'src>,
            /// Style information of the Tag.
            pub style: Style,
            /// Class.
            pub class: Value<'src>,
            /// Span
            pub span: Span,
        }
        #[automatically_derived]
        impl<'src> ::core::marker::StructuralPartialEq for Tag<'src> {}
        #[automatically_derived]
        impl<'src> ::core::cmp::PartialEq for Tag<'src> {
            #[inline]
            fn eq(&self, other: &Tag<'src>) -> bool {
                self.name == other.name && self.kind == other.kind
                    && self.custom == other.custom && self.style == other.style
                    && self.class == other.class && self.span == other.span
            }
        }
        #[automatically_derived]
        impl<'src> ::core::fmt::Debug for Tag<'src> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "name",
                    "kind",
                    "custom",
                    "style",
                    "class",
                    "span",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.name,
                    &self.kind,
                    &self.custom,
                    &self.style,
                    &self.class,
                    &&self.span,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "Tag",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl<'src> ::core::clone::Clone for Tag<'src> {
            #[inline]
            fn clone(&self) -> Tag<'src> {
                Tag {
                    name: ::core::clone::Clone::clone(&self.name),
                    kind: ::core::clone::Clone::clone(&self.kind),
                    custom: ::core::clone::Clone::clone(&self.custom),
                    style: ::core::clone::Clone::clone(&self.style),
                    class: ::core::clone::Clone::clone(&self.class),
                    span: ::core::clone::Clone::clone(&self.span),
                }
            }
        }
        impl<'src> Tag<'src> {
            /// Creates new Tag
            #[must_use]
            pub fn new(name: TagName<'src>, kind: TagKind) -> Self {
                let mut style = Style::new();
                match name {
                    TagName::B => {
                        style.set_prev_intensity(style.intensity());
                        style.set_intensity(Intensity::Bold);
                    }
                    TagName::D => {
                        style.set_prev_intensity(style.intensity());
                        style.set_intensity(Intensity::Dim);
                    }
                    TagName::H => style.set_hide(Hide::Set),
                    TagName::I => style.set_font_style(FontStyle::Italics),
                    TagName::K => style.set_blink(Blink::Slow),
                    TagName::R => style.set_invert(Invert::Set),
                    TagName::S => style.set_delete(Delete::Set),
                    TagName::U => style.set_underline(Underline::Single),
                    _ => {}
                }
                Self {
                    kind,
                    name,
                    custom: Value::None,
                    style,
                    class: Value::None,
                    span: Span::initial(),
                }
            }
            /// Inherits style properties from the source style.
            pub fn inherit(&mut self, src: &Style) {
                if self.style.intensity().is_unset() & src.intensity().is_set() {
                    self.style.set_intensity(src.intensity());
                }
                if self.style.font_style().is_unset() & src.font_style().is_set() {
                    self.style.set_font_style(src.font_style());
                }
                if self.style.underline().is_unset() & src.underline().is_set() {
                    self.style.set_underline(src.underline());
                }
                if self.style.blink().is_unset() & src.blink().is_set() {
                    self.style.set_blink(src.blink());
                }
                if self.style.invert().is_unset() & src.invert().is_set() {
                    self.style.set_invert(src.invert());
                }
                if self.style.hide().is_unset() & src.hide().is_set() {
                    self.style.set_hide(src.hide());
                }
                if self.style.delete().is_unset() & src.delete().is_set() {
                    self.style.set_delete(src.delete());
                }
                if self.style.font().is_unset() & src.font().is_set() {
                    self.style.set_font(src.font());
                }
                if self.style.prop_space().is_unset() & src.prop_space().is_set() {
                    self.style.set_prop_space(src.prop_space());
                }
                if self.style.fg_color().is_unset() & src.fg_color().is_set() {
                    self.style.set_fg_color(src.fg_color());
                }
                if self.style.bg_color().is_unset() & src.bg_color().is_set() {
                    self.style.set_bg_color(src.bg_color());
                }
                if self.style.frame().is_unset() & src.frame().is_set() {
                    self.style.set_frame(src.frame());
                }
                if self.style.overline().is_unset() & src.overline().is_set() {
                    self.style.set_overline(src.overline());
                }
                if self.style.reserved1().is_unset() & src.reserved1().is_set() {
                    self.style.set_reserved1(src.reserved1());
                }
                if self.style.reserved2().is_unset() & src.reserved2().is_set() {
                    self.style.set_reserved2(src.reserved2());
                }
                if self.style.ul_color().is_unset() & src.ul_color().is_set() {
                    self.style.set_ul_color(src.ul_color());
                }
            }
            pub(crate) fn parse_from_ansi(source: &str, span: Span) -> Self {
                let mut parts = source.split(";").peekable();
                let mut style = Style::default();
                loop {
                    let part = parts.next();
                    let part = match part {
                        Some(p) => p,
                        None => break,
                    };
                    match part {
                        "" | "0" => style = Style::default(),
                        "1" => style.set_intensity(Intensity::Bold),
                        "2" => style.set_intensity(Intensity::Dim),
                        "3" => style.set_font_style(FontStyle::Italics),
                        "4" => style.set_underline(Underline::Single),
                        "4:3" => style.set_underline(Underline::Curly),
                        "4:4" => style.set_underline(Underline::Dotted),
                        "4:5" => style.set_underline(Underline::Dashed),
                        "5" => style.set_blink(Blink::Slow),
                        "6" => style.set_blink(Blink::Fast),
                        "7" => style.set_invert(Invert::Set),
                        "8" => style.set_hide(Hide::Set),
                        "9" => style.set_delete(Delete::Set),
                        "10" => style.set_font(Font::Primary),
                        "11" => style.set_font(Font::FirstAlt),
                        "12" => style.set_font(Font::SecondAlt),
                        "13" => style.set_font(Font::ThirdAlt),
                        "14" => style.set_font(Font::FourthAlt),
                        "15" => style.set_font(Font::FifthAlt),
                        "16" => style.set_font(Font::SixthAlt),
                        "17" => style.set_font(Font::SeventhAlt),
                        "18" => style.set_font(Font::EighthAlt),
                        "19" => style.set_font(Font::NinthAlt),
                        "20" => style.set_font_style(FontStyle::Fraktur),
                        "21" => style.set_underline(Underline::Double),
                        "22" => style.set_intensity(Intensity::Unset),
                        "23" => style.set_font_style(FontStyle::Unset),
                        "24" => style.set_underline(Underline::Unset),
                        "25" => style.set_blink(Blink::Unset),
                        "26" => style.set_prop_space(PropSpace::Set),
                        "27" => style.set_invert(Invert::Unset),
                        "28" => style.set_hide(Hide::Unset),
                        "29" => style.set_delete(Delete::Unset),
                        "30" => style.set_fg_color(Color::AnsiColor(AnsiColor::Black)),
                        "31" => style.set_fg_color(Color::AnsiColor(AnsiColor::Red)),
                        "32" => style.set_fg_color(Color::AnsiColor(AnsiColor::Green)),
                        "33" => style.set_fg_color(Color::AnsiColor(AnsiColor::Yellow)),
                        "34" => style.set_fg_color(Color::AnsiColor(AnsiColor::Blue)),
                        "35" => style.set_fg_color(Color::AnsiColor(AnsiColor::Magenta)),
                        "36" => style.set_fg_color(Color::AnsiColor(AnsiColor::Cyan)),
                        "37" => style.set_fg_color(Color::AnsiColor(AnsiColor::White)),
                        "38" => {
                            if part == "2" {
                                let r = match parts.next() {
                                    Some(s) => {
                                        match u8::from_str_radix(s, 10) {
                                            Ok(n) => n,
                                            Err(_) => continue,
                                        }
                                    }
                                    None => continue,
                                };
                                let g = match parts.next() {
                                    Some(s) => {
                                        match u8::from_str_radix(s, 10) {
                                            Ok(n) => n,
                                            Err(_) => continue,
                                        }
                                    }
                                    None => continue,
                                };
                                let b = match parts.next() {
                                    Some(s) => {
                                        match u8::from_str_radix(s, 10) {
                                            Ok(n) => n,
                                            Err(_) => continue,
                                        }
                                    }
                                    None => continue,
                                };
                                style.set_fg_color(Color::Rgb(Rgb(r, g, b)));
                            }
                            if part == "5" {
                                let fixed = match parts.next() {
                                    Some(s) => {
                                        match u8::from_str_radix(s, 10) {
                                            Ok(n) => n,
                                            Err(_) => continue,
                                        }
                                    }
                                    None => continue,
                                };
                                style.set_fg_color(Color::Ansi256(Ansi256(fixed)));
                            }
                        }
                        "39" => style.set_bg_color(Color::Unset),
                        "40" => style.set_bg_color(Color::AnsiColor(AnsiColor::Black)),
                        "41" => style.set_bg_color(Color::AnsiColor(AnsiColor::Red)),
                        "42" => style.set_bg_color(Color::AnsiColor(AnsiColor::Green)),
                        "43" => style.set_bg_color(Color::AnsiColor(AnsiColor::Yellow)),
                        "44" => style.set_bg_color(Color::AnsiColor(AnsiColor::Blue)),
                        "45" => style.set_bg_color(Color::AnsiColor(AnsiColor::Magenta)),
                        "46" => style.set_bg_color(Color::AnsiColor(AnsiColor::Cyan)),
                        "47" => style.set_bg_color(Color::AnsiColor(AnsiColor::White)),
                        "48" => {
                            if part == "2" {
                                let r = match parts.next() {
                                    Some(s) => {
                                        match u8::from_str_radix(s, 10) {
                                            Ok(n) => n,
                                            Err(_) => continue,
                                        }
                                    }
                                    None => continue,
                                };
                                let g = match parts.next() {
                                    Some(s) => {
                                        match u8::from_str_radix(s, 10) {
                                            Ok(n) => n,
                                            Err(_) => continue,
                                        }
                                    }
                                    None => continue,
                                };
                                let b = match parts.next() {
                                    Some(s) => {
                                        match u8::from_str_radix(s, 10) {
                                            Ok(n) => n,
                                            Err(_) => continue,
                                        }
                                    }
                                    None => continue,
                                };
                                style.set_fg_color(Color::Rgb(Rgb(r, g, b)));
                            }
                            if part == "5" {
                                let fixed = match parts.next() {
                                    Some(s) => {
                                        match u8::from_str_radix(s, 10) {
                                            Ok(n) => n,
                                            Err(_) => continue,
                                        }
                                    }
                                    None => continue,
                                };
                                style.set_bg_color(Color::Ansi256(Ansi256(fixed)));
                            }
                        }
                        "49" => style.set_bg_color(Color::Unset),
                        "50" => style.set_prop_space(PropSpace::Unset),
                        "51" => style.set_frame(Frame::Framed),
                        "52" => style.set_frame(Frame::Encircled),
                        "53" => style.set_overline(Overline::Set),
                        "54" => style.set_frame(Frame::Unset),
                        "55" => style.set_overline(Overline::Unset),
                        "56" => style.set_reserved1(Reserved1::Yes),
                        "57" => style.set_reserved2(Reserved2::Yes),
                        "58" => {
                            if part == "2" {
                                let r = match parts.next() {
                                    Some(s) => {
                                        match u8::from_str_radix(s, 10) {
                                            Ok(n) => n,
                                            Err(_) => continue,
                                        }
                                    }
                                    None => continue,
                                };
                                let g = match parts.next() {
                                    Some(s) => {
                                        match u8::from_str_radix(s, 10) {
                                            Ok(n) => n,
                                            Err(_) => continue,
                                        }
                                    }
                                    None => continue,
                                };
                                let b = match parts.next() {
                                    Some(s) => {
                                        match u8::from_str_radix(s, 10) {
                                            Ok(n) => n,
                                            Err(_) => continue,
                                        }
                                    }
                                    None => continue,
                                };
                                style.set_ul_color(Color::Rgb(Rgb(r, g, b)));
                            }
                            if part == "5" {
                                let fixed = match parts.next() {
                                    Some(s) => {
                                        match u8::from_str_radix(s, 10) {
                                            Ok(n) => n,
                                            Err(_) => continue,
                                        }
                                    }
                                    None => continue,
                                };
                                style.set_ul_color(Color::Ansi256(Ansi256(fixed)));
                            }
                        }
                        "59" => style.set_ul_color(Color::Unset),
                        "90" => {
                            style.set_fg_color(Color::AnsiColor(AnsiColor::BrightBlack))
                        }
                        "91" => {
                            style.set_fg_color(Color::AnsiColor(AnsiColor::BrightRed))
                        }
                        "92" => {
                            style.set_fg_color(Color::AnsiColor(AnsiColor::BrightGreen))
                        }
                        "93" => {
                            style.set_fg_color(Color::AnsiColor(AnsiColor::BrightYellow))
                        }
                        "94" => {
                            style.set_fg_color(Color::AnsiColor(AnsiColor::BrightBlue))
                        }
                        "95" => {
                            style
                                .set_fg_color(Color::AnsiColor(AnsiColor::BrightMagenta))
                        }
                        "96" => {
                            style.set_fg_color(Color::AnsiColor(AnsiColor::BrightCyan))
                        }
                        "97" => {
                            style.set_fg_color(Color::AnsiColor(AnsiColor::BrightWhite))
                        }
                        "100" => {
                            style.set_bg_color(Color::AnsiColor(AnsiColor::BrightBlack))
                        }
                        "101" => {
                            style.set_bg_color(Color::AnsiColor(AnsiColor::BrightRed))
                        }
                        "102" => {
                            style.set_bg_color(Color::AnsiColor(AnsiColor::BrightGreen))
                        }
                        "103" => {
                            style.set_bg_color(Color::AnsiColor(AnsiColor::BrightYellow))
                        }
                        "104" => {
                            style.set_bg_color(Color::AnsiColor(AnsiColor::BrightBlue))
                        }
                        "105" => {
                            style
                                .set_bg_color(Color::AnsiColor(AnsiColor::BrightMagenta))
                        }
                        "106" => {
                            style.set_bg_color(Color::AnsiColor(AnsiColor::BrightCyan))
                        }
                        "107" => {
                            style.set_bg_color(Color::AnsiColor(AnsiColor::BrightWhite))
                        }
                        _ => {}
                    }
                }
                Tag {
                    name: TagName::Ansi,
                    kind: TagKind::Open,
                    custom: Value::None,
                    style,
                    class: Value::None,
                    span,
                }
            }
        }
        pub enum TagKind {
            Open,
            Close,
            SelfClose,
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for TagKind {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for TagKind {
            #[inline]
            fn eq(&self, other: &TagKind) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TagKind {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        TagKind::Open => "Open",
                        TagKind::Close => "Close",
                        TagKind::SelfClose => "SelfClose",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TagKind {
            #[inline]
            fn clone(&self) -> TagKind {
                match self {
                    TagKind::Open => TagKind::Open,
                    TagKind::Close => TagKind::Close,
                    TagKind::SelfClose => TagKind::SelfClose,
                }
            }
        }
        pub enum TagName<'src> {
            A,
            Any(&'src str),
            Ansi,
            B,
            Br,
            C,
            Code,
            D,
            Div,
            H,
            K,
            I,
            Let,
            P,
            Pre,
            R,
            S,
            Span,
            U,
            X,
            Ziyy,
            Empty,
            #[default]
            None,
        }
        #[automatically_derived]
        impl<'src> ::core::default::Default for TagName<'src> {
            #[inline]
            fn default() -> TagName<'src> {
                Self::None
            }
        }
        #[automatically_derived]
        impl<'src> ::core::marker::StructuralPartialEq for TagName<'src> {}
        #[automatically_derived]
        impl<'src> ::core::cmp::PartialEq for TagName<'src> {
            #[inline]
            fn eq(&self, other: &TagName<'src>) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (TagName::Any(__self_0), TagName::Any(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl<'src> ::core::fmt::Debug for TagName<'src> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    TagName::A => ::core::fmt::Formatter::write_str(f, "A"),
                    TagName::Any(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Any",
                            &__self_0,
                        )
                    }
                    TagName::Ansi => ::core::fmt::Formatter::write_str(f, "Ansi"),
                    TagName::B => ::core::fmt::Formatter::write_str(f, "B"),
                    TagName::Br => ::core::fmt::Formatter::write_str(f, "Br"),
                    TagName::C => ::core::fmt::Formatter::write_str(f, "C"),
                    TagName::Code => ::core::fmt::Formatter::write_str(f, "Code"),
                    TagName::D => ::core::fmt::Formatter::write_str(f, "D"),
                    TagName::Div => ::core::fmt::Formatter::write_str(f, "Div"),
                    TagName::H => ::core::fmt::Formatter::write_str(f, "H"),
                    TagName::K => ::core::fmt::Formatter::write_str(f, "K"),
                    TagName::I => ::core::fmt::Formatter::write_str(f, "I"),
                    TagName::Let => ::core::fmt::Formatter::write_str(f, "Let"),
                    TagName::P => ::core::fmt::Formatter::write_str(f, "P"),
                    TagName::Pre => ::core::fmt::Formatter::write_str(f, "Pre"),
                    TagName::R => ::core::fmt::Formatter::write_str(f, "R"),
                    TagName::S => ::core::fmt::Formatter::write_str(f, "S"),
                    TagName::Span => ::core::fmt::Formatter::write_str(f, "Span"),
                    TagName::U => ::core::fmt::Formatter::write_str(f, "U"),
                    TagName::X => ::core::fmt::Formatter::write_str(f, "X"),
                    TagName::Ziyy => ::core::fmt::Formatter::write_str(f, "Ziyy"),
                    TagName::Empty => ::core::fmt::Formatter::write_str(f, "Empty"),
                    TagName::None => ::core::fmt::Formatter::write_str(f, "None"),
                }
            }
        }
        #[automatically_derived]
        impl<'src> ::core::clone::Clone for TagName<'src> {
            #[inline]
            fn clone(&self) -> TagName<'src> {
                match self {
                    TagName::A => TagName::A,
                    TagName::Any(__self_0) => {
                        TagName::Any(::core::clone::Clone::clone(__self_0))
                    }
                    TagName::Ansi => TagName::Ansi,
                    TagName::B => TagName::B,
                    TagName::Br => TagName::Br,
                    TagName::C => TagName::C,
                    TagName::Code => TagName::Code,
                    TagName::D => TagName::D,
                    TagName::Div => TagName::Div,
                    TagName::H => TagName::H,
                    TagName::K => TagName::K,
                    TagName::I => TagName::I,
                    TagName::Let => TagName::Let,
                    TagName::P => TagName::P,
                    TagName::Pre => TagName::Pre,
                    TagName::R => TagName::R,
                    TagName::S => TagName::S,
                    TagName::Span => TagName::Span,
                    TagName::U => TagName::U,
                    TagName::X => TagName::X,
                    TagName::Ziyy => TagName::Ziyy,
                    TagName::Empty => TagName::Empty,
                    TagName::None => TagName::None,
                }
            }
        }
        impl<'src> Display for TagName<'src> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(
                    match self {
                        TagName::A => "a",
                        TagName::Any(any) => any,
                        TagName::Ansi => "$ansi",
                        TagName::B => "b",
                        TagName::Br => "br",
                        TagName::C => "c",
                        TagName::Code => "code",
                        TagName::D => "d",
                        TagName::Div => "div",
                        TagName::H => "h",
                        TagName::I => "i",
                        TagName::K => "k",
                        TagName::Let => "let",
                        TagName::P => "p",
                        TagName::Pre => "pre",
                        TagName::R => "r",
                        TagName::S => "s",
                        TagName::Span => "span",
                        TagName::U => "u",
                        TagName::X => "x",
                        TagName::Ziyy => "ziyy",
                        TagName::Empty => "",
                        TagName::None => "",
                    },
                )
            }
        }
    }
    pub struct Context<'src> {
        /// The scanner used to tokenize the input source.
        pub(crate) scanner: Scanner<'src>,
        /// Optional bindings for styles.
        pub(crate) bindings: Option<HashMap<&'src str, Style>>,
        /// The current state of the parser.
        pub(crate) state: State<'src>,
        /// The next chunk to be parsed.
        pub(crate) next_chunk: Option<Chunk<'src>>,
    }
    impl<'src> Context<'src> {
        pub fn new(
            source: &'src str,
            bindings: Option<HashMap<&'src str, Style>>,
        ) -> Self {
            Self {
                scanner: Scanner::new(source),
                bindings,
                state: State::new(),
                next_chunk: None,
            }
        }
    }
    /// A parser for the Ziyy language.
    pub struct Parser {
        /// A buffer to store the parsed output.
        pub(crate) buf: String,
        /// Flag to indicate whether to skip white space.
        pub(crate) skip_ws: bool,
        /// Flag to indicate whether to style the text exactly as it is.
        pub(crate) pre_ws: i16,
        /// The last written printable element.
        pub(crate) block_start: bool,
    }
    impl<'src> Parser {
        /// Creates a new Ziyy Parser.
        ///
        /// # Arguments
        ///
        /// * `source` - The source input to be parsed.
        /// * `bindings` - Optional bindings for styles.
        ///
        /// # Returns
        ///
        /// A new instance of `Parser`.
        pub fn new() -> Parser {
            Parser {
                buf: String::new(),
                skip_ws: true,
                pre_ws: 1,
                block_start: true,
            }
        }
        /// Parses source and Returns a [String].
        /// # Errors
        ///
        /// Returns an `Error` if parsing fails.
        pub fn parse(&mut self, mut ctx: Context<'src>) -> Result<String, Error<'src>> {
            loop {
                let parsed = self.parse_chunk(&mut ctx)?;
                match parsed {
                    Chunk::Comment(_) => {}
                    Chunk::Escape(ch) => {
                        self.buf.push(ch);
                        self.skip_ws = is_whitespace(ch);
                        self.block_start = self.skip_ws;
                    }
                    Chunk::Tag(tag) => {
                        match tag.kind {
                            TagKind::Open => self.parse_open_tag(&mut ctx, tag)?,
                            TagKind::Close => self.parse_close_tag(&mut ctx, &tag)?,
                            TagKind::SelfClose => {
                                self.parse_open_and_close_tag(&mut ctx, tag)?
                            }
                        }
                    }
                    Chunk::Text(text) => {
                        self.buf.push_str(text);
                        self.skip_ws = false;
                        self.block_start = false;
                    }
                    Chunk::WhiteSpace(ws) => {
                        let chunk = self.parse_next_chunk(&mut ctx)?;
                        if self.pre_ws > 0 {
                            self.buf.push_str(ws);
                        } else {
                            if let Chunk::Eof = chunk {
                                if ws.contains('\n') {
                                    self.buf.push('\n');
                                }
                            } else if !self.skip_ws {
                                self.buf.push(' ');
                                self.skip_ws = true;
                            }
                        }
                    }
                    Chunk::Eof => {
                        return Ok(take(&mut self.buf));
                    }
                }
            }
        }
        /// Parses the source and returns a `Vec<u8>`.
        ///
        /// # Errors
        ///
        /// Returns an [Error] if parsing fails.
        pub fn parse_to_bytes(
            &mut self,
            ctx: Context<'src>,
        ) -> Result<Vec<u8>, Error<'src>> {
            match self.parse(ctx) {
                Ok(res) => Ok(res.into_bytes()),
                Err(err) => Err(err),
            }
        }
        /// Checks if the given tag matches the expected tag name.
        ///
        /// # Arguments
        ///
        /// * `tag` - The tag to be checked.
        /// * `to_be` - The expected tag name.
        /// * `err` - The error kind to be returned if the tag does not match.
        ///
        /// # Returns
        ///
        /// Returns `Ok(())` if the tag matches, otherwise returns an `Error`.
        fn expect_tag(
            tag: &Tag<'src>,
            to_be: &TagName<'src>,
            err: ErrorKind<'src>,
        ) -> Result<(), Error<'src>> {
            if tag.name == *to_be {
                Ok(())
            } else {
                Err(Error {
                    kind: err,
                    span: tag.span.clone(),
                })
            }
        }
        /// Writes the style to the buffer and saves the current style state.
        ///
        /// # Arguments
        ///
        /// * `tag_name` - The name of the tag.
        /// * `style` - The style to be written and saved.
        fn write_and_save(
            &mut self,
            ctx: &mut Context<'src>,
            tag_name: &TagName<'src>,
            style: Style,
        ) {
            let prev = ctx.state.previous_style();
            let new = prev + style;
            let delta = style - prev;
            self.buf.push_str(&delta.to_string());
            ctx.state.push(tag_name.clone(), new, delta);
        }
    }
    /// Checks if the given token matches the expected token kind.
    ///
    /// # Arguments
    ///
    /// * `token` - The token to be checked.
    /// * `tt` - The expected token kind.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the token matches, otherwise returns an `Error`.
    fn expect_token<'src>(
        token: &Token<'src>,
        tt: TokenKind,
    ) -> Result<(), Error<'src>> {
        if token.kind != tt {
            return Err(
                Error::new(
                    ErrorKind::UnexpectedToken {
                        expected: tt,
                        found: Some(token.content),
                    },
                    token,
                ),
            );
        }
        Ok(())
    }
}
mod scanner {
    pub mod position {
        use core::fmt::Debug;
        use std::fmt::Display;
        pub struct Position {
            pub row: u16,
            pub col: u16,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Position {
            #[inline]
            fn clone(&self) -> Position {
                let _: ::core::clone::AssertParamIsClone<u16>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Position {}
        #[automatically_derived]
        impl ::core::default::Default for Position {
            #[inline]
            fn default() -> Position {
                Position {
                    row: ::core::default::Default::default(),
                    col: ::core::default::Default::default(),
                }
            }
        }
        impl Position {
            pub fn new(row: u16, col: u16) -> Self {
                Self { row, col }
            }
        }
        impl PartialEq for Position {
            fn eq(&self, other: &Self) -> bool {
                self.row == other.row && self.col == other.col
            }
        }
        impl Debug for Position {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_fmt(format_args!("({0},{1})", self.row + 1, self.col + 1))
            }
        }
        impl Display for Position {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("{0}:{1}", self.row + 1, self.col + 1))
            }
        }
    }
    pub mod span {
        use std::{fmt::Display, ops::{Add, AddAssign}};
        use super::position::Position;
        pub struct Span {
            pub start: Position,
            pub end: Position,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Span {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Span",
                    "start",
                    &self.start,
                    "end",
                    &&self.end,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Span {
            #[inline]
            fn default() -> Span {
                Span {
                    start: ::core::default::Default::default(),
                    end: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Span {
            #[inline]
            fn clone(&self) -> Span {
                let _: ::core::clone::AssertParamIsClone<Position>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Span {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Span {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Span {
            #[inline]
            fn eq(&self, other: &Span) -> bool {
                self.start == other.start && self.end == other.end
            }
        }
        impl Span {
            pub fn new(start: Position, end: Position) -> Self {
                Self { start, end }
            }
            pub(crate) const fn initial() -> Self {
                Self {
                    start: Position { row: 0, col: 0 },
                    end: Position { row: 0, col: 0 },
                }
            }
        }
        impl Add for Span {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self::new(self.start, rhs.end)
            }
        }
        impl AddAssign for Span {
            fn add_assign(&mut self, rhs: Self) {
                self.end = rhs.end;
            }
        }
        impl AddAssign<Position> for Span {
            fn add_assign(&mut self, rhs: Position) {
                self.end = rhs;
            }
        }
        impl Display for Span {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!(":{0}", self.start))
            }
        }
    }
    pub mod token {
        use super::span::Span;
        #[allow(non_camel_case_types)]
        pub enum TokenKind {
            LEFT_PAREN,
            RIGHT_PAREN,
            EQUAL,
            COMMA,
            LESS,
            LESS_SLASH,
            GREAT,
            SLASH_GREAT,
            DOT,
            SLASH,
            ESC_A,
            ESC_B,
            ESC_T,
            ESC_N,
            ESC_V,
            ESC_F,
            ESC_R,
            ESC_E,
            ESC_BACK_SLASH,
            ESC_LESS,
            ESC_GREAT,
            ESC_0,
            ESC_X,
            ESC_U,
            ANSI,
            ANSI_ESC,
            ESCAPED,
            IDENTIFIER,
            STRING,
            NUMBER,
            WHITESPACE,
            TEXT,
            HEX,
            BLACK,
            RED,
            GREEN,
            YELLOW,
            BLUE,
            MAGENTA,
            CYAN,
            WHITE,
            FIXED,
            RGB,
            A,
            B,
            BR,
            C,
            CODE,
            D,
            DIV,
            H,
            I,
            K,
            LET,
            O,
            P,
            PRE,
            R,
            S,
            SPAN,
            U,
            UU,
            X,
            ZIYY,
            CLASS,
            CURLY,
            BLOCK,
            DASHED,
            DOUBLE,
            DOTTED,
            ID,
            INDENT,
            HREF,
            N,
            NONE,
            SINGLE,
            COMMENT,
            EOF,
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::fmt::Debug for TokenKind {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        TokenKind::LEFT_PAREN => "LEFT_PAREN",
                        TokenKind::RIGHT_PAREN => "RIGHT_PAREN",
                        TokenKind::EQUAL => "EQUAL",
                        TokenKind::COMMA => "COMMA",
                        TokenKind::LESS => "LESS",
                        TokenKind::LESS_SLASH => "LESS_SLASH",
                        TokenKind::GREAT => "GREAT",
                        TokenKind::SLASH_GREAT => "SLASH_GREAT",
                        TokenKind::DOT => "DOT",
                        TokenKind::SLASH => "SLASH",
                        TokenKind::ESC_A => "ESC_A",
                        TokenKind::ESC_B => "ESC_B",
                        TokenKind::ESC_T => "ESC_T",
                        TokenKind::ESC_N => "ESC_N",
                        TokenKind::ESC_V => "ESC_V",
                        TokenKind::ESC_F => "ESC_F",
                        TokenKind::ESC_R => "ESC_R",
                        TokenKind::ESC_E => "ESC_E",
                        TokenKind::ESC_BACK_SLASH => "ESC_BACK_SLASH",
                        TokenKind::ESC_LESS => "ESC_LESS",
                        TokenKind::ESC_GREAT => "ESC_GREAT",
                        TokenKind::ESC_0 => "ESC_0",
                        TokenKind::ESC_X => "ESC_X",
                        TokenKind::ESC_U => "ESC_U",
                        TokenKind::ANSI => "ANSI",
                        TokenKind::ANSI_ESC => "ANSI_ESC",
                        TokenKind::ESCAPED => "ESCAPED",
                        TokenKind::IDENTIFIER => "IDENTIFIER",
                        TokenKind::STRING => "STRING",
                        TokenKind::NUMBER => "NUMBER",
                        TokenKind::WHITESPACE => "WHITESPACE",
                        TokenKind::TEXT => "TEXT",
                        TokenKind::HEX => "HEX",
                        TokenKind::BLACK => "BLACK",
                        TokenKind::RED => "RED",
                        TokenKind::GREEN => "GREEN",
                        TokenKind::YELLOW => "YELLOW",
                        TokenKind::BLUE => "BLUE",
                        TokenKind::MAGENTA => "MAGENTA",
                        TokenKind::CYAN => "CYAN",
                        TokenKind::WHITE => "WHITE",
                        TokenKind::FIXED => "FIXED",
                        TokenKind::RGB => "RGB",
                        TokenKind::A => "A",
                        TokenKind::B => "B",
                        TokenKind::BR => "BR",
                        TokenKind::C => "C",
                        TokenKind::CODE => "CODE",
                        TokenKind::D => "D",
                        TokenKind::DIV => "DIV",
                        TokenKind::H => "H",
                        TokenKind::I => "I",
                        TokenKind::K => "K",
                        TokenKind::LET => "LET",
                        TokenKind::O => "O",
                        TokenKind::P => "P",
                        TokenKind::PRE => "PRE",
                        TokenKind::R => "R",
                        TokenKind::S => "S",
                        TokenKind::SPAN => "SPAN",
                        TokenKind::U => "U",
                        TokenKind::UU => "UU",
                        TokenKind::X => "X",
                        TokenKind::ZIYY => "ZIYY",
                        TokenKind::CLASS => "CLASS",
                        TokenKind::CURLY => "CURLY",
                        TokenKind::BLOCK => "BLOCK",
                        TokenKind::DASHED => "DASHED",
                        TokenKind::DOUBLE => "DOUBLE",
                        TokenKind::DOTTED => "DOTTED",
                        TokenKind::ID => "ID",
                        TokenKind::INDENT => "INDENT",
                        TokenKind::HREF => "HREF",
                        TokenKind::N => "N",
                        TokenKind::NONE => "NONE",
                        TokenKind::SINGLE => "SINGLE",
                        TokenKind::COMMENT => "COMMENT",
                        TokenKind::EOF => "EOF",
                    },
                )
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for TokenKind {
            #[inline]
            fn clone(&self) -> TokenKind {
                *self
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::marker::Copy for TokenKind {}
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::marker::StructuralPartialEq for TokenKind {}
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::cmp::PartialEq for TokenKind {
            #[inline]
            fn eq(&self, other: &TokenKind) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        impl TokenKind {
            #[must_use]
            pub fn as_u8(&self) -> u8 {
                *self as u8
            }
        }
        pub struct Token<'src> {
            pub kind: TokenKind,
            pub content: &'src str,
            pub custom: u16,
            pub span: Span,
        }
        #[automatically_derived]
        impl<'src> ::core::fmt::Debug for Token<'src> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "Token",
                    "kind",
                    &self.kind,
                    "content",
                    &self.content,
                    "custom",
                    &self.custom,
                    "span",
                    &&self.span,
                )
            }
        }
        #[automatically_derived]
        impl<'src> ::core::clone::Clone for Token<'src> {
            #[inline]
            fn clone(&self) -> Token<'src> {
                Token {
                    kind: ::core::clone::Clone::clone(&self.kind),
                    content: ::core::clone::Clone::clone(&self.content),
                    custom: ::core::clone::Clone::clone(&self.custom),
                    span: ::core::clone::Clone::clone(&self.span),
                }
            }
        }
        impl<'src> Token<'src> {
            pub fn new(kind: TokenKind, content: &'src str, span: Span) -> Self {
                Token {
                    kind,
                    content,
                    span,
                    custom: 0,
                }
            }
        }
    }
    use position::Position;
    use span::Span;
    use crate::{
        scanner::token::{Token, TokenKind},
        ErrorKind, Result,
    };
    use core::str;
    fn is_alpha(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }
    fn is_valid(c: char) -> bool {
        c == ':' || c == '-' || c == '.'
    }
    fn is_digit(c: char) -> bool {
        c.is_ascii_digit()
    }
    fn is_hexdigit(c: char) -> bool {
        c.is_ascii_hexdigit()
    }
    fn is_octdigit(c: char) -> bool {
        match c {
            '0'..'8' => true,
            _ => false,
        }
    }
    pub fn is_whitespace(c: char) -> bool {
        c.is_ascii_whitespace()
    }
    /// A struct representing a scanner for tokenizing input strings.
    ///
    /// # Type Parameters
    ///
    /// * `T` - A type that can be referenced as a string slice.
    pub struct Scanner<'src> {
        pub(crate) source: &'src str,
        start: usize,
        current: usize,
        pub(crate) text_mode: bool,
        pub(crate) parse_colors: bool,
        pub start_pos: Position,
        pub current_pos: Position,
    }
    #[automatically_derived]
    impl<'src> ::core::clone::Clone for Scanner<'src> {
        #[inline]
        fn clone(&self) -> Scanner<'src> {
            Scanner {
                source: ::core::clone::Clone::clone(&self.source),
                start: ::core::clone::Clone::clone(&self.start),
                current: ::core::clone::Clone::clone(&self.current),
                text_mode: ::core::clone::Clone::clone(&self.text_mode),
                parse_colors: ::core::clone::Clone::clone(&self.parse_colors),
                start_pos: ::core::clone::Clone::clone(&self.start_pos),
                current_pos: ::core::clone::Clone::clone(&self.current_pos),
            }
        }
    }
    impl<'src> Scanner<'src> {
        /// Creates a new `Scanner` instance with the given source string.
        ///
        /// # Arguments
        ///
        /// * `source` - The source string to scan.
        ///
        /// # Returns
        ///
        /// * A new `Scanner` instance.
        pub fn new(source: &'src str) -> Self {
            Scanner {
                source,
                start: 0,
                current: 0,
                text_mode: true,
                parse_colors: false,
                start_pos: Position::default(),
                current_pos: Position::default(),
            }
        }
        /// Checks if the scanner has reached the end of the source string.
        ///
        /// # Returns
        ///
        /// * `true` if the scanner is at the end of the source string, `false` otherwise.
        pub fn is_at_end(&mut self) -> bool {
            self.current as usize + 1 > self.source.len()
        }
        /// Advances the scanner by one character and returns the character.
        ///
        /// # Returns
        ///
        /// * The character at the current position before advancing.
        pub fn advance(&mut self) -> char {
            self.current += 1;
            self.current_pos.col += 1;
            let ch = self.source.as_bytes()[self.current as usize - 1] as char;
            if ch == '\n' {
                self.current_pos.row += 1;
                self.current_pos.col = 0;
            }
            ch
        }
        pub fn advance_n(&mut self, n: usize) {
            self.current += n;
            self.current_pos.col += n as u16;
        }
        /// Peeks at the character at offset without advancing the scanner.
        ///
        /// # Returns
        ///
        /// * The character at the current position, or `'\0'` if at the end.
        pub fn peek(&mut self, offset: usize) -> char {
            if let Some(c) = self.source.as_bytes().get(self.current as usize + offset) {
                *c as char
            } else {
                '\0'
            }
        }
        /// Creates a token of the specified kind.
        ///
        /// # Arguments
        ///
        /// * `kind` - The kind of token to create.
        ///
        /// # Returns
        ///
        /// * A `Result` containing the created token.
        pub fn make_token(&mut self, kind: TokenKind) -> Result<'src, Token<'src>> {
            let s = &self.source[(self.start)..(self.current as usize)];
            let span = Span::new(self.start_pos, self.current_pos);
            if match kind {
                TokenKind::LESS | TokenKind::LESS_SLASH => true,
                _ => false,
            } {
                self.text_mode = false
            } else if match kind {
                TokenKind::GREAT | TokenKind::SLASH_GREAT => true,
                _ => false,
            } {
                self.text_mode = true
            }
            Ok(Token {
                kind,
                content: s,
                custom: 0,
                span,
            })
        }
        /// Creates an error token with the specified error code.
        ///
        /// # Arguments
        ///
        /// * `code` - The error code for the token.
        ///
        /// # Returns
        ///
        /// * A `Result` containing the created error token.
        pub fn error_token(&self, eof: bool) -> Result<'src, Token<'src>> {
            let s = &self.source[(self.start)..(self.current as usize)];
            let span = Span::new(self.start_pos, self.current_pos);
            let kind = if eof {
                ErrorKind::UnexpectedEof
            } else {
                ErrorKind::UnknownToken(s)
            };
            Err(crate::Error { kind, span })
        }
        /// Creates a text token.
        ///
        /// # Returns
        ///
        /// * A `Result` containing the created text token.
        pub fn text_token(&mut self, c: char) -> Result<'src, Token<'src>> {
            if is_whitespace(c) {
                return self.whitespace();
            }
            if c == '\\' {
                return self.escape();
            }
            while !self.is_at_end() {
                match self.peek(0) {
                    '<' | '\\' | '{' | '>' => break,
                    ws if is_whitespace(ws) => break,
                    _ => {
                        self.advance_n(1);
                    }
                }
            }
            self.make_token(TokenKind::TEXT)
        }
        /// Skips whitespace characters in the source string.
        pub fn skip_whitespace(&mut self) {
            loop {
                if self.text_mode {
                    return;
                }
                let c = self.peek(0);
                if is_whitespace(c) {
                    self.advance();
                    continue;
                }
                return;
            }
        }
        /// Checks if the current identifier matches a keyword.
        ///
        /// # Arguments
        ///
        /// * `start` - The starting position of the keyword.
        /// * `length` - The length of the keyword.
        /// * `rest` - The rest of the keyword string.
        /// * `kind` - The kind of token to return if the keyword matches.
        ///
        /// # Returns
        ///
        /// * The token kind if the keyword matches, `TokenKind::Identifier` otherwise.
        pub fn check_keyword(
            &mut self,
            start: usize,
            rest: &str,
            kind: TokenKind,
        ) -> TokenKind {
            let s = &self.source[(self.start + start)..self.current];
            if self.current - self.start == start + rest.len() && s == rest {
                kind
            } else {
                TokenKind::IDENTIFIER
            }
        }
        /// Determines the kind of identifier token.
        ///
        /// # Returns
        ///
        /// * The kind of identifier token.
        pub fn identifier_kind(&mut self) -> TokenKind {
            use token::TokenKind::*;
            match if self.current - self.start > 0 {
                self.source.as_bytes()[self.start + 0] as char
            } else {
                return IDENTIFIER;
            } {
                'a' => return self.check_keyword(1, "", A),
                'b' => {
                    match if self.current - self.start > 1 {
                        self.source.as_bytes()[self.start + 1] as char
                    } else {
                        return B;
                    } {
                        'g' => return self.check_keyword(2, "", X),
                        'l' => {
                            match if self.current - self.start > 2 {
                                self.source.as_bytes()[self.start + 2] as char
                            } else {
                                return IDENTIFIER;
                            } {
                                'a' => return self.check_keyword(3, "ck", BLACK),
                                'i' => return self.check_keyword(3, "nk", K),
                                'o' => return self.check_keyword(3, "ck", BLOCK),
                                'u' => return self.check_keyword(3, "e", BLUE),
                                _ => {}
                            }
                        }
                        'r' => return self.check_keyword(2, "", BR),
                        _ => {}
                    }
                }
                'c' => {
                    match if self.current - self.start > 1 {
                        self.source.as_bytes()[self.start + 1] as char
                    } else {
                        return C;
                    } {
                        'l' => return self.check_keyword(2, "ass", CLASS),
                        'u' => return self.check_keyword(2, "rly", CURLY),
                        'y' => return self.check_keyword(2, "an", CYAN),
                        _ => {}
                    }
                }
                'd' => {
                    match if self.current - self.start > 1 {
                        self.source.as_bytes()[self.start + 1] as char
                    } else {
                        return D;
                    } {
                        'a' => return self.check_keyword(2, "shed", CYAN),
                        'i' => {
                            match if self.current - self.start > 2 {
                                self.source.as_bytes()[self.start + 2] as char
                            } else {
                                return IDENTIFIER;
                            } {
                                'm' => return self.check_keyword(3, "", D),
                                'v' => return self.check_keyword(3, "", DIV),
                                _ => {}
                            }
                        }
                        'o' => {
                            match if self.current - self.start > 2 {
                                self.source.as_bytes()[self.start + 2] as char
                            } else {
                                return IDENTIFIER;
                            } {
                                't' => return self.check_keyword(3, "ted", DOTTED),
                                'u' => {
                                    match if self.current - self.start > 3 {
                                        self.source.as_bytes()[self.start + 3] as char
                                    } else {
                                        return IDENTIFIER;
                                    } {
                                        'b' => {
                                            match if self.current - self.start > 4 {
                                                self.source.as_bytes()[self.start + 4] as char
                                            } else {
                                                return IDENTIFIER;
                                            } {
                                                'l' => {
                                                    match if self.current - self.start > 5 {
                                                        self.source.as_bytes()[self.start + 5] as char
                                                    } else {
                                                        return IDENTIFIER;
                                                    } {
                                                        'e' => {
                                                            match if self.current - self.start > 6 {
                                                                self.source.as_bytes()[self.start + 6] as char
                                                            } else {
                                                                return DOUBLE;
                                                            } {
                                                                '-' => {
                                                                    match if self.current - self.start > 7 {
                                                                        self.source.as_bytes()[self.start + 7] as char
                                                                    } else {
                                                                        return IDENTIFIER;
                                                                    } {
                                                                        'u' => {
                                                                            match if self.current - self.start > 8 {
                                                                                self.source.as_bytes()[self.start + 8] as char
                                                                            } else {
                                                                                return IDENTIFIER;
                                                                            } {
                                                                                'n' => {
                                                                                    match if self.current - self.start > 9 {
                                                                                        self.source.as_bytes()[self.start + 9] as char
                                                                                    } else {
                                                                                        return IDENTIFIER;
                                                                                    } {
                                                                                        'd' => {
                                                                                            match if self.current - self.start > 10 {
                                                                                                self.source.as_bytes()[self.start + 10] as char
                                                                                            } else {
                                                                                                return IDENTIFIER;
                                                                                            } {
                                                                                                'e' => {
                                                                                                    match if self.current - self.start > 11 {
                                                                                                        self.source.as_bytes()[self.start + 11] as char
                                                                                                    } else {
                                                                                                        return IDENTIFIER;
                                                                                                    } {
                                                                                                        'r' => {
                                                                                                            match if self.current - self.start > 12 {
                                                                                                                self.source.as_bytes()[self.start + 12] as char
                                                                                                            } else {
                                                                                                                return U;
                                                                                                            } {
                                                                                                                'l' => return self.check_keyword(3, "ine", UU),
                                                                                                                _ => {}
                                                                                                            }
                                                                                                        }
                                                                                                        _ => {}
                                                                                                    }
                                                                                                }
                                                                                                _ => {}
                                                                                            }
                                                                                        }
                                                                                        _ => {}
                                                                                    }
                                                                                }
                                                                                _ => {}
                                                                            }
                                                                        }
                                                                        _ => {}
                                                                    }
                                                                }
                                                                _ => {}
                                                            }
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                'e' => return self.check_keyword(1, "m", I),
                'f' => {
                    match if self.current - self.start > 1 {
                        self.source.as_bytes()[self.start + 1] as char
                    } else {
                        return IDENTIFIER;
                    } {
                        'i' => return self.check_keyword(2, "xed", FIXED),
                        'g' => return self.check_keyword(2, "", C),
                        _ => {}
                    }
                }
                'g' => return self.check_keyword(1, "reen", GREEN),
                'h' => {
                    match if self.current - self.start > 1 {
                        self.source.as_bytes()[self.start + 1] as char
                    } else {
                        return H;
                    } {
                        'i' => {
                            match if self.current - self.start > 2 {
                                self.source.as_bytes()[self.start + 2] as char
                            } else {
                                return IDENTIFIER;
                            } {
                                'd' => {
                                    match if self.current - self.start > 3 {
                                        self.source.as_bytes()[self.start + 3] as char
                                    } else {
                                        return IDENTIFIER;
                                    } {
                                        'd' => return self.check_keyword(4, "en", H),
                                        'e' => return self.check_keyword(4, "", H),
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        'r' => return self.check_keyword(2, "ef", HREF),
                        _ => {}
                    }
                }
                'i' => {
                    match if self.current - self.start > 1 {
                        self.source.as_bytes()[self.start + 1] as char
                    } else {
                        return I;
                    } {
                        'd' => return self.check_keyword(2, "", ID),
                        'n' => {
                            match if self.current - self.start > 2 {
                                self.source.as_bytes()[self.start + 2] as char
                            } else {
                                return IDENTIFIER;
                            } {
                                'd' => return self.check_keyword(3, "ent", INDENT),
                                's' => return self.check_keyword(3, "", U),
                                'v' => {
                                    match if self.current - self.start > 3 {
                                        self.source.as_bytes()[self.start + 3] as char
                                    } else {
                                        return IDENTIFIER;
                                    } {
                                        'e' => return self.check_keyword(4, "rt", R),
                                        'i' => return self.check_keyword(4, "sible", H),
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        't' => return self.check_keyword(2, "alics", I),
                        _ => {}
                    }
                }
                'k' => return self.check_keyword(1, "", K),
                'l' => return self.check_keyword(1, "et", LET),
                'm' => return self.check_keyword(1, "agenta", MAGENTA),
                'n' => {
                    match if self.current - self.start > 1 {
                        self.source.as_bytes()[self.start + 1] as char
                    } else {
                        return N;
                    } {
                        'e' => return self.check_keyword(2, "gative", R),
                        'o' => return self.check_keyword(2, "ne", NONE),
                        _ => {}
                    }
                }
                'p' => {
                    match if self.current - self.start > 1 {
                        self.source.as_bytes()[self.start + 1] as char
                    } else {
                        return P;
                    } {
                        'r' => return self.check_keyword(2, "e", PRE),
                        _ => {}
                    }
                }
                'r' => {
                    match if self.current - self.start > 1 {
                        self.source.as_bytes()[self.start + 1] as char
                    } else {
                        return R;
                    } {
                        'e' => {
                            match if self.current - self.start > 2 {
                                self.source.as_bytes()[self.start + 2] as char
                            } else {
                                return IDENTIFIER;
                            } {
                                'd' => return self.check_keyword(3, "", RED),
                                'v' => return self.check_keyword(3, "erse", R),
                                _ => {}
                            }
                        }
                        'g' => return self.check_keyword(2, "b", RGB),
                        _ => {}
                    }
                }
                's' => {
                    match if self.current - self.start > 1 {
                        self.source.as_bytes()[self.start + 1] as char
                    } else {
                        return S;
                    } {
                        'i' => return self.check_keyword(2, "ngle", SINGLE),
                        'p' => return self.check_keyword(2, "an", SPAN),
                        't' => {
                            match if self.current - self.start > 2 {
                                self.source.as_bytes()[self.start + 2] as char
                            } else {
                                return IDENTIFIER;
                            } {
                                'r' => {
                                    match if self.current - self.start > 3 {
                                        self.source.as_bytes()[self.start + 3] as char
                                    } else {
                                        return IDENTIFIER;
                                    } {
                                        'i' => {
                                            match if self.current - self.start > 4 {
                                                self.source.as_bytes()[self.start + 4] as char
                                            } else {
                                                return IDENTIFIER;
                                            } {
                                                'k' => {
                                                    match if self.current - self.start > 5 {
                                                        self.source.as_bytes()[self.start + 5] as char
                                                    } else {
                                                        return IDENTIFIER;
                                                    } {
                                                        'e' => {
                                                            match if self.current - self.start > 6 {
                                                                self.source.as_bytes()[self.start + 6] as char
                                                            } else {
                                                                return S;
                                                            } {
                                                                '-' => return self.check_keyword(7, "through", S),
                                                                _ => {}
                                                            }
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                'u' => {
                    match if self.current - self.start > 1 {
                        self.source.as_bytes()[self.start + 1] as char
                    } else {
                        return U;
                    } {
                        'n' => {
                            match if self.current - self.start > 2 {
                                self.source.as_bytes()[self.start + 2] as char
                            } else {
                                return IDENTIFIER;
                            } {
                                'd' => {
                                    match if self.current - self.start > 3 {
                                        self.source.as_bytes()[self.start + 3] as char
                                    } else {
                                        return IDENTIFIER;
                                    } {
                                        'e' => {
                                            match if self.current - self.start > 4 {
                                                self.source.as_bytes()[self.start + 4] as char
                                            } else {
                                                return IDENTIFIER;
                                            } {
                                                'r' => {
                                                    match if self.current - self.start > 5 {
                                                        self.source.as_bytes()[self.start + 5] as char
                                                    } else {
                                                        return U;
                                                    } {
                                                        'l' => return self.check_keyword(6, "ine", U),
                                                        _ => {}
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        'u' => return self.check_keyword(2, "", UU),
                        _ => {}
                    }
                }
                'w' => return self.check_keyword(1, "hite", WHITE),
                'x' => return self.check_keyword(1, "", X),
                'y' => return self.check_keyword(1, "ellow", YELLOW),
                'z' => return self.check_keyword(1, "iyy", ZIYY),
                _ => {}
            }
            IDENTIFIER
        }
        /// Scans an identifier token.
        ///
        /// # Returns
        ///
        /// * A `Result` containing the scanned identifier token.
        pub fn identifier(&mut self) -> Result<'src, Token<'src>> {
            while is_alpha(self.peek(0)) || is_digit(self.peek(0))
                || is_valid(self.peek(0))
            {
                self.advance();
            }
            let kind = self.identifier_kind();
            self.make_token(kind)
        }
        /// Scans a hexadecimal token.
        ///
        /// # Returns
        ///
        /// * A `Result` containing the scanned hexadecimal token.
        pub fn hex(&mut self) -> Result<'src, Token<'src>> {
            while is_hexdigit(self.peek(0)) {
                self.advance();
            }
            self.make_token(TokenKind::HEX)
        }
        /// Scans a number token.
        ///
        /// # Returns
        ///
        /// * A `Result` containing the scanned number token.
        pub fn number(&mut self) -> Result<'src, Token<'src>> {
            while is_digit(self.peek(0)) {
                self.advance();
            }
            self.make_token(TokenKind::NUMBER)
        }
        /// Scans a string token.
        ///
        /// # Arguments
        ///
        /// * `ch` - The character used to delimit the string.
        ///
        /// # Returns
        ///
        /// * A `Result` containing the scanned string token.
        pub fn string(&mut self, ch: char) -> Result<'src, Token<'src>> {
            while self.peek(0) != ch && !self.is_at_end() {
                self.advance();
            }
            if self.is_at_end() {
                return self.error_token(true);
            }
            self.advance_n(1);
            self.make_token(TokenKind::STRING)
        }
        pub fn comment(&mut self) -> Result<'src, Token<'src>> {
            loop {
                if self.is_at_end() {
                    break;
                }
                if match self.peek(0) {
                    '-' => true,
                    _ => false,
                }
                    && match self.peek(1) {
                        '-' => true,
                        _ => false,
                    }
                    && match self.peek(2) {
                        '>' => true,
                        _ => false,
                    }
                {
                    break;
                }
                self.advance();
            }
            if self.is_at_end() {
                return self.make_token(TokenKind::COMMENT);
            }
            self.advance_n(3);
            self.make_token(TokenKind::COMMENT)
        }
        pub fn ansi_sgr(&mut self, esc: bool) -> Result<'src, Token<'src>> {
            if !match self.peek(0) {
                '\x30'..='\x39' | '\x3b' | '\x40'..='\x7e' => true,
                _ => false,
            } {
                let c = self.peek(0);
                return self.text_token(c);
            }
            while !self.is_at_end()
                && !match self.peek(0) {
                    '\x40'..='\x7e' => true,
                    _ => false,
                }
            {
                self.advance_n(1);
            }
            if self.peek(0) == 'm' {
                self.advance_n(1);
                self.make_token(if esc { TokenKind::ANSI_ESC } else { TokenKind::ANSI })
            } else {
                let c = self.peek(0);
                return self.text_token(c);
            }
        }
        /// Scans a whitespace token.
        ///
        /// # Returns
        ///
        /// * A `Result` containing the scanned whitespace token.
        pub fn whitespace(&mut self) -> Result<'src, Token<'src>> {
            while is_whitespace(self.peek(0)) {
                self.advance();
            }
            self.make_token(TokenKind::WHITESPACE)
        }
        /// Scans an escape sequence token.
        ///
        /// # Returns
        ///
        /// * A `Result` containing the scanned escape sequence token.
        pub fn escape(&mut self) -> Result<'src, Token<'src>> {
            let c = self.advance();
            let kind = match c {
                'a' => TokenKind::ESC_A,
                'b' => TokenKind::ESC_B,
                'e' => TokenKind::ESC_R,
                'f' => TokenKind::ESC_F,
                'n' => TokenKind::ESC_N,
                'r' => TokenKind::ESC_R,
                't' => TokenKind::ESC_T,
                'v' => TokenKind::ESC_V,
                '\\' => TokenKind::ESC_BACK_SLASH,
                '<' => TokenKind::ESC_LESS,
                '>' => TokenKind::ESC_GREAT,
                '0' => {
                    let mut i = 0;
                    while i < 3 && is_octdigit(self.peek(0)) {
                        self.advance();
                        i += 1;
                    }
                    TokenKind::ESC_0
                }
                'x' => {
                    if self.peek(0) == '1'
                        && match self.peek(1) {
                            'b' | 'B' => true,
                            _ => false,
                        } && self.peek(2) == '['
                    {
                        self.advance_n(3);
                        return self.ansi_sgr(true);
                    }
                    let mut i = 0;
                    while i < 2 && is_hexdigit(self.peek(0)) {
                        self.advance();
                        i += 1;
                    }
                    TokenKind::ESC_X
                }
                'u' => {
                    let mut i = 0;
                    while i < 4 && is_hexdigit(self.peek(0)) {
                        self.advance();
                        i += 1;
                    }
                    TokenKind::ESC_U
                }
                'U' => {
                    let mut i = 0;
                    while i < 8 && is_hexdigit(self.peek(0)) {
                        self.advance();
                        i += 1;
                    }
                    TokenKind::ESC_U
                }
                _ => {
                    return self.text_token(c);
                }
            };
            self.make_token(kind)
        }
        /// Scans the next token from the source string.
        ///
        /// # Returns
        ///
        /// * A `Result` containing the scanned token.
        pub fn scan_token(&mut self) -> Result<'src, Token<'src>> {
            self.skip_whitespace();
            self.start = self.current;
            self.start_pos = self.current_pos;
            if self.is_at_end() {
                return self.make_token(TokenKind::EOF);
            }
            let c = self.advance();
            if self.text_mode
                && !match c {
                    '<' | '>' => true,
                    _ => false,
                }
            {
                return self.text_token(c);
            }
            if is_alpha(c) {
                return self.identifier();
            }
            if is_digit(c) {
                return self.number();
            }
            match c {
                '(' => self.make_token(TokenKind::LEFT_PAREN),
                ')' => self.make_token(TokenKind::RIGHT_PAREN),
                ',' => self.make_token(TokenKind::COMMA),
                '.' => self.make_token(TokenKind::DOT),
                '=' => self.make_token(TokenKind::EQUAL),
                '"' => self.string('"'),
                '\'' => self.string('\''),
                '\x1b' if self.peek(0) == '[' => {
                    self.advance_n(1);
                    self.ansi_sgr(false)
                }
                '/' => {
                    match self.peek(0) {
                        '>' => {
                            self.advance();
                            self.text_mode = true;
                            self.make_token(TokenKind::SLASH_GREAT)
                        }
                        _ => self.make_token(TokenKind::SLASH),
                    }
                }
                '>' => self.make_token(TokenKind::GREAT),
                '<' => {
                    match self.peek(0) {
                        'e' => {
                            match self.peek(1) {
                                '>' => {
                                    self.advance_n(2);
                                    while !self.is_at_end() {
                                        if self.peek(0) == '<' && self.peek(1) == '/'
                                            && self.peek(2) == 'e' && self.peek(3) == '>'
                                        {
                                            self.advance_n(4);
                                            break;
                                        } else {
                                            self.advance_n(1);
                                        }
                                    }
                                    self.make_token(TokenKind::ESCAPED)
                                }
                                _ => self.make_token(TokenKind::LESS),
                            }
                        }
                        '/' => {
                            self.advance();
                            self.make_token(TokenKind::LESS_SLASH)
                        }
                        '!' => {
                            match self.peek(1) {
                                '-' => {
                                    match self.peek(2) {
                                        '-' => {
                                            self.advance_n(3);
                                            self.comment()
                                        }
                                        _ => self.make_token(TokenKind::LESS),
                                    }
                                }
                                _ => self.make_token(TokenKind::LESS),
                            }
                        }
                        _ => self.make_token(TokenKind::LESS),
                    }
                }
                _ => {
                    if self.parse_colors && c == '#' {
                        self.hex()
                    } else {
                        self.error_token(false)
                    }
                }
            }
        }
        pub fn scan_one(&mut self) -> Option<Token<'src>> {
            let token = self.scan_token().ok()?;
            let eof = self.scan_token().ok()?;
            if eof.kind == TokenKind::EOF { Some(token) } else { None }
        }
    }
}
mod style {
    #![allow(unused)]
    use convert::{FromU32, FromU8};
    pub use effect::*;
    use std::{
        fmt::{Debug, Display},
        ops::{Add, Not, Sub},
    };
    mod convert {
        pub trait FromU8: Sized {
            fn from_u8(value: u8) -> Self;
        }
        pub trait FromU32: Sized {
            fn from_u32(value: u32) -> Self;
        }
    }
    mod effect {
        use super::convert::FromU8;
        pub use blink::*;
        pub use color::*;
        pub use font::*;
        pub use frame::*;
        pub use intensity::*;
        pub use italics::*;
        use std::ops::{Add, Not, Sub};
        pub use switch::*;
        pub use underline::*;
        mod blink {
            use super::super::convert::FromU8;
            use std::ops::{Add, Not, Sub};
            pub enum Blink {
                #[default]
                None,
                Slow,
                Fast,
                Unset,
            }
            #[automatically_derived]
            impl ::core::default::Default for Blink {
                #[inline]
                fn default() -> Blink {
                    Self::None
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Blink {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            Blink::None => "None",
                            Blink::Slow => "Slow",
                            Blink::Fast => "Fast",
                            Blink::Unset => "Unset",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Blink {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Blink {
                #[inline]
                fn eq(&self, other: &Blink) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Blink {
                #[inline]
                fn clone(&self) -> Blink {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Blink {}
            impl Blink {
                pub fn as_str(&self) -> &str {
                    use Blink::*;
                    match self {
                        None => "",
                        Slow => "\x1b[5m",
                        Fast => "\x1b[6m",
                        Unset => "\x1b[25m",
                    }
                }
                pub fn as_bytes(&self) -> &[u8] {
                    self.as_str().as_bytes()
                }
            }
            impl FromU8 for Blink {
                fn from_u8(value: u8) -> Self {
                    use Blink::*;
                    match value {
                        0 => None,
                        1 => Slow,
                        2 => Fast,
                        3 => Unset,
                        _ => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
            }
            impl Add for Blink {
                type Output = Blink;
                fn add(self, rhs: Self) -> Self::Output {
                    use Blink::*;
                    match (self, rhs) {
                        (None, Unset) => None,
                        (None, rhs) => rhs,
                        (lhs, None) => lhs,
                        (_, rhs) => rhs,
                    }
                }
            }
            impl Sub for Blink {
                type Output = Blink;
                fn sub(self, rhs: Self) -> Self::Output {
                    use Blink::*;
                    match (self, rhs) {
                        (None, rhs) => !rhs,
                        (lhs, rhs) if lhs == rhs => None,
                        (lhs, _) => lhs,
                    }
                }
            }
            impl Not for Blink {
                type Output = Blink;
                fn not(self) -> Self::Output {
                    use Blink::*;
                    match self {
                        Slow => Unset,
                        Fast => Unset,
                        _ => None,
                    }
                }
            }
        }
        mod color {
            pub use ansi256::Ansi256;
            pub use ansi_color::AnsiColor;
            pub use rgb::Rgb;
            use crate::number;
            use crate::scanner::span::Span;
            use crate::scanner::token::{Token, TokenKind};
            use crate::scanner::Scanner;
            use crate::style::convert::FromU32;
            use crate::{Error, ErrorKind, Result};
            use std::ops::{Add, Not, Sub};
            mod ansi256 {
                use super::ColorKind;
                pub struct Ansi256(pub u8);
                #[automatically_derived]
                impl ::core::default::Default for Ansi256 {
                    #[inline]
                    fn default() -> Ansi256 {
                        Ansi256(::core::default::Default::default())
                    }
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for Ansi256 {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Ansi256",
                            &&self.0,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for Ansi256 {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for Ansi256 {
                    #[inline]
                    fn eq(&self, other: &Ansi256) -> bool {
                        self.0 == other.0
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for Ansi256 {
                    #[inline]
                    fn clone(&self) -> Ansi256 {
                        let _: ::core::clone::AssertParamIsClone<u8>;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for Ansi256 {}
                impl Ansi256 {
                    pub fn to_string(&self, kind: ColorKind) -> String {
                        let Ansi256(n) = self;
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("\u{1b}[{0};5;{1}m", kind as u8 + 8, n),
                            )
                        })
                    }
                }
            }
            mod ansi_color {
                use super::ColorKind;
                pub enum AnsiColor {
                    Black,
                    Red,
                    Green,
                    Yellow,
                    Blue,
                    Magenta,
                    Cyan,
                    White,
                    #[default]
                    Default = 9,
                    BrightBlack = 60,
                    BrightRed,
                    BrightGreen,
                    BrightYellow,
                    BrightBlue,
                    BrightMagenta,
                    BrightCyan,
                    BrightWhite,
                }
                #[automatically_derived]
                impl ::core::default::Default for AnsiColor {
                    #[inline]
                    fn default() -> AnsiColor {
                        Self::Default
                    }
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for AnsiColor {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(
                            f,
                            match self {
                                AnsiColor::Black => "Black",
                                AnsiColor::Red => "Red",
                                AnsiColor::Green => "Green",
                                AnsiColor::Yellow => "Yellow",
                                AnsiColor::Blue => "Blue",
                                AnsiColor::Magenta => "Magenta",
                                AnsiColor::Cyan => "Cyan",
                                AnsiColor::White => "White",
                                AnsiColor::Default => "Default",
                                AnsiColor::BrightBlack => "BrightBlack",
                                AnsiColor::BrightRed => "BrightRed",
                                AnsiColor::BrightGreen => "BrightGreen",
                                AnsiColor::BrightYellow => "BrightYellow",
                                AnsiColor::BrightBlue => "BrightBlue",
                                AnsiColor::BrightMagenta => "BrightMagenta",
                                AnsiColor::BrightCyan => "BrightCyan",
                                AnsiColor::BrightWhite => "BrightWhite",
                            },
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for AnsiColor {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for AnsiColor {
                    #[inline]
                    fn eq(&self, other: &AnsiColor) -> bool {
                        let __self_discr = ::core::intrinsics::discriminant_value(self);
                        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                        __self_discr == __arg1_discr
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for AnsiColor {
                    #[inline]
                    fn clone(&self) -> AnsiColor {
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for AnsiColor {}
                impl AnsiColor {
                    pub fn to_string(&self, kind: ColorKind) -> String {
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("\u{1b}[{0}m", kind as u8 + *self as u8),
                            )
                        })
                    }
                }
                impl TryFrom<u8> for AnsiColor {
                    type Error = u8;
                    fn try_from(value: u8) -> Result<Self, Self::Error> {
                        use AnsiColor::*;
                        match value {
                            0 => Ok(Black),
                            1 => Ok(Red),
                            2 => Ok(Green),
                            3 => Ok(Yellow),
                            4 => Ok(Blue),
                            5 => Ok(Magenta),
                            6 => Ok(Cyan),
                            7 => Ok(White),
                            9 => Ok(Default),
                            60 => Ok(BrightBlack),
                            61 => Ok(BrightRed),
                            62 => Ok(BrightGreen),
                            63 => Ok(BrightYellow),
                            64 => Ok(BrightBlue),
                            65 => Ok(BrightMagenta),
                            66 => Ok(BrightCyan),
                            67 => Ok(BrightWhite),
                            n => Err(n),
                        }
                    }
                }
            }
            mod rgb {
                use crate::number;
                use crate::scanner::token::TokenKind;
                use crate::scanner::Scanner;
                use crate::Error;
                use crate::ErrorKind;
                use crate::Result;
                use super::expect;
                use super::ColorKind;
                pub struct Rgb(pub u8, pub u8, pub u8);
                #[automatically_derived]
                impl ::core::default::Default for Rgb {
                    #[inline]
                    fn default() -> Rgb {
                        Rgb(
                            ::core::default::Default::default(),
                            ::core::default::Default::default(),
                            ::core::default::Default::default(),
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for Rgb {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_tuple_field3_finish(
                            f,
                            "Rgb",
                            &self.0,
                            &self.1,
                            &&self.2,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for Rgb {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for Rgb {
                    #[inline]
                    fn eq(&self, other: &Rgb) -> bool {
                        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for Rgb {
                    #[inline]
                    fn clone(&self) -> Rgb {
                        let _: ::core::clone::AssertParamIsClone<u8>;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for Rgb {}
                impl Rgb {
                    pub fn to_string(&self, kind: ColorKind) -> String {
                        let Rgb(r, g, b) = self;
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "\u{1b}[{0};2;{1};{2};{3}m",
                                    kind as u8 + 8,
                                    r,
                                    g,
                                    b,
                                ),
                            )
                        })
                    }
                    pub fn parse<'src>(
                        scanner: &mut Scanner<'src>,
                    ) -> Result<'src, Self> {
                        let token = scanner.scan_token()?;
                        let mut r = 0;
                        let mut g = 0;
                        let mut b = 0;
                        match token.kind {
                            TokenKind::NUMBER => {
                                r = match (&token).kind {
                                    TokenKind::NUMBER => {
                                        crate::str_to_u32(token.content, 10)
                                            .map_err(|k| Error::<'src>::new(k, &token))? as u8
                                    }
                                    _ => {
                                        return Err(
                                            Error::new(
                                                ErrorKind::UnexpectedToken {
                                                    expected: TokenKind::NUMBER,
                                                    found: Some((&token).content),
                                                },
                                                &token,
                                            ),
                                        );
                                    }
                                };
                                let token = scanner.scan_token()?;
                                expect(&token, TokenKind::COMMA)?;
                                let token = scanner.scan_token()?;
                                g = match (&token).kind {
                                    TokenKind::NUMBER => {
                                        crate::str_to_u32(token.content, 10)
                                            .map_err(|k| Error::<'src>::new(k, &token))? as u8
                                    }
                                    _ => {
                                        return Err(
                                            Error::new(
                                                ErrorKind::UnexpectedToken {
                                                    expected: TokenKind::NUMBER,
                                                    found: Some((&token).content),
                                                },
                                                &token,
                                            ),
                                        );
                                    }
                                };
                                let token = scanner.scan_token()?;
                                expect(&token, TokenKind::COMMA)?;
                                let token = scanner.scan_token()?;
                                b = match (&token).kind {
                                    TokenKind::NUMBER => {
                                        crate::str_to_u32(token.content, 10)
                                            .map_err(|k| Error::<'src>::new(k, &token))? as u8
                                    }
                                    _ => {
                                        return Err(
                                            Error::new(
                                                ErrorKind::UnexpectedToken {
                                                    expected: TokenKind::NUMBER,
                                                    found: Some((&token).content),
                                                },
                                                &token,
                                            ),
                                        );
                                    }
                                };
                            }
                            TokenKind::HEX => {
                                match token.content.len() {
                                    4 => {}
                                    7 => {
                                        r = match (&token).kind {
                                            TokenKind::NUMBER => {
                                                crate::str_to_u32(&token.content[1..3], 16)
                                                    .map_err(|k| Error::<'src>::new(k, &token))? as u8
                                            }
                                            _ => {
                                                return Err(
                                                    Error::new(
                                                        ErrorKind::UnexpectedToken {
                                                            expected: TokenKind::NUMBER,
                                                            found: Some((&token).content),
                                                        },
                                                        &token,
                                                    ),
                                                );
                                            }
                                        };
                                        g = match (&token).kind {
                                            TokenKind::NUMBER => {
                                                crate::str_to_u32(&token.content[3..5], 16)
                                                    .map_err(|k| Error::<'src>::new(k, &token))? as u8
                                            }
                                            _ => {
                                                return Err(
                                                    Error::new(
                                                        ErrorKind::UnexpectedToken {
                                                            expected: TokenKind::NUMBER,
                                                            found: Some((&token).content),
                                                        },
                                                        &token,
                                                    ),
                                                );
                                            }
                                        };
                                        b = match (&token).kind {
                                            TokenKind::NUMBER => {
                                                crate::str_to_u32(&token.content[5..7], 16)
                                                    .map_err(|k| Error::<'src>::new(k, &token))? as u8
                                            }
                                            _ => {
                                                return Err(
                                                    Error::new(
                                                        ErrorKind::UnexpectedToken {
                                                            expected: TokenKind::NUMBER,
                                                            found: Some((&token).content),
                                                        },
                                                        &token,
                                                    ),
                                                );
                                            }
                                        };
                                    }
                                    _ => {}
                                }
                            }
                            _ => {
                                return Err(
                                    Error::new(
                                        ErrorKind::UnexpectedToken {
                                            expected: TokenKind::NUMBER,
                                            found: Some(token.content),
                                        },
                                        &token,
                                    ),
                                );
                            }
                        }
                        Ok(Rgb(r, g, b))
                    }
                }
            }
            pub enum Color {
                #[default]
                None,
                Rgb(Rgb),
                Ansi256(Ansi256),
                AnsiColor(AnsiColor),
                Unset,
            }
            #[automatically_derived]
            impl ::core::default::Default for Color {
                #[inline]
                fn default() -> Color {
                    Self::None
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Color {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        Color::None => ::core::fmt::Formatter::write_str(f, "None"),
                        Color::Rgb(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Rgb",
                                &__self_0,
                            )
                        }
                        Color::Ansi256(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Ansi256",
                                &__self_0,
                            )
                        }
                        Color::AnsiColor(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "AnsiColor",
                                &__self_0,
                            )
                        }
                        Color::Unset => ::core::fmt::Formatter::write_str(f, "Unset"),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Color {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Color {
                #[inline]
                fn eq(&self, other: &Color) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                        && match (self, other) {
                            (Color::Rgb(__self_0), Color::Rgb(__arg1_0)) => {
                                __self_0 == __arg1_0
                            }
                            (Color::Ansi256(__self_0), Color::Ansi256(__arg1_0)) => {
                                __self_0 == __arg1_0
                            }
                            (Color::AnsiColor(__self_0), Color::AnsiColor(__arg1_0)) => {
                                __self_0 == __arg1_0
                            }
                            _ => true,
                        }
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Color {
                #[inline]
                fn clone(&self) -> Color {
                    let _: ::core::clone::AssertParamIsClone<Rgb>;
                    let _: ::core::clone::AssertParamIsClone<Ansi256>;
                    let _: ::core::clone::AssertParamIsClone<AnsiColor>;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Color {}
            pub enum ColorKind {
                Foreground = 30,
                Background = 40,
                Underline = 50,
            }
            impl Color {
                const UNSET: Color = Color::AnsiColor(AnsiColor::Default);
                pub fn to_string(&self, kind: ColorKind) -> String {
                    match self {
                        Color::Rgb(rgb) => rgb.to_string(kind),
                        Color::Ansi256(ansi256) => ansi256.to_string(kind),
                        Color::AnsiColor(ansi_color) => ansi_color.to_string(kind),
                        Color::Unset => {
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("\u{1b}[{0}m", kind as u8 + 9),
                                )
                            })
                        }
                        Color::None => String::new(),
                    }
                }
                pub fn to_vec(&self, kind: ColorKind) -> Vec<u8> {
                    self.to_string(kind).into_bytes()
                }
                pub fn parse<'src>(source: &'src str, span: Span) -> Result<'src, Self> {
                    let mut scanner = Scanner::new(source);
                    scanner.text_mode = false;
                    scanner.parse_colors = true;
                    scanner.current_pos = span.start;
                    let token = scanner.scan_token()?;
                    let color = match token.kind {
                        TokenKind::BLACK => Color::AnsiColor(AnsiColor::Black),
                        TokenKind::RED => Color::AnsiColor(AnsiColor::Red),
                        TokenKind::GREEN => Color::AnsiColor(AnsiColor::Green),
                        TokenKind::YELLOW => Color::AnsiColor(AnsiColor::Yellow),
                        TokenKind::BLUE => Color::AnsiColor(AnsiColor::Blue),
                        TokenKind::MAGENTA => Color::AnsiColor(AnsiColor::Magenta),
                        TokenKind::CYAN => Color::AnsiColor(AnsiColor::Cyan),
                        TokenKind::WHITE => Color::AnsiColor(AnsiColor::White),
                        TokenKind::FIXED => {
                            let token = scanner.scan_token()?;
                            expect(&token, TokenKind::LEFT_PAREN)?;
                            let token = scanner.scan_token()?;
                            expect(&token, TokenKind::NUMBER)?;
                            let n = match (&token).kind {
                                TokenKind::NUMBER => {
                                    crate::str_to_u32(token.content, 10)
                                        .map_err(|k| Error::<'src>::new(k, &token))? as u8
                                }
                                _ => {
                                    return Err(
                                        Error::new(
                                            ErrorKind::UnexpectedToken {
                                                expected: TokenKind::NUMBER,
                                                found: Some((&token).content),
                                            },
                                            &token,
                                        ),
                                    );
                                }
                            };
                            let token = scanner.scan_token()?;
                            expect(&token, TokenKind::RIGHT_PAREN)?;
                            Color::Ansi256(Ansi256(n))
                        }
                        TokenKind::RGB => {
                            let token = scanner.scan_token()?;
                            expect(&token, TokenKind::LEFT_PAREN)?;
                            let rgb = Color::Rgb(Rgb::parse(&mut scanner)?);
                            let token = scanner.scan_token()?;
                            expect(&token, TokenKind::RIGHT_PAREN)?;
                            rgb
                        }
                        TokenKind::NONE => Color::Unset,
                        _ => {
                            return Err(Error {
                                kind: ErrorKind::InvalidColor(source),
                                span: span,
                            });
                        }
                    };
                    Ok(color)
                }
            }
            fn expect<'src>(token: &Token<'src>, tt: TokenKind) -> Result<'src, ()> {
                if token.kind != tt {
                    return Err(
                        Error::new(
                            ErrorKind::UnexpectedToken {
                                expected: tt,
                                found: Some(token.content),
                            },
                            token,
                        ),
                    );
                }
                Ok(())
            }
            impl FromU32 for Color {
                fn from_u32(n: u32) -> Self {
                    if n == 0 {
                        Color::None
                    } else if n & 0b01 >= 1 {
                        Color::Rgb(
                            Rgb(
                                ((n >> 1) & 0xFF) as u8,
                                ((n >> 9) & 0xFF) as u8,
                                ((n >> 17) & 0xFF) as u8,
                            ),
                        )
                    } else if n & 0b10 >= 1 {
                        Color::Ansi256(Ansi256((n >> 2) as u8))
                    } else {
                        let x = ((n >> 2) - 1) as u8;
                        if x == 9 {
                            Color::Unset
                        } else {
                            Color::AnsiColor(
                                match AnsiColor::try_from(x) {
                                    Ok(x) => x,
                                    Err(_) => {
                                        ::core::panicking::panic(
                                            "internal error: entered unreachable code",
                                        )
                                    }
                                },
                            )
                        }
                    }
                }
            }
            impl Into<u32> for Color {
                fn into(self) -> u32 {
                    match self {
                        Color::None => 0,
                        Color::Rgb(Rgb(r, g, b)) => {
                            0b1 | ((r as u32) << 1) | ((g as u32) << 9)
                                | ((b as u32) << 17)
                        }
                        Color::Ansi256(Ansi256(n)) => 0b10 | (n as u32) << 2,
                        Color::AnsiColor(n) => ((n as u32) + 1) << 2,
                        Color::Unset => (9 + 1) << 2,
                    }
                }
            }
            impl Add for Color {
                type Output = Color;
                fn add(self, rhs: Self) -> Self::Output {
                    use Color::*;
                    match (self, rhs) {
                        (None, Color::UNSET | Unset) => None,
                        (None, rhs) => rhs,
                        (lhs, None) => lhs,
                        (_, rhs) => rhs,
                    }
                }
            }
            impl Sub for Color {
                type Output = Color;
                fn sub(self, rhs: Self) -> Self::Output {
                    use Color::*;
                    match (self, rhs) {
                        (None, rhs) => None,
                        (lhs, rhs) if lhs == rhs => None,
                        (lhs, _) => lhs,
                    }
                }
            }
            impl Not for Color {
                type Output = Color;
                fn not(self) -> Self::Output {
                    use Color::*;
                    match self {
                        None => None,
                        Color::UNSET | Unset => None,
                        _ => Unset,
                    }
                }
            }
        }
        mod font {
            use crate::style::convert::FromU8;
            use std::ops::{Add, Not, Sub};
            pub enum Font {
                #[default]
                None,
                Primary,
                FirstAlt,
                SecondAlt,
                ThirdAlt,
                FourthAlt,
                FifthAlt,
                SixthAlt,
                SeventhAlt,
                EighthAlt,
                NinthAlt,
            }
            #[automatically_derived]
            impl ::core::default::Default for Font {
                #[inline]
                fn default() -> Font {
                    Self::None
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Font {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            Font::None => "None",
                            Font::Primary => "Primary",
                            Font::FirstAlt => "FirstAlt",
                            Font::SecondAlt => "SecondAlt",
                            Font::ThirdAlt => "ThirdAlt",
                            Font::FourthAlt => "FourthAlt",
                            Font::FifthAlt => "FifthAlt",
                            Font::SixthAlt => "SixthAlt",
                            Font::SeventhAlt => "SeventhAlt",
                            Font::EighthAlt => "EighthAlt",
                            Font::NinthAlt => "NinthAlt",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Font {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Font {
                #[inline]
                fn eq(&self, other: &Font) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Font {
                #[inline]
                fn clone(&self) -> Font {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Font {}
            impl Font {
                pub fn as_str(&self) -> &str {
                    use Font::*;
                    match self {
                        None => "",
                        Primary => "\x1b[10m",
                        FirstAlt => "\x1b[11m",
                        SecondAlt => "\x1b[12m",
                        ThirdAlt => "\x1b[13m",
                        FourthAlt => "\x1b[14m",
                        FifthAlt => "\x1b[15m",
                        SixthAlt => "\x1b[16m",
                        SeventhAlt => "\x1b[17m",
                        EighthAlt => "\x1b[18m",
                        NinthAlt => "\x1b[19m",
                    }
                }
                pub fn as_bytes(&self) -> &[u8] {
                    self.as_str().as_bytes()
                }
            }
            impl FromU8 for Font {
                fn from_u8(value: u8) -> Self {
                    use Font::*;
                    match value {
                        0 => None,
                        1 => Primary,
                        2 => FirstAlt,
                        3 => SecondAlt,
                        4 => ThirdAlt,
                        5 => FourthAlt,
                        6 => FifthAlt,
                        7 => SixthAlt,
                        8 => SeventhAlt,
                        9 => EighthAlt,
                        10 => NinthAlt,
                        _ => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
            }
            impl Add for Font {
                type Output = Font;
                fn add(self, rhs: Self) -> Self::Output {
                    use Font::*;
                    match (self, rhs) {
                        (None, Primary) => None,
                        (None, rhs) => rhs,
                        (lhs, None) => lhs,
                        (_, rhs) => rhs,
                    }
                }
            }
            impl Sub for Font {
                type Output = Font;
                fn sub(self, rhs: Self) -> Self::Output {
                    use Font::*;
                    match (self, rhs) {
                        (None, rhs) => !rhs,
                        (lhs, rhs) if lhs == rhs => None,
                        (lhs, _) => lhs,
                    }
                }
            }
            impl Not for Font {
                type Output = Font;
                fn not(self) -> Self::Output {
                    use Font::*;
                    match self {
                        None | Primary => None,
                        _ => Primary,
                    }
                }
            }
        }
        mod frame {
            use super::super::convert::FromU8;
            use std::ops::{Add, Not, Sub};
            pub enum Frame {
                #[default]
                None,
                Framed,
                Encircled,
                Unset,
            }
            #[automatically_derived]
            impl ::core::default::Default for Frame {
                #[inline]
                fn default() -> Frame {
                    Self::None
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Frame {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            Frame::None => "None",
                            Frame::Framed => "Framed",
                            Frame::Encircled => "Encircled",
                            Frame::Unset => "Unset",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Frame {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Frame {
                #[inline]
                fn eq(&self, other: &Frame) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Frame {
                #[inline]
                fn clone(&self) -> Frame {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Frame {}
            impl Frame {
                pub fn as_str(&self) -> &str {
                    use Frame::*;
                    match self {
                        None => "",
                        Framed => "\x1b[51m",
                        Encircled => "\x1b[52m",
                        Unset => "\x1b[54m",
                    }
                }
                pub fn as_bytes(&self) -> &[u8] {
                    self.as_str().as_bytes()
                }
            }
            impl FromU8 for Frame {
                fn from_u8(value: u8) -> Self {
                    use Frame::*;
                    match value {
                        0 => None,
                        1 => Framed,
                        2 => Encircled,
                        3 => Unset,
                        _ => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
            }
            impl Add for Frame {
                type Output = Frame;
                fn add(self, rhs: Self) -> Self::Output {
                    use Frame::*;
                    match (self, rhs) {
                        (None, Unset) => None,
                        (None, rhs) => rhs,
                        (lhs, None) => lhs,
                        (_, rhs) => rhs,
                    }
                }
            }
            impl Sub for Frame {
                type Output = Frame;
                fn sub(self, rhs: Self) -> Self::Output {
                    use Frame::*;
                    match (self, rhs) {
                        (None, rhs) => !rhs,
                        (lhs, rhs) if lhs == rhs => None,
                        (lhs, _) => lhs,
                    }
                }
            }
            impl Not for Frame {
                type Output = Frame;
                fn not(self) -> Self::Output {
                    use Frame::*;
                    match self {
                        Framed => Unset,
                        Encircled => Unset,
                        _ => None,
                    }
                }
            }
        }
        mod intensity {
            use super::super::convert::FromU8;
            use std::ops::{Add, Not, Sub};
            #[repr(u8)]
            pub enum Intensity {
                #[default]
                None,
                Bold,
                Dim,
                NoBold,
                NoDim,
                Unset,
            }
            #[automatically_derived]
            impl ::core::default::Default for Intensity {
                #[inline]
                fn default() -> Intensity {
                    Self::None
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Intensity {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            Intensity::None => "None",
                            Intensity::Bold => "Bold",
                            Intensity::Dim => "Dim",
                            Intensity::NoBold => "NoBold",
                            Intensity::NoDim => "NoDim",
                            Intensity::Unset => "Unset",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Intensity {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Intensity {
                #[inline]
                fn eq(&self, other: &Intensity) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Intensity {
                #[inline]
                fn clone(&self) -> Intensity {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Intensity {}
            impl Intensity {
                pub(in crate::style) fn _as_str(&self, prev: Intensity) -> &str {
                    use Intensity::*;
                    match (prev, self) {
                        (Bold, Dim) => "\x1b[22;2m",
                        (Dim, Bold) => "\x1b[22;1m",
                        (_, None) => "",
                        (_, Bold) => "\x1b[1m",
                        (_, Dim) => "\x1b[2m",
                        (_, Unset) => "\x1b[22m",
                        (_, NoBold) => "\x1b[22m",
                        (_, NoDim) => "\x1b[22m",
                    }
                }
                pub(in crate::style) fn _as_bytes(&self, prev: Intensity) -> &[u8] {
                    self._as_str(prev).as_bytes()
                }
                pub fn as_str(&self) -> &str {
                    self._as_str(Intensity::None)
                }
                pub fn as_bytes(&self) -> &[u8] {
                    self.as_str().as_bytes()
                }
            }
            impl FromU8 for Intensity {
                fn from_u8(value: u8) -> Self {
                    use Intensity::*;
                    match value {
                        0 => None,
                        1 => Bold,
                        2 => Dim,
                        3 => NoBold,
                        4 => NoDim,
                        5 => Unset,
                        _ => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
            }
            impl Add for Intensity {
                type Output = Intensity;
                fn add(self, rhs: Self) -> Self::Output {
                    use Intensity::*;
                    match (self, rhs) {
                        (None, Unset | NoBold | NoDim) => None,
                        (None, rhs) => rhs,
                        (lhs, None) => lhs,
                        (Bold, Dim | NoBold) => rhs,
                        (Bold, Bold | NoDim) => Bold,
                        (Dim, Bold | NoDim) => rhs,
                        (Dim, Dim | NoBold) => Dim,
                        (NoBold, Bold | Dim) => rhs,
                        (NoBold, _) => NoBold,
                        (NoDim, Bold | Dim) => rhs,
                        (NoDim, _) => NoDim,
                        (Unset, Bold | Dim) => rhs,
                        (Unset, NoBold | NoDim) => Unset,
                        (_, Unset) => Unset,
                    }
                }
            }
            impl Sub for Intensity {
                type Output = Intensity;
                fn sub(self, rhs: Self) -> Self::Output {
                    use Intensity::*;
                    match (self, rhs) {
                        (None, rhs) => !rhs,
                        (lhs, rhs) if lhs == rhs => None,
                        (lhs, _) => lhs,
                    }
                }
            }
            impl Not for Intensity {
                type Output = Intensity;
                fn not(self) -> Self::Output {
                    use Intensity::*;
                    match self {
                        Bold => NoBold,
                        Dim => NoDim,
                        _ => None,
                    }
                }
            }
        }
        mod italics {
            use super::super::convert::FromU8;
            use std::ops::{Add, Not, Sub};
            pub enum FontStyle {
                #[default]
                None,
                Italics,
                Fraktur,
                Unset,
            }
            #[automatically_derived]
            impl ::core::default::Default for FontStyle {
                #[inline]
                fn default() -> FontStyle {
                    Self::None
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for FontStyle {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            FontStyle::None => "None",
                            FontStyle::Italics => "Italics",
                            FontStyle::Fraktur => "Fraktur",
                            FontStyle::Unset => "Unset",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for FontStyle {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for FontStyle {
                #[inline]
                fn eq(&self, other: &FontStyle) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for FontStyle {
                #[inline]
                fn clone(&self) -> FontStyle {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for FontStyle {}
            impl FontStyle {
                pub fn as_str(&self) -> &str {
                    use FontStyle::*;
                    match self {
                        None => "",
                        Italics => "\x1b[3m",
                        Fraktur => "\x1b[20m",
                        Unset => "\x1b[23m",
                    }
                }
                pub fn as_bytes(&self) -> &[u8] {
                    self.as_str().as_bytes()
                }
            }
            impl FromU8 for FontStyle {
                fn from_u8(value: u8) -> Self {
                    use FontStyle::*;
                    match value {
                        0 => None,
                        1 => Italics,
                        2 => Fraktur,
                        3 => Unset,
                        _ => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
            }
            impl Add for FontStyle {
                type Output = FontStyle;
                fn add(self, rhs: Self) -> Self::Output {
                    use FontStyle::*;
                    match (self, rhs) {
                        (None, Unset) => None,
                        (None, rhs) => rhs,
                        (lhs, None) => lhs,
                        (_, rhs) => rhs,
                    }
                }
            }
            impl Sub for FontStyle {
                type Output = FontStyle;
                fn sub(self, rhs: Self) -> Self::Output {
                    use FontStyle::*;
                    match (self, rhs) {
                        (None, rhs) => !rhs,
                        (lhs, rhs) if lhs == rhs => None,
                        (lhs, _) => lhs,
                    }
                }
            }
            impl Not for FontStyle {
                type Output = FontStyle;
                fn not(self) -> Self::Output {
                    use FontStyle::*;
                    match self {
                        Italics => Unset,
                        Fraktur => Unset,
                        _ => None,
                    }
                }
            }
        }
        mod switch {
            use super::super::convert::FromU8;
            use std::ops::{Add, Not, Sub};
            pub enum Reset {
                #[default]
                No,
                Yes,
            }
            #[automatically_derived]
            impl ::core::default::Default for Reset {
                #[inline]
                fn default() -> Reset {
                    Self::No
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Reset {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            Reset::No => "No",
                            Reset::Yes => "Yes",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Reset {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Reset {
                #[inline]
                fn eq(&self, other: &Reset) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Reset {
                #[inline]
                fn clone(&self) -> Reset {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Reset {}
            impl Reset {
                pub fn as_str(&self) -> &str {
                    use Reset::*;
                    match self {
                        No => "",
                        Yes => "\x1b[0m",
                    }
                }
                pub fn as_bytes(&self) -> &[u8] {
                    self.as_str().as_bytes()
                }
                pub fn is_set(&self) -> bool {
                    use Reset::*;
                    match self {
                        No => false,
                        Yes => true,
                    }
                }
                pub fn is_unset(&self) -> bool {
                    !self.is_set()
                }
            }
            impl FromU8 for Reset {
                fn from_u8(value: u8) -> Self {
                    use Reset::*;
                    match value {
                        0 => No,
                        1 => Yes,
                        _ => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
            }
            impl Add for Reset {
                type Output = Reset;
                fn add(self, rhs: Self) -> Self::Output {
                    use Reset::*;
                    match (self, rhs) {
                        (No, No) => No,
                        (_, _) => Yes,
                    }
                }
            }
            impl Sub for Reset {
                type Output = Reset;
                fn sub(self, rhs: Self) -> Self::Output {
                    use Reset::*;
                    match (self, rhs) {
                        (Yes, No) => Yes,
                        (_, _) => No,
                    }
                }
            }
            impl Not for Reset {
                type Output = Reset;
                fn not(self) -> Self::Output {
                    use Reset::*;
                    match self {
                        No => No,
                        Yes => No,
                    }
                }
            }
            pub enum Reserved1 {
                #[default]
                No,
                Yes,
            }
            #[automatically_derived]
            impl ::core::default::Default for Reserved1 {
                #[inline]
                fn default() -> Reserved1 {
                    Self::No
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Reserved1 {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            Reserved1::No => "No",
                            Reserved1::Yes => "Yes",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Reserved1 {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Reserved1 {
                #[inline]
                fn eq(&self, other: &Reserved1) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Reserved1 {
                #[inline]
                fn clone(&self) -> Reserved1 {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Reserved1 {}
            impl Reserved1 {
                pub fn as_str(&self) -> &str {
                    use Reserved1::*;
                    match self {
                        No => "",
                        Yes => "\x1b[56m",
                    }
                }
                pub fn as_bytes(&self) -> &[u8] {
                    self.as_str().as_bytes()
                }
                pub fn is_set(&self) -> bool {
                    use Reserved1::*;
                    match self {
                        No => false,
                        Yes => true,
                    }
                }
                pub fn is_unset(&self) -> bool {
                    !self.is_set()
                }
            }
            impl FromU8 for Reserved1 {
                fn from_u8(value: u8) -> Self {
                    use Reserved1::*;
                    match value {
                        0 => No,
                        1 => Yes,
                        _ => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
            }
            impl Add for Reserved1 {
                type Output = Reserved1;
                fn add(self, rhs: Self) -> Self::Output {
                    use Reserved1::*;
                    match (self, rhs) {
                        (No, No) => No,
                        (_, _) => Yes,
                    }
                }
            }
            impl Sub for Reserved1 {
                type Output = Reserved1;
                fn sub(self, rhs: Self) -> Self::Output {
                    use Reserved1::*;
                    match (self, rhs) {
                        (Yes, No) => Yes,
                        (_, _) => No,
                    }
                }
            }
            impl Not for Reserved1 {
                type Output = Reserved1;
                fn not(self) -> Self::Output {
                    use Reserved1::*;
                    match self {
                        No => No,
                        Yes => No,
                    }
                }
            }
            pub enum Reserved2 {
                #[default]
                No,
                Yes,
            }
            #[automatically_derived]
            impl ::core::default::Default for Reserved2 {
                #[inline]
                fn default() -> Reserved2 {
                    Self::No
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Reserved2 {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            Reserved2::No => "No",
                            Reserved2::Yes => "Yes",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Reserved2 {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Reserved2 {
                #[inline]
                fn eq(&self, other: &Reserved2) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Reserved2 {
                #[inline]
                fn clone(&self) -> Reserved2 {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Reserved2 {}
            impl Reserved2 {
                pub fn as_str(&self) -> &str {
                    use Reserved2::*;
                    match self {
                        No => "",
                        Yes => "\x1b[57m",
                    }
                }
                pub fn as_bytes(&self) -> &[u8] {
                    self.as_str().as_bytes()
                }
                pub fn is_set(&self) -> bool {
                    use Reserved2::*;
                    match self {
                        No => false,
                        Yes => true,
                    }
                }
                pub fn is_unset(&self) -> bool {
                    !self.is_set()
                }
            }
            impl FromU8 for Reserved2 {
                fn from_u8(value: u8) -> Self {
                    use Reserved2::*;
                    match value {
                        0 => No,
                        1 => Yes,
                        _ => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
            }
            impl Add for Reserved2 {
                type Output = Reserved2;
                fn add(self, rhs: Self) -> Self::Output {
                    use Reserved2::*;
                    match (self, rhs) {
                        (No, No) => No,
                        (_, _) => Yes,
                    }
                }
            }
            impl Sub for Reserved2 {
                type Output = Reserved2;
                fn sub(self, rhs: Self) -> Self::Output {
                    use Reserved2::*;
                    match (self, rhs) {
                        (Yes, No) => Yes,
                        (_, _) => No,
                    }
                }
            }
            impl Not for Reserved2 {
                type Output = Reserved2;
                fn not(self) -> Self::Output {
                    use Reserved2::*;
                    match self {
                        No => No,
                        Yes => No,
                    }
                }
            }
        }
        mod underline {
            use super::super::convert::FromU8;
            use std::{
                io::{self, Write},
                ops::{Add, Not, Sub},
            };
            pub enum Underline {
                #[default]
                None,
                Single,
                Double,
                Curly,
                Dotted,
                Dashed,
                Unset,
            }
            #[automatically_derived]
            impl ::core::default::Default for Underline {
                #[inline]
                fn default() -> Underline {
                    Self::None
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Underline {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            Underline::None => "None",
                            Underline::Single => "Single",
                            Underline::Double => "Double",
                            Underline::Curly => "Curly",
                            Underline::Dotted => "Dotted",
                            Underline::Dashed => "Dashed",
                            Underline::Unset => "Unset",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Underline {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Underline {
                #[inline]
                fn eq(&self, other: &Underline) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Underline {
                #[inline]
                fn clone(&self) -> Underline {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Underline {}
            impl Underline {
                pub fn as_str(&self) -> &str {
                    use Underline::*;
                    match self {
                        None => "",
                        Single => "\x1b[4m",
                        Double => "\x1b[21m",
                        Curly => "\x1b[4:3m",
                        Dotted => "\x1b[4:4m",
                        Dashed => "\x1b[4:5m",
                        Unset => "\x1b[24m",
                    }
                }
                pub fn as_bytes(&self) -> &[u8] {
                    self.as_str().as_bytes()
                }
            }
            impl FromU8 for Underline {
                fn from_u8(value: u8) -> Self {
                    use Underline::*;
                    match value {
                        0 => None,
                        1 => Single,
                        2 => Double,
                        3 => Curly,
                        4 => Dotted,
                        5 => Dashed,
                        6 => Unset,
                        _ => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                }
            }
            impl Add for Underline {
                type Output = Underline;
                fn add(self, rhs: Self) -> Self::Output {
                    use Underline::*;
                    match (self, rhs) {
                        (None, rhs) => rhs,
                        (lhs, None) => lhs,
                        (_, rhs) => rhs,
                    }
                }
            }
            impl Sub for Underline {
                type Output = Underline;
                fn sub(self, rhs: Self) -> Self::Output {
                    use Underline::*;
                    match (self, rhs) {
                        (None, rhs) => !rhs,
                        (lhs, rhs) if lhs == rhs => None,
                        (lhs, _) => lhs,
                    }
                }
            }
            impl Not for Underline {
                type Output = Underline;
                fn not(self) -> Self::Output {
                    use Underline::*;
                    match self {
                        None => None,
                        Single => Unset,
                        Double => Unset,
                        Curly => Unset,
                        Dotted => Unset,
                        Dashed => Unset,
                        Unset => None,
                    }
                }
            }
        }
        pub enum Invert {
            #[default]
            None,
            Set,
            Unset,
        }
        #[automatically_derived]
        impl ::core::default::Default for Invert {
            #[inline]
            fn default() -> Invert {
                Self::None
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Invert {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        Invert::None => "None",
                        Invert::Set => "Set",
                        Invert::Unset => "Unset",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Invert {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Invert {
            #[inline]
            fn eq(&self, other: &Invert) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        impl Invert {
            pub fn as_str(&self) -> &str {
                use Invert::*;
                match self {
                    None => "",
                    Set => "\x1b[7m",
                    Unset => "\x1b[27m",
                }
            }
            pub fn as_bytes(&self) -> &[u8] {
                self.as_str().as_bytes()
            }
        }
        impl FromU8 for Invert {
            fn from_u8(value: u8) -> Self {
                use Invert::*;
                match value {
                    0 => None,
                    1 => Set,
                    2 => Unset,
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
        }
        impl Add for Invert {
            type Output = Invert;
            fn add(self, rhs: Self) -> Self::Output {
                use Invert::*;
                match (self, rhs) {
                    (None, None) => None,
                    (None, Set) => Set,
                    (None, Unset) => None,
                    (Set, None) => Set,
                    (Set, Set) => Set,
                    (Set, Unset) => Unset,
                    (Unset, None) => Unset,
                    (Unset, Set) => Set,
                    (Unset, Unset) => Unset,
                }
            }
        }
        impl Sub for Invert {
            type Output = Invert;
            fn sub(self, rhs: Self) -> Self::Output {
                use Invert::*;
                match (self, rhs) {
                    (None, rhs) => !rhs,
                    (lhs, rhs) if lhs == rhs => None,
                    (lhs, _) => lhs,
                }
            }
        }
        impl Not for Invert {
            type Output = Invert;
            fn not(self) -> Self::Output {
                use Invert::*;
                match self {
                    Set => Unset,
                    _ => None,
                }
            }
        }
        pub enum Hide {
            #[default]
            None,
            Set,
            Unset,
        }
        #[automatically_derived]
        impl ::core::default::Default for Hide {
            #[inline]
            fn default() -> Hide {
                Self::None
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Hide {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        Hide::None => "None",
                        Hide::Set => "Set",
                        Hide::Unset => "Unset",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Hide {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Hide {
            #[inline]
            fn eq(&self, other: &Hide) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        impl Hide {
            pub fn as_str(&self) -> &str {
                use Hide::*;
                match self {
                    None => "",
                    Set => "\x1b[8m",
                    Unset => "\x1b[28m",
                }
            }
            pub fn as_bytes(&self) -> &[u8] {
                self.as_str().as_bytes()
            }
        }
        impl FromU8 for Hide {
            fn from_u8(value: u8) -> Self {
                use Hide::*;
                match value {
                    0 => None,
                    1 => Set,
                    2 => Unset,
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
        }
        impl Add for Hide {
            type Output = Hide;
            fn add(self, rhs: Self) -> Self::Output {
                use Hide::*;
                match (self, rhs) {
                    (None, None) => None,
                    (None, Set) => Set,
                    (None, Unset) => None,
                    (Set, None) => Set,
                    (Set, Set) => Set,
                    (Set, Unset) => Unset,
                    (Unset, None) => Unset,
                    (Unset, Set) => Set,
                    (Unset, Unset) => Unset,
                }
            }
        }
        impl Sub for Hide {
            type Output = Hide;
            fn sub(self, rhs: Self) -> Self::Output {
                use Hide::*;
                match (self, rhs) {
                    (None, rhs) => !rhs,
                    (lhs, rhs) if lhs == rhs => None,
                    (lhs, _) => lhs,
                }
            }
        }
        impl Not for Hide {
            type Output = Hide;
            fn not(self) -> Self::Output {
                use Hide::*;
                match self {
                    Set => Unset,
                    _ => None,
                }
            }
        }
        pub enum Delete {
            #[default]
            None,
            Set,
            Unset,
        }
        #[automatically_derived]
        impl ::core::default::Default for Delete {
            #[inline]
            fn default() -> Delete {
                Self::None
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Delete {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        Delete::None => "None",
                        Delete::Set => "Set",
                        Delete::Unset => "Unset",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Delete {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Delete {
            #[inline]
            fn eq(&self, other: &Delete) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        impl Delete {
            pub fn as_str(&self) -> &str {
                use Delete::*;
                match self {
                    None => "",
                    Set => "\x1b[9m",
                    Unset => "\x1b[29m",
                }
            }
            pub fn as_bytes(&self) -> &[u8] {
                self.as_str().as_bytes()
            }
        }
        impl FromU8 for Delete {
            fn from_u8(value: u8) -> Self {
                use Delete::*;
                match value {
                    0 => None,
                    1 => Set,
                    2 => Unset,
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
        }
        impl Add for Delete {
            type Output = Delete;
            fn add(self, rhs: Self) -> Self::Output {
                use Delete::*;
                match (self, rhs) {
                    (None, None) => None,
                    (None, Set) => Set,
                    (None, Unset) => None,
                    (Set, None) => Set,
                    (Set, Set) => Set,
                    (Set, Unset) => Unset,
                    (Unset, None) => Unset,
                    (Unset, Set) => Set,
                    (Unset, Unset) => Unset,
                }
            }
        }
        impl Sub for Delete {
            type Output = Delete;
            fn sub(self, rhs: Self) -> Self::Output {
                use Delete::*;
                match (self, rhs) {
                    (None, rhs) => !rhs,
                    (lhs, rhs) if lhs == rhs => None,
                    (lhs, _) => lhs,
                }
            }
        }
        impl Not for Delete {
            type Output = Delete;
            fn not(self) -> Self::Output {
                use Delete::*;
                match self {
                    Set => Unset,
                    _ => None,
                }
            }
        }
        pub enum Overline {
            #[default]
            None,
            Set,
            Unset,
        }
        #[automatically_derived]
        impl ::core::default::Default for Overline {
            #[inline]
            fn default() -> Overline {
                Self::None
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Overline {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        Overline::None => "None",
                        Overline::Set => "Set",
                        Overline::Unset => "Unset",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Overline {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Overline {
            #[inline]
            fn eq(&self, other: &Overline) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        impl Overline {
            pub fn as_str(&self) -> &str {
                use Overline::*;
                match self {
                    None => "",
                    Set => "\x1b[53m",
                    Unset => "\x1b[55m",
                }
            }
            pub fn as_bytes(&self) -> &[u8] {
                self.as_str().as_bytes()
            }
        }
        impl FromU8 for Overline {
            fn from_u8(value: u8) -> Self {
                use Overline::*;
                match value {
                    0 => None,
                    1 => Set,
                    2 => Unset,
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
        }
        impl Add for Overline {
            type Output = Overline;
            fn add(self, rhs: Self) -> Self::Output {
                use Overline::*;
                match (self, rhs) {
                    (None, None) => None,
                    (None, Set) => Set,
                    (None, Unset) => None,
                    (Set, None) => Set,
                    (Set, Set) => Set,
                    (Set, Unset) => Unset,
                    (Unset, None) => Unset,
                    (Unset, Set) => Set,
                    (Unset, Unset) => Unset,
                }
            }
        }
        impl Sub for Overline {
            type Output = Overline;
            fn sub(self, rhs: Self) -> Self::Output {
                use Overline::*;
                match (self, rhs) {
                    (None, rhs) => !rhs,
                    (lhs, rhs) if lhs == rhs => None,
                    (lhs, _) => lhs,
                }
            }
        }
        impl Not for Overline {
            type Output = Overline;
            fn not(self) -> Self::Output {
                use Overline::*;
                match self {
                    Set => Unset,
                    _ => None,
                }
            }
        }
        pub enum PropSpace {
            #[default]
            None,
            Set,
            Unset,
        }
        #[automatically_derived]
        impl ::core::default::Default for PropSpace {
            #[inline]
            fn default() -> PropSpace {
                Self::None
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PropSpace {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        PropSpace::None => "None",
                        PropSpace::Set => "Set",
                        PropSpace::Unset => "Unset",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for PropSpace {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for PropSpace {
            #[inline]
            fn eq(&self, other: &PropSpace) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        impl PropSpace {
            pub fn as_str(&self) -> &str {
                use PropSpace::*;
                match self {
                    None => "",
                    Set => "\x1b[26m",
                    Unset => "\x1b[50m",
                }
            }
            pub fn as_bytes(&self) -> &[u8] {
                self.as_str().as_bytes()
            }
        }
        impl FromU8 for PropSpace {
            fn from_u8(value: u8) -> Self {
                use PropSpace::*;
                match value {
                    0 => None,
                    1 => Set,
                    2 => Unset,
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
        }
        impl Add for PropSpace {
            type Output = PropSpace;
            fn add(self, rhs: Self) -> Self::Output {
                use PropSpace::*;
                match (self, rhs) {
                    (None, None) => None,
                    (None, Set) => Set,
                    (None, Unset) => None,
                    (Set, None) => Set,
                    (Set, Set) => Set,
                    (Set, Unset) => Unset,
                    (Unset, None) => Unset,
                    (Unset, Set) => Set,
                    (Unset, Unset) => Unset,
                }
            }
        }
        impl Sub for PropSpace {
            type Output = PropSpace;
            fn sub(self, rhs: Self) -> Self::Output {
                use PropSpace::*;
                match (self, rhs) {
                    (None, rhs) => !rhs,
                    (lhs, rhs) if lhs == rhs => None,
                    (lhs, _) => lhs,
                }
            }
        }
        impl Not for PropSpace {
            type Output = PropSpace;
            fn not(self) -> Self::Output {
                use PropSpace::*;
                match self {
                    Set => Unset,
                    _ => None,
                }
            }
        }
        impl Blink {
            pub fn is_set(&self) -> bool {
                use Blink::*;
                match self {
                    None => false,
                    _ => true,
                }
            }
            pub fn is_unset(&self) -> bool {
                !self.is_set()
            }
        }
        impl Color {
            pub fn is_set(&self) -> bool {
                use Color::*;
                match self {
                    None => false,
                    _ => true,
                }
            }
            pub fn is_unset(&self) -> bool {
                !self.is_set()
            }
        }
        impl Delete {
            pub fn is_set(&self) -> bool {
                use Delete::*;
                match self {
                    None => false,
                    _ => true,
                }
            }
            pub fn is_unset(&self) -> bool {
                !self.is_set()
            }
        }
        impl Font {
            pub fn is_set(&self) -> bool {
                use Font::*;
                match self {
                    None => false,
                    _ => true,
                }
            }
            pub fn is_unset(&self) -> bool {
                !self.is_set()
            }
        }
        impl FontStyle {
            pub fn is_set(&self) -> bool {
                use FontStyle::*;
                match self {
                    None => false,
                    _ => true,
                }
            }
            pub fn is_unset(&self) -> bool {
                !self.is_set()
            }
        }
        impl Frame {
            pub fn is_set(&self) -> bool {
                use Frame::*;
                match self {
                    None => false,
                    _ => true,
                }
            }
            pub fn is_unset(&self) -> bool {
                !self.is_set()
            }
        }
        impl Hide {
            pub fn is_set(&self) -> bool {
                use Hide::*;
                match self {
                    None => false,
                    _ => true,
                }
            }
            pub fn is_unset(&self) -> bool {
                !self.is_set()
            }
        }
        impl Intensity {
            pub fn is_set(&self) -> bool {
                use Intensity::*;
                match self {
                    None => false,
                    _ => true,
                }
            }
            pub fn is_unset(&self) -> bool {
                !self.is_set()
            }
        }
        impl Invert {
            pub fn is_set(&self) -> bool {
                use Invert::*;
                match self {
                    None => false,
                    _ => true,
                }
            }
            pub fn is_unset(&self) -> bool {
                !self.is_set()
            }
        }
        impl Overline {
            pub fn is_set(&self) -> bool {
                use Overline::*;
                match self {
                    None => false,
                    _ => true,
                }
            }
            pub fn is_unset(&self) -> bool {
                !self.is_set()
            }
        }
        impl PropSpace {
            pub fn is_set(&self) -> bool {
                use PropSpace::*;
                match self {
                    None => false,
                    _ => true,
                }
            }
            pub fn is_unset(&self) -> bool {
                !self.is_set()
            }
        }
        impl Underline {
            pub fn is_set(&self) -> bool {
                use Underline::*;
                match self {
                    None => false,
                    _ => true,
                }
            }
            pub fn is_unset(&self) -> bool {
                !self.is_set()
            }
        }
    }
    const MAX_ONE_BIT: u8 = 0b1;
    const MAX_TWO_BITS: u8 = 0b11;
    const MAX_THREE_BITS: u8 = 0b111;
    const MAX_FOUR_BITS: u8 = 0b1111;
    #[repr(transparent)]
    pub struct Style([u8; 14]);
    #[automatically_derived]
    impl ::core::default::Default for Style {
        #[inline]
        fn default() -> Style {
            Style(::core::default::Default::default())
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Style {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Style {
        #[inline]
        fn eq(&self, other: &Style) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Style {
        #[inline]
        fn clone(&self) -> Style {
            let _: ::core::clone::AssertParamIsClone<[u8; 14]>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Style {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Style {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<[u8; 14]>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Style {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    impl Style {
        pub const fn new() -> Self {
            Style([0; 14])
        }
        pub fn set_reset(&mut self, val: Reset) {
            static MASK: u8 = !(MAX_ONE_BIT << 0);
            self.0[0] = (self.0[0] & MASK) | ((val as u8) << 0);
        }
        pub fn reset(&self) -> Reset {
            Reset::from_u8((self.0[0] >> 0) & MAX_ONE_BIT)
        }
        pub(crate) fn set_reserved1(&mut self, val: Reserved1) {
            static MASK: u8 = !(MAX_ONE_BIT << 1);
            self.0[0] = (self.0[0] & MASK) | ((val as u8) << 1);
        }
        pub(crate) fn reserved1(&self) -> Reserved1 {
            Reserved1::from_u8((self.0[0] >> 1) & MAX_ONE_BIT)
        }
        pub fn set_intensity(&mut self, val: Intensity) {
            static MASK: u8 = !(MAX_THREE_BITS << 2);
            self.0[0] = (self.0[0] & MASK) | ((val as u8) << 2);
        }
        pub fn intensity(&self) -> Intensity {
            Intensity::from_u8((self.0[0] >> 2) & MAX_THREE_BITS)
        }
        pub fn set_font_style(&mut self, val: FontStyle) {
            static MASK: u8 = !(MAX_THREE_BITS << 5);
            self.0[0] = (self.0[0] & MASK) | ((val as u8) << 5);
        }
        pub fn font_style(&self) -> FontStyle {
            FontStyle::from_u8((self.0[0] >> 5) & MAX_THREE_BITS)
        }
        pub fn set_invert(&mut self, val: Invert) {
            static MASK: u8 = !(MAX_TWO_BITS << 0);
            self.0[1] = (self.0[1] & MASK) | ((val as u8) << 0);
        }
        pub fn invert(&self) -> Invert {
            Invert::from_u8((self.0[1] >> 0) & MAX_TWO_BITS)
        }
        pub fn set_underline(&mut self, val: Underline) {
            static MASK: u8 = !(MAX_THREE_BITS << 2);
            self.0[1] = (self.0[1] & MASK) | ((val as u8) << 2);
        }
        pub fn underline(&self) -> Underline {
            Underline::from_u8((self.0[1] >> 2) & MAX_THREE_BITS)
        }
        pub fn set_blink(&mut self, val: Blink) {
            static MASK: u8 = !(MAX_THREE_BITS << 5);
            self.0[1] = (self.0[1] & MASK) | ((val as u8) << 5);
        }
        pub fn blink(&self) -> Blink {
            Blink::from_u8((self.0[1] >> 5) & MAX_THREE_BITS)
        }
        pub fn set_hide(&mut self, val: Hide) {
            static MASK: u8 = !(MAX_TWO_BITS << 0);
            self.0[2] = (self.0[2] & MASK) | ((val as u8) << 0);
        }
        pub fn hide(&self) -> Hide {
            Hide::from_u8((self.0[2] >> 0) & MAX_TWO_BITS)
        }
        pub fn set_delete(&mut self, val: Delete) {
            static MASK: u8 = !(MAX_TWO_BITS << 2);
            self.0[2] = (self.0[2] & MASK) | ((val as u8) << 2);
        }
        pub fn delete(&self) -> Delete {
            Delete::from_u8((self.0[2] >> 2) & MAX_TWO_BITS)
        }
        pub fn set_font(&mut self, val: Font) {
            static MASK: u8 = !(MAX_FOUR_BITS << 4);
            self.0[2] = (self.0[2] & MASK) | ((val as u8) << 4);
        }
        pub fn font(&self) -> Font {
            Font::from_u8((self.0[2] >> 4) & MAX_FOUR_BITS)
        }
        pub fn set_prop_space(&mut self, val: PropSpace) {
            static MASK: u8 = !(MAX_TWO_BITS << 0);
            self.0[3] = (self.0[3] & MASK) | ((val as u8) << 0);
        }
        pub fn prop_space(&self) -> PropSpace {
            PropSpace::from_u8((self.0[3] >> 0) & MAX_TWO_BITS)
        }
        pub fn set_frame(&mut self, val: Frame) {
            static MASK: u8 = !(MAX_THREE_BITS << 2);
            self.0[3] = (self.0[3] & MASK) | ((val as u8) << 2);
        }
        pub fn frame(&self) -> Frame {
            Frame::from_u8((self.0[3] >> 2) & MAX_THREE_BITS)
        }
        pub fn set_fg_color(&mut self, val: Color) {
            let n: u32 = val.into();
            let b = (n >> 1).to_le_bytes();
            self.0[4] = b[0];
            self.0[4 + 1] = b[1];
            self.0[4 + 2] = b[2];
            static MASK: u8 = !(MAX_ONE_BIT << 0);
            self.0[13] = (self.0[13] & MASK) | ((n as u8 & MAX_ONE_BIT) << 0);
        }
        pub fn fg_color(&self) -> Color {
            let mut b = [0; 4];
            b[0] = self.0[4];
            b[1] = self.0[4 + 1];
            b[2] = self.0[4 + 2];
            let n = (u32::from_le_bytes(b) << 1)
                | ((self.0[13] >> 0) & MAX_ONE_BIT) as u32;
            Color::from_u32(n)
        }
        pub fn set_bg_color(&mut self, val: Color) {
            let n: u32 = val.into();
            let b = (n >> 1).to_le_bytes();
            self.0[7] = b[0];
            self.0[7 + 1] = b[1];
            self.0[7 + 2] = b[2];
            static MASK: u8 = !(MAX_ONE_BIT << 1);
            self.0[13] = (self.0[13] & MASK) | ((n as u8 & MAX_ONE_BIT) << 1);
        }
        pub fn bg_color(&self) -> Color {
            let mut b = [0; 4];
            b[0] = self.0[7];
            b[1] = self.0[7 + 1];
            b[2] = self.0[7 + 2];
            let n = (u32::from_le_bytes(b) << 1)
                | ((self.0[13] >> 1) & MAX_ONE_BIT) as u32;
            Color::from_u32(n)
        }
        pub fn set_ul_color(&mut self, val: Color) {
            let n: u32 = val.into();
            let b = (n >> 1).to_le_bytes();
            self.0[10] = b[0];
            self.0[10 + 1] = b[1];
            self.0[10 + 2] = b[2];
            static MASK: u8 = !(MAX_ONE_BIT << 2);
            self.0[13] = (self.0[13] & MASK) | ((n as u8 & MAX_ONE_BIT) << 2);
        }
        pub fn ul_color(&self) -> Color {
            let mut b = [0; 4];
            b[0] = self.0[10];
            b[1] = self.0[10 + 1];
            b[2] = self.0[10 + 2];
            let n = (u32::from_le_bytes(b) << 1)
                | ((self.0[13] >> 2) & MAX_ONE_BIT) as u32;
            Color::from_u32(n)
        }
        pub(crate) fn set_reserved2(&mut self, val: Reserved2) {
            static MASK: u8 = !(MAX_ONE_BIT << 3);
            self.0[13] = (self.0[13] & MASK) | ((val as u8) << 3);
        }
        pub(crate) fn reserved2(&self) -> Reserved2 {
            Reserved2::from_u8((self.0[13] >> 3) & MAX_ONE_BIT)
        }
        pub fn set_overline(&mut self, val: Overline) {
            static MASK: u8 = !(MAX_TWO_BITS << 4);
            self.0[13] = (self.0[13] & MASK) | ((val as u8) << 4);
        }
        pub fn overline(&self) -> Overline {
            Overline::from_u8((self.0[13] >> 4) & MAX_TWO_BITS)
        }
        pub(crate) fn set_prev_intensity(&mut self, val: Intensity) {
            static MASK: u8 = !(MAX_TWO_BITS << 6);
            self.0[13] = (self.0[13] & MASK) | ((val as u8) << 6);
        }
        pub(crate) fn prev_intensity(&self) -> Intensity {
            Intensity::from_u8((self.0[13] >> 6) & MAX_TWO_BITS)
        }
    }
    impl Style {
        pub fn to_string(&self) -> String {
            let mut s = String::with_capacity(128);
            s.push_str("\x1b[");
            let style = [
                cut(self.intensity().as_str()),
                cut(self.font_style().as_str()),
                cut(self.underline().as_str()),
                cut(self.blink().as_str()),
                cut(self.invert().as_str()),
                cut(self.hide().as_str()),
                cut(self.delete().as_str()),
                cut(self.font().as_str()),
                cut(self.prop_space().as_str()),
                cut(&self.fg_color().to_string(ColorKind::Foreground)),
                cut(&self.bg_color().to_string(ColorKind::Background)),
                cut(self.frame().as_str()),
                cut(self.overline().as_str()),
                cut(self.reserved1().as_str()),
                cut(self.reserved2().as_str()),
                cut(&self.ul_color().to_string(ColorKind::Underline)),
            ]
                .iter()
                .filter_map(|x| *x)
                .collect::<Vec<_>>()
                .join(";");
            s.push_str(&style);
            s.push('m');
            if s.len() == 3 {
                s.clear();
            }
            s
        }
    }
    impl Add for Style {
        type Output = Style;
        fn add(mut self, rhs: Self) -> Self::Output {
            self.set_prev_intensity(
                match self.intensity() {
                    intensity @ Intensity::None | intensity @ Intensity::Bold => {
                        intensity
                    }
                    _ => Intensity::None,
                },
            );
            self.set_reset(self.reset() + rhs.reset());
            self.set_intensity(self.intensity() + rhs.intensity());
            self.set_font_style(self.font_style() + rhs.font_style());
            self.set_underline(self.underline() + rhs.underline());
            self.set_blink(self.blink() + rhs.blink());
            self.set_invert(self.invert() + rhs.invert());
            self.set_hide(self.hide() + rhs.hide());
            self.set_delete(self.delete() + rhs.delete());
            self.set_font(self.font() + rhs.font());
            self.set_prop_space(self.prop_space() + rhs.prop_space());
            self.set_fg_color(self.fg_color() + rhs.fg_color());
            self.set_bg_color(self.bg_color() + rhs.bg_color());
            self.set_frame(self.frame() + rhs.frame());
            self.set_overline(self.overline() + rhs.overline());
            self.set_reserved1(self.reserved1() + rhs.reserved1());
            self.set_reserved2(self.reserved2() + rhs.reserved2());
            self.set_ul_color(self.ul_color() + rhs.ul_color());
            self
        }
    }
    impl Sub for Style {
        type Output = Style;
        fn sub(mut self, rhs: Self) -> Self::Output {
            self.set_prev_intensity(
                match rhs.intensity() {
                    intensity @ Intensity::None | intensity @ Intensity::Bold => {
                        intensity
                    }
                    _ => Intensity::None,
                },
            );
            self.set_reset(self.reset() - rhs.reset());
            self.set_intensity(self.intensity() - rhs.intensity());
            self.set_font_style(self.font_style() - rhs.font_style());
            self.set_underline(self.underline() - rhs.underline());
            self.set_blink(self.blink() - rhs.blink());
            self.set_invert(self.invert() - rhs.invert());
            self.set_hide(self.hide() - rhs.hide());
            self.set_delete(self.delete() - rhs.delete());
            self.set_font(self.font() - rhs.font());
            self.set_prop_space(self.prop_space() - rhs.prop_space());
            self.set_fg_color(self.fg_color() - rhs.fg_color());
            self.set_bg_color(self.bg_color() - rhs.bg_color());
            self.set_frame(self.frame() - rhs.frame());
            self.set_overline(self.overline() - rhs.overline());
            self.set_reserved1(self.reserved1() - rhs.reserved1());
            self.set_reserved2(self.reserved2() - rhs.reserved2());
            self.set_ul_color(self.ul_color() - rhs.ul_color());
            self
        }
    }
    impl Not for Style {
        type Output = Style;
        fn not(mut self) -> Self::Output {
            self.set_prev_intensity(Intensity::None);
            self.set_reset(!self.reset());
            self.set_intensity(!self.intensity());
            self.set_font_style(!self.font_style());
            self.set_underline(!self.underline());
            self.set_blink(!self.blink());
            self.set_invert(!self.invert());
            self.set_hide(!self.hide());
            self.set_delete(!self.delete());
            self.set_font(!self.font());
            self.set_prop_space(!self.prop_space());
            self.set_fg_color(!self.fg_color());
            self.set_bg_color(!self.bg_color());
            self.set_frame(!self.frame());
            self.set_overline(!self.overline());
            self.set_reserved1(!self.reserved1());
            self.set_reserved2(!self.reserved2());
            self.set_ul_color(!self.ul_color());
            self
        }
    }
    impl Debug for Style {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if f.alternate() {
                f.debug_struct("Style")
                    .field("reset", &self.reset())
                    .field("intensity", &self.intensity())
                    .field("font_style", &self.font_style())
                    .field("underline", &self.underline())
                    .field("blink", &self.blink())
                    .field("invert", &self.invert())
                    .field("delete", &self.delete())
                    .field("font", &self.font())
                    .field("prop_space", &self.prop_space())
                    .field("fg_color", &self.fg_color())
                    .field("bg_color", &self.bg_color())
                    .field("frame", &self.frame())
                    .field("overline", &self.overline())
                    .field("reserved1", &self.reserved1())
                    .field("reserved2", &self.reserved2())
                    .field("ul_color", &self.ul_color())
                    .finish()
            } else {
                f.debug_tuple("Style").field(&self.0).finish()
            }
        }
    }
    impl Display for Style {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if f.alternate() {
                f.write_str(&self.not().to_string())
            } else {
                f.write_str(&self.to_string())
            }
        }
    }
    fn cut(s: &str) -> Option<&str> {
        if s.is_empty() { None } else { Some(&s[2..s.len() - 1]) }
    }
}
/// Styles the given text using the ziyy parser.
///
/// This function takes a string slice and returns a styled string. It uses the `Parser`
/// to parse the input text and apply the specified styles. If the parsing is successful,
/// it returns the styled string; otherwise, it panics with the error message.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to be styled.
///
/// # Example
///
/// ```
/// use ziyy::style;
///
/// let styled_text = style("This is <b>bold</b> text");
/// assert_eq!(styled_text, "This is <b>bold</b> text");
/// ```
///
/// # Panics
///
/// This function will panic if the parser encounters an error while parsing the input text.
///
/// # Returns
///
/// A `String` containing the styled text.
pub fn style(text: &str) -> String {
    let mut parser = Parser::new();
    match parser.parse(Context::new(text, None)) {
        Ok(s) => s,
        Err(e) => {
            ::core::panicking::panic_fmt(format_args!("{0}", e));
        }
    }
}
/// Styles text without removing excess whitespace.
pub fn ziyy(text: &str) -> String {
    let mut parser = Parser::new();
    parser.pre_ws = 1;
    match parser.parse(Context::new(text, None)) {
        Ok(s) => s,
        Err(e) => {
            ::core::panicking::panic_fmt(format_args!("{0}", e));
        }
    }
}
/// Result
pub type Result<'src, T> = std::result::Result<T, Error<'src>>;
