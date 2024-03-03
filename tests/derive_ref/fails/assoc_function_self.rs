use blanket::blanket;
use impls::impls;

#[blanket(derive(Ref))]
pub trait MyDefault {
    fn default() -> Self;
}

#[derive(Default)]
struct Item;

impl MyDefault for Item {
    fn default() -> Self {
        Self
    }
}

fn main() {
    assert!(impls!( Item: MyDefault));
    assert!(impls!(&Item: MyDefault));
}
