use crate::common::*;

#[test]
fn test_di() {
    let hoge = Hoge::di();
    assert!(ptr_eq(hoge.foo1.as_ref(), hoge.foo2.as_ref()));
    assert!(!ptr_eq(hoge.bar1.as_ref(), hoge.bar2.as_ref()));
}

use bar::*;
use baz::*;
use foo::*;

#[derive(DIPortal)]
struct Hoge {
    // di by "provide" & Ident
    foo1: DI<dyn Foo>,
    // di by "provide" & Path
    foo2: DI<dyn foo::Foo>,
    // di by implicit Provider (BarImpl)
    bar1: DI<dyn Bar>,
    // di by explicit type in "inject"
    #[inject(BarImpl2)]
    bar2: DI<dyn Bar>,
    // di by manual Provider
    _baz: DI<dyn Baz>,
}

mod foo {
    use super::*;
    pub trait Foo: DITarget {}

    #[derive(DIPortal, PartialEq)]
    #[provide(Foo)]
    struct FooImpl {}

    impl Foo for FooImpl {}
}

mod bar {
    use super::*;

    pub trait Bar: DITarget {}

    #[derive(DIPortal)]
    struct BarImpl {}
    impl Bar for BarImpl {}

    #[derive(DIPortal)]
    pub struct BarImpl2 {}
    impl Bar for BarImpl2 {}
}

mod baz {
    use super::*;

    pub trait Baz: DITarget {}

    struct BazTest {}
    impl Baz for BazTest {}

    di_provider!(dyn Baz, |_c| BazTest {});

    pub trait Baz2<A, B>: DITarget {}

    struct Baz2Test {}
    impl Baz2<String, bool> for Baz2Test {}

    di_provider!(dyn Baz2<String, bool>, |_c| Baz2Test {});
}
