use std::future::Future;

use container::DIContainer;
use once_cell::sync::Lazy;
use types::DI;

mod container;
mod types;

static DI_CONTAINER: Lazy<DIContainer> = Lazy::new(|| DIContainer::new());

pub fn get_or_init<T, F>(init: F) -> DI<T>
where
    T: Send + Sync + 'static,
    F: Fn() -> T,
{
    DI_CONTAINER.get_or_init(init)
}

pub async fn get_or_init_async<T, F, Fut>(init: F) -> DI<T>
where
    T: Send + Sync + 'static,
    F: Fn() -> Fut,
    Fut: Future<Output = T>,
{
    DI_CONTAINER.get_or_init_async(init).await
}
