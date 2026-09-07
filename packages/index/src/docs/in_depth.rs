//! # In depth
//!
//! Less common configuration and lower-level details. Most users only need
//! [`how_to_use`](crate::docs::how_to_use) and [`tips`](crate::docs::tips).
//!
//! ### Customizing injection with `#[inject]`
//!
//! By default the provider for a field is inferred from its type ([`DI<Foo>`](crate::DI) uses `FooProvider`,
//! [`DI<dyn FooI>`](crate::DI) uses `FooIProvider`). Put `#[inject(...)]` on a field to override this:
//!
//! * `#[inject(Foo)]` — resolve a trait-object field through a concrete type.
//! * `#[inject(MyProvider)]` — resolve through an explicit provider path (for example a
//!   provider defined in another crate).
//! * `#[inject(async)]` — the field needs async creation; the enclosing struct then gets an
//!   [`AsyncDIPortal`](crate::AsyncDIPortal) implementation instead of [`DIPortal`](trait@crate::DIPortal).
//! * The forms can be combined, for example `#[inject(AsyncFoo, async)]`.
//!
//! See the [`DIPortal` derive macro](derive@crate::DIPortal) for a worked example covering
//! every `#[inject]` form.
//!
//! ### The [`di!`](crate::di) macro and generated provider names
//!
//! Every component is resolved through a *provider* type. The macros build its name as
//! `{Type}{TypeParams}Provider`: each generic type argument is appended by name, and the unit
//! type `()` becomes `Unit`.
//!
//! | Field type | Provider |
//! |---|---|
//! | `DI<Foo>` | `FooProvider` |
//! | `DI<dyn FooI>` | `FooIProvider` |
//! | `DI<dyn FooI<Bar>>` | `FooIBarProvider` |
//! | `DI<Tagged<Foo, ()>>` | `TaggedFooUnitProvider` |
//!
//! The [`di!`](crate::di) macro expands to a call on that provider, so you rarely have to spell the name out:
//!
//! ```ignore
//! di![Foo]                // FooProvider::di()
//! di![Foo on container]   // FooProvider::di_on(container)
//! di![Foo<Bar, ()>]       // FooBarUnitProvider::di()
//! ```
//!
//! ### Using a scoped container
//!
//! [`di()`](crate::DIPortal::di) resolves against a process-global container. For tests, or to keep several independent
//! object graphs, create a local [`DIContainer`](crate::DIContainer) and resolve with [`di_on`](crate::DIPortal::di_on)
//! (or `di![T on c]`):
//!
//! ```
//! use portaldi::*;
//!
//! #[derive(DIPortal)]
//! struct Hoge {
//!     foo: DI<Foo>,
//! }
//!
//! #[derive(DIPortal)]
//! struct Foo {}
//!
//! let container = DIContainer::new();
//! let hoge = Hoge::di_on(&container); // this graph is isolated from the global one
//! let _ = hoge;
//! ```
//!
//! Each component is a singleton within a given container, and a fresh [`DIContainer`](crate::DIContainer) starts
//! empty. You can pre-populate one with [`DIContainer::put_if_absent`](crate::DIContainer::put_if_absent) to inject a stub before
//! the graph is built.
//!
//! ### [`#[provider]`](macro@crate::provider): generating a provider from an `impl` block
//!
//! [`def_di_provider!`](crate::def_di_provider) / [`def_async_di_provider!`](crate::def_async_di_provider) are the recommended shorthands. When you would
//! rather write the constructor as a full [`DIPortal`](trait@crate::DIPortal) / [`AsyncDIPortal`](crate::AsyncDIPortal) `impl` block, put
//! [`#[provider(...)]`](macro@crate::provider) on that block to also generate the provider:
//!
//! ```
//! use portaldi::*;
//!
//! pub trait FooI: DITarget {}
//!
//! struct Foo {}
//! impl FooI for Foo {}
//!
//! #[provider(FooI)] // generates `FooIProvider`
//! impl DIPortal for Foo {
//!     fn create_for_di(_container: &DIContainer) -> Self {
//!         Foo {}
//!     }
//! }
//!
//! # let _ = FooIProvider::di_on(&DIContainer::new());
//! ```
//!
//! * `#[provider(Self)]` generates the provider for the concrete type itself.
//! * `#[provider(FooI)]` (or `#[provider(FooI<A>)]`) generates it for a trait object.
//!
//! ### [`Tagged`](crate::Tagged) internals
//!
//! [`Tagged<T, Tag>`](crate::Tagged) derefs to [`DI<T>`](crate::DI), so you can call the inner value's methods directly.
//! Build one with [`Tagged::new`](crate::Tagged::new) (from a value) or [`Tagged::wrap`](crate::Tagged::wrap) (from an existing [`DI<T>`](crate::DI)),
//! and call [`.target()`](crate::Tagged::target) to get the [`DI<T>`](crate::DI) back.
//!
//! ```
//! use portaldi::*;
//!
//! struct Tag;
//!
//! struct Foo {}
//! impl Foo {
//!     fn hello(&self) -> &str { "hi" }
//! }
//!
//! let tagged: Tagged<Foo, Tag> = Tagged::new(Foo {});
//! assert_eq!(tagged.hello(), "hi"); // via Deref to DI<Foo>
//! let _inner: &DI<Foo> = tagged.target();
//! ```
//!
//! ### Circular dependencies
//!
//! A component is fully constructed before it is stored in the container, so a dependency
//! cycle (`A` needs `B`, `B` needs `A`) cannot be resolved and leads to unbounded recursion
//! (stack overflow) the first time the graph is built. Break the cycle at design time, or let
//! one side look the other up on demand inside a method via `di![Other on container]` instead
//! of holding it as a field.
//!
//! ### Thread-safety and Wasm
//!
//! On native targets [`DI<T>`](crate::DI) is `Arc<T>` and every [`DITarget`](crate::DITarget) must be `Send + Sync + 'static`.
//! On `wasm32` (without the `multi-thread` feature) [`DI<T>`](crate::DI) is `Rc<T>` and the `Send + Sync`
//! bound is dropped, so `!Send` components are allowed. Enable `multi-thread` to keep the
//! `Arc` / `Send + Sync` model on `wasm32` as well.
