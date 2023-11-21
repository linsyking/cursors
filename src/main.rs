use cursors::common::Doc;

fn main() {
    let doc = Doc::from(String::from("123"));
    assert_eq!(doc.content(), String::from("123"));
}
