//! Type definitions.

/// Represents depencency (component) type.
#[cfg(any(not(target_arch = "wasm32"), feature = "multi-thread"))]
pub type DI<T> = std::sync::Arc<T>;
#[cfg(all(target_arch = "wasm32", not(feature = "multi-thread")))]
pub type DI<T> = std::rc::Rc<T>;
