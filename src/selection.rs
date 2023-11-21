use crate::common::Selection;

impl Selection {
    /// Create a new Selection set
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub fn clear(&mut self) {
        self.data.clear();
    }
}
