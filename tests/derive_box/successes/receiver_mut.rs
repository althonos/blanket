extern crate blanket;
extern crate impls;

use blanket::blanket;
use impls::impls;

#[blanket(derive(Box))]
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
    assert!(impls!(Box<AtomicCounter>: Counter));
}
