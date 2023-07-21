use std::cmp::min;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Default)]
pub struct Row {
    string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(s: &str) -> Self {
        let mut row = Self {
            string: String::from(s),
            len: s.graphemes(true).count(),
        };
        row.update_len();
        row
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = min(end, self.string.len());
        let start = min(start, end);
        #[allow(clippy::integer_arithmetic)]
        self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .map(|c| if c == "\t" { "    " } else { c })
            .collect()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn update_len(&mut self) {
        self.len = self.string[..].graphemes(true).count()
    }

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.string.push(c);
            self.len += 1;
            return;
        }

        let mut string = String::new();
        let mut len = 0;
        for (idx, grapheme) in self.string.graphemes(true).enumerate() {
            len += 1;
            if idx == at {
                len += 1;
                string.push(c);
            }
            string += grapheme;
        }
        *self = Self { string, len };
    }

    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        }

        let mut string = String::new();
        let mut len = 0;
        for (idx, grapheme) in self.string.graphemes(true).enumerate() {
            if idx == at {
                continue;
            }
            len += 1;
            string += grapheme;
        }
        *self = Self { string, len };
    }

    pub fn append(&mut self, new: &Self) {
        self.string.push_str(&new.string);
        self.update_len();
    }

    pub fn split(&mut self, at: usize) -> Self {
        let mut beginning: String = String::new();
        let mut remainder: String = String::new();
        let mut beginning_len = 0;
        let mut remainder_len = 0;
        for (idx, grapheme) in self.string.graphemes(true).enumerate() {
            if idx <= at {
                beginning += grapheme;
                beginning_len += 1;
            } else {
                remainder += grapheme;
                remainder_len += 1;
            }
        }

        *self = Self {
            string: beginning,
            len: beginning_len,
        };
        Self {
            string: remainder,
            len: remainder_len,
        }
    }

    pub fn find(&self, query: &str) -> Option<usize> {
        let matching_byte_index = self.string.find(query);
        if let Some(matching_byte_index) = matching_byte_index {
            for (grapheme_index, (byte_index, _)) in self.string.grapheme_indices(true).enumerate()
            {
                if matching_byte_index == byte_index {
                    return Some(grapheme_index);
                }
            }
        }
        None
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }
}
