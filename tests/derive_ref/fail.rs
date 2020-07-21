use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering;

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

struct CounterWrapper<C: Counter> {
    inner: C
}

impl<C: Counter> From<C> for CounterWrapper<C> {
    fn from(inner: C) -> Self {
        Self { inner }
    }
}

fn main() {
    // counter wrapper should be able to wrap AtomicCounter
    let counter = AtomicCounter::default();
    let wrapper_by_value = CounterWrapper::from(counter);
    // but this will fail because no implementation was derived
    let counter = AtomicCounter::default();
    let wrapper_by_ref = CounterWrapper::from(&counter);
}
