use crate::common::{Pos, StringRef};

/// Validate if the pos is in doc
/// Panic if not
pub fn validate(pos: Pos, doc: StringRef) {
    if pos > doc.borrow().len() {
        panic!("Cursor position must not be greater than the lengthe of the string");
    }
}
