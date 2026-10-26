use crate::helper::*;

#[test]
fn test_di() {
    let hoge = Hoge::di();
    assert!(ptr_eq(hoge.foo1.as_ref(), hoge.foo2.as_ref()));
    assert!(!ptr_eq(hoge.bar1.as_ref(), hoge.bar2.as_ref()));

    Piyo3StringUnitProvider::di();

    di![Piyo3<String, ()>];
    let c = &DIContainer::new();
    di![Piyo3<String, ()> on c];
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
    // di by manual Provider
    _baz_tagged: DI<Tagged<dyn Baz, String>>,
    // di for a trait with generics
    _piyo: DI<dyn Piyo<String, bool>>,
    // di for a trait with generics
    _piyo2: DI<dyn Piyo2<String, bool>>,
    // di for a trait with generics
    _piyo3: DI<dyn Piyo3<String, bool>>,
    // di for a trait with generics that contains ()
    _piyo3_unit: DI<dyn Piyo3<String, ()>>,
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

    def_di_provider!(dyn Baz, |_c| BazTest {});

    def_di_provider!(Tagged<dyn Baz, String>, |_c| {
        // Tagged::wrap(di![Baz on c])
        Tagged::wrap(DI::new(BazTest {}) as DI<dyn Baz>)
        // Tagged::<dyn Baz, _>::wrap(DI::new(BazTest {}))
    });
}

mod piyo {
    use super::*;

    pub trait Piyo<A, B>: DITarget {}

    struct PiyoTest {}
    impl Piyo<String, bool> for PiyoTest {}

    def_di_provider!(dyn Piyo<String, bool>, |_c| PiyoTest {});

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
    impl Piyo3<String, ()> for Piyo3Test {}

    #[portaldi::provider(Piyo3<String, bool>)]
    impl DIPortal for Piyo3Test {
        fn create_for_di(_container: &DIContainer) -> Self {
            Piyo3Test {}
        }
    }

    def_di_provider!(dyn Piyo3<String, ()>, |_| { Piyo3Test {} });
}

#[test]
fn test_di_provider_raw() {
    let c = &DIContainer::new();
    assert_eq!(QuxProvider::di_on(c).name(), "A");

    let c = &DIContainer::new();
    c.put_if_absent(&DI::new(UseQuxB {}));
    let qux1 = QuxProvider::di_on(c);
    let qux2 = QuxProvider::di_on(c);
    assert_eq!(qux1.name(), "B");
    assert!(ptr_eq(qux1.as_ref(), qux2.as_ref()));
}

use qux::*;

mod qux {
    use super::*;

    pub trait Qux: DITarget {
        fn name(&self) -> &'static str;
    }

    struct QuxA {}
    impl Qux for QuxA {
        fn name(&self) -> &'static str {
            "A"
        }
    }

    struct QuxB {}
    impl Qux for QuxB {
        fn name(&self) -> &'static str {
            "B"
        }
    }

    pub struct UseQuxB {}

    // switch the implementation by a condition
    def_di_provider_raw!(dyn Qux, |c| {
        if c.get::<UseQuxB>().is_some() {
            c.get_or_init(|| QuxB {})
        } else {
            c.get_or_init(|| QuxA {})
        }
    });
}
