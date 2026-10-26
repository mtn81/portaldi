# Getting started

This chapter injects one struct into another, and introduces the few concepts
that the rest of the guide builds on.

## Installation

```bash
cargo add portaldi
```

If you use async components (see [Async components](crate::docs::async_components)),
also add [`async-trait`](https://crates.io/crates/async-trait):

```bash
cargo add async-trait
```

## A first injected struct

Declare each dependency as a [`DI<T>`](crate::DI) field and derive
[`DIPortal`](derive@crate::DIPortal) on both structs:

```rust
use portaldi::*;

#[derive(DIPortal)]
struct Hoge {
    foo: DI<Foo>, // FooProvider must be in this scope
}

#[derive(DIPortal)] // generates the DIPortal implementation and FooProvider
struct Foo {}

let hoge: DI<Hoge> = Hoge::di();
assert!(DI::ptr_eq(&hoge, &Hoge::di())); // the same instance every time
assert!(DI::ptr_eq(&hoge.foo, &Foo::di()));
```

[`Hoge::di()`](crate::DIPortal::di) creates `Hoge` on the first call, together
with every dependency it needs, and returns the same instance afterwards.
You rarely touch a container directly.

## Concepts

* **Constructor injection.** Dependencies are struct fields of type
  [`DI<T>`](crate::DI), a shared reference (`Arc<T>` on native targets).

* **Providers.** Each field is resolved through a *provider* type named after it:
  `DI<Foo>` is resolved by `FooProvider`, which implements
  [`DIProvider`](crate::DIProvider). `#[derive(DIPortal)]` generates the provider
  next to the struct, and **the provider must be in scope where the field is
  declared**. The naming rules are covered in
  [Resolving dependencies](crate::docs::resolving).

* **Thread-safe components.** Every component must implement
  [`DITarget`](crate::DITarget). Any `Send + Sync + 'static` type implements it
  automatically. On Wasm the `Send + Sync` bound is dropped
  (see [Configuration](crate::docs::configuration)).

* **Lazy singletons.** A component is created the first time it is requested,
  and then shared by everyone in the same container
  (see [Containers](crate::docs::containers)).

## What's next

- Depending on a trait instead of a concrete type is covered in
  [Trait objects](crate::docs::traits).
- Types you can not derive on, such as types from other crates, are covered in
  [Custom creation](crate::docs::custom_creation).

Next: [Trait objects](crate::docs::traits)
