use super::{range::Range, range_divider::RangeDivider, DisplayUnit, Placement};

pub struct DisplayBlock {
    // aka HashMap<(initial relative line placement), DisplayUnit>
    units: RangeDivider<isize, DisplayUnit>,
    placement: Placement,
    relative_line_placement: Range<isize>,
}

impl DisplayBlock {
    pub fn new(placement: Placement) -> Self {
        DisplayBlock {
            units: RangeDivider::new(),
            placement,
            relative_line_placement: Range::new(0, 0),
        }
    }

    pub fn min_line_placement(&self) -> isize {
        self.relative_line_placement.get_min().abs()
    }

    pub fn max_line_placement(&self) -> isize {
        self.relative_line_placement.get_max().abs()
    }

    pub fn add_unit_at(
        &mut self,
        line_placement: isize,
        unit: DisplayUnit,
    ) -> Result<(), crate::shared::errors::MutationOperationError> {
        let r = match &unit {
            DisplayUnit::DisplayBlock(b) => Range::new(
                line_placement - b.min_line_placement(),
                line_placement + b.max_line_placement(),
            ),
            DisplayUnit::DisplaySegment(_) => Range::new(line_placement, line_placement),
        };

        let r_min = *r.get_min();
        let r_max = *r.get_max();

        if let Err(e) = self.units.insert(r, unit) {
            return Err(e);
        }

        if r_max > *self.relative_line_placement.get_max() {
            self.relative_line_placement.set_max(r_max);
        }

        if r_min < *self.relative_line_placement.get_min() {
            self.relative_line_placement.set_min(r_min);
        }

        Ok(())
    }

    pub fn get_unit_at(&self, line_placement: isize) -> Option<&DisplayUnit> {
        match self.units.get(line_placement) {
            Some(entry) => Some(entry.content()),
            None => None,
        }
    }
}

impl ToString for DisplayBlock {
    fn to_string(&self) -> String {
        let mut result = String::new();

        for (_, unit) in self.units.sorted() {
            result.push_str(&unit.to_string());
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::display::display_segment::DisplaySegment;

    #[test]
    fn display_block_works() {
        let mut block = DisplayBlock::new(Placement {
            line_placement: 0,
            char_placement: 0,
        });

        block
            .add_unit_at(
                0,
                DisplayUnit::DisplaySegment(DisplaySegment::new(
                    Placement {
                        line_placement: 0,
                        char_placement: 0,
                    },
                    "hello".to_string(),
                )),
            )
            .unwrap();

        let segment = block.get_unit_at(0).unwrap();

        match segment {
            DisplayUnit::DisplaySegment(segment) => assert_eq!(segment.to_string(), "hello"),
            _ => panic!("expected display segment"),
        }

        let mut child_block = DisplayBlock::new(Placement {
            line_placement: 0,
            char_placement: 0,
        });

        child_block
            .add_unit_at(
                0,
                DisplayUnit::DisplaySegment(DisplaySegment::new(
                    Placement {
                        line_placement: 0,
                        char_placement: 0,
                    },
                    "YOU TOO?".to_string(),
                )),
            )
            .unwrap();

        child_block
            .add_unit_at(
                1,
                DisplayUnit::DisplaySegment(DisplaySegment::new(
                    Placement {
                        line_placement: 0,
                        char_placement: 0,
                    },
                    "YES ME TOO".to_string(),
                )),
            )
            .unwrap();

        let stringified_child_block = child_block.to_string();

        block
            .add_unit_at(1, DisplayUnit::DisplayBlock(child_block))
            .unwrap();

        let should_be_child_block = match block.get_unit_at(2).unwrap() {
            DisplayUnit::DisplayBlock(b) => b,
            DisplayUnit::DisplaySegment(_) => {
                panic!("should've been a display block, not a segment")
            }
        };

        assert_eq!(should_be_child_block.to_string(), stringified_child_block);
    }
}
