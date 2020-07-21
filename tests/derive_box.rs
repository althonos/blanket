extern crate blanket;

use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering;

use blanket::blanket;

#[blanket(derive(Box))]
pub trait MyTrait {
    fn do_something(&self);
}

#[test]
fn test_trait() {

    #[derive(Default)]
    struct Something {
        things_done: AtomicU8
    }

    impl MyTrait for Something {
        fn do_something(&self) {
            self.things_done.fetch_add(1, Ordering::SeqCst);
        }
    }

    let something = Something::default();
    assert_eq!(something.things_done.load(Ordering::SeqCst), 0);
    something.do_something();
    assert_eq!(something.things_done.load(Ordering::SeqCst), 1);

    let boxed = Box::new(something);
    boxed.do_something();
    assert_eq!(boxed.things_done.load(Ordering::SeqCst), 2);
}

#[blanket(derive(Box))]
pub trait MyTraitMut {
    fn do_something_else(&mut self);
}

#[test]
fn test_trait_mut() {
    #[derive(Default)]
    struct Something {
        other_things_done: usize
    }

    impl MyTraitMut for Something {
        fn do_something_else(&mut self) {
            self.other_things_done += 1;
        }
    }

    let mut something = Something::default();
    assert_eq!(something.other_things_done, 0);
    something.do_something_else();
    assert_eq!(something.other_things_done, 1);

    let mut boxed = Box::new(something);
    boxed.do_something_else();
    assert_eq!(boxed.other_things_done, 2);
}

#[blanket(derive(Box))]
pub trait MyTraitMix {
    fn do_something(&self);
    fn do_something_else(&mut self);
}

#[test]
fn test_trait_mix() {
    #[derive(Default)]
    struct Something{
        things_done: AtomicU8,
        other_things_done: usize
    }

    impl MyTraitMix for Something {
        fn do_something(&self) {
            self.things_done.fetch_add(1, Ordering::SeqCst);
        }
        fn do_something_else(&mut self) {
            self.other_things_done += 1;
        }
    }

    let mut something = Something::default();
    assert_eq!(something.things_done.load(Ordering::SeqCst), 0);
    something.do_something();
    assert_eq!(something.things_done.load(Ordering::SeqCst), 1);
    assert_eq!(something.other_things_done, 0);
    something.do_something_else();
    assert_eq!(something.other_things_done, 1);

    let mut boxed = Box::new(something);
    boxed.do_something();
    assert_eq!(boxed.things_done.load(Ordering::SeqCst), 2);
    boxed.do_something_else();
    assert_eq!(boxed.other_things_done, 2);
}
