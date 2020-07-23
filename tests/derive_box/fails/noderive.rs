extern crate impls;
extern crate static_assertions;

use impls::impls;
use static_assertions::const_assert;

pub trait Counter {
    fn increment(&mut self);
}

struct AtomicCounter(u8);

impl Counter for AtomicCounter {
    fn increment(&mut self) {
        self.0 += 1;
    }
}

fn main() {
    const_assert!(impls!(AtomicCounter:      Counter));
    const_assert!(impls!(Box<AtomicCounter>: Counter));
}
