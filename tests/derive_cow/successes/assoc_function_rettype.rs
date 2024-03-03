use blanket::blanket;
use impls::impls;

use std::borrow::Cow;

#[blanket(derive(Cow))]
pub trait StaticChecker {
    fn check() -> Result<(), String>;
}

#[derive(Default, Clone)]
struct NoOpChecker;

impl StaticChecker for NoOpChecker {
    fn check() -> Result<(), String> { Ok(()) }
}

fn main() {
    assert!(impls!(    NoOpChecker:  StaticChecker));
    assert!(impls!(Cow<NoOpChecker>: StaticChecker));
}
