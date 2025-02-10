use core::fmt::Debug;

#[derive(Clone, Default)]
pub struct Position(pub u16, pub u16);

impl Position {
    #[must_use]
    pub fn new(line: u16, column: u16) -> Self {
        Self(line, column)
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.0 + 1, self.1 + 1))?;

        Ok(())
    }
}
