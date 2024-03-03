use blanket::blanket;
use impls::impls;

use std::sync::Arc;

#[blanket(derive(Arc))]
pub trait StaticChecker {
    fn check() -> Result<(), String>;
}

#[derive(Default)]
struct NoOpChecker;

impl StaticChecker for NoOpChecker {
    fn check() -> Result<(), String> { Ok(()) }
}

fn main() {
    assert!(impls!(    NoOpChecker:  StaticChecker));
    assert!(impls!(Arc<NoOpChecker>: StaticChecker));
}
