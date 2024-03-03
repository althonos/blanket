use impls::impls;

#[blanket::blanket(derive(Ref))]
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
    assert!(impls!(&Baz: Foo));
}