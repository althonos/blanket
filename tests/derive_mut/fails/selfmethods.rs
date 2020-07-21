extern crate blanket;

use blanket::blanket;

#[blanket(derive(Mut))]
pub trait Extract {
    fn extract(self);
}

fn main() {}
