use crate::container::DIContainer;
use crate::globals::INSTANCE;
use crate::types::DI;

pub trait DIPortal {
    type Output: Sized + Send + Sync + 'static;

    fn di_on(container: &DIContainer) -> DI<Self::Output> {
        container.get_or_init(|| Self::create_for_di(container))
    }

    fn di() -> DI<Self::Output> {
        Self::di_on(&INSTANCE)
    }

    fn create_for_di(container: &DIContainer) -> Self::Output;
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

    #[test]
    fn test_same_instance() {
        let hoge1 = Hoge::di().as_ref() as *const _;
        let hoge2 = Hoge::di().as_ref() as *const _;
        assert!(std::ptr::eq(hoge1, hoge2));
    }
}
