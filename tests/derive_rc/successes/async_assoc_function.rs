use std::rc::Rc;

use impls::impls;

#[blanket::blanket(derive(Rc))]
trait Foo {
    async fn bar();
}

#[derive(Default)]
struct Baz;

impl Foo for Baz {
    async fn bar() {}
}

fn main() {
    assert!(impls!(Baz:  Foo));
    assert!(impls!(Rc<Baz>: Foo));
}