extern crate blanket;
extern crate trybuild;

use blanket::blanket;

#[test]
fn test_derive() {
    #[blanket(derive(Mut))]
    pub trait Counter {
        fn increment(&mut self);
    }

    #[derive(Default)]
    struct AtomicCounter {
        count: u8,
    }

    impl Counter for AtomicCounter {
        fn increment(&mut self) {
            self.count += 1;
        }
    }

    struct CounterWrapper<C: Counter> {
        inner: C,
    }

    impl<C: Counter> From<C> for CounterWrapper<C> {
        fn from(inner: C) -> Self {
            Self { inner }
        }
    }

    // counter wrapper should be able to wrap AtomicCounter
    let counter = AtomicCounter::default();
    let mut wrapper_by_value = CounterWrapper::from(counter);
    assert_eq!(wrapper_by_value.inner.count, 0);
    wrapper_by_value.inner.increment();
    assert_eq!(wrapper_by_value.inner.count, 1);

    // and since we derived it, it should be able to wrap &mut AtomicCounter too
    let mut counter = AtomicCounter::default();
    let wrapper_by_ref = CounterWrapper::from(&mut counter);
    assert_eq!(wrapper_by_ref.inner.count, 0);
    wrapper_by_ref.inner.increment();
    assert_eq!(wrapper_by_ref.inner.count, 1);
}

#[test]
#[cfg(not(tarpaulin))]
fn test_failures() {
    #[cfg(not(tarpaulin))]
    let t = trybuild::TestCases::new();
    // check that the same test case but without a derive does not work
    t.compile_fail(file!().replace("mod.rs", "fails/noderive.rs"));
    // check that deriving fails if the input trait declares methods taking ownership
    t.compile_fail(file!().replace("mod.rs", "fails/selfmethods.rs"));
    // check that deriving fails if the input trait declares methods with exotic receivers
    t.compile_fail(file!().replace("mod.rs", "fails/boxmethods.rs"));
}
