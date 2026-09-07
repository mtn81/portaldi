//! # How to use
//!
//! For less common configuration (`#[inject]` overrides, scoped containers, feature flags,
//! Wasm, environment variables) see [`in_depth`](crate::docs::in_depth).
//!
//! ### Basics
//!
//! * PortalDI handles dependencies as fields (constructor injection). Each dependency must be declared as `DI<T>`.
//!
//! * PortalDI injects dependencies through their corresponding provider types.
//!   * `DI<T>` is resolved by `TProvider`, which implements `portaldi::DIProvider`.
//!
//! * Dependency types must implement `DITarget` and be thread-safe.
//!   * Trait dependencies must have `DITarget` as a supertrait.
//!   ```
//!   use portaldi::*;
//!   trait MyTrait: DITarget { }
//!   ```
//!   * Any type that is `Send + Sync + 'static` implements `DITarget` automatically.
//!
//! * By default, components are treated as lazily-initialized singletons.
//!   * If a component must be initialized in advance, call its `di` method explicitly wherever you need it.
//!
//! ### Struct dependencies
//!
//! When a dependency is a struct, simply annotate the target struct.
//! PortalDI's macro generates a `DIPortal` implementation for the target struct.
//!
//! ```
//! use portaldi::*;
//!
//! #[derive(DIPortal)]
//! struct Hoge {
//!   foo: DI<Foo>, // FooProvider must be in this scope
//!   // other deps
//! }
//!
//! #[derive(DIPortal)]  // DIPortal implementation and FooProvider are generated.
//! struct Foo { /* other deps */ }
//!
//! // Use component
//! Hoge::di();
//!
//! ```
//!
//!
//! ### Trait object dependencies
//!
//! When a dependency is a trait object, annotate the target struct with a `provide` attribute.
//! PortalDI's macro generates a `DIPortal` implementation for the target struct and a `portaldi::DIProvider` implementation for the trait.
//! The dependency's `portaldi::DIProvider` must be in scope where the dependent struct is defined.
//!
//! ```
//! use portaldi::*;
//!
//! #[derive(DIPortal)]
//! struct Hoge {
//!   foo: DI<dyn FooI>, // FooIProvider must be in this scope.
//!   // other deps
//! }
//!
//! pub trait FooI: DITarget {}
//!
//! #[derive(DIPortal)]
//! #[provide(FooI)]
//! struct Foo { /* other deps */ }
//!
//! impl FooI for Foo {}
//!
//! // Use component
//! Hoge::di();
//! // Use FooI component
//! FooIProvider::di();
//!
//! ```
//!
//! ### Custom creation logic
//!
//! When a component needs custom construction logic (for example, a type from an external crate), define a `portaldi::DIProvider` implementation with the `portaldi::def_di_provider!` shorthand macro.
//!
//! #### Struct dependency
//!
//! ```
//! use portaldi::*;
//!
//! #[derive(DIPortal)]
//! struct Hoge {
//!   foo: DI<Foo>, // FooProvider must be in this scope
//!   // other deps
//! }
//!
//! pub struct Foo { /* other deps */ }
//!
//! // you can use the shorthand macro to define a provider.
//! def_di_provider!(Foo, |_c| {
//!     // custom creation logic
//!     Foo {}
//! });
//!
//! // Use component
//! Hoge::di();
//!
//! ```
//!
//! #### Trait object dependency
//! Use the `dyn` keyword with the `portaldi::def_di_provider!` macro.
//!
//! ```
//! use portaldi::*;
//!
//! #[derive(DIPortal)]
//! struct Hoge {
//!   foo: DI<dyn FooI>, // FooIProvider must be in this scope
//!   // other deps
//! }
//!
//! pub trait FooI: DITarget {}
//!
//! struct Foo { /* other deps */ }
//!
//! impl FooI for Foo {}
//!
//! def_di_provider!(dyn FooI, |_c| {
//!     // custom creation logic
//!     Foo {}
//! });
//!
//! // Use component
//! Hoge::di();
//!
//! ```
//!
//! #### Async creation logic
//! Use the `portaldi::def_async_di_provider!` macro.
//! Also annotate the dependency field with `#[inject(async)]`.
//!
//! ```
//! use portaldi::*;
//! use async_trait::async_trait;
//!
//! #[derive(DIPortal)]
//! struct Hoge {
//!   #[inject(async)]
//!   foo: DI<Foo>, // FooProvider must be in this scope
//!   // other deps
//! }
//!
//! pub struct Foo { /* other deps */ }
//!
//! def_async_di_provider!(Foo, |_c| async {
//!     // custom creation logic
//!     Foo {}
//! });
//!
//! async {
//!     // Use component
//!     Hoge::di().await;
//! };
//!
//! ```
//!
//! #### Creation logic that depends on other components
//! If the custom creation logic needs other components,
//! combine the `portaldi::def_di_provider!` macro (or its async variant) with the `portaldi::di!` macro.
//!
//! ```
//! use portaldi::*;
//! use async_trait::async_trait;
//!
//! #[derive(DIPortal)]
//! struct Hoge {
//!     #[inject(async)]
//!     foo: DI<Foo>,
//!     // other deps
//! }
//!
//! pub struct Foo {
//!     bar: DI<Bar>,
//!     // other deps
//! }
//!
//! #[derive(DIPortal)]
//! struct Bar { /* other deps */ }
//!
//! def_async_di_provider!(Foo, |c| async move {
//!     // custom creation logic
//!     Foo {
//!         bar: di![Bar on c], // BarProvider must be in this scope
//!         // other deps
//!     }
//! });
//!
//! async {
//!     // Use component
//!     Hoge::di().await;
//! };
//!
//! ```
//!
