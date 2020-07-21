extern crate trybuild;

#[test]
fn fails() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fails/*.rs");
}
