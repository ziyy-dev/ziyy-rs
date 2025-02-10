use super::{Condition, Style};

/// A builder for creating [Style] instances.
///
/// # Examples
///
/// ```
/// use ziyy_core::style::builder::StyleBuilder;
///
/// let style = StyleBuilder::new()
///     .bold()
///     .italics()
///     .under()
///     .finish();
/// ```
///
#[derive(Default)]
#[repr(transparent)]
pub struct StyleBuilder(Style);

impl StyleBuilder {
    /// Creates a new Style Builder.
    #[must_use]
    pub fn new() -> Self {
        StyleBuilder::default()
    }

    /// Turns on bold style (turns off dim style).
    #[must_use]
    pub fn bold(mut self) -> Self {
        self.0.brightness = Condition::A;
        self
    }

    /// Turns on dim style (turns off bold style).
    #[must_use]
    pub fn dim(mut self) -> Self {
        self.0.brightness = Condition::B;
        self
    }

    /// Turns on italics style.
    #[must_use]
    pub fn italics(mut self) -> Self {
        self.0.italics = true;
        self
    }

    /// Turns on strike-through.
    #[must_use]
    pub fn strike(mut self) -> Self {
        self.0.strike = true;
        self
    }

    /// Turns on underline
    #[must_use]
    pub fn under(mut self) -> Self {
        self.0.under = Condition::A;
        self
    }

    /// Completes the build and returns the [Style].
    #[must_use]
    pub fn finish(self) -> Style {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bold() {
        let style = StyleBuilder::new().bold().finish();
        assert_eq!(style.brightness, Condition::A);
    }

    #[test]
    fn test_dim() {
        let style = StyleBuilder::new().dim().finish();
        assert_eq!(style.brightness, Condition::B);
    }

    #[test]
    fn test_italics() {
        let style = StyleBuilder::new().italics().finish();
        assert!(style.italics);
    }

    #[test]
    fn test_strike() {
        let style = StyleBuilder::new().strike().finish();
        assert!(style.strike);
    }

    #[test]
    fn test_under() {
        let style = StyleBuilder::new().under().finish();
        assert_eq!(style.under, Condition::A);
    }

    #[test]
    fn test_combined_styles() {
        let style = StyleBuilder::new().bold().italics().under().finish();
        assert_eq!(style.brightness, Condition::A);
        assert!(style.italics);
        assert_eq!(style.under, Condition::A);
    }
}
