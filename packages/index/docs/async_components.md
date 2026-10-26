# Async components

Some components can only be created asynchronously, for example a database pool
that connects on startup. PortalDI supports async creation natively.

Async components use [`async-trait`](https://crates.io/crates/async-trait), so
add it to your dependencies and import `async_trait::async_trait` where the
macros are used.

## Async creation logic

Define the provider with
[`def_async_di_provider!`](crate::def_async_di_provider), and mark the field that
depends on it with `#[inject(async)]`:

```rust
use portaldi::*;
use async_trait::async_trait;

pub struct Pool {}

impl Pool {
    async fn connect() -> Pool { Pool {} }
}

def_async_di_provider!(Pool, |_c| async { Pool::connect().await });

#[derive(DIPortal)]
struct Repository {
    #[inject(async)]
    pool: DI<Pool>,
}

# async fn run() {
let repo: DI<Repository> = Repository::di().await;
# }
```

A struct with at least one `#[inject(async)]` field gets an
[`AsyncDIPortal`](crate::AsyncDIPortal) implementation instead of
[`DIPortal`](trait@crate::DIPortal), so its `di()` must be awaited too. Anything
that depends on it is then async as well, so mark those fields with
`#[inject(async)]` in turn.

To resolve other components inside the closure, use `async move` and the
[`di!`](crate::di) macro, just like the sync version:

```rust
use portaldi::*;
use async_trait::async_trait;

#[derive(DIPortal)]
struct Config {}

pub struct Pool {
    config: DI<Config>,
}

def_async_di_provider!(Pool, |c| async move {
    Pool { config: di![Config on c] }
});
```

An async field and a concrete type can be combined:
`#[inject(PoolImpl, async)]` resolves a `DI<dyn PoolI>` field through
`PoolImpl` asynchronously.

## Switching the implementation asynchronously

[`def_async_di_provider_raw!`](crate::def_async_di_provider_raw) is the async
counterpart of `def_di_provider_raw!` (see
[Custom creation](crate::docs::custom_creation)). Use
[`DIContainer::get_or_init_async`](crate::DIContainer::get_or_init_async) inside:

```rust
use portaldi::*;
use async_trait::async_trait;

pub trait FooI: DITarget {}

struct FooA {}
impl FooI for FooA {}

struct FooB {}
impl FooI for FooB {}

def_async_di_provider_raw!(dyn FooI, |c| async {
    if std::env::var("USE_FOO_B").is_ok() {
        c.get_or_init_async(|| async { FooB {} }).await
    } else {
        c.get_or_init_async(|| async { FooA {} }).await
    }
});

# async fn run() {
FooIProvider::di().await;
# }
```

The closure is expanded inline rather than called, so it must take exactly one
argument (the container).

## Creating async fields concurrently

By default, the `#[inject(async)]` fields of a struct are created one after
another. Enable the `futures-join` feature to create them concurrently with
`futures::join!` when a struct has more than one async field:

```toml
[dependencies]
portaldi = { version = "*", features = ["futures-join"] }
futures = "0.3" # required by the generated code
```

## Making every component async

Setting the `PORTALDI_ALWAYS_ASYNC` environment variable to `true` makes every
`#[derive(DIPortal)]` generate an `AsyncDIPortal` implementation and await every
field, without writing `#[inject(async)]`. This only compiles if every
dependency in the graph is async as well. See
[Configuration](crate::docs::configuration).

Next: [Resolving dependencies](crate::docs::resolving)
