use portaldi::*;

#[derive(DIPortal, Debug)]
pub struct Hoge {
    foo: DI<Foo>,
}

#[derive(DIPortal, Debug)]
pub struct Foo {}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async() {
        let hoge = Hoge::di().await;
        println!("{:?}", hoge)
    }
}
