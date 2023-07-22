use crate::{FileType, Position, Row, SearchDirection};
use std::{
    fs,
    io::{self, Write},
    path::Path,
};

#[derive(Default, Debug)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
    dirty: bool,
    file_type: FileType,
}

impl Document {
    pub fn open(path: &str) -> Result<Self, std::io::Error> {
        let path = Path::new(path);
        let file_name = if let Some(s) = path.to_str() {
            Some(s.to_string())
        } else {
            None
        };

        let content = fs::read_to_string(path)?;
        let file_type = FileType::from(path.file_name().unwrap().to_str().unwrap());
        let mut rows = Vec::new();
        for value in content.lines() {
            let mut row = Row::from(value);
            row.highlight(file_type.highlighting_options(), None);
            rows.push(row);
        }
        let rows: Vec<Row> = content.lines().map(Row::from).collect();
        Ok(Self {
            rows,
            file_name,
            dirty: false,
            file_type,
        })
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn insert(&mut self, at: &Position, c: char) {
        if at.y > self.len() {
            return;
        }
        self.dirty = true;
        if c == '\n' {
            self.insert_newline(at);
            return;
        }
        if at.y == self.len() {
            let mut row = Row::default();
            row.insert(0, c);
            row.highlight(self.file_type.highlighting_options(), None);
            self.rows.push(row);
        } else {
            self.rows[at.y].insert(at.x, c);
            self.rows[at.y].highlight(self.file_type.highlighting_options(), None);
        }
    }

    fn insert_newline(&mut self, at: &Position) {
        if at.y > self.len() {
            return;
        }
        if at.y >= self.len() {
            self.rows.push(Row::default());
            return;
        }
        let current_row = &mut self.rows[at.y];
        let mut new_row = current_row.split(at.x);
        current_row.highlight(self.file_type.highlighting_options(), None);
        new_row.highlight(self.file_type.highlighting_options(), None);
        self.rows.insert(at.y + 1, new_row);
    }

    #[allow(clippy::integer_arithmetic, clippy::indexing_slicing)]
    pub fn delete(&mut self, at: &Position) {
        let len = self.len();
        if at.y >= len {
            return;
        }

        self.dirty = true;
        if at.x == self.rows[at.y].len() && at.y + 1 < len {
            let next_row = self.rows.remove(at.y + 1);
            self.rows[at.y].append(&next_row);
            self.rows[at.y].highlight(self.file_type.highlighting_options(), None);
        } else {
            self.rows[at.y].delete(at.x);
            self.rows[at.y].highlight(self.file_type.highlighting_options(), None);
        }
    }

    pub fn save(&mut self) -> Result<(), io::Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(file_name)?;
            self.file_type = FileType::from(file_name);
            for row in &mut self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
                row.highlight(self.file_type.highlighting_options(), None);
            }
            self.dirty = false;
        }
        Ok(())
    }

    pub fn find(&self, query: &str, at: &Position, direction: SearchDirection) -> Option<Position> {
        if at.y >= self.rows.len() {
            return None;
        }
        let mut position = at.clone();
        let (start, end) = if direction == SearchDirection::Forward {
            (at.y, self.rows.len())
        } else {
            (0, at.y.saturating_add(1))
        };

        for _ in start..end {
            if let Some(row) = self.rows.get(position.y) {
                if let Some(x) = row.find(&query, position.x, direction) {
                    position.x = x;
                    return Some(position);
                }
                if direction == SearchDirection::Forward {
                    position.y = position.y.saturating_add(1);
                    position.x = 0;
                } else {
                    position = Position {
                        x: self.rows[position.y].len(),
                        y: position.y.saturating_sub(1),
                    };
                }
            } else {
                return None;
            }
        }
        None
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn highlight(&mut self, word: Option<&str>) {
        for row in &mut self.rows {
            row.highlight(self.file_type.highlighting_options(), word);
        }
    }

    pub fn file_type(&self) -> String {
        self.file_type.name()
    }
}
