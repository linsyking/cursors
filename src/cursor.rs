use crate::{
    common::{Cursor, CursorData, CursorMode, CursorMove, Doc, Pos, StringRef},
    pos::validate,
};

impl Doc<CursorMode> {
    /// Add a new cursor
    pub fn add_cursor(&mut self, pos: Pos) {
        self.data.borrow_mut().cursors.add_cursor(pos)
    }

    pub fn move_it(&self, mov: CursorMove) {
        let mut data = self.data.borrow_mut();
        let content = data.content.clone();
        data.cursors.move_it(content, mov);
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

    pub fn move_it(&mut self, doc: StringRef, mov: CursorMove) {
        for cur in self.data.iter_mut() {
            cur.move_it(doc.clone(), mov);
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

    pub fn move_it(&mut self, doc: StringRef, mov: CursorMove) {
        match mov {
            CursorMove::StartOfFile => self.pos = 0,
            CursorMove::EndOfFile => self.pos = doc.borrow().len(),
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
            _ => todo!(),
        }
    }

    pub fn find_forward(&mut self, doc: &String, pat: &str) {
        let pos = doc[self.pos..].find(pat);
        if let Some(pos) = pos {
            self.pos += pos;
        }
    }

    pub fn find_forward_more(&mut self, doc: &String, pat: &str) {
        let pos = doc[self.pos..].find(pat);
        if let Some(pos) = pos {
            self.pos += pos + pat.len();
        }
    }

    pub fn find_backward(&mut self, doc: &String, pat: &str) {
        let pos = doc[..self.pos].find(pat);
        if let Some(pos) = pos {
            self.pos = pos + pat.len();
        }
    }

    pub fn find_backward_more(&mut self, doc: &String, pat: &str) {
        let pos = doc[..self.pos].rfind(pat);
        if let Some(pos) = pos {
            self.pos = pos;
        }
    }
}
