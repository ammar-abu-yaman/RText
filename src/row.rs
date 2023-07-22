use crate::filetype::HighLightingOptions;
use crate::highlighting as hl;
use crate::SearchDirection;
use std::cmp::min;
use termion::color;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Default)]
pub struct Row {
    string: String,
    len: usize,
    highlighting: Vec<hl::Type>,
}

impl From<&str> for Row {
    fn from(s: &str) -> Self {
        let mut row = Self {
            string: String::from(s),
            len: s.graphemes(true).count(),
            highlighting: Vec::new(),
        };
        row.update_len();
        row
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let mut result = String::new();
        let mut current_hightlighting = &hl::Type::None;
        let end = min(end, self.string.len());
        let start = min(start, end);

        #[allow(clippy::integer_arithmetic)]
        for (index, grapheme) in self
            .string
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .enumerate()
        {
            if let Some(c) = grapheme.chars().next() {
                let highlighting_type = self.highlighting.get(index).unwrap_or(&hl::Type::None);
                if highlighting_type != current_hightlighting {
                    current_hightlighting = highlighting_type;
                    let start_highlighting =
                        format!("{}", termion::color::Fg(highlighting_type.to_color()));
                    result.push_str(&start_highlighting);
                }
                if c == '\t' {
                    result.push_str("    ");
                } else {
                    result.push(c);
                }
            }
        }
        let end_highlight = format!("{}", termion::color::Fg(color::Reset));
        result.push_str(&end_highlight);
        result
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
        self.string = string;
        self.len = len;
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
        self.string = string;
        self.len = len;
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
        self.string = beginning;
        self.len = beginning_len;

        Self {
            string: remainder,
            len: remainder_len,
            highlighting: Vec::new(),
        }
    }

    pub fn find(&self, query: &str, at: usize, direction: SearchDirection) -> Option<usize> {
        if at > self.len || query.is_empty() {
            return None;
        }
        let (start, end) = if direction == SearchDirection::Forward {
            (at, self.len)
        } else {
            (0, at)
        };
        let substring: String = self
            .string
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .collect();
        let matching_byte_index = if direction == SearchDirection::Forward {
            substring.find(query)
        } else {
            substring.rfind(query)
        };
        if let Some(matching_byte_index) = matching_byte_index {
            for (grapheme_index, (byte_index, _)) in substring.grapheme_indices(true).enumerate() {
                if matching_byte_index == byte_index {
                    #[allow(clippy::integer_arithmetic)]
                    return Some(start + grapheme_index);
                }
            }
        }
        None
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }

    pub fn highlight(&mut self, opts: HighLightingOptions, word: Option<&str>) {
        let mut highlighting = Vec::new();
        let chars: Vec<char> = self.string.chars().collect();
        let mut matches = Vec::new();
        let mut search_index = 0;
        if let Some(word) = word {
            while let Some(search_match) = self.find(word, search_index, SearchDirection::Forward) {
                matches.push(search_match);
                if let Some(next_index) = search_match.checked_add(word.graphemes(true).count()) {
                    search_index = next_index;
                } else {
                    break;
                }
            }
        }
        let mut prev_is_separator = true;
        let mut index = 0;
        while let Some(c) = chars.get(index) {
            if let Some(word) = word {
                if matches.contains(&index) {
                    for _ in word.graphemes(true) {
                        index += 1;
                        highlighting.push(hl::Type::Match);
                    }
                    continue;
                }
            }
            let previous_highlight = if index > 0 {
                highlighting.get(index - 1).unwrap_or(&hl::Type::None)
            } else {
                &hl::Type::None
            };
            match c {
                _ if opts.numbers()
                    && ((c.is_ascii_digit()
                        && (prev_is_separator || previous_highlight == &hl::Type::Number))
                        || (c == &'.' && previous_highlight == &hl::Type::Number)) =>
                {
                    highlighting.push(hl::Type::Number)
                }
                _ => highlighting.push(hl::Type::None),
            }
            prev_is_separator = c.is_ascii_punctuation() || c.is_ascii_whitespace();
            index += 1;
        }
        self.highlighting = highlighting;
    }
}
