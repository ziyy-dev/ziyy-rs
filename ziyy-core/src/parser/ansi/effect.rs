use std::ops::{Add, Not, Sub};

/// Ansi effect
#[derive(Default, Debug, PartialEq)]
pub enum Effect {
    #[default]
    None,
    Apply,
    Clear,
}

impl Effect {
    pub fn is_set(&self) -> bool {
        !matches!(self, Effect::None)
    }

    pub fn is_unset(&self) -> bool {
        !self.is_set()
    }
}

impl Add for Effect {
    type Output = Effect;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Effect::None, Effect::None) => Effect::None,
            (Effect::None, Effect::Apply) => Effect::Apply,
            (Effect::None, Effect::Clear) => Effect::None,
            (Effect::Apply, Effect::None) => Effect::Apply,
            (Effect::Apply, Effect::Apply) => Effect::Apply,
            (Effect::Apply, Effect::Clear) => Effect::Clear,
            (Effect::Clear, Effect::None) => Effect::Clear,
            (Effect::Clear, Effect::Apply) => Effect::Apply,
            (Effect::Clear, Effect::Clear) => Effect::Clear,
        }
    }
}

impl Sub for Effect {
    type Output = Effect;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Effect::None, _) => Effect::None,
            (Effect::Apply, Effect::Apply) => Effect::None,
            (Effect::Clear, Effect::Clear) => Effect::None,
            (Effect::Apply, _) => Effect::Apply,
            (Effect::Clear, Effect::None) => Effect::None,
            (Effect::Clear, _) => Effect::Clear,
        }
    }
}

impl Not for Effect {
    type Output = Effect;

    fn not(self) -> Self::Output {
        match self {
            Effect::None => Effect::None,
            Effect::Apply => Effect::Clear,
            Effect::Clear => Effect::None,
        }
    }
}

impl From<(bool, bool)> for Effect {
    fn from(value: (bool, bool)) -> Self {
        match value {
            (false, false) => Effect::None,
            (true, false) => Effect::Apply,
            (false, true) => Effect::Clear,
            _ => unreachable!("Invalid Effect"),
        }
    }
}

/// Two Effects that are cleared by same ansi sequence and only
/// one can be set at a time.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum DuoEffect {
    #[default]
    /// No effect is set
    None,
    /// The first effect is set
    A,
    /// The second effect is set
    B,
    /// The effect changed from the first effect to the second effect
    AB,
    /// The effect changed from the second effect to the first effect
    BA,
    AE,
    BE,
    /// Both effects are cleared
    E,
}

impl DuoEffect {
    /// If any effect is set
    pub fn is_set(&self) -> bool {
        !matches!(self, DuoEffect::None)
    }

    /// If no effect is set
    pub fn is_unset(&self) -> bool {
        !self.is_set()
    }
}

impl Add for DuoEffect {
    type Output = DuoEffect;

    /// Merges two [DuoEffect]. This method is not commutative.
    /// `self` is the initial [DuoEffect] while `rhs` is a new state it is moving to.
    /// The result is the value after such movement.
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // do no clear, since nothing is set
            (DuoEffect::None, DuoEffect::E | DuoEffect::AE | DuoEffect::BE) => DuoEffect::None,
            // we have left the initial state
            (DuoEffect::None, rhs) => rhs,
            // we can't go back to initial state without an explicit clear
            (lhs, DuoEffect::None) => lhs,
            (DuoEffect::E, rhs) => rhs,
            // downgrade clear all to only clear A and resist upgrade
            (DuoEffect::A | DuoEffect::BA | DuoEffect::AE, DuoEffect::E) => DuoEffect::AE,
            // downgrade clear all to only clear B and resist upgrade
            (DuoEffect::B | DuoEffect::AB | DuoEffect::BE, DuoEffect::E) => DuoEffect::BE,
            // similar effects so keep it
            (DuoEffect::A | DuoEffect::BA, DuoEffect::A) => DuoEffect::A,
            // similar effects so keep it
            (DuoEffect::B | DuoEffect::AB, DuoEffect::B) => DuoEffect::B,
            // track movement of effect
            (DuoEffect::A | DuoEffect::BA, DuoEffect::B) => DuoEffect::AB,
            // track movement of effect
            (DuoEffect::B | DuoEffect::AB, DuoEffect::A) => DuoEffect::BA,
            // clear A and BA only
            (DuoEffect::A | DuoEffect::BA, DuoEffect::AE) => DuoEffect::AE,
            // clear B and AB only
            (DuoEffect::B | DuoEffect::AB, DuoEffect::BE) => DuoEffect::BE,

            (DuoEffect::AE, DuoEffect::B) => DuoEffect::AB,
            (DuoEffect::BE, DuoEffect::A) => DuoEffect::BA,
            // AE can only clear A and BA
            (lhs, DuoEffect::AE) => lhs,
            // BE can only clear B and AB
            (lhs, DuoEffect::BE) => lhs,
            // all others, keep `rhs`
            (_, rhs) => rhs,
        }
    }
}

impl Sub for DuoEffect {
    type Output = DuoEffect;

    /// Differentiates two [DuoEffect]. This is used by [Resolver::optimize_styles][crate::Resolver::optimize_styles]
    /// to ensure an effect is declared once and cleared once.
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (DuoEffect::E, DuoEffect::None) => DuoEffect::None,
            // if none, no need to redeclare effect or clear all effects
            (DuoEffect::None | DuoEffect::E, _) => self,
            // we have left the initial state or state of being cleared
            (lhs, DuoEffect::None | DuoEffect::E) => lhs,
            // we can't declare an effect twice
            (DuoEffect::A, DuoEffect::A | DuoEffect::BA) => DuoEffect::None,
            //
            (DuoEffect::B, DuoEffect::A | DuoEffect::BA) => DuoEffect::AB,
            (DuoEffect::A, DuoEffect::B | DuoEffect::AB) => DuoEffect::BA,
            (DuoEffect::B, DuoEffect::B | DuoEffect::AB) => DuoEffect::None,
            (DuoEffect::AE, DuoEffect::A | DuoEffect::BA) => DuoEffect::AE,
            (DuoEffect::BE, DuoEffect::B | DuoEffect::AB) => DuoEffect::BE,
            (DuoEffect::AE, _) => DuoEffect::None,
            (DuoEffect::BE, _) => DuoEffect::None,
            (lhs, _) => lhs,
        }
    }
}

impl Not for DuoEffect {
    type Output = DuoEffect;

    fn not(self) -> Self::Output {
        match self {
            DuoEffect::None => DuoEffect::None,
            DuoEffect::A | DuoEffect::BA => DuoEffect::AE,
            DuoEffect::B | DuoEffect::AB => DuoEffect::BE,
            DuoEffect::E | DuoEffect::AE | DuoEffect::BE => DuoEffect::None,
        }
    }
}

impl From<(bool, bool, bool)> for DuoEffect {
    fn from(value: (bool, bool, bool)) -> Self {
        match value {
            (false, false, false) => DuoEffect::None,
            (true, false, false) => DuoEffect::A,
            (false, true, false) => DuoEffect::B,
            (true, true, false) => DuoEffect::AB,
            (false, false, true) => DuoEffect::BA,
            (false, true, true) => DuoEffect::AE,
            (true, false, true) => DuoEffect::BE,
            (true, true, true) => DuoEffect::E,
        }
    }
}
