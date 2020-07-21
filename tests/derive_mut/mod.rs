extern crate blanket;
extern crate trybuild;

use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering;

use blanket::blanket;

#[test]
fn test_receiver_ref() {
    #[blanket(derive(Mut))]
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
    let wrapper_by_value = CounterWrapper::from(counter);
    assert_eq!(wrapper_by_value.inner.count.load(Ordering::Relaxed), 0);
    wrapper_by_value.inner.increment();
    assert_eq!(wrapper_by_value.inner.count.load(Ordering::Relaxed), 1);

    // and since we derived it, it should be able to wrap &mut AtomicCounter too
    let mut counter = AtomicCounter::default();
    let wrapper_by_mut = CounterWrapper::from(&mut counter);
    assert_eq!(wrapper_by_mut.inner.count.load(Ordering::Relaxed), 0);
    wrapper_by_mut.inner.increment();
    assert_eq!(wrapper_by_mut.inner.count.load(Ordering::Relaxed), 1);
}

#[test]
fn test_receiver_mut() {
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
    let wrapper_by_mut = CounterWrapper::from(&mut counter);
    assert_eq!(wrapper_by_mut.inner.count, 0);
    wrapper_by_mut.inner.increment();
    assert_eq!(wrapper_by_mut.inner.count, 1);
}

#[test]
fn test_receiver_mix() {
    #[blanket(derive(Mut))]
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
    assert_eq!(wrapper_by_value.inner.count.load(Ordering::Relaxed), 0);
    wrapper_by_value.inner.increment();
    wrapper_by_value.inner.increment();
    assert_eq!(wrapper_by_value.inner.count.load(Ordering::Relaxed), 2);
    wrapper_by_value.inner.decrement();
    assert_eq!(wrapper_by_value.inner.count.load(Ordering::Relaxed), 1);

    // and since we derived it, it should be able to wrap &mut AtomicCounter too
    let mut counter = AtomicCounter::default();
    let wrapper_by_mut = CounterWrapper::from(&mut counter);
    assert_eq!(wrapper_by_mut.inner.count.load(Ordering::Relaxed), 0);
    wrapper_by_mut.inner.increment();
    wrapper_by_mut.inner.increment();
    assert_eq!(wrapper_by_mut.inner.count.load(Ordering::Relaxed), 2);
    wrapper_by_mut.inner.decrement();
    assert_eq!(wrapper_by_mut.inner.count.load(Ordering::Relaxed), 1);
}

#[test]
fn test_trait_generic() {
    #[blanket(derive(Mut))]
    pub trait AsRef2<T> {
        fn as_ref2(&self) -> &T;
    }

    #[derive(Default)]
    struct Owner<T> {
        owned: T,
    }

    impl<T> AsRef2<T> for Owner<T> {
        fn as_ref2(&self) -> &T {
            &self.owned
        }
    }

    struct Wrapper<T, A: AsRef2<T>> {
        __marker: std::marker::PhantomData<T>,
        wrapped: A,
    };

    impl<T, A: AsRef2<T>> From<A> for Wrapper<T, A> {
        fn from(wrapped: A) -> Self {
            Wrapper {
                wrapped,
                __marker: std::marker::PhantomData,
            }
        }
    }

    let string_owner: Owner<String> = Owner::default();
    assert_eq!(string_owner.as_ref2(), "");
    let string_owner_wrapper = Wrapper::from(string_owner);
    assert_eq!(string_owner_wrapper.wrapped.as_ref2(), "");

    let mut string_owner: Owner<String> = Owner::default();
    assert_eq!(string_owner.as_ref2(), "");
    let string_owner_mut_wrapper = Wrapper::from(&mut string_owner);
    assert_eq!(string_owner_mut_wrapper.wrapped.as_ref2(), "");
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
