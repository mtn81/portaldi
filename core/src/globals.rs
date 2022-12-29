use std::future::Future;

use crate::container::DIContainer;
use crate::types::DI;
use once_cell::sync::Lazy;

pub(crate) static INSTANCE: Lazy<DIContainer> = Lazy::new(|| DIContainer::new());

pub fn get_or_init<T, F>(init: F) -> DI<T>
where
    T: Send + Sync + 'static,
    F: Fn() -> T,
{
    INSTANCE.get_or_init(init)
}

pub async fn get_or_init_async<T, F, Fut>(init: F) -> DI<T>
where
    T: Send + Sync + 'static,
    F: Fn() -> Fut,
    Fut: Future<Output = T>,
{
    INSTANCE.get_or_init_async(init).await
}
