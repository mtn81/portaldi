# Configuration

PortalDI is configured with Cargo feature flags and with environment variables
that are read while the macros expand.

## Feature flags

| Feature | Effect |
|---|---|
| `multi-thread` | Force the thread-safe container ([`DI<T>`](crate::DI) = `Arc<T>`, [`DITarget`](crate::DITarget)`: Send + Sync`) even on `wasm32`. On non-wasm targets the thread-safe container is always used, so this flag only matters for `wasm32`. |
| `futures-join` | When a struct has more than one `#[inject(async)]` field, create those fields concurrently with `futures::join!` instead of sequentially. Requires the `futures` crate as a dependency of your crate. |

## Environment variables

The macros read these with `std::env::var` while expanding
[`#[derive(DIPortal)]`](derive@crate::DIPortal) and
[`#[provider]`](macro@crate::provider). Set them in the `[env]` table of
`.cargo/config.toml`:

```toml
[env]
PORTALDI_ALWAYS_ASYNC = "false"
PORTALDI_PROVIDER_PATTERN = "^(\\S+)Impl$"
```

| Variable | Effect |
|---|---|
| `PORTALDI_ALWAYS_ASYNC` | When the value parses as `true`, every [`#[derive(DIPortal)]`](derive@crate::DIPortal) emits an [`AsyncDIPortal`](crate::AsyncDIPortal) implementation (and `await`s every field) instead of [`DIPortal`](trait@crate::DIPortal). This only compiles if every dependency in the graph is also async. |
| `PORTALDI_PROVIDER_PATTERN` | A regex matched against the struct name. For a [`#[derive(DIPortal)]`](derive@crate::DIPortal) with no `#[provide(...)]` (or a bare [`#[provider]`](macro@crate::provider)), capture group 1 is taken as a trait name and a `<group1>Provider` (implementing [`DIProvider`](crate::DIProvider) with `Output = dyn <group1>`) is generated in addition to the self provider. Example: `^(\S+)Impl$` makes `FooImpl` provide `dyn Foo` as `FooProvider`. A non-matching name generates nothing; an invalid regex aborts compilation. |

Cargo does not track these reads, so changing a value may not trigger a rebuild
on its own. Editing `.cargo/config.toml` does; otherwise run `cargo clean`.

## Thread-safety and Wasm

On native targets [`DI<T>`](crate::DI) is `Arc<T>`, and every
[`DITarget`](crate::DITarget) must be `Send + Sync + 'static`.

On `wasm32` (without the `multi-thread` feature) [`DI<T>`](crate::DI) is `Rc<T>`
and the `Send + Sync` bound is dropped, so `!Send` components are allowed. The
global container is then kept per thread. Enable `multi-thread` to keep the
`Arc` / `Send + Sync` model on `wasm32` as well.

Next: [Errors](crate::docs::errors)
