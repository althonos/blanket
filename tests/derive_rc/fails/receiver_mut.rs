extern crate blanket;

use blanket::blanket;

#[blanket(derive(Rc))]
pub trait Counter {
    fn increment(&mut self);
}

fn main() {}
