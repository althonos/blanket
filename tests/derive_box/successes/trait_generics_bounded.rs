#![feature(associated_type_bounds)]

extern crate blanket;
extern crate impls;

use blanket::blanket;
use impls::impls;

#[blanket(derive(Box))]
pub trait AsRef2<T: 'static + Send> {
    fn as_ref2(&self) -> &T;
}

#[derive(Default)]
struct Owner<T> {
    owned: T,
}

impl<T: 'static + Send> AsRef2<T> for Owner<T> {
    fn as_ref2(&self) -> &T {
        &self.owned
    }
}

fn main() {
    assert!(impls!(Owner<String>:      AsRef2<String>));
    assert!(impls!(Box<Owner<String>>: AsRef2<String>));
    assert!(impls!(Owner<bool>:        AsRef2<bool>));
    assert!(impls!(Box<Owner<bool>>:   AsRef2<bool>));
}
