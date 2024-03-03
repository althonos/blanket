use blanket::blanket;
use impls::impls;

use std::borrow::Cow;

#[blanket(derive(Cow))]
pub trait StaticChecker {
    fn check();
}

#[derive(Default, Clone)]
struct NoOpChecker;

impl StaticChecker for NoOpChecker {
    fn check() {}
}

fn main() {
    assert!(impls!(    NoOpChecker:  StaticChecker));
    assert!(impls!(Cow<NoOpChecker>: StaticChecker));
}
