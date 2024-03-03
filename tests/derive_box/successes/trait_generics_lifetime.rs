#![feature(associated_type_bounds)]

extern crate blanket;
extern crate impls;

use blanket::blanket;
use impls::impls;

#[blanket(derive(Box))]
pub trait AsRef2<'a, T: 'static + Send> {
    fn as_ref2(&'a self) -> &'a T;
}

#[derive(Default)]
struct Owner<T> {
    owned: T,
}

impl<'a, T: 'static + Send> AsRef2<'a, T> for Owner<T> {
    fn as_ref2(&'a self) -> &'a T {
        &self.owned
    }
}

fn main() {
    assert!(impls!(Owner<String>:      AsRef2<'static, String>));
    assert!(impls!(Box<Owner<String>>: AsRef2<'static, String>));
    assert!(impls!(Owner<bool>:        AsRef2<'static, bool>));
    assert!(impls!(Box<Owner<bool>>:   AsRef2<'static, bool>));
}
