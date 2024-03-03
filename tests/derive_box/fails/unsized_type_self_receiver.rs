use blanket::blanket;
use impls::impls;

#[blanket(derive(Box))]
pub trait StringLike {
    fn into_utf8(self) -> Vec<u8>;
}

impl StringLike for str {
    fn into_utf8(self) -> Vec<u8> {
        self.as_bytes().into()
    }
}

fn main() {
    assert!(impls!(str:      StringLike));
    assert!(impls!(Box<str>: StringLike));
}
