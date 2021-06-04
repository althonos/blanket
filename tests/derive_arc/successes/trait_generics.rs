extern crate blanket;
extern crate impls;

use std::sync::Arc;

use blanket::blanket;
use impls::impls;

#[blanket(derive(Arc))]
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

fn main() {
    assert!(impls!(Owner<String>:      AsRef2<String>));
    assert!(impls!(Arc<Owner<String>>: AsRef2<String>));
    assert!(impls!(Owner<bool>:        AsRef2<bool>));
    assert!(impls!(Arc<Owner<bool>>:   AsRef2<bool>));
}
