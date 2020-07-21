#![allow(unused)]

extern crate blanket;
use blanket::blanket;

#[blanket(derive(Box))]
pub trait MyTrait {
    fn do_something(&self);
}

#[blanket(derive(Box))]
pub trait MyTraitMut {
    fn do_something_else(&mut self);
}

#[blanket(derive(Box))]
pub trait MyTraitMix {
    fn do_something(&self);
    fn do_something_else(&mut self);
}


// #[test]
// fn test_default() {
//
//     #[derive(Default)]
//     struct CharCounter {
//         count: usize
//     }
//
//     impl Visitor for CharCounter {
//         fn visit_char(&mut self, c: char) {
//             self.count += 1
//         }
//     }
//
//     let mut counter = CharCounter::default();
//     let string = String::from("Hello, world!");
//     counter.visit_str(&string);
//
//     assert_eq!(counter.count, string.len());
// }
//
// #[test]
// fn test_overload() {
//
//     #[derive(Default)]
//     struct CharBytesCounter {
//         count: usize,
//         bytes: usize,
//     }
//
//     impl Visitor for CharBytesCounter {
//         fn visit_str(&mut self, s: &str) {
//             self.bytes += s.as_bytes().len();
//             self::visitor::visit_str(self, s);
//         }
//         fn visit_char(&mut self, c: char) {
//             self.count += 1
//         }
//     }
//
//     let mut counter = CharBytesCounter::default();
//     let string = String::from("Hello, 🌍!");
//     counter.visit_str(&string);
//
//     assert_eq!(counter.count, string.chars().enumerate().last().unwrap().0 + 1);
//     assert_eq!(counter.bytes, string.as_bytes().len());
// }
