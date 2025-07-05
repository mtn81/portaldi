//! Type definitions.

use std::{marker::PhantomData, ops::Deref};

/// Represents depencency (component) type.
#[cfg(any(not(target_arch = "wasm32"), feature = "multi-thread"))]
pub type DI<T> = std::sync::Arc<T>;
#[cfg(all(target_arch = "wasm32", not(feature = "multi-thread")))]
pub type DI<T> = std::rc::Rc<T>;

/// Tagged type.
///
/// You can distinct same components by a tag.
#[derive(Debug, Clone, PartialEq)]
pub struct Tagged<A: ?Sized, T> {
    target: DI<A>,
    tag: PhantomData<T>,
}

impl<A: ?Sized, T> Tagged<A, T> {
    pub fn wrap(target: DI<A>) -> Self {
        Self {
            target,
            tag: PhantomData,
        }
    }

    pub fn new(target: A) -> Self
    where
        A: Sized,
    {
        Self {
            target: DI::new(target),
            tag: PhantomData,
        }
    }

    pub fn target(&self) -> &DI<A> {
        &self.target
    }
}

impl<A: ?Sized, T> Deref for Tagged<A, T> {
    type Target = DI<A>;

    fn deref(&self) -> &Self::Target {
        &self.target
    }
}

#[cfg(test)]
mod tests {
    use crate::container::DIContainer;

    use super::*;

    #[test]
    fn test_usage() {
        struct Hoge {}
        impl Hoge {
            fn hello(&self) {}
        }

        let t: DI<Tagged<Hoge, String>> = DI::new(Tagged::new(Hoge {}));
        let _: &DI<Hoge> = t.target();
        let _ = t.hello();
    }

    #[test]
    fn test_usage_for_trait() {
        trait HogeI {
            fn hello(&self);
        }
        struct Hoge {}
        impl HogeI for Hoge {
            fn hello(&self) {}
        }

        let t: DI<Tagged<dyn HogeI, String>> = DI::new(Tagged::wrap(DI::new(Hoge {})));
        let _: &DI<dyn HogeI> = t.target();
        let _ = t.hello();
    }

    #[test]
    fn test_multiple_tags_on_container() {
        struct Hoge {}

        let c = DIContainer::new();

        let t1: DI<Tagged<Hoge, String>> = DI::new(Tagged::new(Hoge {}));
        c.put_if_absent(&t1);

        let t2: DI<Tagged<Hoge, bool>> = DI::new(Tagged::new(Hoge {}));
        c.put_if_absent(&t2);

        let r1 = c.get::<Tagged<Hoge, String>>().unwrap();
        let r2 = c.get::<Tagged<Hoge, bool>>().unwrap();
        assert!(!std::ptr::eq(
            r1.target().as_ref() as *const _,
            r2.target().as_ref() as *const _
        ));
        assert!(std::ptr::eq(
            r1.as_ref() as *const _,
            t1.as_ref() as *const _
        ));
        assert!(std::ptr::eq(
            r2.as_ref() as *const _,
            t2.as_ref() as *const _
        ));
    }
}
