use crate::common::*;

#[tokio::test]
async fn test_di() {
    let hoge = Hoge::di().await;
    assert!(!ptr_eq(hoge.foo1.as_ref(), hoge.foo2.as_ref()));
}

use bar::*;
use baz::*;
use foo::*;
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

    async_di_provider!(dyn Baz, |_c| async { BazTest {} });

    pub trait Baz2<A, B>: DITarget {}

    struct Baz2Test {}
    impl Baz2<String, bool> for Baz2Test {}

    async_di_provider!(dyn Baz2<String, bool>, |_c| async { Baz2Test {} });
}
