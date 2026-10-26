//! DI container functionality.

use crate::{traits::DITarget, types::DI};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    future::Future,
};

#[cfg(all(target_arch = "wasm32", not(feature = "multi-thread")))]
use std::cell::RefCell;
#[cfg(any(not(target_arch = "wasm32"), feature = "multi-thread"))]
use std::sync::RwLock;

/// DI container holds component refs.
#[derive(Debug)]
pub struct DIContainer {
    /// Hold components by its TypeId.
    #[cfg(all(target_arch = "wasm32", not(feature = "multi-thread")))]
    components: RefCell<HashMap<TypeId, DI<dyn Any>>>,
    #[cfg(any(not(target_arch = "wasm32"), feature = "multi-thread"))]
    components: RwLock<HashMap<TypeId, DI<dyn Any + Send + Sync>>>,
}

impl Default for DIContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl DIContainer {
    /// Create new instance.
    pub fn new() -> DIContainer {
        DIContainer {
            #[cfg(all(target_arch = "wasm32", not(feature = "multi-thread")))]
            components: RefCell::new(HashMap::new()),
            #[cfg(any(not(target_arch = "wasm32"), feature = "multi-thread"))]
            components: RwLock::new(HashMap::new()),
        }
    }

    /// Get a component by type.
    pub fn get<T: DITarget>(&self) -> Option<DI<T>> {
        #[cfg(all(target_arch = "wasm32", not(feature = "multi-thread")))]
        let comps = self.components.borrow();
        #[cfg(any(not(target_arch = "wasm32"), feature = "multi-thread"))]
        let comps = self.components.read().unwrap();
        comps
            .get(&TypeId::of::<T>())
            .map(|c| c.clone().downcast::<T>().unwrap())
    }

    /// Put a component into the container.
    pub fn put_if_absent<T: DITarget>(&self, c: &DI<T>) -> DI<T> {
        #[cfg(all(target_arch = "wasm32", not(feature = "multi-thread")))]
        let mut components = self.components.borrow_mut();
        #[cfg(any(not(target_arch = "wasm32"), feature = "multi-thread"))]
        let mut components = self.components.write().unwrap();
        let key = TypeId::of::<T>();
        let value = components
            .get(&key)
            .map(|c| c.clone().downcast::<T>().unwrap());
        if let Some(c) = value {
            c
        } else {
            components.insert(key, c.clone());
            c.clone()
        }
    }

    /// Get a component by type with a initialization.
    /// If a target component does not exists, create and put into the container.
    pub fn get_or_init<T, F>(&self, init: F) -> DI<T>
    where
        T: DITarget,
        F: Fn() -> T,
    {
        if let Some(c) = self.get::<T>() {
            c
        } else {
            let c = DI::new(init());
            self.put_if_absent(&c)
        }
    }

    /// Get a component by type with a async initialization.
    /// If a target component does not exists, create and put into the container.
    pub async fn get_or_init_async<T, F, Fut>(&self, init: F) -> DI<T>
    where
        T: DITarget,
        F: Fn() -> Fut,
        Fut: Future<Output = T>,
    {
        if let Some(c) = self.get::<T>() {
            c
        } else {
            let v = init().await;
            let c = DI::new(v);
            self.put_if_absent(&c)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    #[test]
    fn test_default_is_empty() {
        struct Hoge {}

        let c = DIContainer::default();
        assert!(c.get::<Hoge>().is_none());
    }

    #[test]
    fn test_put_if_absent_keeps_first() {
        struct Hoge {}

        let c = DIContainer::new();
        let first = DI::new(Hoge {});
        let second = DI::new(Hoge {});

        assert!(DI::ptr_eq(&c.put_if_absent(&first), &first));
        assert!(DI::ptr_eq(&c.put_if_absent(&second), &first));
        assert!(DI::ptr_eq(&c.get::<Hoge>().unwrap(), &first));
    }

    #[test]
    fn test_get_or_init_inits_once() {
        struct Hoge {}

        let c = DIContainer::new();
        let count = AtomicUsize::new(0);
        let init = || {
            count.fetch_add(1, Ordering::SeqCst);
            Hoge {}
        };

        let r1 = c.get_or_init(init);
        let r2 = c.get_or_init(init);
        assert!(DI::ptr_eq(&r1, &r2));
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_get_or_init_async_inits_once() {
        struct Hoge {}

        let c = DIContainer::new();
        let count = AtomicUsize::new(0);
        let init = || async {
            count.fetch_add(1, Ordering::SeqCst);
            Hoge {}
        };

        let r1 = c.get_or_init_async(init).await;
        let r2 = c.get_or_init_async(init).await;
        assert!(DI::ptr_eq(&r1, &r2));
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }
}
