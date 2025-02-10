use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Default, PartialEq, Debug)]
pub enum Condition {
    /// Initial state A.
    A,
    /// Initial state B.
    B,
    /// Current state is B and previous state is A
    AB,
    /// Current state is A and previous state is B
    BA,
    /// Default state (0)
    #[default]
    None,
}

impl Condition {
    pub fn is_none(&self) -> bool {
        matches!(self, Condition::None)
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    pub fn is_a(&self) -> bool {
        matches!(self, Condition::A)
    }

    pub fn is_ab(&self) -> bool {
        matches!(self, Condition::AB)
    }

    pub fn is_b(&self) -> bool {
        matches!(self, Condition::B)
    }

    pub fn is_ba(&self) -> bool {
        matches!(self, Condition::BA)
    }

    pub fn ends_with_a(&self) -> bool {
        self.is_a() || self.is_ba()
    }

    pub fn ends_with_b(&self) -> bool {
        self.is_b() || self.is_ab()
    }
}

impl Add for Condition {
    type Output = Condition;

    fn add(self, rhs: Self) -> Self::Output {
        // self is previous state and rhs is current state
        match self {
            Condition::A | Condition::BA => match rhs {
                Condition::A | Condition::None => Condition::A,
                Condition::B => Condition::AB,
                _ => {
                    panic!("This is an impossible situation as rhs is in an initial state")
                }
            },

            Condition::B | Condition::AB => match rhs {
                Condition::A => Condition::BA,
                Condition::B | Condition::None => Condition::B,
                _ => {
                    panic!("This is an impossible situation as rhs is in an initial state")
                }
            },

            Condition::None => rhs,
        }
    }
}

impl AddAssign for Condition {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl Sub for Condition {
    type Output = Condition;

    fn sub(self, rhs: Self) -> Self::Output {
        // self is the current style state and rhs is the previous style state
        match self {
            Condition::A => match rhs {
                Condition::A | Condition::BA => Condition::None, // A is already declared
                Condition::B | Condition::AB => Condition::BA,   // it is moving from B to A
                Condition::None => Condition::A,                 // A was not declared
            },

            Condition::B => match rhs {
                Condition::B | Condition::AB => Condition::None, // B is already declared
                Condition::A | Condition::BA => Condition::AB,   // it is moving from A to B
                Condition::None => Condition::B,                 // B was not declared
            },

            Condition::None => Condition::None, // no style was declared so do nothing

            _ => panic!("This is an impossible situation as self is in an initial state"),
        }
    }
}

impl SubAssign for Condition {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
    }
}
