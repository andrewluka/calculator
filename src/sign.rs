use std::ops::{Neg, Not};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Sign {
    Positive = 1,
    Negative = -1,
}

impl Sign {
    pub fn build(b: bool) -> Sign {
        if b {
            Sign::Positive
        } else {
            Sign::Negative
        }
    }

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
        assert_eq!(Sign::Positive, Sign::build(true));
        assert_eq!(Sign::Negative, Sign::build(false));
    }

    #[test]
    fn sign_negation() {
        assert_eq!(Sign::Positive, -Sign::Negative);
        assert_eq!(Sign::Negative, !Sign::Positive);
    }
}
