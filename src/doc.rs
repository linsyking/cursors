use std::{rc::Rc, cell::RefCell};

use crate::common::{Cursor, Doc};

impl Doc {
    pub fn new_cursor(self) -> Cursor {
        Cursor {
            doc: Rc::new(RefCell::new(self)),
            pos: 0,
        }
    }
}
