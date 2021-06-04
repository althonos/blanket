#![allow(unused)]

extern crate blanket;
use blanket::blanket;

#[blanket(default = "visitor")]
pub trait Visitor {
    fn visit_str(&mut self, s: &str);
    fn visit_char(&mut self, c: char);
}

pub mod visitor {
    use super::Visitor;

    pub fn visit_str<V: Visitor + ?Sized>(v: &mut V, s: &str) {
        for c in s.chars() {
            v.visit_char(c);
        }
    }

    pub fn visit_char<V: Visitor + ?Sized>(v: &mut V, c: char) {}
}

#[test]
fn test_default() {
    #[derive(Default)]
    struct CharCounter {
        count: usize,
    }

    impl Visitor for CharCounter {
        fn visit_char(&mut self, c: char) {
            self.count += 1
        }
    }

    let mut counter = CharCounter::default();
    let string = String::from("Hello, world!");
    counter.visit_str(&string);

    assert_eq!(counter.count, string.len());
}

#[test]
fn test_overload() {
    #[derive(Default)]
    struct CharBytesCounter {
        count: usize,
        bytes: usize,
    }

    impl Visitor for CharBytesCounter {
        fn visit_str(&mut self, s: &str) {
            self.bytes += s.as_bytes().len();
            self::visitor::visit_str(self, s);
        }

        fn visit_char(&mut self, c: char) {
            self.count += 1
        }
    }

    let mut counter = CharBytesCounter::default();
    let string = String::from("Hello, 🌍!");
    counter.visit_str(&string);

    assert_eq!(
        counter.count,
        string.chars().enumerate().last().unwrap().0 + 1
    );
    assert_eq!(counter.bytes, string.as_bytes().len());
}
