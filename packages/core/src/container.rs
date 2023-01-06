use crate::{traits::DITarget, types::DI};
use std::{any::Any, collections::HashMap, future::Future, sync::Mutex};

pub struct DIContainer {
    components: Mutex<HashMap<String, DI<dyn Any + Send + Sync>>>,
}

impl DIContainer {
    pub fn new() -> DIContainer {
        DIContainer {
            components: Mutex::new(HashMap::new()),
        }
    }

    pub fn get<T: DITarget>(&self) -> Option<DI<T>> {
        self.components
            .lock()
            .unwrap()
            .get(std::any::type_name::<T>())
            .map(|c| c.clone().downcast::<T>().unwrap())
    }

    pub fn get_or_init<T, F>(&self, init: F) -> DI<T>
    where
        T: DITarget,
        F: Fn() -> T,
    {
        if let Some(c) = self.get::<T>() {
            c
        } else {
            let c = DI::new(init());
            self.put(&c);
            c
        }
    }

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
            self.put(&c);
            c
        }
    }

    pub fn put<T: DITarget>(&self, c: &DI<T>) {
        self.components
            .lock()
            .unwrap()
            .insert(std::any::type_name::<T>().into(), c.clone());
    }
}
