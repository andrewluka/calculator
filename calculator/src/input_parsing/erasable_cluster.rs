use std::slice::Iter;

use super::erasable::Erasable;
use crate::{
    display::DisplayUnit,
    shared::{
        errors::{MutationOperationError, ParsingError},
        sign::Sign,
    },
};

pub struct ErasableCluster {
    erasables: Vec<Erasable>,
    cursor: Cursor,
    display_cache: Vec<DisplayUnit>,
}

enum CursorPosition {
    Empty,
    NotEmpty(usize),
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

    fn move_toward(&mut self, direction: Sign) {
        match &self.position {
            CursorPosition::Empty | CursorPosition::Start => {
                if let Sign::Positive = direction {
                    self.position = CursorPosition::NotEmpty(0);
                }
            }
            CursorPosition::NotEmpty(position) => {
                self.position = {
                    match direction {
                        Sign::Positive => CursorPosition::NotEmpty(position + 1),
                        Sign::Negative => {
                            if (*position) == 0 {
                                CursorPosition::Start
                            } else {
                                CursorPosition::NotEmpty(position - 1)
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
    DisplaySegmentChars,
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
            display_cache: Vec::new(),
        }
    }

    fn refresh_display_cache(&mut self) {
        self.display_cache.clear();

        // let char_count = 0;

        // while let Some(erasable) = self.erasables.iter().next() {
        //     let mut segment = DisplaySegment::new(
        //         Placement {
        //             char_placement: 0,
        //             line_placement: 0,
        //         },
        //         "".to_string(),
        //     );

        //     let erasable_type = ErasableType::from(erasable);

        //     match erasable_type {
        //         ErasableType::OpeningBracket => {
        //             let segment = if segment.is_empty() {
        //                 segment
        //             } else {
        //                 self.display_cache
        //                     .push(DisplayUnit::DisplaySegment(segment));
        //                 DisplaySegment::new(placement, content)
        //             };
        //         }
        //     }
        // }

        let mut content = String::with_capacity(self.erasables.len());

        for e in self.erasables.iter() {
            if e == &Erasable::FractionDivider {
                let bracket_depth = 0;
                // let start
            } else {
                content.push_str(e.into());
            }
        }

        //
        //
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
                    position: CursorPosition::NotEmpty(erasables.len() - 1),
                };

                let mut result = Self {
                    erasables,
                    cursor,
                    display_cache: Vec::new(),
                };

                result.refresh_display_cache();

                Ok(result)
            }
            Err(e) => Err(e),
        }
    }

    // /// Gets the current position of the cursor, in terms of the number of
    // /// characters in the expression or the number of erasables in the expression.
    // ///
    // /// The cursor position will have a value ranging from 0 to the number of
    // /// characters/erasables. This means it either points to an element in the
    // /// list of erasables or to the last item.
    // ///
    // /// Upon building a new ErasableCluster, the cursor position is automatically
    // /// moved to the last element.
    // ///
    // /// Note: This returns None also when the cursor is at the start, even if there are
    // /// erasables.
    // ///
    // pub fn get_cursor_position(&self, unit: CursorPositionUnit) -> Option<usize> {
    //     match &(self.cursor.position) {
    //         CursorPosition::Empty | CursorPosition::Start => None,
    //         CursorPosition::NotEmpty(position) => match unit {
    //             CursorPositionUnit::ErasableCount => Some(*position),
    //             _ => todo!(),
    //         },
    //     }
    // }

    /// Attempts to move the cursor to the next erasable.
    pub fn move_cursor_to_next_erasable(&mut self) -> Option<&Erasable> {
        if self.is_cursor_at_end() {
            return None;
        }

        match &(self.cursor.position) {
            CursorPosition::Empty => None,
            CursorPosition::NotEmpty(position) => {
                let index = position + 1;
                let e = self.erasables.get(index);
                self.cursor.move_toward(Sign::Positive);

                e
            }
            CursorPosition::Start => {
                let e = self.erasables.get(0);
                self.cursor.move_toward(Sign::Positive);

                e
            }
        }
    }

    /// Attempts to move the cursor to the previous erasable, and returns the erasable
    /// that has been moved from (if successful).
    pub fn move_cursor_to_prev_erasable(&mut self) -> Option<&Erasable> {
        match &(self.cursor.position) {
            CursorPosition::Empty | CursorPosition::Start => None,
            CursorPosition::NotEmpty(position) => {
                // since the cursor points to the value after the referenced one,
                // when moving backwards, this will work
                let index = position;

                let e = self.erasables.get(*index);
                self.cursor.move_toward(Sign::Negative);
                e
            }
        }
    }

    fn is_cursor_at_end(&self) -> bool {
        if let CursorPosition::NotEmpty(position) = &(self.cursor.position) {
            if self.erasables.len() == 0 {
                false
            } else {
                *position == self.erasables.len() - 1
            }
        } else {
            false
        }
    }

    /// Attempts to parse `c` into an erasable and adds it to the vector after
    /// the element pointed to by the cursor.
    ///
    /// It also updates the cursor to look at the element that has just been added.
    pub fn add_at_cursor_position(&mut self, c: char) -> Result<&Erasable, ParsingError> {
        let e = Erasable::build(c)?;

        match &(self.cursor.position) {
            CursorPosition::Empty | CursorPosition::Start => {
                let index = 0;
                self.cursor.move_toward(Sign::Positive);
                self.erasables.insert(index, e);
                Ok(&self.erasables[index])
            }
            CursorPosition::NotEmpty(position) => {
                let index = if let None = self.erasables.get(*position) {
                    *position
                } else {
                    (*position) + 1
                };

                self.cursor.move_toward(Sign::Positive);
                self.erasables.insert(index, e);
                Ok(&self.erasables[index])
            }
        }
    }

    /// Removes the element the cursor points to. If there are no erasables
    /// or if the cursor is at the start, an error is returned.
    pub fn remove_at_cursor_position(&mut self) -> Result<Erasable, MutationOperationError> {
        match &(self.cursor.position) {
            CursorPosition::Empty | CursorPosition::Start => {
                Err(MutationOperationError::RemovalError)
            }
            CursorPosition::NotEmpty(position) => {
                if self.erasables.len() == 0 {
                    return Err(MutationOperationError::RemovalError);
                }

                let index = position;
                let e = self.erasables.remove(*index);
                self.cursor.move_toward(Sign::Negative);

                Ok(e)
            }
        }
    }

    pub fn iter(&self) -> Iter<Erasable> {
        self.erasables.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.erasables.is_empty()
    }
}

// impl IntoIterator for ErasableCluster {
//     type Item = Erasable;
//     type IntoIter = std::vec::IntoIter<Erasable>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.erasables.into_iter()
//     }
// }

impl ToString for ErasableCluster {
    fn to_string(&self) -> String {
        let mut result = String::with_capacity(self.erasables.len());
        for e in &self.erasables {
            result.push_str(e.into())
        }

        result
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

        let pos = cluster.cursor.position;

        match pos {
            CursorPosition::NotEmpty(pos) => assert_eq!(pos, 2),
            _ => panic!(),
        }
    }

    #[test]
    fn moving_the_cursor_to_next_works() {
        let mut cluster = ErasableCluster::build("1 + 1").unwrap();

        cluster.move_cursor_to_prev_erasable().unwrap();
        cluster.move_cursor_to_prev_erasable().unwrap();

        cluster.move_cursor_to_next_erasable().unwrap();

        let pos = cluster.cursor.position;

        match pos {
            CursorPosition::NotEmpty(pos) => assert_eq!(pos, 3),
            _ => panic!(),
        }
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

        let pos = cluster.cursor.position;

        match pos {
            CursorPosition::NotEmpty(pos) => assert_eq!(pos, 3),
            _ => panic!(),
        }
    }

    #[test]
    fn removing_from_a_cluster_at_cursor_position() {
        let mut cluster = ErasableCluster::build("1 + 1").unwrap();
        cluster.move_cursor_to_prev_erasable().unwrap();

        cluster.remove_at_cursor_position().unwrap();
        cluster.remove_at_cursor_position().unwrap();
        cluster.remove_at_cursor_position().unwrap();

        // assert_eq!(vec![Erasable::One, Erasable::One], cluster.erasables);

        let pos = cluster.cursor.position;

        match pos {
            CursorPosition::NotEmpty(pos) => assert_eq!(pos, 0),
            _ => panic!(),
        }
    }

    #[test]
    fn displaying_a_cluster_works() {
        let cluster = ErasableCluster::build("s(30d)").unwrap();
        assert_eq!(cluster.to_string(), "sin(30°)");
    }
}
