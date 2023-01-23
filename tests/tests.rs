#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/basic_from.rs");
}