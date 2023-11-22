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

    #[test]
    fn cursor_and_selection() {
        let doc = Doc::from(String::from("hi, world"));
        doc.cursors()
            .add_cursor(3)
            .selections()
            .add_selection(0, 2)
            .add_selection(0, 5)
            .add_selection(4, 7)
            .cursors()
            .insert("!");
        assert_eq!(doc.content().as_str(), "hi,! world")
    }
}
