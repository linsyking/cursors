use crate::{
    common::{Doc, Pos, Selection, SelectionData, SelectionMode, StringRef},
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

    pub fn len(&self) -> usize {
        self.data_ref().selections.len()
    }
}

impl Selection {
    /// Create a new Selection set
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn add(&mut self, l: Pos, r: Pos) {
        self.data.push(SelectionData::from(l, r));
        self.refresh();
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn refresh(&mut self) {
        self.data.sort();
        self.data.dedup();
    }
}

impl SelectionData {
    pub fn from(l: Pos, r: Pos) -> Self {
        Self { l, r }
    }

    pub fn replace(&mut self, doc: &StringRef, rep: &str) {
        doc.borrow_mut().replace_range(self.l..self.r, rep);
    }
}
