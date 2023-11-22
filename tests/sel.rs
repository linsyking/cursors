#[cfg(test)]
mod selection_tests {
    use cursors::common::Doc;

    #[test]
    fn add() {
        let doc = Doc::from(String::from("hihihi"));
        doc.selections().add_selection(0, 3); // Select "hih"
    }
}
