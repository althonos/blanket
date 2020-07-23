extern crate impls;
extern crate static_assertions;

use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering;

use impls::impls;
use static_assertions::const_assert;

pub trait Counter {
    fn increment(&self);
}

#[derive(Default)]
struct AtomicCounter {
    count: AtomicU8
}

impl Counter for AtomicCounter {
    fn increment(&self) {
        self.count.fetch_add(1, Ordering::SeqCst);
    }
}

fn main() {
    const_assert!(impls!(AtomicCounter:      Counter));
    const_assert!(impls!(&AtomicCounter:     Counter));
}
