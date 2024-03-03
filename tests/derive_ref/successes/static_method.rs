use blanket::blanket;
use impls::impls;

#[blanket(derive(Ref))]
pub trait StaticChecker {
    fn check();
}

#[derive(Default)]
struct NoOpChecker;

impl StaticChecker for NoOpChecker {
    fn check() {}
}

fn main() {
    assert!(impls!(NoOpChecker:  StaticChecker));
    assert!(impls!(&NoOpChecker: StaticChecker));
}
