use std::marker::PhantomData;

use crate::common::*;

#[tokio::test]
async fn test_di() {
    assert!(AHoge::di().await == AHoge::di().await);

    di![AYah2<String, ()>].await;
    let c = &DIContainer::new();
    di![AYah2<String, ()> on c].await;
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
    yah_tagged: DI<Tagged<AYah, String>>,
    #[inject(async)]
    yah2: DI<AYah2<String, u8>>,
    #[inject(async)]
    yah2_unit: DI<AYah2<String, ()>>, // with unit type
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
def_async_di_provider!(AYah, |_c| async { AYah {} });
def_async_di_provider!(Tagged<AYah, String>, |c| async move { Tagged::wrap(di![AYah on c].await) });

#[derive(PartialEq)]
pub struct AYah2<A, B> {
    a: PhantomData<A>,
    b: PhantomData<B>,
}

// implements manually & self provider attribute
#[provider(Self)]
#[async_trait]
impl AsyncDIPortal for AYah2<String, u8> {
    async fn create_for_di(_container: &portaldi::DIContainer) -> Self {
        AYah2 {
            a: PhantomData,
            b: PhantomData,
        }
    }
}

// with unit type
def_async_di_provider!(AYah2<String, ()>, |_c| async {
    AYah2 {
        a: PhantomData,
        b: PhantomData,
    }
});
