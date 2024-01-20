//! Global variables.

#[cfg(not(target_arch = "wasm32"))]
use crate::container::DIContainer;
#[cfg(not(target_arch = "wasm32"))]
use once_cell::sync::Lazy;

#[cfg(not(target_arch = "wasm32"))]
/// Global container instance.
pub(crate) static INSTANCE: Lazy<DIContainer> = Lazy::new(|| DIContainer::new());
