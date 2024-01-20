//! DI container functionality.

use crate::{traits::DITarget, types::DI};
use std::{any::Any, collections::HashMap, future::Future};

#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Mutex;

/// DI container holds component refs.
#[derive(Debug)]
pub struct DIContainer {
    /// Hold components by its type name (FQTN).
    #[cfg(target_arch = "wasm32")]
    components: RefCell<HashMap<String, DI<dyn Any>>>,
    #[cfg(not(target_arch = "wasm32"))]
    components: Mutex<HashMap<String, DI<dyn Any + Send + Sync>>>,
}

impl DIContainer {
    /// Create new instance.
    pub fn new() -> DIContainer {
        DIContainer {
            #[cfg(target_arch = "wasm32")]
            components: RefCell::new(HashMap::new()),
            #[cfg(not(target_arch = "wasm32"))]
            components: Mutex::new(HashMap::new()),
        }
    }

    /// Get a component by type.
    pub fn get<T: DITarget>(&self) -> Option<DI<T>> {
        #[cfg(target_arch = "wasm32")]
        let comps = self.components.borrow();
        #[cfg(not(target_arch = "wasm32"))]
        let comps = self.components.lock().unwrap();
        comps
            .get(std::any::type_name::<T>())
            .map(|c| c.clone().downcast::<T>().unwrap())
    }

    /// Put a component into the container.
    pub fn put_if_absent<T: DITarget>(&self, c: &DI<T>) -> DI<T> {
        #[cfg(target_arch = "wasm32")]
        let mut components = self.components.borrow_mut();
        #[cfg(not(target_arch = "wasm32"))]
        let mut components = self.components.lock().unwrap();
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
