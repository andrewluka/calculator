use itertools::Itertools;

use super::range::Range;
use crate::shared::errors::MutationOperationError;
use std::{collections::HashMap, hash::Hash};

pub struct RangeEntry<K: PartialOrd + Hash + Eq + Ord, V> {
    min: K,
    max: K,
    content: V,
}

impl<K: PartialOrd + Hash + Eq + Ord, V> RangeEntry<K, V> {
    pub fn new(min: K, max: K, content: V) -> Self {
        Self { min, max, content }
    }

    pub fn min(&self) -> &K {
        &self.min
    }

    pub fn max(&self) -> &K {
        &self.max
    }

    pub fn content(&self) -> &V {
        &self.content
    }
}

pub struct RangeDivider<K: PartialOrd + Hash + Eq + Ord, V> {
    boundaries: HashMap<Range<K>, V>,
    max: Option<K>,
    min: Option<K>,
}

impl<K: PartialOrd + Hash + Eq + Ord, V> RangeDivider<K, V> {
    pub fn new() -> Self {
        Self {
            min: None,
            max: None,
            boundaries: HashMap::new(),
        }
    }

    pub fn insert(&mut self, r: Range<K>, content: V) -> Result<(), MutationOperationError> {
        for (range, _) in &self.boundaries {
            if range.overlaps(&r) {
                return Err(MutationOperationError::AdditionError);
            }
        }

        self.boundaries.insert(r, content);

        Ok(())
    }

    pub fn get(&self, k: K) -> Option<RangeEntry<&K, &V>> {
        for (range, content) in &self.boundaries {
            if range.contains(&k) {
                return Some(RangeEntry {
                    min: &range.get_min(),
                    max: &range.get_max(),
                    content,
                });
            }
        }

        None
    }

    pub fn sorted(&self) -> std::vec::IntoIter<(&Range<K>, &V)> {
        self.boundaries.iter().sorted_by_key(|x| (x.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inserting_into_a_range_divider_works() {
        let mut divider = RangeDivider::<isize, String>::new();

        divider
            .insert(Range::new(-1, 1), "HEY\nI\nAM-".to_string())
            .unwrap();

        let entry = divider.get(0).unwrap();

        assert_eq!(**entry.min(), -1);
        assert_eq!(**entry.max(), 1);
        assert_eq!(*entry.content(), "HEY\nI\nAM-");
    }

    #[test]
    fn sorted_range_divider_works() {
        let mut divider = RangeDivider::new();

        divider.insert(Range::new(0, 1), "HEY");
        divider.insert(Range::new(3, 5), "LOVE");
        divider.insert(Range::new(2, 2), "I");
        divider.insert(Range::new(6, 10), "YOU");

        let phrase_of_endearment: Vec<&str> = divider.sorted().map(|(_, s)| *s).collect();
        let phrase_of_endearment = phrase_of_endearment.join(" ");

        assert_eq!(phrase_of_endearment, "HEY I LOVE YOU");
    }
}
