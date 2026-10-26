# Errors

PortalDI resolves the whole dependency graph at compile time, so most mistakes
are reported by the compiler. This chapter lists the common errors, what causes
them, and how to fix them.

## A provider is not in scope

```text
error[E0433]: cannot find type `FooProvider` in this scope
   |
   |     foo: DI<Foo>,
   |             ^^^ use of undeclared type `FooProvider`
```

The provider of a field must be in scope where the dependent struct is declared.
Here `Foo` is imported, but its generated `FooProvider` is not:

```rust,compile_fail,E0433
mod foo {
    use portaldi::*;

    #[derive(DIPortal)]
    pub struct Foo {}
}

use portaldi::*;
use foo::Foo; // FooProvider is not imported

#[derive(DIPortal)]
struct Hoge {
    foo: DI<Foo>,
}
```

Import the provider as well (`use foo::{Foo, FooProvider};`), or collect the
providers in a barrel module (see [Resolving dependencies](crate::docs::resolving)).
The same applies to trait objects: `DI<dyn FooI>` needs `FooIProvider`.

## An async dependency without `#[inject(async)]`

```text
error[E0308]: mismatched types
   |
   |     foo: DI<Foo>,
   |     ^^^ expected `Arc<Foo>`, found `Pin<Box<...>>`
```

The provider of `Foo` is async, so the field must be awaited. Mark it with
`#[inject(async)]`:

```rust,compile_fail,E0308
use portaldi::*;
use async_trait::async_trait;

pub struct Foo {}
def_async_di_provider!(Foo, |_c| async { Foo {} });

#[derive(DIPortal)]
struct Hoge {
    foo: DI<Foo>, // needs #[inject(async)]
}
```

## A component that is not thread-safe

```text
error[E0277]: `Rc<u8>` cannot be shared between threads safely
   |
   = note: required for `Foo` to implement `DITarget`
```

On native targets every component must be `Send + Sync + 'static`
(see [Configuration](crate::docs::configuration)). Use thread-safe types such as
`Arc` and `Mutex` inside components:

```rust,compile_fail,E0277
use portaldi::*;
use std::rc::Rc;

#[derive(DIPortal)]
struct Foo {
    bar: DI<Bar>,
}

pub struct Bar {
    value: Rc<u8>, // Rc is neither Send nor Sync
}

def_di_provider!(Bar, |_c| Bar { value: Rc::new(1) });
```

## Errors from `#[derive(DIPortal)]`

| Message | Cause |
|---|---|
| `Must be struct type` | The derive is used on an enum or a union. |
| `<field> is not DI type` | A field is not declared as `DI<T>` (or `DI<dyn Trait>`). Every field is a dependency. |
| `invalid inject attribute: ...` | `#[inject(...)]` contains something other than a path and `async`. |
| `invalid provide attribute: ...` | `#[provide(...)]` is not a trait name such as `FooI` or `FooI<A>`. |
| `invalid generic type: ...` | The struct has a lifetime parameter. Components must be `'static`. |

For example, a field that is not a dependency:

```rust,compile_fail
use portaldi::*;

#[derive(DIPortal)]
struct Hoge {
    name: String, // error: name is not DI type
}
```

Keep such values in a dependency (for example a `Config` component), or build
the struct with [`def_di_provider!`](crate::def_di_provider) instead
(see [Custom creation](crate::docs::custom_creation)).

## Errors from `#[provider]`

| Message | Cause |
|---|---|
| `[provider] must be on impl block` | The attribute is put on something other than an `impl` block. |
| `invalid attribute args` | The argument is not `Self`, a trait name, or empty. |
| `must be on impl DIPortal or AsyncDIPortal block` | The `impl` block does not implement `DIPortal` / `AsyncDIPortal`. |
| `'create_for_di' method must be defined.` | The `impl` block has no `create_for_di` method. |

## Errors from `def_di_provider!` and friends

`invalid input format` means the arguments do not match
`([dyn] Type[<Generics>], |c| expression)`. The message shows the expected
usage of each macro. For [`def_async_di_provider_raw!`](crate::def_async_di_provider_raw),
the closure must also take exactly one argument (the container).

## Stack overflow at runtime

A dependency cycle compiles, but overflows the stack the first time the graph is
built. See [Containers](crate::docs::containers) for how to break the cycle.

Next: [Recipes](crate::docs::recipes)
