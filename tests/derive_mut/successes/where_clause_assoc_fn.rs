extern crate blanket;
extern crate impls;

use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering;

use blanket::blanket;
use impls::impls;

#[blanket(derive(Mut))]
pub trait Counter<T>
where
    T: Clone,
{
    fn increment(&self, t: T);

    fn super_helpful_helper(&self, t: T)
    {
        self.increment(t.clone())
    }
}

struct AtomicCounter {
    count: AtomicU8,
}

impl Counter<u8> for AtomicCounter {
    fn increment(&self, value: u8) {
        self.count.fetch_add(value, Ordering::SeqCst);
    }
}

fn main() {
    assert!(impls!(AtomicCounter:      Counter<u8>));
    assert!(impls!(&mut AtomicCounter: Counter<u8>));
}
