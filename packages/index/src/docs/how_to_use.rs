//! # How to use
//!
//! ### Basics
//!
//! * portaldi handle a dependency as a field (constructor injection). Dependencies must be specified as `DI<T>`.
//!
//! * Depencency types must implement `DITarget` and be thread safe.
//!   * Traits must be DITarget.
//!   ```
//!   use portaldi::*;
//!   trait MyTrait: DITarget { }
//!   ```
//!   * Structs automatically become DITarget when its fileds are `Send + Sync`.
//!
//! * In portaldi, components are handled as singleton and with lazy initioalization by default.
//!   * If a component must be initialized in advance, you can explicitly call `di` method in where you want.
//!   * If a component must be prototype (1 instance by 1 ref), you can annotate with `prototype`.
//!
//! ### Structs dependencies
//!
//! When a dependency is a struct, you can simply annotate on a target.
//! The portaldi-macro generates DIPortal implementation for a target struct.
//!
//! ```
//! use portaldi::*;
//!
//! #[derive(DIPortal)]
//! struct Hoge {
//!   foo: DI<Foo>,
//!   // other deps
//! }
//!
//! #[derive(DIPortal)]
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
//! When a dependency is a trait object, you can annotate a target struct with a `provide` attribute.
//! The portaldi-macro generates DIPortal implementation for the target struct and DIProvider implementation for the trait.
//! The dependent struct's scope must have the depencency DIProvider.
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
//! ### Manually component creation
//!
//! When you need a custom creation logic for a compoonent, you manually define a implementation for `DIPortal`.
//!
//! #### For struct type depencency
//!
//! ```
//! use portaldi::*;
//!
//! #[derive(DIPortal)]
//! struct Hoge {
//!   foo: DI<Foo>,
//!   // other deps
//! }
//!
//! struct Foo { /* other deps */ }
//!
//! impl DIPortal for Foo {
//!     fn create_for_di(container: &DIContainer) -> Self {
//!         // custom creation logic
//!         Foo {}
//!     }
//! }
//!
//! // Use component
//! Hoge::di();
//!
//! ```
//!
//! #### For trait object depencency
//! If a depencency is a trait object and has custom creation logic, you can annotate `provider` on a `DIPortal` implementation.
//!
//! ```
//! use portaldi::*;
//!
//! #[derive(DIPortal)]
//! struct Hoge {
//!   foo: DI<dyn FooI>,
//!   // other deps
//! }
//!
//! pub trait FooI: DITarget {}
//!
//! struct Foo { /* other deps */ }
//!
//! impl FooI for Foo {}
//!
//! #[provider(FooI)]
//! impl DIPortal for Foo {
//!     fn create_for_di(container: &DIContainer) -> Self {
//!         // custom creation logic
//!         Foo {}
//!     }
//! }
//!
//! // Use component
//! Hoge::di();
//!
//! ```
//!
//! #### For async creation logic
//! If a depencency has async custom creation logic, you manually define a implementation for `AsyncDIPortal`.
//! Also you need anotate `inject` with `async` on the depencency field.
//!
//! ```
//! use portaldi::*;
//! use async_trait::async_trait;
//!
//! #[derive(DIPortal)]
//! struct Hoge {
//!   #[inject(async)]
//!   foo: DI<Foo>,
//!   // other deps
//! }
//!
//! struct Foo { /* other deps */ }
//!
//! #[async_trait]
//! impl AsyncDIPortal for Foo {
//!     async fn create_for_di(container: &DIContainer) -> Self {
//!         // custom creation logic
//!         Foo {}
//!     }
//! }
//!
//! async {
//!     // Use component
//!     Hoge::di().await;
//! };
//!
//! ```
//!
//! #### For complex creation logic that involves other components.
//! If a depencency has custom creation logic that needs other components, you manually define a factory component and implementation for `DIPortal`.
//!
//! ```
//! use portaldi::*;
//! use async_trait::async_trait;
//!
//! #[derive(DIPortal)]
//! struct Hoge {
//!   #[inject(async)]
//!   foo: DI<Foo>,
//!   // other deps
//! }
//!
//! struct Foo { /* other deps */ }
//!
//! #[async_trait]
//! impl AsyncDIPortal for Foo {
//!     async fn create_for_di(container: &DIContainer) -> Self {
//!         FooFactory::di_on(container).create().await
//!     }
//! }
//!
//! #[derive(DIPortal)]
//! struct FooFactory {
//!     bar: DI<Bar>,
//!     // other deps
//! }
//!
//! impl FooFactory {
//!   async fn create(&self) -> Foo {
//!     // custom creation logic that needs a bar.
//!     Foo {}
//!   }
//! }
//!
//! #[derive(DIPortal)]
//! struct Bar { /* other deps */ }
//!
//! async {
//!     // Use component
//!     Hoge::di().await;
//! };
//!
//! ```
//!
