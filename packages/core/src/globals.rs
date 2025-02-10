//! Global variables.

use crate::container::DIContainer;
#[cfg(any(not(target_arch = "wasm32"), feature = "multi-thread"))]
use once_cell::sync::Lazy;

/// Global container instance.
#[cfg(any(not(target_arch = "wasm32"), feature = "multi-thread"))]
pub(crate) static INSTANCE: Lazy<DIContainer> = Lazy::new(|| DIContainer::new());
#[cfg(all(target_arch = "wasm32", not(feature = "multi-thread")))]
thread_local! {
    pub(crate) static INSTANCE: std::rc::Rc<DIContainer> = std::rc::Rc::new(DIContainer::new());
}
