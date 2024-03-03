use std::borrow::Cow;

use impls::impls;

#[blanket::blanket(derive(Cow))]
trait Foo {
    async fn bar(&self);
}

#[derive(Default, Clone)]
struct Baz;

impl Foo for Baz {
    async fn bar(&self) {}
}

fn main() {
    assert!(impls!(Baz:  Foo));
    assert!(impls!(Cow<Baz>: Foo));
}