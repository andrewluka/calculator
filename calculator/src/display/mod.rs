use self::display_block::DisplayBlock;
use self::display_segment::DisplaySegment;

pub mod display_block;
pub mod display_segment;
mod range;
mod range_divider;

pub enum DisplayUnit {
    DisplaySegment(DisplaySegment),
    DisplayBlock(DisplayBlock),
}

impl ToString for DisplayUnit {
    fn to_string(&self) -> String {
        match self {
            DisplayUnit::DisplayBlock(b) => b.to_string(),
            DisplayUnit::DisplaySegment(s) => s.to_string(),
        }
    }
}

pub struct Placement {
    // relative to parent
    pub line_placement: isize,
    pub char_placement: usize,
}
