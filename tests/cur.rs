#[cfg(test)]
mod cursor_tests {
    use cursors::common::{CursorMove, Doc};

    #[test]
    fn add() {
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
        assert_eq!(doc.content().as_str(), "???a???b???c");
        doc.cursors()
            .clear()
            .add_cursor(0)
            .move_it(CursorMove::EndOfLine)
            .add_cursor(0);
        assert_eq!(doc.cursors().len(), 2);
        doc.cursors().insert("!");
        assert_eq!(doc.content().as_str(), "!???a???b???c!");
    }

    #[test]
    fn find() {
        let mystr = "abc abc def def";
        let doc = Doc::from(String::from(mystr));
        doc.cursors().add_cursor(0).find_forward("def").insert("!");
        assert_eq!(doc.content().as_str(), "abc abc !def def");
        doc.cursors().find_backward("abc").insert("!");
        assert_eq!(doc.content().as_str(), "abc abc! !def def");
        doc.cursors().find_backward_more(" ").insert("!");
        assert_eq!(doc.content().as_str(), "abc! abc! !def def");
        doc.cursors().find_forward_more("!def").insert("!");
        assert_eq!(doc.content().as_str(), "abc! abc! !def! def");
    }
}
