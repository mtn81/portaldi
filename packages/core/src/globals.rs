//! Global variables.

#[cfg(not(feature = "wasm"))]
use crate::container::DIContainer;
#[cfg(not(feature = "wasm"))]
use once_cell::sync::Lazy;

#[cfg(not(feature = "wasm"))]
/// Global container instance.
pub(crate) static INSTANCE: Lazy<DIContainer> = Lazy::new(|| DIContainer::new());
