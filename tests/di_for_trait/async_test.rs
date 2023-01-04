use crate::common::*;

#[tokio::test]
async fn test_di() {
    let hoge = Hoge::di().await;
    assert!(!ptr_eq(hoge.foo1.as_ref(), hoge.foo2.as_ref()));
}

use bar::*;
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
    bar1: DI<dyn Bar>,
}

mod foo {
    use super::*;
    pub trait Foo: Sync + Send {}

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
    pub trait Bar: Sync + Send {}

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
