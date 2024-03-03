use std::sync::Arc;

use impls::impls;

#[blanket::blanket(derive(Arc))]
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
    assert!(impls!(Arc<Baz>: Foo));
}