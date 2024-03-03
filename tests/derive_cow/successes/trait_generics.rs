extern crate blanket;
extern crate impls;

use std::borrow::Cow;

use blanket::blanket;
use impls::impls;

#[blanket(derive(Cow))]
pub trait AsRef2<T> {
    fn as_ref2(&self) -> &T;
}

#[derive(Default, Clone)]
struct Owner<T> {
    owned: T,
}

impl<T> AsRef2<T> for Owner<T> {
    fn as_ref2(&self) -> &T {
        &self.owned
    }
}

fn main() {
    assert!(impls!(Owner<String>:     AsRef2<String>));
    assert!(impls!(Cow<Owner<String>>: AsRef2<String>));
    assert!(impls!(Owner<bool>:       AsRef2<bool>));
    assert!(impls!(Cow<Owner<bool>>:   AsRef2<bool>));
}
