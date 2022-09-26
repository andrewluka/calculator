use num_traits::FromPrimitive;
use striminant_macro::striminant;
use strum_macros::{EnumIter, IntoStaticStr};

use crate::shared::errors::ParsingError;

#[repr(u8)]
#[striminant(except = [b'h', b'q'])]
#[derive(Debug, PartialEq, EnumIter, FromPrimitive, IntoStaticStr)]
pub enum Erasable {
    // digits
    Zero = b'0',
    One = b'1',
    Two = b'2',
    Three = b'3',
    Four = b'4',
    Five = b'5',
    Six = b'6',
    Seven = b'7',
    Eight = b'8',
    Nine = b'9',

    // arithmetic operators
    PlusSign = b'+',
    NegativeSign = b'-',
    MultiplicationSign = b'*',
    DivisionSign = b'/',

    // brackets
    LeftParenthesis = b'(',
    RightParenthesis = b')',
    LeftCurly = b'{',
    RightCurly = b'}',
    LeftSquare = b'[',
    RightSquare = b']',

    // for formatting
    Space = b' ',

    // notation
    DecimalPoint = b'.',
    // scientific notation; eg: 2.43E-3
    TimesTenToThePowerOf = b'E',

    // named constants
    #[strum(serialize = "pi")]
    Pi = b'p',
    E = b'e',
    I = b'i',

    // functions
    #[strum(serialize = "abs")]
    Absolute = b'a',
    #[strum(serialize = "sin")]
    Sin = b's',
    #[strum(serialize = "cos")]
    Cos = b'c',
    #[strum(serialize = "tan")]
    Tan = b't',
    #[strum(serialize = "asin")]
    Arcsin = b'S',
    #[strum(serialize = "acos")]
    Arccos = b'C',
    #[strum(serialize = "atan")]
    Arctan = b'T',
    #[strum(serialize = "NthRoot")]
    NthRoot = b'R',

    // complex erasable (requires complex rendering)
    FractionDivider = b'_',
    ExponentPlaceholder = b'^',

    // angle units
    #[strum(serialize = "°")]
    Degrees = b'd',
    #[strum(serialize = "rad")]
    Radians = b'r',
}

pub enum ErasableType {
    Digit,
    ArithmeticOperator,
    OpeningBracket,
    ClosingBracket,
    Formatting,
    Notation,
    NamedConstant,
    FunctionName,
    Complex,
    AngleUnit,
}

impl From<&Erasable> for ErasableType {
    fn from(e: &Erasable) -> Self {
        use Erasable::*;

        match e {
            Zero | One | Two | Three | Four | Five | Six | Seven | Eight | Nine => {
                ErasableType::Digit
            }
            PlusSign | NegativeSign | MultiplicationSign | DivisionSign => {
                ErasableType::ArithmeticOperator
            }
            LeftCurly | LeftParenthesis | LeftSquare => ErasableType::OpeningBracket,
            RightCurly | RightParenthesis | RightSquare => ErasableType::ClosingBracket,
            Space => ErasableType::Formatting,
            DecimalPoint | TimesTenToThePowerOf => ErasableType::Notation,
            Pi | E | I => ErasableType::NamedConstant,
            Absolute | Sin | Cos | Tan | Arcsin | Arccos | Arctan | NthRoot => {
                ErasableType::FunctionName
            }
            FractionDivider | ExponentPlaceholder => ErasableType::Complex,
            Degrees | Radians => ErasableType::AngleUnit,
        }
    }
}

impl Erasable {
    pub fn build(c: char) -> Result<Self, ParsingError> {
        match <Erasable as FromPrimitive>::from_u8(c as u8) {
            Some(e) => Ok(e),
            None => Err(ParsingError::NoSuchCharacterCode),
        }
    }

    pub fn length_in_chars(&self) -> usize {
        let str: &'static str = self.into();
        str.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn building_an_erasable_works() {
        let acos = Erasable::build('C').unwrap();
        assert_eq!(acos, Erasable::Arccos);
    }

    #[test]
    #[should_panic]
    fn invalid_characters_panic() {
        Erasable::build('h').unwrap();
    }
}