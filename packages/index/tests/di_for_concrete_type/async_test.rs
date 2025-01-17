use std::marker::PhantomData;

use crate::common::*;

#[tokio::test]
async fn test_di() {
    assert!(AHoge::di().await == AHoge::di().await)
}

#[derive(DIPortal, PartialEq)]
struct AHoge {
    foo: DI<Foo>,
    #[inject(async, ABar)]
    bar: DI<ABar>,
    #[inject(ABazProvider, async)]
    baz: DI<ABaz>,
    #[inject(async)]
    yah: DI<AYah>,
    #[inject(async)]
    yah2: DI<AYah2<String, u8>>,
}

#[derive(DIPortal, PartialEq)]
struct Foo {}

#[derive(PartialEq)]
struct ABar {}

#[async_trait]
impl AsyncDIPortal for ABar {
    async fn create_for_di(_container: &portaldi::DIContainer) -> Self {
        ABar {}
    }
}

#[derive(PartialEq)]
pub struct ABaz {}
struct ABazProvider {}
#[async_trait]
impl AsyncDIProvider for ABazProvider {
    type Output = ABaz;
    async fn di_on(container: &DIContainer) -> DI<Self::Output> {
        container.get_or_init_async(|| async { ABaz {} }).await
    }
}

#[derive(PartialEq)]
pub struct AYah {}
async_di_provider!(AYah, |_c| async { AYah {} });

#[derive(PartialEq)]
pub struct AYah2<A, B> {
    a: PhantomData<A>,
    b: PhantomData<B>,
}

// implements manually & self provider attribute
#[provider(self)]
#[async_trait]
impl AsyncDIPortal for AYah2<String, u8> {
    async fn create_for_di(_container: &portaldi::DIContainer) -> Self {
        AYah2 {
            a: PhantomData,
            b: PhantomData,
        }
    }
}
