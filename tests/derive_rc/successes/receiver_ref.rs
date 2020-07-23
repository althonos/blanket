extern crate blanket;
extern crate impls;

use std::rc::Rc;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering;

use blanket::blanket;
use impls::impls;

#[blanket(derive(Rc))]
pub trait Counter {
    fn increment(&self);
}

#[derive(Default)]
struct AtomicCounter {
    count: AtomicU8,
}

impl Counter for AtomicCounter {
    fn increment(&self) {
        self.count.fetch_add(1, Ordering::SeqCst);
    }
}

fn main() {
    assert!(impls!(AtomicCounter:     Counter));
    assert!(impls!(Rc<AtomicCounter>: Counter));
}
