extern crate blanket;
extern crate impls;

use blanket::blanket;
use impls::impls;

#[blanket(derive(Box))]
pub trait StringBuilder {
    fn build(self) -> String;
}

struct Concat {
    strings: Vec<String>,
}

impl StringBuilder for Concat {
    fn build(self) -> String {
        self.strings.join("")
    }
}

fn main() {
    assert!(impls!(Concat:      StringBuilder));
    assert!(impls!(Box<Concat>: StringBuilder));
}
