use std::{error::Error, fmt::Display};

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

        msg.fmt(f)
    }
}

#[derive(Debug)]
pub enum Surprise {
    Expected(&'static str),
    Unexpected(&'static str),
}

impl Display for Surprise {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Surprise::Expected(msg) => format!("error: expected {msg}"),
            Surprise::Unexpected(msg) => format!("error: unexpected {msg}"),
        };

        msg.fmt(f)
    }
}

#[derive(Debug)]
pub struct SyntaxError(Surprise);

impl Error for SyntaxError {}
impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
