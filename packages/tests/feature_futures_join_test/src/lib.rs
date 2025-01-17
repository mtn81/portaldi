use portaldi::*;

#[derive(DIPortal, Debug)]
pub struct Hoge {
    #[inject(async)]
    foo: DI<Foo>,
    #[inject(async)]
    bar: DI<Bar>,
}
#[derive(DIPortal, Debug)]
pub struct Hoge2 {
    #[inject(async)]
    foo: DI<Foo>,
    baz: DI<Baz>,
}
#[derive(DIPortal, Debug)]
pub struct Hoge3 {
    baz: DI<Baz>,
}

#[derive(Debug)]
pub struct Foo {}

#[provider(Self)]
#[async_trait::async_trait]
impl AsyncDIPortal for Foo {
    async fn create_for_di(_c: &DIContainer) -> Self {
        Foo {}
    }
}

#[derive(Debug)]
pub struct Bar {}

#[provider(Self)]
#[async_trait::async_trait]
impl AsyncDIPortal for Bar {
    async fn create_for_di(_c: &DIContainer) -> Self {
        Bar {}
    }
}

#[derive(DIPortal, Debug)]
pub struct Baz {}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async() {
        println!("{:?}", Hoge::di().await);
        println!("{:?}", Hoge2::di().await);
        println!("{:?}", Hoge3::di());
    }
}
