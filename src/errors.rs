use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum MovementError {
    NoNextElement,
    NoPrevElement,
}

impl Error for MovementError {}
impl Display for MovementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "Error moving the cursor".fmt(f)
    }
}

#[derive(Debug)]
pub enum ParsingError {
    NoSuchCharacterCode,
    CannotParseEmptyString,
}

impl Error for ParsingError {}
impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ParsingError::NoSuchCharacterCode => "couldn't parse character".fmt(f),
            ParsingError::CannotParseEmptyString => "cannot parse an empty string".fmt(f),
        }
    }
}
