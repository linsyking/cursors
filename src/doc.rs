use std::{cell::RefCell, rc::Rc};

use crate::{
    common::{Cursor, DocData, Pos, Selection},
    pos::validate,
};

impl DocData {
    /// Create Doc from a string
    pub fn from(s: String) -> Self {
        Self {
            content: Rc::new(RefCell::new(s)),
            cursors: Cursor::new(),
            selections: Selection::new(),
        }
    }

    /// Get a clone of the string
    pub fn content(&self) -> String {
        self.content.borrow().clone()
    }

    /// Create a new cursor
    pub fn new_cursor(&mut self, pos: Pos) {
        validate(pos, &self.content);
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
