use std::cell::{Ref, RefMut};

use crate::{
    common::{Cursor, CursorData, CursorMode, CursorMove, Doc, DocData, Pos, StringRef},
    pos::validate,
};

impl Doc<CursorMode> {
    /// Add a new cursor
    pub fn add_cursor(&self, pos: Pos) -> &Self {
        self.data.borrow_mut().cursors.add_cursor(pos);
        self
    }

    fn data_ref(&self) -> Ref<DocData> {
        self.data.borrow()
    }

    fn data_mutref(&self) -> RefMut<DocData> {
        self.data.borrow_mut()
    }

    pub fn move_it(&self, mov: CursorMove) -> &Self {
        let mut data = self.data_mutref();
        let content = &data.content.clone();
        data.cursors.move_it(content, mov);
        self
    }

    pub fn insert(&self, string: &str) -> &Self {
        let doc = &self.data_ref().content.clone();
        self.data_mutref().cursors.insert(doc, string);
        self
    }

    pub fn len(&self) -> usize {
        self.data_ref().cursors.len()
    }

    pub fn clear(&self) -> &Self {
        self.data_mutref().cursors.clear();
        self
    }
}

impl Cursor {
    /// Create a new Cursor set
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Add a new cursor
    pub fn add_cursor(&mut self, pos: Pos) {
        self.data.push(CursorData::from(pos));
    }

    pub fn move_it(&mut self, doc: &StringRef, mov: CursorMove) {
        for cur in self.data.iter_mut() {
            cur.move_it(doc, mov);
        }
    }

    pub fn find_forward(&mut self, doc: &StringRef, pat: &str) {
        for cur in self.data.iter_mut() {
            let res = cur.find_forward(doc, pat);
            cur.apply_changes(res);
        }
    }

    pub fn find_forward_more(&mut self, doc: &StringRef, pat: &str) {
        for cur in self.data.iter_mut() {
            let res = cur.find_forward_more(doc, pat);
            cur.apply_changes(res);
        }
    }

    pub fn find_backward(&mut self, doc: &StringRef, pat: &str) {
        for cur in self.data.iter_mut() {
            let res = cur.find_backward(doc, pat);
            cur.apply_changes(res);
        }
    }

    pub fn find_backward_more(&mut self, doc: &StringRef, pat: &str) {
        for cur in self.data.iter_mut() {
            let res = cur.find_backward_more(doc, pat);
            cur.apply_changes(res);
        }
    }

    /// Sort cursors and remove duplicates
    pub fn refresh(&mut self) {
        self.data.sort();
        self.data.dedup();
    }

    pub fn insert(&mut self, doc: &StringRef, string: &str) {
        self.refresh();
        let mut offset = 0;
        for cur in self.data.iter_mut() {
            cur.add(offset);
            cur.insert(doc, string);
            offset += string.len();
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

    pub fn move_it(&mut self, doc: &StringRef, mov: CursorMove) {
        match mov {
            CursorMove::StartOfString => self.pos = 0,
            CursorMove::EndOfString => self.pos = doc.borrow().len(),
            CursorMove::CharForward(l) => {
                let new_pos = self.pos + l;
                validate(new_pos, doc);
                self.pos = new_pos;
            }
            CursorMove::CharBackward(l) => {
                let new_pos = self.pos - l;
                validate(new_pos, doc);
                self.pos = new_pos;
            }
            CursorMove::EndOfLine => {
                if let Some(pos) = self.find_forward(doc, "\n") {
                    self.pos = pos;
                } else {
                    // Not found, move to the end of doc
                    self.pos = doc.borrow().len();
                }
            }
            CursorMove::StartOfLine => {
                if let Some(pos) = self.find_backward(doc, "\n") {
                    self.pos = pos;
                } else {
                    // Not found, move to the start of doc
                    self.pos = 0;
                }
            }
            _ => todo!(),
        }
    }

    pub fn apply_changes(&mut self, res: Option<Pos>) {
        if let Some(pos) = res {
            self.pos = pos;
        }
    }

    pub fn find_forward(&mut self, doc: &StringRef, pat: &str) -> Option<Pos> {
        let doc = doc.borrow();
        let pos = doc[self.pos..].find(pat)?;
        Some(self.pos + pos)
    }

    pub fn find_forward_more(&mut self, doc: &StringRef, pat: &str) -> Option<Pos> {
        let doc = doc.borrow();
        let pos = doc[self.pos..].find(pat)?;
        Some(self.pos + pos + pat.len())
    }

    pub fn find_backward(&mut self, doc: &StringRef, pat: &str) -> Option<Pos> {
        let doc = doc.borrow();
        let pos = doc[..self.pos].find(pat)?;
        Some(pos + pat.len())
    }

    pub fn find_backward_more(&mut self, doc: &StringRef, pat: &str) -> Option<Pos> {
        let doc = doc.borrow();
        let pos = doc[..self.pos].rfind(pat)?;
        Some(pos)
    }

    pub fn insert(&mut self, doc: &StringRef, string: &str) {
        self.insert_without_move(doc, string);
        self.pos += string.len();
    }

    pub fn insert_without_move(&self, doc: &StringRef, string: &str) {
        doc.borrow_mut().insert_str(self.pos, string);
    }

    pub fn add(&mut self, num: Pos) {
        self.pos += num;
    }
}
