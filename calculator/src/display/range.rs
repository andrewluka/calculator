use std::{hash::Hash, ops::Sub};

#[derive(Hash, PartialEq, Eq, Ord)]
pub struct Range {
    // inclusive
    min: isize,
    // inclusive
    max: isize,
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.overlaps(other) {
            panic!("cannot compare ranges that overlap");
        }

        if self.max < other.min {
            Some(std::cmp::Ordering::Less)
        } else if self.min > other.max {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

impl Range {
    pub fn new(min: isize, max: isize) -> Self {
        assert!(max >= min);

        Self { min, max }
    }

    pub fn overlaps(&self, r: &Range) -> bool {
        (r.min >= self.min && r.min <= self.max) || (r.max >= self.min && r.max <= self.max)
    }

    pub fn contains(&self, k: isize) -> bool {
        k >= self.min && k <= self.max
    }

    pub fn get_min(&self) -> isize {
        self.min
    }

    pub fn get_max(&self) -> isize {
        self.max
    }

    pub fn set_min(&mut self, val: isize) {
        assert!(val <= self.max);
        self.min = val;
    }

    pub fn set_max(&mut self, val: isize) {
        assert!(val >= self.min);
        self.max = val;
    }

    pub fn magnitude(&self) -> isize {
        (self.min - self.max).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_overlap_works() {
        let r1 = Range::new(0, 3);
        let r2 = Range::new(0, 3);

        assert!(r1.overlaps(&r2));

        let r3 = Range::new(-1, 2);

        assert!(r3.overlaps(&r1));

        let r4 = Range::new(-5, -2);

        assert!(!r4.overlaps(&r1));
        assert!(!r4.overlaps(&r3));
    }

    #[test]
    fn range_contains_value_works() {
        let r = Range::new(2, 7);

        assert!(r.contains(2));
        assert!(r.contains(7));
        assert!(r.contains(5));
        assert!(!r.contains(1));
        assert!(!r.contains(9));
    }

    #[test]
    #[should_panic]
    fn panic_if_range_min_greater_than_max() {
        Range::new(0, -1);
    }
}
