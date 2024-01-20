//! Type definitions.

/// Represents depencency (component) type.
#[cfg(not(target_arch = "wasm32"))]
pub type DI<T> = std::sync::Arc<T>;
#[cfg(target_arch = "wasm32")]
pub type DI<T> = std::rc::Rc<T>;
