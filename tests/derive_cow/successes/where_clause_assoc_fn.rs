use std::borrow::Cow;
use std::sync::Arc;
use std::sync::RwLock;

use blanket::blanket;
use impls::impls;

#[blanket(derive(Cow))]
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

#[derive(Default, Clone)]
struct AtomicCounter {
    count: Arc<RwLock<u8>>,
}

impl Counter<u8> for AtomicCounter {
    fn increment(&self, value: u8) {
        let mut guard = self.count.try_write().unwrap();
        *guard += value;
    }
}

fn main() {
    assert!(impls!(AtomicCounter:     Counter<u8>));
    assert!(impls!(Cow<AtomicCounter>: Counter<u8>));
}
