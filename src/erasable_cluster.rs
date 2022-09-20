use crate::{
    errors::{MovementError, ParsingError, RemovalError},
    named_constants::NamedConstant,
    sign::Sign,
};

// used with erasables
#[derive(Debug, PartialEq)]
enum FunctionName {
    Absolute,
    Sin,
    Cos,
    Tan,
    Arcsin,
    Arccos,
    Arctan,
}

#[derive(Debug, PartialEq)]
enum Erasable {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    PlusSign,
    NegativeSign,
    MultiplicationSign,
    DivisionSign,
    LeftBracket,
    RightBracket,
    Space,
    FractionDivider,
    DecimalPoint,
    Root,
    ExponentPlaceholder,
    NamedConstant(NamedConstant),
    Function(FunctionName),
    RadiansSign,
    DegreesSign,
}

impl Erasable {
    // some characters will have to use shift
    fn build(c: char) -> Result<Self, ParsingError> {
        let e = match c {
            '0' => Erasable::Zero,
            '1' => Erasable::One,
            '2' => Erasable::Two,
            '3' => Erasable::Three,
            '4' => Erasable::Four,
            '5' => Erasable::Five,
            '6' => Erasable::Six,
            '7' => Erasable::Seven,
            '8' => Erasable::Eight,
            '9' => Erasable::Nine,
            '+' => Erasable::PlusSign,
            '-' => Erasable::NegativeSign,
            '*' => Erasable::MultiplicationSign,
            '/' => Erasable::DivisionSign,
            '[' | '{' | '(' => Erasable::LeftBracket,
            ']' | '}' | ')' => Erasable::RightBracket,
            'f' => Erasable::FractionDivider,
            '.' => Erasable::DecimalPoint,
            'R' => Erasable::Root,
            '^' => Erasable::ExponentPlaceholder,
            'p' => Erasable::NamedConstant(NamedConstant::Pi),
            'e' => Erasable::NamedConstant(NamedConstant::E),
            'i' => Erasable::NamedConstant(NamedConstant::I),
            'a' => Erasable::Function(FunctionName::Absolute),
            's' => Erasable::Function(FunctionName::Sin),
            'c' => Erasable::Function(FunctionName::Cos),
            't' => Erasable::Function(FunctionName::Tan),
            'S' => Erasable::Function(FunctionName::Arcsin),
            'C' => Erasable::Function(FunctionName::Arccos),
            'T' => Erasable::Function(FunctionName::Arctan),
            ' ' => Erasable::Space,
            'r' => Erasable::RadiansSign,
            'd' => Erasable::DegreesSign,
            _ => return Err(ParsingError::NoSuchCharacterCode),
        };

        Ok(e)
    }

    fn length_in_chars(&self) -> usize {
        match *self {
            Erasable::Function(FunctionName::Absolute)
            | Erasable::Function(FunctionName::Sin)
            | Erasable::Function(FunctionName::Cos)
            | Erasable::Function(FunctionName::Tan) => 3,
            Erasable::Function(FunctionName::Arcsin)
            | Erasable::Function(FunctionName::Arccos)
            | Erasable::Function(FunctionName::Arctan) => 4,
            _ => 1,
        }
    }
}

pub struct ErasableCluster {
    erasables: Vec<Erasable>,
    cursor: Cursor,
}

enum CursorPosition {
    Empty,
    NotEmpty {
        position_in_chars: usize,
        position_in_erasable_count: usize,
    },
    // before any element
    Start,
}

struct Cursor {
    position: CursorPosition,
}

impl Cursor {
    fn new() -> Self {
        Self {
            position: CursorPosition::Empty,
        }
    }

    fn move_by(&mut self, e: Option<&Erasable>, direction: Sign) {
        let char_increment = match e {
            Some(e) => e.length_in_chars(),
            None => 1,
        };

        match &self.position {
            CursorPosition::Empty | CursorPosition::Start => {
                self.position = CursorPosition::NotEmpty {
                    position_in_chars: char_increment - 1,
                    position_in_erasable_count: 0,
                };
            }
            CursorPosition::NotEmpty {
                position_in_chars,
                position_in_erasable_count,
            } => {
                self.position = {
                    match direction {
                        Sign::Positive => CursorPosition::NotEmpty {
                            position_in_chars: position_in_chars + char_increment,
                            position_in_erasable_count: position_in_erasable_count + 1,
                        },
                        Sign::Negative => {
                            if (*position_in_erasable_count) == 0 {
                                CursorPosition::Start
                            } else {
                                CursorPosition::NotEmpty {
                                    position_in_chars: position_in_chars - char_increment,
                                    position_in_erasable_count: position_in_erasable_count - 1,
                                }
                            }
                        }
                    }
                };
            }
        }
    }
}

pub enum CursorPositionUnit {
    Chars,
    ErasableCount,
}

