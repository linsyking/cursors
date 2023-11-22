use crate::common::{Cursor, CursorData, CursorMode, CursorMove, Doc, Pos};

impl Doc<CursorMode> {
    /// Add a new cursor
    pub fn add_cursor(&mut self, pos: Pos) {
        self.data.borrow_mut().cursors.add_cursor(pos)
    }
}

impl Cursor {
    /// Create a new Cursor set
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Add a new cursor
    pub fn add_cursor(&mut self, pos: Pos) {
        self.data.push(CursorData::from(pos));
    }

    pub fn move_it(&mut self, doc: &String, mov: CursorMove) {
        for cur in self.data.iter_mut() {
            cur.move_it(doc, mov);
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl CursorData {
    pub fn from(pos: Pos) -> Self {
        Self { pos }
    }

    pub fn move_it(&mut self, doc: &String, mov: CursorMove) {}
}
