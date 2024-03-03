use impls::impls;

#[blanket::blanket(derive(Ref))]
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
    assert!(impls!(&Baz: Foo));
}