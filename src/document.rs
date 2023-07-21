use crate::{Position, Row};
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
        let rows: Vec<Row> = content.lines().map(Row::from).collect();
        Ok(Self {
            rows,
            file_name,
            dirty: false,
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
            self.rows.push(row);
        } else {
            self.rows[at.y].insert(at.x, c);
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
        let new_row = self.rows[at.y].split(at.x);
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
        } else {
            self.rows[at.y].delete(at.x);
        }
    }

    pub fn save(&mut self) -> Result<(), io::Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(file_name)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
            self.dirty = false;
        }
        Ok(())
    }

    pub fn find(&self, query: &str) -> Option<Position> {
        self.rows
            .iter()
            .enumerate()
            .filter_map(|(idx, row)| {
                if let Some(x) = row.find(query) {
                    Some(Position { x, y: idx })
                } else {
                    None
                }
            })
            .take(1)
            .next()
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}
