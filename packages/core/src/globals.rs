//! Global variables.

use crate::container::DIContainer;
#[cfg(any(not(target_arch = "wasm32"), feature = "multi-thread"))]
use std::sync::LazyLock;

/// Global container instance.
#[cfg(any(not(target_arch = "wasm32"), feature = "multi-thread"))]
pub(crate) static INSTANCE: LazyLock<DIContainer> = LazyLock::new(DIContainer::new);
#[cfg(all(target_arch = "wasm32", not(feature = "multi-thread")))]
thread_local! {
    pub(crate) static INSTANCE: std::rc::Rc<DIContainer> = std::rc::Rc::new(DIContainer::new());
}
