use crate::common::*;

#[tokio::test]
async fn test_di() {
    assert!(AHoge::di().await == AHoge::di().await)
}

#[derive(DIPortal, PartialEq)]
struct AHoge {
    foo: DI<Foo>,
    #[inject(async)]
    bar: DI<ABar>,
    #[inject(ABazProvider, async)]
    baz: DI<ABaz>,
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
struct ABaz {}

struct ABazProvider {}
#[async_trait]
impl AsyncDIProvider for ABazProvider {
    type Output = ABaz;
    async fn di_on(container: &DIContainer) -> DI<Self::Output> {
        container.get_or_init(|| ABaz {})
    }
}
