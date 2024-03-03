use blanket::blanket;
use impls::impls;

use std::rc::Rc;

#[blanket(derive(Rc))]
pub trait StaticChecker {
    fn check() -> Result<(), String>;
}

#[derive(Default)]
struct NoOpChecker;

impl StaticChecker for NoOpChecker {
    fn check() -> Result<(), String> { Ok(()) }
}

fn main() {
    assert!(impls!(   NoOpChecker:  StaticChecker));
    assert!(impls!(Rc<NoOpChecker>: StaticChecker));
}
