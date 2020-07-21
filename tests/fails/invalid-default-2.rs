extern crate blanket;
use blanket::blanket;

#[blanket(default = "a b")]
pub trait MyTrait {}

pub fn main() {}
