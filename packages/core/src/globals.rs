use crate::container::DIContainer;
use once_cell::sync::Lazy;

pub(crate) static INSTANCE: Lazy<DIContainer> = Lazy::new(|| DIContainer::new());
