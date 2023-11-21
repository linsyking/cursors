use crate::common::{Cursor, Doc, DocData, DocMode, Pos, Selection};

impl Doc<DocMode> {
    /// Get a clone of the string
    pub fn content(&self) -> String {
        self.data.borrow().content()
    }
}

impl DocData {
    /// Create Doc from a string
    pub fn from(s: String) -> Self {
        Self {
            content: s,
            cursors: Cursor::new(),
            selections: Selection::new(),
        }
    }

    /// Get a clone of the string
    pub fn content(&self) -> String {
        self.content.clone()
    }

    /// Create a new cursor
    pub fn new_cursor(&mut self, pos: Pos) {
        if pos > self.content.len() {
            panic!("Cursor position must not be greater than the lengthe of the string");
        }
        self.cursors.add_cursor(pos);
    }

    pub fn clear(&mut self) {
        self.cursors.clear();
        self.selections.clear();
    }

    pub fn clear_cursors(&mut self) {
        self.cursors.clear();
        self.selections.clear();
    }

    pub fn clear_selections(&mut self) {
        self.cursors.clear();
        self.selections.clear();
    }
}
