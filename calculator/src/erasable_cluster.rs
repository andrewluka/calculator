use crate::{
    errors::{MovementError, ParsingError, RemovalError},
    sign::Sign,
};
use num_traits::{FromPrimitive, ToPrimitive};
use striminant_macro::striminant;
use strum_macros::{EnumIter, IntoStaticStr}; // 0.17.1

#[repr(u8)]
#[striminant]
#[derive(Debug, PartialEq, EnumIter, FromPrimitive, ToPrimitive, IntoStaticStr)]
enum Erasable {
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
    Degrees = b'd',
    Radians = b'r',
}

impl Erasable {
    // fn from_has_character_code<T: ToErasable>(t: T) -> Self {
    //     t.to_erasable()
    // }

    fn build(c: char) -> Result<Self, ParsingError> {
        match <Erasable as FromPrimitive>::from_u8(c as u8) {
            Some(e) => Ok(e),
            None => Err(ParsingError::NoSuchCharacterCode),
        }
    }

    fn length_in_chars(&self) -> usize {
        let str: &'static str = self.into();
        str.len()
    }
}

impl ToString for Erasable {
    fn to_string(&self) -> String {
        // match *self {}
        String::new()
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

// impl ToString for ErasableCluster {
//     fn to_string(&self) -> String {}
// }

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

        // assert_eq!(vec![Erasable::One, Erasable::One], cluster.erasables);

        let pos = cluster
            .get_cursor_position(CursorPositionUnit::ErasableCount)
            .unwrap();

        assert_eq!(pos, 0);
    }
}
