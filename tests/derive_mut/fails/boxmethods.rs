extern crate blanket;

use blanket::blanket;

#[blanket(derive(Mut))]
pub trait Counter {
    fn increment(self: Box<Self>);
}

fn main() {}
