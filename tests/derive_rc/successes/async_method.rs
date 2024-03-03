use std::rc::Rc;

use impls::impls;

#[blanket::blanket(derive(Rc))]
trait Foo {
    async fn bar(&self);
}

#[derive(Default)]
struct Baz;

impl Foo for Baz {
    async fn bar(&self) {}
}

fn main() {
    assert!(impls!(Baz:  Foo));
    assert!(impls!(Rc<Baz>: Foo));
}