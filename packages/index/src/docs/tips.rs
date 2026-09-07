//! # Tips
//!
//! ### Hide `DIProvider` implementation details.
//!
//! The PortalDI's macro generates `DIProvider` implementation block next to a target struct.
//! And you needs import the DIProvider into dependent's scope.
//!
//! ```
//! mod service {
//!     use portaldi::*;
//!
//!     pub trait FooI: DITarget {}
//!     pub trait BarI: DITarget {}
//! }
//!
//! mod foo_impl {
//!     use portaldi::*;
//!     use super::service::*;
//!
//!     #[derive(DIPortal)]
//!     #[provide(FooI)]
//!     pub struct Foo {}
//!     impl FooI for Foo {}
//! }
//!
//! mod bar_impl {
//!     use portaldi::*;
//!     use super::service::*;
//!
//!     #[derive(DIPortal)]
//!     #[provide(BarI)]
//!     pub struct Bar {}
//!     impl BarI for Bar {}
//! }
//!
//! use portaldi::*;
//! use foo_impl::FooIProvider; // must be in this scope
//! use bar_impl::BarIProvider; // must be in this scope
//! use service::*;
//!
//! #[derive(DIPortal)]
//! struct Hoge {
//!      foo: DI<dyn FooI>,
//!      bar: DI<dyn BarI>,
//! }
//! ```
//!
//! You may want to avoid this implementation imports.
//! In that case, add provider barrel module with re-export to hide detailed imports.
//! Also you can switch multiple barrel modules via cfg.
//!
//! ```
//! mod service {
//!     use portaldi::*;
//!
//!     pub trait FooI: DITarget {}
//!     pub trait BarI: DITarget {}
//! }
//!
//! mod foo_impl {
//!     use portaldi::*;
//!     use super::service::*;
//!
//!     #[derive(DIPortal)]
//!     #[provide(FooI)]
//!     pub struct Foo {}
//!     impl FooI for Foo {}
//! }
//!
//! mod bar_impl {
//!     use portaldi::*;
//!     use super::service::*;
//!
//!     #[derive(DIPortal)]
//!     #[provide(BarI)]
//!     pub struct Bar {}
//!     impl BarI for Bar {}
//! }
//!
//! mod providers {
//!     pub use super::foo_impl::FooIProvider;
//!     pub use super::bar_impl::BarIProvider;
//! }
//!
//! use portaldi::*;
//! use providers::*;
//! use service::*;
//!
//! #[derive(DIPortal)]
//! struct Hoge {
//!      foo: DI<dyn FooI>,
//!      bar: DI<dyn BarI>,
//! }
//! ```
//!
//! ### Use multiple components with a same type.
//! You can define multiple components with tags using `portaldi::Tagged`.
//!
//! ```
//! use portaldi::*;
//!
//! struct Tag1;
//! struct Tag2;
//!
//! struct Foo {
//!     // deps
//! }
//!
//! def_di_provider!(Tagged<Foo, Tag1>, |c| Tagged::wrap(di![Foo on c]));
//! def_di_provider!(Tagged<Foo, Tag2>, |_c| Tagged::new(create_another_foo()));
//!
//! #[derive(DIPortal)]
//! struct Hoge {
//!     foo1: DI<Tagged<Foo, Tag1>>,
//!     foo2: DI<Tagged<Foo, Tag2>>,
//!     // other deps
//! }
//! ```
