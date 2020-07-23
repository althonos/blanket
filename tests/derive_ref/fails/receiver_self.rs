extern crate blanket;

use blanket::blanket;

#[blanket(derive(Ref))]
pub trait Extract {
    fn extract(self);
}

fn main() {}
