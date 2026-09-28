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
