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
use piyo::*;

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
    // di for a trait with generics
    _piyo: DI<dyn Piyo<String, bool>>,
    // di for a trait with generics
    _piyo2: DI<dyn Piyo2<String, bool>>,
    // di for a trait with generics
    _piyo3: DI<dyn Piyo3<String, bool>>,
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
}

mod piyo {
    use super::*;

    pub trait Piyo<A, B>: DITarget {}

    struct PiyoTest {}
    impl Piyo<String, bool> for PiyoTest {}

    di_provider!(dyn Piyo<String, bool>, |_c| PiyoTest {});

    //

    pub trait Piyo2<A, B>: DITarget {}

    #[derive(DIPortal, PartialEq)]
    #[provide(Piyo2<String, bool>)]
    struct Piyo2Test {}
    impl Piyo2<String, bool> for Piyo2Test {}

    //

    pub trait Piyo3<A, B>: DITarget {}

    struct Piyo3Test {}
    impl Piyo3<String, bool> for Piyo3Test {}

    #[portaldi::provider(Piyo3<String, bool>)]
    #[async_trait]
    impl DIPortal for Piyo3Test {
        fn create_for_di(_container: &DIContainer) -> Self {
            Piyo3Test {}
        }
    }
}
