//! Global variables.

use crate::{container::DIContainer, traits::DITarget, types::DI};

/// Global container instance.
#[cfg(any(not(target_arch = "wasm32"), feature = "multi-thread"))]
pub(crate) static INSTANCE: std::sync::LazyLock<DIContainer> =
    std::sync::LazyLock::new(DIContainer::new);
#[cfg(all(target_arch = "wasm32", not(feature = "multi-thread")))]
thread_local! {
    pub(crate) static INSTANCE: std::rc::Rc<DIContainer> = std::rc::Rc::new(DIContainer::new());
}

pub fn get<T: DITarget>() -> Option<DI<T>> {
    #[cfg(any(not(target_arch = "wasm32"), feature = "multi-thread"))]
    let result = INSTANCE.get::<T>();

    #[cfg(all(target_arch = "wasm32", not(feature = "multi-thread")))]
    let result = INSTANCE.with(|c| c.get::<T>());

    result
}

#[cfg(test)]
mod tests {
    use crate::traits::DIPortal;

    use super::*;

    #[test]
    fn test_get_unregistered() {
        struct Hoge {}

        assert!(get::<Hoge>().is_none());
    }

    #[test]
    fn test_get_registered() {
        struct Hoge {}
        impl DIPortal for Hoge {
            fn create_for_di(_: &DIContainer) -> Self {
                Hoge {}
            }
        }

        let hoge = Hoge::di();
        assert!(DI::ptr_eq(&get::<Hoge>().unwrap(), &hoge));
    }
}
