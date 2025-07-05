use crate::common::*;

#[tokio::test]
async fn test_di() {
    let hoge = Hoge::di().await;
    assert!(!ptr_eq(hoge.foo1.as_ref(), hoge.foo2.as_ref()));

    di![Piyo2<String, ()>].await;
    let c = &DIContainer::new();
    di![Piyo2<String, ()> on c].await;
}

use bar::*;
use baz::*;
use foo::*;
use piyo::*;

#[derive(DIPortal)]
struct Hoge {
    // async di by Provider
    #[inject(async)]
    foo1: DI<dyn Foo>,
    // async di by explicit type
    #[inject(FooImpl2, async)]
    foo2: DI<dyn Foo>,
    // async di by implicit Provider
    #[inject(async)]
    _bar1: DI<dyn Bar>,
    // di by manual Provider
    #[inject(async)]
    _baz: DI<dyn Baz>,
    // di by manual Provider
    #[inject(async)]
    _baz_tagged: DI<Tagged<dyn Baz, String>>,
    // di for a trait with generics
    #[inject(async)]
    _piyo: DI<dyn Piyo<String, bool>>,
    #[inject(async)]
    _piyo2: DI<dyn Piyo2<String, bool>>,
    #[inject(async)]
    _piyo2_unit: DI<dyn Piyo2<String, ()>>,
}

mod foo {
    use super::*;
    pub trait Foo: DITarget {}

    struct FooImpl {}
    impl Foo for FooImpl {}

    #[portaldi::provider(Foo)]
    #[async_trait]
    impl AsyncDIPortal for FooImpl {
        async fn create_for_di(_container: &DIContainer) -> Self {
            FooImpl {}
        }
    }

    pub struct FooImpl2 {}
    impl Foo for FooImpl2 {}

    #[async_trait]
    impl AsyncDIPortal for FooImpl2 {
        async fn create_for_di(_container: &DIContainer) -> Self {
            FooImpl2 {}
        }
    }
}

mod bar {
    use super::*;
    pub trait Bar: DITarget {}

    struct BarImpl {}
    impl Bar for BarImpl {}

    #[portaldi::provider]
    #[async_trait]
    impl AsyncDIPortal for BarImpl {
        async fn create_for_di(_container: &DIContainer) -> Self {
            BarImpl {}
        }
    }
}

mod baz {
    use super::*;

    pub trait Baz: DITarget {}

    struct BazTest {}
    impl Baz for BazTest {}

    def_async_di_provider!(dyn Baz, |_c| async { BazTest {} });
    def_async_di_provider!(Tagged<dyn Baz, String>, |c| async move {
        Tagged::new(di![Baz on c].await)
    });
}

mod piyo {
    use super::*;

    pub trait Piyo<A, B>: DITarget {}

    struct PiyoTest {}
    impl Piyo<String, bool> for PiyoTest {}

    def_async_di_provider!(dyn Piyo<String, bool>, |_c| async { PiyoTest {} });

    pub trait Piyo2<A, B>: DITarget {}

    struct Piyo2Test {}
    impl Piyo2<String, bool> for Piyo2Test {}
    impl Piyo2<String, ()> for Piyo2Test {}

    #[portaldi::provider(Piyo2<String, bool>)]
    #[async_trait]
    impl AsyncDIPortal for Piyo2Test {
        async fn create_for_di(_container: &DIContainer) -> Self {
            Piyo2Test {}
        }
    }

    def_async_di_provider!(dyn Piyo2<String, ()>, |_| async { Piyo2Test {} });
}
