use std::collections::HashMap;
#[cfg(feature = "terminfo")]
use std::sync::LazyLock;

use crate::parser::Chunk;
use crate::scanner::Scanner;
use crate::shared::Input;
use crate::style::Style;

use state::State;
#[cfg(feature = "terminfo")]
use terminfo::Database;

mod state;

#[cfg(feature = "terminfo")]
static DATABASE: LazyLock<Option<Database>> = LazyLock::new(|| Database::from_env().ok());

pub struct Context<'src, I: ?Sized + Input> {
    /// The scanner used to tokenize the input source.
    pub(crate) scanner: Scanner<'src, I>,
    /// Optional bindings for styles.
    #[cfg(feature = "bindings")]
    pub(crate) bindings: Option<HashMap<&'src [u8], Style>>,
    /// The current state of the parser.
    pub(crate) state: State<'src, I>,
    /// The next chunk to be parsed.
    pub(crate) next_chunk: Option<Chunk<'src, I>>,
}

impl<'src, I: ?Sized + Input> Context<'src, I> {
    #[must_use]
    #[cfg(feature = "bindings")]
    pub fn new(input: &'src I, bindings: Option<HashMap<&'src [u8], Style>>) -> Self {
        Self {
            scanner: Scanner::new(input),
            bindings,
            state: State::new(),
            next_chunk: None,
        }
    }

    #[must_use]
    #[cfg(not(feature = "bindings"))]
    pub fn new(input: &'src I) -> Self {
        Self {
            scanner: Scanner::new(input),
            state: State::new(),
            next_chunk: None,
        }
    }
}
