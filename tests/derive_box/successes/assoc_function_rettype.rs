use blanket::blanket;
use impls::impls;

#[blanket(derive(Box))]
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
    assert!(impls!(Box<NoOpChecker>: StaticChecker));
}
