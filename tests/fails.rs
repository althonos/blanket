extern crate trybuild;

#[test]
#[cfg(not(tarpaulin))]
fn fails() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fails/*.rs");
}
