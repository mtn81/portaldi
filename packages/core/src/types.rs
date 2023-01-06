use std::sync::Arc;

/// Represents depencency (component) type.
pub type DI<T> = Arc<T>;
