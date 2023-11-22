#[cfg(test)]
mod cursor_tests {
    use cursors::common::Doc;

    const TEST_STRING: &str = &"\
    abc def
    dd asdjlk
    s sd sd dsklasd\n";
    #[test]
    fn test_1() {
        let doc = Doc::from(String::from(TEST_STRING));
        assert_eq!(doc.content().as_str(), TEST_STRING);
        assert_eq!(doc.cursors().len(), 0);
        doc.cursors().add_cursor(0);
        assert_eq!(doc.cursors().len(), 1);
    }
}
