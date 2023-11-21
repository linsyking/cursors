use crate::common::{Cursor, CursorData, Pos};

impl Cursor {
    /// Create a new Cursor set
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    /// Add a new cursor
    pub fn add_cursor(&mut self, pos: Pos) {
        self.data.push(CursorData { pos });
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}
