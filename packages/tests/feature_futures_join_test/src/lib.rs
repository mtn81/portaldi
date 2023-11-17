use portaldi::*;

#[derive(DIPortal, Debug)]
pub struct Hoge {
    #[inject(async)]
    foo: DI<Foo>,
    #[inject(async)]
    bar: DI<Bar>,
}

#[derive(Debug)]
pub struct Foo {}
#[async_trait::async_trait]
impl AsyncDIPortal for Foo {
    async fn create_for_di(_c: &DIContainer) -> Self {
        Foo {}
    }
}

#[derive(Debug)]
pub struct Bar {}
#[async_trait::async_trait]
impl AsyncDIPortal for Bar {
    async fn create_for_di(_c: &DIContainer) -> Self {
        Bar {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async() {
        let hoge = Hoge::di().await;
        println!("{:?}", hoge)
    }
}