/// IMPORTANT: The cursor's position is after the element it refers to.
impl ErasableCluster {
    /// Initializes an ErasableCluster with defaults: The vector of erasables is
    /// empty and the cursor positions are None.
    ///  
    pub fn new() -> Self {
        Self {
            erasables: Vec::new(),
            cursor: Cursor::new(),
        }
    }

    /// Builds a new cluster from the string input. Each string character
    /// corresponds to an erasable. Each erasable has a char code, as
    /// in the Erasable::build function.
    ///
    /// The cursor is automatically moved to the next element to add in
    /// the vector of erasables (but the index is never out of bounds).
    ///
    pub fn build(s: &str) -> Result<Self, ParsingError> {
        if s.is_empty() {
            return Err(ParsingError::CannotParseEmptyString);
        }

        let mut position_in_chars: usize = 0;
        let erasables: Result<Vec<Erasable>, ParsingError> = s
            .chars()
            .map(|c| {
                let erasable = Erasable::build(c);

                if let Ok(erasable) = erasable {
                    position_in_chars += erasable.length_in_chars();
                    Ok(erasable)
                } else {
                    erasable
                }
            })
            .collect();

        match erasables {
            Ok(erasables) => {
                let cursor = Cursor {
                    position: CursorPosition::NotEmpty {
                        position_in_chars: position_in_chars - 1,
                        position_in_erasable_count: erasables.len() - 1,
                    },
                };

                Ok(Self { erasables, cursor })
            }
            Err(e) => Err(e),
        }
    }

    /// Gets the current position of the cursor, in terms of the number of
    /// characters in the expression or the number of erasables in the expression.
    ///
    /// The cursor position will have a value ranging from 0 to the number of
    /// characters/erasables. This means it either points to an element in the
    /// list of erasables or to the last item.
    ///
    /// Upon building a new ErasableCluster, the cursor position is automatically
    /// moved to the last element.
    ///
    /// Note: This returns None also when the cursor is at the start, even if there are
    /// erasables.
    ///
    pub fn get_cursor_position(&self, unit: CursorPositionUnit) -> Option<usize> {
        match &(self.cursor.position) {
            CursorPosition::Empty | CursorPosition::Start => None,
            CursorPosition::NotEmpty {
                position_in_chars,
                position_in_erasable_count,
            } => match unit {
                CursorPositionUnit::Chars => Some(*position_in_chars),
                CursorPositionUnit::ErasableCount => Some(*position_in_erasable_count),
            },
        }
    }

    /// Attempts to move the cursor to the next erasable.
    pub fn move_cursor_to_next_erasable(&mut self) -> Result<(), MovementError> {
        if self.is_cursor_at_end() {
            return Err(MovementError::NoNextElement);
        }

        match &(self.cursor.position) {
            CursorPosition::Empty => Err(MovementError::NoNextElement),
            CursorPosition::NotEmpty {
                position_in_erasable_count,
                ..
            } => {
                let index = position_in_erasable_count + 1;
                let e = self.erasables.get(index);
                self.cursor.move_by(e, Sign::Positive);

                Ok(())
            }
            CursorPosition::Start => {
                let e = self.erasables.get(0);
                self.cursor.move_by(e, Sign::Positive);

                Ok(())
            }
        }
    }

    /// Attempts to move the cursor to the previous erasable.
    pub fn move_cursor_to_prev_erasable(&mut self) -> Result<(), MovementError> {
        match &(self.cursor.position) {
            CursorPosition::Empty | CursorPosition::Start => Err(MovementError::NoPrevElement),
            CursorPosition::NotEmpty {
                position_in_erasable_count,
                ..
            } => {
                // since the cursor points to the value after the referenced one,
                // when moving backwards, this will work
                let index = position_in_erasable_count;

                let e = self.erasables.get(*index);
                self.cursor.move_by(e, Sign::Negative);

                Ok(())
            }
        }
    }

    fn is_cursor_at_end(&self) -> bool {
        if let CursorPosition::NotEmpty {
            position_in_erasable_count,
            ..
        } = &(self.cursor.position)
        {
            *position_in_erasable_count == self.erasables.len() - 1
        } else {
            false
        }
    }

    /// Attempts to parse c into an erasable and adds it to the vector after
    /// the element pointed to by the cursor.
    ///
    /// It also updates the cursor to look at the element that has just been added.
    pub fn add_at_cursor_position(&mut self, c: char) -> Result<(), ParsingError> {
        let e = Erasable::build(c)?;

        match &(self.cursor.position) {
            CursorPosition::Empty | CursorPosition::Start => {
                self.cursor.move_by(Some(&e), Sign::Positive);
                self.erasables.insert(0, e);
            }
            CursorPosition::NotEmpty {
                position_in_erasable_count,
                ..
            } => {
                let index = position_in_erasable_count + 1;
                self.cursor.move_by(Some(&e), Sign::Positive);
                self.erasables.insert(index, e);
            }
        }

        Ok(())
    }

