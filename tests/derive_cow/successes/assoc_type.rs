use std::borrow::Cow;
use std::sync::Arc;
use std::sync::RwLock;

use blanket::blanket;
use impls::impls;

#[blanket(derive(Cow))]
pub trait Counter {
    type Return: Clone; // <- verify this
    fn increment(&self) -> Self::Return;
}

#[derive(Default, Clone)]
struct AtomicCounter {
    count: Arc<RwLock<u8>>,
}

impl Counter for AtomicCounter {
    // Generate something like `type Return = <A as Assoc>::Return;`.
    type Return = u8;
    fn increment(&self) -> u8 {
        let mut guard = self.count.try_write().unwrap();
        let out = *guard;
        *guard += 1;
        out
    }
}

fn main() {
    assert!(impls!(AtomicCounter:     Counter));
    assert!(impls!(Cow<AtomicCounter>: Counter));
}
