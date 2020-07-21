extern crate blanket;

use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering;

use blanket::blanket;

#[test]
fn test_receiver_ref() {
    #[blanket(derive(Box))]
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
    let counter = AtomicCounter::default();
    let wrapper_by_box = CounterWrapper::from(Box::new(counter));
    assert_eq!(wrapper_by_box.inner.count.load(Ordering::Relaxed), 0);
    wrapper_by_box.inner.increment();
    assert_eq!(wrapper_by_box.inner.count.load(Ordering::Relaxed), 1);
}

#[test]
fn test_receiver_mut() {
    #[blanket(derive(Box))]
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
    let counter = AtomicCounter::default();
    let mut wrapper_by_box = CounterWrapper::from(Box::new(counter));
    assert_eq!(wrapper_by_box.inner.count, 0);
    wrapper_by_box.inner.increment();
    assert_eq!(wrapper_by_box.inner.count, 1);
}

#[test]
fn test_receiver_mix() {
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
    let counter = AtomicCounter::default();
    let mut wrapper_by_box = CounterWrapper::from(Box::new(counter));
    assert_eq!(wrapper_by_box.inner.count.load(Ordering::Relaxed), 0);
    wrapper_by_box.inner.increment();
    wrapper_by_box.inner.increment();
    assert_eq!(wrapper_by_box.inner.count.load(Ordering::Relaxed), 2);
    wrapper_by_box.inner.decrement();
    assert_eq!(wrapper_by_box.inner.count.load(Ordering::Relaxed), 1);
}

#[test]
fn test_receiver_own() {
    #[blanket(derive(Box))]
    pub trait StringBuilder {
        fn build(self) -> String;
    }

    #[derive(Default)]
    struct Concat {
        strings: Vec<String>,
    }

    impl StringBuilder for Concat {
        fn build(self) -> String {
            self.strings.join("")
        }
    }

    struct StringBuilderWrapper<B: StringBuilder> {
        inner: B,
    };

    impl<B: StringBuilder> From<B> for StringBuilderWrapper<B> {
        fn from(inner: B) -> Self {
            StringBuilderWrapper { inner }
        }
    }

    let mut concat = Concat::default();
    concat.strings.push(String::from("Hello "));
    concat.strings.push(String::from("World!"));
    let wrapper = StringBuilderWrapper::from(concat);
    assert_eq!(wrapper.inner.build(), String::from("Hello World!"));

    let mut concat = Concat::default();
    concat.strings.push(String::from("Hello "));
    concat.strings.push(String::from("World!"));
    let wrapper = StringBuilderWrapper::from(Box::new(concat));
    assert_eq!(wrapper.inner.build(), String::from("Hello World!"));
}

#[test]
fn test_trait_generic() {
    #[blanket(derive(Box))]
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

    let string_owner: Owner<String> = Owner::default();
    assert_eq!(string_owner.as_ref2(), "");
    let string_owner_box_wrapper = Wrapper::from(Box::new(string_owner));
    assert_eq!(string_owner_box_wrapper.wrapped.as_ref2(), "");
}

#[test]
#[cfg(not(tarpaulin))]
fn test_failures() {
    #[cfg(not(tarpaulin))]
    let t = trybuild::TestCases::new();
    // check that the same test case but without a derive does not work
    t.compile_fail(file!().replace("mod.rs", "fails/noderive.rs"));
}
