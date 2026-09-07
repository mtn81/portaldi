//! # Tips
//!
//! ### Hide `DIProvider` implementation details
//!
//! PortalDI's macros generate a `DIProvider` implementation next to the target struct.
//! You then need to import that provider into the scope of the dependent struct.
//!
//! ```
//! mod demo {
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
//! ```
//!
//! You may want to avoid importing these implementation details directly.
//! In that case, add a *barrel* module that re-exports the providers to hide the detailed imports.
//! You can also swap between several barrel modules with `#[cfg(...)]`.
//!
//! ```
//! mod demo {
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
//! ```
//!
//! ### Multiple components of the same type
//! Use `portaldi::Tagged<T, Tag>` to register several components that share the same underlying type.
//!
//! ```
//! mod demo {
//!     use portaldi::*;
//!
//!     pub struct Tag1;
//!     pub struct Tag2;
//!
//!     #[derive(DIPortal)]
//!     pub struct Foo {
//!         // deps
//!     }
//!
//!     fn create_another_foo() -> Foo { unimplemented!() }
//!
//!     def_di_provider!(Tagged<Foo, Tag1>, |c| Tagged::wrap(di![Foo on c]));
//!     def_di_provider!(Tagged<Foo, Tag2>, |_c| Tagged::new(create_another_foo()));
//!
//!     #[derive(DIPortal)]
//!     struct Hoge {
//!         foo1: DI<Tagged<Foo, Tag1>>,
//!         foo2: DI<Tagged<Foo, Tag2>>,
//!         // other deps
//!     }
//! }
//! ```
