use std::{cell::RefCell, rc::Rc};

use crate::common::{Cursor, Doc, DocData, DocMode, Selection};

impl Doc<DocMode> {
    pub fn clear(&self) -> &Self {
        self.data.borrow_mut().clear();
        self
    }
}

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

    pub fn clear(&mut self) {
        self.cursors.clear();
        self.selections.clear();
    }
}
