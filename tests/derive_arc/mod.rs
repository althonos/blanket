extern crate trybuild;

fn main() {
    #[cfg(not(tarpaulin))]
    let t = trybuild::TestCases::new();
    t.compile_fail(file!().replace("mod.rs", "fails/*.rs"));
    t.pass(file!().replace("mod.rs", "successes/*.rs"));
}
