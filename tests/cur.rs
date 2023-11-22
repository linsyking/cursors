#[cfg(test)]
mod cursor_tests {
    use cursors::common::{CursorMove, Doc};

    #[test]
    fn test_add_cursor() {
        let mystr = "abc";
        let doc = Doc::from(String::from(mystr));
        assert_eq!(doc.content().as_str(), mystr);
        assert_eq!(doc.cursors().len(), 0);
        doc.cursors()
            .add_cursor(0)
            .add_cursor(2)
            .add_cursor(1)
            .add_cursor(0);
        assert_eq!(doc.cursors().len(), 4);
        doc.cursors().insert("???");
        assert_eq!(doc.cursors().len(), 3);
        assert_eq!(doc.content(), String::from("???a???b???c"));
        doc.cursors()
            .clear()
            .add_cursor(0)
            .move_it(CursorMove::EndOfLine)
            .add_cursor(0);
        assert_eq!(doc.cursors().len(), 2);
        doc.cursors().insert("!");
        assert_eq!(doc.content(), String::from("!???a???b???c!"));
    }
}
