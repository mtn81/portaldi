use std::sync::Arc;

pub type DI<T: ?Sized> = Arc<T>;
