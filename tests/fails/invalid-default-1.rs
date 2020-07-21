extern crate blanket;
use blanket::blanket;

#[blanket(default = 1)]
pub trait MyTrait {}

pub fn main() {}
