//! DI container functionality.

use crate::{traits::DITarget, types::DI};
use std::{any::Any, collections::HashMap, future::Future};

#[cfg(all(target_arch = "wasm32", not(feature = "multi-thread")))]
use std::cell::RefCell;
#[cfg(any(not(target_arch = "wasm32"), feature = "multi-thread"))]
use std::sync::RwLock;

/// DI container holds component refs.
#[derive(Debug)]
pub struct DIContainer {
    /// Hold components by its type name (FQTN).
    #[cfg(all(target_arch = "wasm32", not(feature = "multi-thread")))]
    components: RefCell<HashMap<String, DI<dyn Any>>>,
    #[cfg(any(not(target_arch = "wasm32"), feature = "multi-thread"))]
    components: RwLock<HashMap<String, DI<dyn Any + Send + Sync>>>,
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
            .get(std::any::type_name::<T>())
            .map(|c| c.clone().downcast::<T>().unwrap())
    }

    /// Put a component into the container.
    pub fn put_if_absent<T: DITarget>(&self, c: &DI<T>) -> DI<T> {
        #[cfg(all(target_arch = "wasm32", not(feature = "multi-thread")))]
        let mut components = self.components.borrow_mut();
        #[cfg(any(not(target_arch = "wasm32"), feature = "multi-thread"))]
        let mut components = self.components.write().unwrap();
        let key = std::any::type_name::<T>();
        let value = components
            .get(key)
            .map(|c| c.clone().downcast::<T>().unwrap());
        if let Some(c) = value {
            c
        } else {
            components.insert(key.into(), c.clone());
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
