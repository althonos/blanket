extern crate blanket;

use blanket::blanket;

#[blanket(derive(Arc))]
pub trait Extract {
    fn extract(self);
}

fn main() {}
