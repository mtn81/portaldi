pub(crate) use async_trait::async_trait;
pub(crate) use portaldi::*;
pub(crate) use tokio;

pub(crate) fn ptr_eq<T: ?Sized>(ref1: &T, ref2: &T) -> bool {
    std::ptr::eq(ref1 as *const _, ref2 as *const _)
}
