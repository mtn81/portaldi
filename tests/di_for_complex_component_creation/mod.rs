use crate::common::*;

#[tokio::test]
async fn test_di() {
    assert!(Hoge::di().await == Hoge::di().await)
}

#[derive(PartialEq)]
struct Hoge {
    foo: DI<Foo>,
}
#[derive(DIPortal, PartialEq)]
struct Foo {}

#[async_trait]
impl AsyncDIPortal for Hoge {
    async fn create_for_di(container: &DIContainer) -> Self {
        HogeFactory::di_on(container).create().await
    }
}

#[derive(DIPortal)]
struct HogeFactory {
    foo: DI<Foo>,
}
impl HogeFactory {
    async fn create(&self) -> Hoge {
        Hoge {
            foo: self.foo.clone(),
        }
    }
}