    pub fn remove_at_cursor_position(&mut self) -> Result<(), RemovalError> {
        match &(self.cursor.position) {
            CursorPosition::Empty | CursorPosition::Start => Err(RemovalError),
            CursorPosition::NotEmpty {
                position_in_erasable_count,
                ..
            } => {
                let index = position_in_erasable_count;
                let e = self.erasables.remove(*index);
                self.cursor.move_by(Some(&e), Sign::Negative);

                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_into_erasables() {
        let cluster = ErasableCluster::build("120 + 23").unwrap();
        let expected = vec![
            Erasable::One,
            Erasable::Two,
            Erasable::Zero,
            Erasable::Space,
            Erasable::PlusSign,
            Erasable::Space,
            Erasable::Two,
            Erasable::Three,
        ];

        assert_eq!(cluster.erasables.len(), expected.len());

        for i in 0..expected.len() {
            assert_eq!(cluster.erasables[i], expected[i]);
        }
    }

    #[test]
    fn getting_the_cursor_works() {
        let cluster = ErasableCluster::build("120 + 23c(30)").unwrap();

        let pos_in_chars = cluster
            .get_cursor_position(CursorPositionUnit::Chars)
            .unwrap();
        let pos_in_e_count = cluster
            .get_cursor_position(CursorPositionUnit::ErasableCount)
            .unwrap();

        assert_eq!(pos_in_chars, "120 + 23cos(30)".len() - 1);
        assert_eq!(pos_in_e_count, "120 + 23c(30)".len() - 1);
    }

    #[test]
    #[should_panic]
    fn moving_the_cursor_past_the_last_doesnt_work() {
        let mut cluster = ErasableCluster::build("1 + 1").unwrap();
        cluster.move_cursor_to_next_erasable().unwrap();
    }

    #[test]
    #[should_panic]
    fn moving_the_cursor_before_the_first_doesnt_work() {
        let mut cluster = ErasableCluster::build("1 + 1").unwrap();

        loop {
            cluster.move_cursor_to_prev_erasable().unwrap();
        }
    }

    #[test]
    fn moving_the_cursor_to_prev_works() {
        let mut cluster = ErasableCluster::build("1 + 1").unwrap();

        cluster.move_cursor_to_prev_erasable().unwrap();
        cluster.move_cursor_to_prev_erasable().unwrap();

        let pos_in_chars = cluster
            .get_cursor_position(CursorPositionUnit::Chars)
            .unwrap();
        let pos_in_e_count = cluster
            .get_cursor_position(CursorPositionUnit::ErasableCount)
            .unwrap();

        assert_eq!(pos_in_chars, pos_in_e_count);
        assert_eq!(pos_in_chars, 2);
    }

    #[test]
    fn moving_the_cursor_to_next_works() {
        let mut cluster = ErasableCluster::build("1 + 1").unwrap();

        cluster.move_cursor_to_prev_erasable().unwrap();
        cluster.move_cursor_to_prev_erasable().unwrap();

        cluster.move_cursor_to_next_erasable().unwrap();

        let pos_in_chars = cluster
            .get_cursor_position(CursorPositionUnit::Chars)
            .unwrap();
        let pos_in_e_count = cluster
            .get_cursor_position(CursorPositionUnit::ErasableCount)
            .unwrap();

        assert_eq!(pos_in_chars, pos_in_e_count);
        assert_eq!(pos_in_chars, 3);
    }

    #[test]
    fn cursor_convenience_methods() {
        let cluster = ErasableCluster::build("1 + 1").unwrap();
        assert!(cluster.is_cursor_at_end());
    }

    #[test]
    fn adding_to_a_cluster_at_cursor_position() {
        let mut cluster = ErasableCluster::new();
        cluster.add_at_cursor_position('1').unwrap();
        cluster.add_at_cursor_position('+').unwrap();
        cluster.add_at_cursor_position('1').unwrap();
        cluster.add_at_cursor_position('1').unwrap();

        let pos = cluster
            .get_cursor_position(CursorPositionUnit::ErasableCount)
            .unwrap();

        assert_eq!(pos, 3);
    }

    #[test]
    fn removing_from_a_cluster_at_cursor_position() {
        let mut cluster = ErasableCluster::build("1 + 1").unwrap();
        cluster.move_cursor_to_prev_erasable().unwrap();

        cluster.remove_at_cursor_position().unwrap();
        cluster.remove_at_cursor_position().unwrap();
        cluster.remove_at_cursor_position().unwrap();

        assert_eq!(vec![Erasable::One, Erasable::One], cluster.erasables);

        let pos = cluster
            .get_cursor_position(CursorPositionUnit::ErasableCount)
            .unwrap();

        assert_eq!(pos, 0);
    }
}
