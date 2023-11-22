#[cfg(test)]
mod selection_tests {
    use cursors::common::{CursorMove, Doc};

    #[test]
    fn add() {
        let doc = Doc::from(String::from("hihihi"));
        doc.selections().add_selection(0, 3); // Select "hih"
    }

    #[test]
    fn from_cursor() {
        let doc = Doc::from(String::from("hi, world"));
        assert_eq!(doc.selections().len(), 0);
        doc.cursors()
            .add_cursor(0)
            .move_select(CursorMove::EndOfLine);
        assert_eq!(doc.selections().len(), 1);
    }
}
