use std::sync::Arc;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering;

use blanket::blanket;
use impls::impls;

#[blanket(derive(Arc))]
pub trait Counter {
    type Return: Clone; // <- verify this
    fn increment(&self) -> Self::Return;
}

#[derive(Default)]
struct AtomicCounter {
    count: AtomicU8,
}

impl Counter for AtomicCounter {
    // Generate something like `type Return = <A as Assoc>::Return;`.
    type Return = u8;
    fn increment(&self) -> u8 {
        self.count.fetch_add(1, Ordering::SeqCst)
    }
}

fn main() {
    assert!(impls!(AtomicCounter:      Counter));
    assert!(impls!(Arc<AtomicCounter>: Counter));
}
