# Trait objects

Depending on a trait instead of a concrete type lets you swap the implementation
without touching the dependent code. This chapter shows how to inject a
`DI<dyn Trait>`.

## Providing a trait with `#[provide]`

A trait used as a dependency must have [`DITarget`](crate::DITarget) as a
supertrait. Annotate the implementing struct with `#[provide(Trait)]`:

```rust
use portaldi::*;

pub trait FooI: DITarget {}

#[derive(DIPortal)]
#[provide(FooI)] // generates FooIProvider in addition to FooProvider
struct Foo {}

impl FooI for Foo {}

#[derive(DIPortal)]
struct Hoge {
    foo: DI<dyn FooI>, // FooIProvider must be in this scope
}

Hoge::di();
FooIProvider::di(); // DI<dyn FooI>
```

For a generic trait, the type arguments are part of the provider name:
`#[provide(FooI<Bar>)]` generates `FooIBarProvider`.

## Resolving through a concrete type with `#[inject]`

Instead of a trait provider, you can tell a field which concrete type to use:

```rust
use portaldi::*;

pub trait FooI: DITarget {}

#[derive(DIPortal)]
struct Foo {}

impl FooI for Foo {}

#[derive(DIPortal)]
struct Hoge {
    #[inject(Foo)] // resolved by Foo::di_on, then coerced to DI<dyn FooI>
    foo: DI<dyn FooI>,
}

Hoge::di();
```

This is handy when only one struct needs that implementation. When many structs
depend on the trait, prefer `#[provide]` so the choice is made in one place.

## Generating trait providers by naming convention

If your implementations follow a naming convention such as `FooImpl` for
`trait Foo`, set the `PORTALDI_PROVIDER_PATTERN` environment variable in
`.cargo/config.toml`:

```toml
[env]
PORTALDI_PROVIDER_PATTERN = "^(\\S+)Impl$"
```

The pattern is matched against the name of every `#[derive(DIPortal)]` struct
without `#[provide]`, and capture group 1 is taken as the trait name:

```rust,ignore
use portaldi::*;

pub trait Foo: DITarget {}

#[derive(DIPortal)] // also generates FooProvider for dyn Foo
struct FooImpl {}

impl Foo for FooImpl {}

let foo: DI<dyn Foo> = FooProvider::di();
```

A struct name that does not match generates nothing extra. See
[Configuration](crate::docs::configuration) for details on the environment variables.

## What's next

- Implementations that need custom construction are covered in
  [Custom creation](crate::docs::custom_creation).
- Keeping provider imports tidy across modules is covered in
  [Resolving dependencies](crate::docs::resolving).

Next: [Custom creation](crate::docs::custom_creation)
