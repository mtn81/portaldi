//! Define traits that expose DI apis to user.

use async_trait::async_trait;

use crate::container::DIContainer;
#[cfg(not(feature = "wasm"))]
use crate::globals::INSTANCE;
use crate::types::DI;

/// Represent DI target type.
/// It requires thread safety.
#[cfg(feature = "wasm")]
pub trait DITarget: 'static {}
#[cfg(not(feature = "wasm"))]
pub trait DITarget: Send + Sync + 'static {}

#[cfg(feature = "wasm")]
impl<T: 'static> DITarget for T {}
#[cfg(not(feature = "wasm"))]
impl<T: Send + Sync + 'static> DITarget for T {}

/// Add `di` methods for DI target types.
pub trait DIPortal {
    /// DI on a container.
    fn di_on(container: &DIContainer) -> DI<Self>
    where
        Self: Sized + DITarget,
    {
        container.get_or_init(|| Self::create_for_di(container))
    }

    #[cfg(not(feature = "wasm"))]
    /// DI on the global container.
    fn di() -> DI<Self>
    where
        Self: Sized + DITarget,
    {
        Self::di_on(&INSTANCE)
    }

    /// Create new instance for DI.
    fn create_for_di(container: &DIContainer) -> Self;
}

/// Add `di` methods for DI target types that needs async creation.
#[cfg_attr(feature="wasm", async_trait(?Send))]
#[cfg_attr(not(feature = "wasm"), async_trait)]
pub trait AsyncDIPortal {
    /// DI on a container.
    async fn di_on(container: &DIContainer) -> DI<Self>
    where
        Self: Sized + DITarget,
    {
        container
            .get_or_init_async(|| Self::create_for_di(container))
            .await
    }

    #[cfg(not(feature = "wasm"))]
    /// DI on the global container.
    async fn di() -> DI<Self>
    where
        Self: Sized + DITarget,
    {
        Self::di_on(&INSTANCE).await
    }

    /// Create new instance for DI.
    async fn create_for_di(container: &DIContainer) -> Self;
}

/// Provides component instance for trait DI types.
pub trait DIProvider {
    /// Target trait type.
    type Output: ?Sized;

    /// DI on a container.
    fn di_on(container: &DIContainer) -> DI<Self::Output>;

    #[cfg(not(feature = "wasm"))]
    /// DI on the global container.
    fn di() -> DI<Self::Output> {
        Self::di_on(&INSTANCE)
    }
}

/// Provides component instance for trait DI types that needs async creation.
#[cfg_attr(feature="wasm", async_trait(?Send))]
#[cfg_attr(not(feature = "wasm"), async_trait)]
pub trait AsyncDIProvider {
    /// Target trait type.
    type Output: ?Sized;

    /// DI on a container.
    async fn di_on(container: &DIContainer) -> DI<Self::Output>;

    #[cfg(not(feature = "wasm"))]
    /// DI on the global container.
    async fn di() -> DI<Self::Output> {
        Self::di_on(&INSTANCE).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    trait FooI {}

    mod sync_test {
        use super::*;

        struct Hoge;
        impl DIPortal for Hoge {
            fn create_for_di(_container: &DIContainer) -> Self {
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

        #[test]
        #[allow(non_snake_case)]
        fn test_same_instance_for_DIPortal() {
            let c = DIContainer::new();
            let hoge1 = Hoge::di_on(&c).as_ref() as *const _;
            let hoge2 = Hoge::di_on(&c).as_ref() as *const _;
            assert!(std::ptr::eq(hoge1, hoge2));
        }

        #[test]
        #[allow(non_snake_case)]
        fn test_same_instance_for_DIProvider() {
            let c = DIContainer::new();
            let foo1 = FooIProvider::di_on(&c).as_ref() as *const _;
            let foo2 = FooIProvider::di_on(&c).as_ref() as *const _;
            assert!(std::ptr::eq(foo1, foo2));
        }
    }

    mod async_test {
        use super::*;

        struct AsyncHoge;
        #[async_trait]
        impl AsyncDIPortal for AsyncHoge {
            async fn create_for_di(_container: &DIContainer) -> Self {
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

        #[tokio::test]
        #[allow(non_snake_case)]
        async fn test_same_instance_for_AsyncDIPortal() {
            let c = DIContainer::new();
            let hoge1 = AsyncHoge::di_on(&c).await.as_ref() as *const _;
            let hoge2 = AsyncHoge::di_on(&c).await.as_ref() as *const _;
            println!("check !!! {:?} {:?}", hoge1, hoge2);
            assert!(std::ptr::eq(hoge1, hoge2));
        }

        #[tokio::test]
        #[allow(non_snake_case)]
        async fn test_same_instance_for_AsyncDIProvider() {
            let c = DIContainer::new();
            let foo1 = AsyncFooIProvider::di_on(&c).await.as_ref() as *const _;
            let foo2 = AsyncFooIProvider::di_on(&c).await.as_ref() as *const _;
            println!("check !!! {:?} {:?}", foo1, foo2);
            assert!(std::ptr::eq(foo1, foo2));
        }
    }
}
