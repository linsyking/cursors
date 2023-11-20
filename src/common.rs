use std::{cell::RefCell, rc::Rc};

type Pos = usize;

#[derive(Debug, Clone)]
pub struct Doc {
    pub content: String,
    pub cursors: Vec<Cursor>,
    pub selections: Vec<Selection>
}

#[derive(Debug, Clone)]
pub struct Cursor {
    pub doc: Rc<RefCell<Doc>>,
    pub pos: Pos
}

#[derive(Debug, Clone)]
pub struct Selection {
    pub doc: Rc<RefCell<Doc>>,
    pub l: Pos,
    pub r: Pos,
    pub cur: Rc<RefCell<Cursor>>,
}
