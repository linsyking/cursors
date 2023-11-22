use std::{
    cell::{Ref, RefCell, RefMut},
    marker::PhantomData,
    rc::Rc,
};

pub type Pos = usize;
pub type DocRef = Rc<RefCell<DocData>>;
pub type StringRef = Rc<RefCell<String>>;

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

    pub fn selections(&self) -> Doc<SelectionMode> {
        Doc {
            data: self.data.clone(),
            phantom: PhantomData,
        }
    }

    pub fn cursors(&self) -> Doc<CursorMode> {
        Doc {
            data: self.data.clone(),
            phantom: PhantomData,
        }
    }
}

impl Doc<SelectionMode> {
    pub fn cursors(&self) -> Doc<CursorMode> {
        Doc {
            data: self.data.clone(),
            phantom: PhantomData,
        }
    }
}

impl Doc<CursorMode> {
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

    // pub fn doc(&self) -> Doc<DocMode> {
    //     Doc {
    //         data: self.data.clone(),
    //         phantom: PhantomData,
    //     }
    // }

    pub fn data_ref(&self) -> Ref<DocData> {
        self.data.borrow()
    }

    pub fn data_mutref(&self) -> RefMut<DocData> {
        self.data.borrow_mut()
    }
}

#[derive(Debug, Clone)]
pub struct DocData {
    pub content: StringRef,
    pub cursors: Cursor,
    pub selections: Selection,
}

#[derive(Debug, Clone)]
pub struct Cursor {
    pub data: Vec<CursorData>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CursorData {
    pub pos: Pos,
}

#[derive(Debug, Clone)]
pub struct Selection {
    pub data: Vec<SelectionData>,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
pub struct SelectionData {
    pub l: Pos,
    pub r: Pos,
}

impl PartialOrd for SelectionData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.r.partial_cmp(&other.r)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CursorMove {
    StartOfString,
    StartOfLine,
    CharForward(usize),
    CharBackward(usize),
    WordForward(usize),
    WordBackward(usize),
    VerticalUp(usize),
    VerticalDown(usize),
    EndOfLine,
    EndOfString,
}
