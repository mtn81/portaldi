use async_trait::async_trait;

use crate::container::DIContainer;
use crate::globals::INSTANCE;
use crate::types::DI;

pub trait DIPortal {
    type Output: Send + Sync + 'static;

    fn di_on(container: &DIContainer) -> DI<Self::Output> {
        container.get_or_init(|| Self::create_for_di(container))
    }

    fn di() -> DI<Self::Output> {
        Self::di_on(&INSTANCE)
    }

    fn create_for_di(container: &DIContainer) -> Self::Output;
}

#[async_trait]
pub trait AsyncDIPortal {
    type Output: Send + Sync + 'static;

    async fn di_on(container: &DIContainer) -> DI<Self::Output> {
        container
            .get_or_init_async(|| Self::create_for_di(container))
            .await
    }

    async fn di() -> DI<Self::Output> {
        Self::di_on(&INSTANCE).await
    }

    async fn create_for_di(container: &DIContainer) -> Self::Output;
}

pub trait DIProvider {
    type Output: ?Sized;

    fn di_on(container: &DIContainer) -> DI<Self::Output>;

    fn di() -> DI<Self::Output> {
        Self::di_on(&INSTANCE)
    }
}

#[async_trait]
pub trait AsyncDIProvider {
    type Output: ?Sized;

    async fn di_on(container: &DIContainer) -> DI<Self::Output>;

    async fn di() -> DI<Self::Output> {
        Self::di_on(&INSTANCE).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    trait FooI {}

    // sync component setup
    struct Hoge;
    impl DIPortal for Hoge {
        type Output = Hoge;
        fn create_for_di(_container: &DIContainer) -> Hoge {
            Hoge {}
        }
    }
    impl FooI for Hoge {}

    struct FooIProvider;
    impl DIProvider for FooIProvider {
        type Output = dyn FooI;
        fn di_on(container: &DIContainer) -> DI<Self::Output> {
            Hoge::di_on(container)
        }
    }

    // async component setup
    struct AsyncHoge;
    #[async_trait]
    impl AsyncDIPortal for AsyncHoge {
        type Output = AsyncHoge;
        async fn create_for_di(_container: &DIContainer) -> AsyncHoge {
            AsyncHoge {}
        }
    }
    impl FooI for AsyncHoge {}

    struct AsyncFooIProvider;
    #[async_trait]
    impl AsyncDIProvider for AsyncFooIProvider {
        type Output = dyn FooI;
        async fn di_on(container: &DIContainer) -> DI<Self::Output> {
            AsyncHoge::di_on(container).await
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_same_instance_for_DIPortal() {
        let hoge1 = Hoge::di().as_ref() as *const _;
        let hoge2 = Hoge::di().as_ref() as *const _;
        assert!(std::ptr::eq(hoge1, hoge2));
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_same_instance_for_DIProvider() {
        let foo1 = FooIProvider::di().as_ref() as *const _;
        let foo2 = FooIProvider::di().as_ref() as *const _;
        assert!(std::ptr::eq(foo1, foo2));
    }

    #[tokio::test]
    #[allow(non_snake_case)]
    async fn test_same_instance_for_AsyncDIPortal() {
        let hoge1 = AsyncHoge::di().await.as_ref() as *const _;
        let hoge2 = AsyncHoge::di().await.as_ref() as *const _;
        assert!(std::ptr::eq(hoge1, hoge2));
    }

    #[tokio::test]
    #[allow(non_snake_case)]
    async fn test_same_instance_for_AsyncDIProvider() {
        let foo1 = AsyncFooIProvider::di().await.as_ref() as *const _;
        let foo2 = AsyncFooIProvider::di().await.as_ref() as *const _;
        assert!(std::ptr::eq(foo1, foo2));
    }
}
