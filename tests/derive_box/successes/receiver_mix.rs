extern crate blanket;
extern crate impls;

use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering;

use blanket::blanket;
use impls::impls;

#[blanket(derive(Box))]
pub trait Counter {
    fn increment(&mut self);
    fn decrement(&self);
}

#[derive(Default)]
struct AtomicCounter {
    count: AtomicU8,
}

impl Counter for AtomicCounter {
    fn increment(&mut self) {
        self.count.fetch_add(1, Ordering::SeqCst);
    }
    fn decrement(&self) {
        self.count.fetch_sub(1, Ordering::SeqCst);
    }
}

fn main() {
    assert!(impls!(AtomicCounter:      Counter));
    assert!(impls!(Box<AtomicCounter>: Counter));
}
