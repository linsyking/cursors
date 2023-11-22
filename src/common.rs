use std::{cell::RefCell, marker::PhantomData, rc::Rc};

pub type Pos = usize;
pub type DocRef = Rc<RefCell<DocData>>;

pub struct DocMode;

pub struct CursorMode;

pub struct SelectionMode;

#[derive(Debug, Clone)]
pub struct Doc<T> {
    pub data: DocRef,
    phantom: PhantomData<T>,
}

impl Doc<DocMode> {
    pub fn from(s: String) -> Self {
        Doc {
            data: Rc::new(RefCell::new(DocData::from(s))),
            phantom: PhantomData,
        }
    }

    pub fn cursors(&self) -> Doc<CursorMode> {
        Doc {
            data: self.data.clone(),
            phantom: PhantomData,
        }
    }

    pub fn selections(&self) -> Doc<SelectionMode> {
        Doc {
            data: self.data.clone(),
            phantom: PhantomData,
        }
    }
}

impl<T> Doc<T> {
    /// Get a clone of the string
    pub fn content(&self) -> String {
        self.data.borrow().content()
    }
}

// impl Doc<CursorMode> {
//     pub fn doc(&self) -> Doc<DocMode> {
//         Doc {
//             data: self.data.clone(),
//             phantom: PhantomData,
//         }
//     }
// }

// impl Doc<SelectionMode> {
//     pub fn doc(&self) -> Doc<DocMode> {
//         Doc {
//             data: self.data.clone(),
//             phantom: PhantomData,
//         }
//     }
// }

#[derive(Debug, Clone)]
pub struct DocData {
    pub content: String,
    pub cursors: Cursor,
    pub selections: Selection,
}

#[derive(Debug, Clone)]
pub struct Cursor {
    pub data: Vec<CursorData>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CursorData {
    pub pos: Pos,
}

#[derive(Debug, Clone)]
pub struct Selection {
    pub data: Vec<SelectionData>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionData {
    pub l: Pos,
    pub r: Pos,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CursorMove {
    StartOfFile,
    StartOfLine,
    CharForward(usize),
    CharBackward(usize),
    WordForward(usize),
    WordBackward(usize),
    VerticalUp(usize),
    VerticalDown(usize),
    EndOfLine,
    EndOfFile,
}
