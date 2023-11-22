use crate::{
    common::{Doc, Pos, Selection, SelectionData, SelectionMode},
    pos::validate,
};

impl Doc<SelectionMode> {
    pub fn add_selection(&self, l: Pos, r: Pos) -> &Self {
        if l > r {
            panic!("Right might not be less than left position.");
        }
        let doc = &self.data.borrow().content.clone();
        validate(l, doc);
        validate(r, doc);
        self.data.borrow_mut().selections.add(l, r);
        self
    }

    pub fn clear(&self) -> &Self {
        self.data.borrow_mut().clear();
        self
    }
}

impl Selection {
    /// Create a new Selection set
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn add(&mut self, l: Pos, r: Pos) {
        self.data.push(SelectionData::from(l, r))
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl SelectionData {
    pub fn from(l: Pos, r: Pos) -> Self {
        Self { l, r }
    }
}
