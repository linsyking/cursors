use std::{cell::RefCell, rc::Rc};

type Pos = usize;

#[derive(Debug, Clone)]
pub struct Doc {
    content: Rc<RefCell<String>>,
    cursors: Cursor,
    selections: Selection,
}

#[derive(Debug, Clone)]
pub struct Cursor {
    content: Rc<RefCell<String>>,
    data: Vec<CursorData>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CursorData {
    pos: Pos,
}

#[derive(Debug, Clone)]
pub struct Selection {
    content: Rc<RefCell<String>>,
    data: Vec<SelectionData>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SelectionData {
    l: Pos,
    r: Pos,
    cur: CursorData,
}

impl Cursor {
    pub fn from(content: Rc<RefCell<String>>) -> Self {
        Self {
            content,
            data: Vec::new(),
        }
    }
}

impl Selection {
    pub fn from(content: Rc<RefCell<String>>) -> Self {
        Self {
            content,
            data: Vec::new(),
        }
    }
}

impl Doc {
    pub fn from(s: String) -> Self {
        let s_ref = Rc::new(RefCell::new(s));
        Self {
            content: s_ref.clone(),
            cursors: Cursor::from(s_ref.clone()),
            selections: Selection::from(s_ref),
        }
    }
}
