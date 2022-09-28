use std::ops::{Neg, Not};

#[derive(PartialEq, Debug, Clone, Copy, Default)]
pub enum Sign {
    #[default]
    Positive = 1,
    Negative = -1,
}

impl From<bool> for Sign {
    fn from(b: bool) -> Self {
        if b {
            Sign::Positive
        } else {
            Sign::Negative
        }
    }
}

impl Sign {
    fn inverse(&self) -> Self {
        match *self {
            Self::Positive => Self::Negative,
            Self::Negative => Self::Positive,
        }
    }
}

impl Not for Sign {
    type Output = Self;
    fn not(self) -> Self::Output {
        self.inverse()
    }
}

impl Neg for Sign {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.inverse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn building_a_sign() {
        assert_eq!(Sign::Positive, Sign::from(true));
        assert_eq!(Sign::Negative, Sign::from(false));
    }

    #[test]
    fn sign_negation() {
        assert_eq!(Sign::Positive, -Sign::Negative);
        assert_eq!(Sign::Negative, !Sign::Positive);
    }
}
