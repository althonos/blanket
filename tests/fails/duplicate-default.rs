extern crate blanket;
use blanket::blanket;

#[blanket(default = "default", default = "other")]
pub trait MyTrait {}

pub fn main() {}
