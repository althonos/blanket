extern crate blanket;
extern crate impls;

use blanket::blanket;
use impls::impls;

#[blanket(derive(Mut))]
pub trait Counter {
    fn increment(&mut self);
}

#[derive(Default)]
struct AtomicCounter {
    count: u8,
}

impl Counter for AtomicCounter {
    fn increment(&mut self) {
        self.count += 1;
    }
}

fn main() {
    assert!(impls!(AtomicCounter:      Counter));
    assert!(impls!(&mut AtomicCounter: Counter));
}
