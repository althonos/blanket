extern crate blanket;
extern crate impls;

use std::borrow::Cow;

use blanket::blanket;
use impls::impls;

#[blanket(derive(Cow))]
pub trait Counter {
    fn count(&self);
}

#[derive(Default, Clone)]
struct AtomicCounter {}

impl Counter for AtomicCounter {
    fn count(&self) {}
}

fn main() {
    assert!(impls!(AtomicCounter:  Counter));
    assert!(impls!(Cow<AtomicCounter>: Counter));
}
