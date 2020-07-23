extern crate blanket;

use blanket::blanket;

#[blanket(derive(Rc))]
pub trait Extract {
    fn extract(self);
}

fn main() {}
