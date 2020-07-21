extern crate blanket;
extern crate trybuild;

use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering;

use blanket::blanket;

#[blanket(derive(Ref))]
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

#[test]
fn test_derive() {
    // counter wrapper should be able to wrap AtomicCounter
    let counter = AtomicCounter::default();
    let wrapper_by_value = CounterWrapper::from(counter);
    assert_eq!(wrapper_by_value.inner.count.load(Ordering::Relaxed), 0);
    wrapper_by_value.inner.increment();
    assert_eq!(wrapper_by_value.inner.count.load(Ordering::Relaxed), 1);

    // and since we derived it, it should be able to wrap &AtomicCounter too
    let counter = AtomicCounter::default();
    let wrapper_by_ref = CounterWrapper::from(&counter);
    assert_eq!(wrapper_by_ref.inner.count.load(Ordering::Relaxed), 0);
    assert_eq!(counter.count.load(Ordering::Relaxed), 0);
    wrapper_by_ref.inner.increment();
    assert_eq!(wrapper_by_ref.inner.count.load(Ordering::Relaxed), 1);
    assert_eq!(counter.count.load(Ordering::Relaxed), 1);
}

#[test]
fn test_no_derive() {
    let t = trybuild::TestCases::new();
    t.compile_fail( file!().replace("mod.rs", "fail.rs") );
}
