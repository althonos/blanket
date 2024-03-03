use blanket::blanket;
use impls::impls;

use std::sync::Arc;

#[blanket(derive(Arc))]
pub trait StaticChecker {
    fn check();
}

#[derive(Default)]
struct NoOpChecker;

impl StaticChecker for NoOpChecker {
    fn check() {}
}

fn main() {
    assert!(impls!(    NoOpChecker:  StaticChecker));
    assert!(impls!(Arc<NoOpChecker>: StaticChecker));
}
