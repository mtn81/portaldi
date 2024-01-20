//! Type definitions.

/// Represents depencency (component) type.
#[cfg(not(feature = "wasm"))]
pub type DI<T> = std::sync::Arc<T>;
#[cfg(feature = "wasm")]
pub type DI<T> = std::rc::Rc<T>;
