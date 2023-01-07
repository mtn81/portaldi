//! # TIPS
//!
//! ### Hide `DIProvider` implementation details.
//!
//! portaldi's macro generates `DIProvider` implementation block next to a target struct.
//! And you needs import the DIProvider into dependent's scope.
//!
//! ```
//! mod tips1 {
//!
//!     mod service {
//!         use portaldi::*;
//!
//!         pub trait FooI: DITarget {}
//!         pub trait BarI: DITarget {}
//!     }
//!
//!     mod foo_impl {
//!         use portaldi::*;
//!         use super::service::*;
//!
//!         #[derive(DIPortal)]
//!         #[provide(FooI)]
//!         pub struct Foo {}
//!         impl FooI for Foo {}
//!     }
//!
//!     mod bar_impl {
//!         use portaldi::*;
//!         use super::service::*;
//!
//!         #[derive(DIPortal)]
//!         #[provide(BarI)]
//!         pub struct Bar {}
//!         impl BarI for Bar {}
//!     }
//!
//!     use portaldi::*;
//!     use foo_impl::FooIProvider; // must be in this scope
//!     use bar_impl::BarIProvider; // must be in this scope
//!     use service::*;
//!
//!     #[derive(DIPortal)]
//!     struct Hoge {
//!          foo: DI<dyn FooI>,
//!          bar: DI<dyn BarI>,
//!     }
//! }
//!
//! ```
//!
//! You may want to avoid this implementation imports.
//! In that case, add provider barrel module with re-export to hide detailed imports.
//!
//!
//! ```
//! mod tips1 {
//!
//!     mod service {
//!         use portaldi::*;
//!
//!         pub trait FooI: DITarget {}
//!         pub trait BarI: DITarget {}
//!     }
//!
//!     mod foo_impl {
//!         use portaldi::*;
//!         use super::service::*;
//!
//!         #[derive(DIPortal)]
//!         #[provide(FooI)]
//!         pub struct Foo {}
//!         impl FooI for Foo {}
//!     }
//!
//!     mod bar_impl {
//!         use portaldi::*;
//!         use super::service::*;
//!
//!         #[derive(DIPortal)]
//!         #[provide(BarI)]
//!         pub struct Bar {}
//!         impl BarI for Bar {}
//!     }
//!
//!     mod providers {
//!         pub use super::foo_impl::FooIProvider;
//!         pub use super::bar_impl::BarIProvider;
//!     }
//!
//!     use portaldi::*;
//!     use providers::*;
//!     use service::*;
//!
//!     #[derive(DIPortal)]
//!     struct Hoge {
//!          foo: DI<dyn FooI>,
//!          bar: DI<dyn BarI>,
//!     }
//! }
//!
//! ```
//!
//!
