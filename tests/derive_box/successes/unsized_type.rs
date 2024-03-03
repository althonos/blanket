use blanket::blanket;
use impls::impls;

#[blanket(derive(Box))]
pub trait StringLike {
    fn is_utf8(&self) -> bool;
}

impl StringLike for str {
    fn is_utf8(&self) -> bool {
        true
    }
}

fn main() {
    assert!(impls!(str:      StringLike));
    assert!(impls!(Box<str>: StringLike));
}
