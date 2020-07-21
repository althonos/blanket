extern crate blanket;
use blanket::blanket;

#[blanket(default = "default")]
pub trait MyTrait {
    fn method() {}
}

pub fn main() {}
