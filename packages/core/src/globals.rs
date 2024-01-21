//! Global variables.

use crate::container::DIContainer;
#[cfg(not(target_arch = "wasm32"))]
use once_cell::sync::Lazy;

/// Global container instance.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) static INSTANCE: Lazy<DIContainer> = Lazy::new(|| DIContainer::new());
#[cfg(target_arch = "wasm32")]
thread_local! {
    pub(crate) static INSTANCE: std::rc::Rc<DIContainer> = std::rc::Rc::new(DIContainer::new());
}
