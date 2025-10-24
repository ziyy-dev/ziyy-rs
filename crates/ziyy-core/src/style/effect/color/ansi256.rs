use super::ColorKind;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Ansi256(pub u8);

impl Ansi256 {
    #[must_use]
    pub fn to_string(&self, kind: ColorKind) -> String {
        let Ansi256(n) = self;
        format!("\x1b[{};5;{n}m", kind as u8 + 8)
    }
}
