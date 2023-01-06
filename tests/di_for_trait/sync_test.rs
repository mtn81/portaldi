use crate::common::*;

#[test]
fn test_di() {
    let hoge = Hoge::di();
    assert!(ptr_eq(hoge.foo1.as_ref(), hoge.foo2.as_ref()));
    assert!(!ptr_eq(hoge.bar1.as_ref(), hoge.bar2.as_ref()));
}

use bar::*;
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
