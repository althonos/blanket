extern crate blanket;

use blanket::blanket;

#[blanket(derive(Ref))]
pub trait Counter {
    fn increment(&mut self);
}

fn main() {}
