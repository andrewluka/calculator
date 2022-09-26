use super::Placement;

pub struct DisplaySegment {
    content: String,
    placement: Placement,
}

impl DisplaySegment {
    pub fn new(placement: Placement, content: String) -> Self {
        Self { placement, content }
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn push(&mut self, s: &str) {
        self.content.push_str(s);
    }

    pub fn pop(&mut self) -> Option<char> {
        self.content.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

impl std::fmt::Display for DisplaySegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.content.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appending_content_to_display_segments_works() {
        let mut segment = DisplaySegment::new(
            Placement {
                line_placement: 0,
                char_placement: 0,
            },
            "he".to_string(),
        );
        segment.push("llo");

        assert_eq!(segment.to_string(), "hello")
    }
}
