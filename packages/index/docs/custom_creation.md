# Custom creation

`#[derive(DIPortal)]` builds a struct from its `DI<T>` fields. When a component
needs its own construction code, for example a type from another crate or a
client built from a configuration value, define its provider yourself.

## `def_di_provider!`

[`def_di_provider!`](crate::def_di_provider) defines a provider from a closure
that receives the container and returns the component:

```rust
use portaldi::*;

pub struct Foo { /* fields from another crate, etc. */ }

def_di_provider!(Foo, |_c| {
    // custom creation logic
    Foo {}
}); // generates FooProvider

#[derive(DIPortal)]
struct Hoge {
    foo: DI<Foo>, // FooProvider must be in this scope
}

Hoge::di();
```

For a trait object, add `dyn`. The closure returns a concrete type, which is
coerced to the trait object:

```rust
use portaldi::*;

pub trait FooI: DITarget {}

struct Foo {}
impl FooI for Foo {}

def_di_provider!(dyn FooI, |_c| Foo {}); // generates FooIProvider

#[derive(DIPortal)]
struct Hoge {
    foo: DI<dyn FooI>,
}

Hoge::di();
```

## Depending on other components

The closure receives the container, so it can resolve other components with the
[`di!`](crate::di) macro:

```rust
use portaldi::*;

pub struct Foo {
    bar: DI<Bar>,
}

#[derive(DIPortal)]
struct Bar {}

def_di_provider!(Foo, |c| Foo {
    bar: di![Bar on c], // BarProvider must be in this scope
});

let foo = FooProvider::di();
assert!(DI::ptr_eq(&foo.bar, &Bar::di()));
```

Always resolve through the container passed to the closure (`on c`), so the
component joins the same object graph as its dependent
(see [Containers](crate::docs::containers)).

## Switching the implementation by a condition

`def_di_provider!` caches the value by the concrete type the closure returns, so
the closure must return a single concrete type. To choose between
implementations, use [`def_di_provider_raw!`](crate::def_di_provider_raw),
whose closure returns [`DI<T>`](crate::DI) by calling
[`DIContainer::get_or_init`](crate::DIContainer::get_or_init) itself:

```rust
use portaldi::*;

pub trait FooI: DITarget {}

struct FooA {}
impl FooI for FooA {}

struct FooB {}
impl FooI for FooB {}

def_di_provider_raw!(dyn FooI, |c| {
    if std::env::var("USE_FOO_B").is_ok() {
        c.get_or_init(|| FooB {})
    } else {
        c.get_or_init(|| FooA {})
    }
});

FooIProvider::di();
```

How the two macros differ:

* [`def_di_provider!`](crate::def_di_provider) wraps the closure as
  `c.get_or_init(|| (closure)(c))`: the closure returns a plain value and runs
  only once per container.
* [`def_di_provider_raw!`](crate::def_di_provider_raw) uses the closure as the
  body of `di_on` as is: the closure runs on **every** call, and caching is done
  by whatever type it passes to the container (`FooA` / `FooB` above, not
  `dyn FooI`). Each branch's `DI<Concrete>` is coerced to `DI<Output>`.

The async variants are covered in [Async components](crate::docs::async_components).

## Writing an `impl` block with `#[provider]`

If you prefer to write the constructor as a full
[`DIPortal`](trait@crate::DIPortal) implementation, put
[`#[provider(...)]`](macro@crate::provider) on the `impl` block to generate the
provider as well:

```rust
use portaldi::*;

pub trait FooI: DITarget {}

struct Foo {}
impl FooI for Foo {}

#[provider(FooI)] // generates FooIProvider
impl DIPortal for Foo {
    fn create_for_di(_container: &DIContainer) -> Self {
        Foo {}
    }
}

FooIProvider::di();
```

* `#[provider(Self)]` generates the provider for the concrete type itself.
* `#[provider(FooI)]` (or `#[provider(FooI<A>)]`) generates it for a trait object.
* It also works on an [`AsyncDIPortal`](crate::AsyncDIPortal) `impl` block.

Next: [Async components](crate::docs::async_components)
