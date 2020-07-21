extern crate blanket;

use blanket::blanket;

#[blanket(derive(Rc))]
pub trait Counter {
    fn increment(self: Box<Self>);
}

fn main() {}
