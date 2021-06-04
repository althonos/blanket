extern crate blanket;

use blanket::blanket;

#[blanket(derive(Arc))]
pub trait Counter {
    fn increment(&mut self);
}

fn main() {}
