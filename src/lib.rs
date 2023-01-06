//! **portaldi** is a compile time ergonomic dependency injection library.
//!
//! # features
//! * natively async support
//!   * components and traits must be `thread-safe` (`Sync + Send`).
//!   * asynchronous component creation.
//! * ergonomic apis
//!   * in most cases, you are not aware of containers.
//!   ```
//!   let hoge = HogeService::di();
//!   ```
//! * dry support by macros
//!   * almost boiler code can be generated by proc macros.
//!   ```
//!   #[derive(DIPortal)]
//!   struct HogeService {
//!     foo: DI<FooService>,
//!     ...,
//!   }
//!   ```
//!

pub use portaldi_core::{container::*, traits::*, types::*};
pub use portaldi_macros::*;
