//! # How to use
//!
//! ### Basics
//!
//! * PortalDI handle a dependency as a field (constructor injection). Dependencies must be specified as `DI<T>`.
//!
//! * PortalDI injects depencencies via corresponding Provider types.
//!   * DI<T> is resolved by TProvider which implements `portaldi::DIProvider`.
//!
//! * Depencency types must implement `DITarget` and be thread safe.
//!   * Trait dependencies must be DITarget.
//!   ```
//!   use portaldi::*;
//!   trait MyTrait: DITarget { }
//!   ```
//!   * Structs automatically become DITarget when its fileds are `Send + Sync`.
//!
//! * In PortalDI, components are handled as singleton and with lazy initioalization by default.
//!   * If a component must be initialized in advance, you can explicitly call `di` method in where you want.
//!   * If a component must be prototype (1 instance by 1 ref), you can annotate with `prototype`.
//!
//! ### Use struct dependencies
//!
//! When a dependency is a struct, you can simply annotate on a target.
//! The PortalDI's macro generates DIPortal implementation for a target struct.
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
//! ### Use trait object dependencies
//!
//! When a dependency is a trait object, you can annotate a target struct with a `provide` attribute.
//! The PortalDI's macro generates DIPortal implementation for the target struct and `portaldi::DIProvider` implementation for the trait.
//! The dependent struct's scope must have the depencency `portaldi::DIProvider`.
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
//! ### Create components via manual creation logic.
//!
//! When you need a custom creation logic for a compoonent (ex. components from external library), you can define a `portaldi::DIProvider` implementation with shorthand macro `portaldi::def_di_provider`.
//!
//! #### For struct dependency
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
//! #### For trait object depencency
//! Use the dyn keyword with `portaldi::def_di_provider` macro.
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
//! #### For async creation logic
//! You can use `portaldi::def_async_di_provider` macro.
//! Also you need anotate `inject` with `async` on the depencency field.
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
//! #### For complex creation logic that involves other components.
//! If a depencency has custom creation logic that needs other components,
//! you can use acombination of `portaldi::def_di_provider` and `portaldi::di`.
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
//! def_async_di_provider!(Foo, |c| {
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
