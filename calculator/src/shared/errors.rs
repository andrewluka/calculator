use std::{
    error::Error,
    fmt::{Debug, Display},
};

// #[derive(Debug)]
// pub enum MovementError {
//     NoNextElement,
//     NoPrevElement,
// }

// impl Error for MovementError {}
// impl Display for MovementError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         "Error moving the cursor".fmt(f)
//     }
// }

pub enum ParsingError {
    NoSuchCharacterCode,
    CannotParseEmptyString,
    MismatchedBrackets,
    ExpectedButFound { expected: String, found: String },
    EndOfInput,
    Unexpected(String),
    ExcessiveDecimalPoints,
    Custom(String),
}

impl Error for ParsingError {}
impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            ParsingError::NoSuchCharacterCode => "couldn't parse character".to_string(),
            ParsingError::CannotParseEmptyString => "cannot parse an empty string".to_string(),
            ParsingError::MismatchedBrackets => "mismatched brackets".to_string(),
            ParsingError::ExpectedButFound { expected, found } => {
                format!("expected {expected} but found {found}")
            }
            ParsingError::EndOfInput => "unexpected end of input".to_string(),
            ParsingError::ExcessiveDecimalPoints => {
                "only one decimal point is allowed in a decimal".to_string()
            }
            ParsingError::Unexpected(x) => format!("unexpected {}", x),
            ParsingError::Custom(s) => s.to_string(),
        };
        let msg = format!("error: {msg}");

        Display::fmt(&msg, f)
    }
}
impl Debug for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

#[derive(Debug)]
pub enum MutationOperationError {
    AdditionError,
    RemovalError,
}

impl Error for MutationOperationError {}
impl Display for MutationOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match *self {
            MutationOperationError::AdditionError => "cannot overwrite existing data",
            MutationOperationError::RemovalError => "couldn't remove any further",
        };

        Display::fmt(&msg, f)
    }
}

#[derive(Debug)]
pub struct CalculationError(String);

impl CalculationError {
    pub fn new(msg: String) -> Self {
        Self(msg)
    }
}

impl Error for CalculationError {}
impl Display for CalculationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = format!("error: {}", self.0);
        Display::fmt(&msg, f)
    }
}
