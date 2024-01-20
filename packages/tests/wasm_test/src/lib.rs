use portaldi::*;
use async_trait::async_trait;

#[derive(DIPortal, Debug)]
pub struct Hoge {
    foo: DI<Foo>,
}

#[derive(DIPortal, Debug)]
pub struct Foo {}


#[derive(DIPortal, Debug)]
pub struct AsyncHoge {
    #[inject(async)]
    foo: DI<AsyncFoo>,
}

#[derive(Debug)]
pub struct AsyncFoo {}

#[async_trait(?Send)]
impl AsyncDIPortal for AsyncFoo {
    async fn create_for_di(_container: &portaldi::DIContainer) -> Self {
        AsyncFoo {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync() {
        let c = DIContainer::new();
        let hoge = Hoge::di_on(&c);
        println!("{:?}", hoge)
    }

    #[tokio::test]
    async fn test_async() {
        let c = DIContainer::new();
        let hoge = AsyncHoge::di_on(&c).await;
        println!("{:?}", hoge)
    }
}
