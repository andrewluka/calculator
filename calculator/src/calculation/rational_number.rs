use crate::shared::sign::Sign;
use std::ops::Add;

use super::{
    calculation_precision::{SignedValuePrecision, UnsignedValuePrecision},
    helpers::hcf,
};

#[derive(PartialEq, Debug)]
pub struct RationalNumber {
    numerator: UnsignedValuePrecision,
    denominator: UnsignedValuePrecision,
    sign: Sign,
}

impl RationalNumber {
    pub fn new(numerator: SignedValuePrecision, denominator: SignedValuePrecision) -> Self {
        assert!(denominator != 0);

        let is_numerator_negative = numerator < 0;
        let is_denominator_negative = denominator < 0;

        let numerator = numerator.abs() as UnsignedValuePrecision;
        let denominator = denominator.abs() as UnsignedValuePrecision;

        let hcf = hcf(numerator, denominator);

        RationalNumber {
            numerator: numerator / hcf,
            denominator: denominator / hcf,
            sign: Sign::from(is_denominator_negative == is_numerator_negative),
        }
    }
}

impl ToString for RationalNumber {
    fn to_string(&self) -> String {
        let sign = match self.sign {
            Sign::Positive => "",
            Sign::Negative => "-",
        };

        format!("{}{}/{}", sign, self.numerator, self.denominator)
    }
}

impl Add for RationalNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let self_like_numerator = self.numerator * rhs.denominator;
        let rhs_like_numerator = rhs.numerator * self.denominator;

        let numerator = if self.sign == rhs.sign {
            self_like_numerator + rhs_like_numerator
        } else {
            let (larger_numerator, smaller_numerator) = if self_like_numerator > rhs_like_numerator
            {
                (self_like_numerator, rhs_like_numerator)
            } else {
                (rhs_like_numerator, self_like_numerator)
            };

            larger_numerator - smaller_numerator
        };

        let denominator = self.denominator * rhs.denominator;

        let hcf = hcf(numerator, denominator);

        Self {
            numerator: numerator / hcf,
            denominator: denominator / hcf,
            sign: if self_like_numerator > rhs_like_numerator {
                self.sign
            } else {
                rhs.sign
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn disallow_zero_denominator() {
        RationalNumber::new(10, 0);
    }

    #[test]
    fn create_rational_number() {
        let two_by_three = RationalNumber::new(2, 3);
        assert_eq!(two_by_three.to_string(), "2/3");

        let negative_two_by_three = RationalNumber::new(40, -60);
        assert_eq!(negative_two_by_three.to_string(), "-2/3");
    }

    #[test]
    fn adding_rational_numbers_works() {
        let a = RationalNumber::new(3, 4);
        let b = RationalNumber::new(4, 3);
        let sum = a + b;
        assert_eq!(sum, RationalNumber::new(25, 12));
    }
}
