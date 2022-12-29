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

pub trait DIProvider {
    type Output: ?Sized;

    fn di_on(container: &DIContainer) -> DI<Self::Output>;

    fn di() -> DI<Self::Output> {
        Self::di_on(&INSTANCE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Hoge;

    impl DIPortal for Hoge {
        type Output = Hoge;
        fn create_for_di(_container: &DIContainer) -> Hoge {
            Hoge {}
        }
    }

    trait FooI {}
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
}
