# Resolving dependencies

Every `DI<T>` field is resolved through a provider type. This chapter explains
how provider names are built, how to override them, and how to keep provider
imports manageable.

## Provider names and the `di!` macro

The macros build a provider name as `{Type}{TypeParams}Provider`: each generic
type argument is appended by name, and the unit type `()` becomes `Unit`.

| Field type | Provider |
|---|---|
| `DI<Foo>` | `FooProvider` |
| `DI<dyn FooI>` | `FooIProvider` |
| `DI<dyn FooI<Bar>>` | `FooIBarProvider` |
| `DI<Tagged<Foo, ()>>` | `TaggedFooUnitProvider` |

The [`di!`](crate::di) macro expands to a call on that provider, so you rarely
have to spell the name out:

```rust,ignore
di![Foo]                // FooProvider::di()
di![Foo on container]   // FooProvider::di_on(container)
di![Foo<Bar, ()>]       // FooBarUnitProvider::di()
```

## Overriding the provider with `#[inject]`

`#[inject(Path)]` resolves a field with `Path::di_on(container)` instead of the
inferred provider. `Path` can be:

* a concrete type that implements [`DIPortal`](trait@crate::DIPortal), to resolve
  a trait-object field through it (see [Trait objects](crate::docs::traits));
* a provider type, for example one defined in another crate under a different name.

```rust
use portaldi::*;

mod other_crate {
    use portaldi::*;

    pub struct Baz {}
    def_di_provider!(Baz, |_c| Baz {});
    pub use BazProvider as MyBazProvider;
}

use other_crate::{Baz, MyBazProvider};

#[derive(DIPortal)]
struct Hoge {
    #[inject(MyBazProvider)]
    baz: DI<Baz>,
}

Hoge::di();
```

Combine it with `async` as `#[inject(Path, async)]`
(see [Async components](crate::docs::async_components)).

## Multiple components of the same type

The container holds one component per type. To register several components that
share the same underlying type, wrap them in [`Tagged<T, Tag>`](crate::Tagged),
where `Tag` is any marker type:

```rust
use portaldi::*;

pub struct Primary;
pub struct Replica;

pub struct Db {
    url: &'static str,
}

def_di_provider!(Tagged<Db, Primary>, |_c| Tagged::new(Db { url: "primary" }));
def_di_provider!(Tagged<Db, Replica>, |_c| Tagged::new(Db { url: "replica" }));

#[derive(DIPortal)]
struct Repository {
    write: DI<Tagged<Db, Primary>>,
    read: DI<Tagged<Db, Replica>>,
}

let repo = Repository::di();
assert_eq!(repo.write.url, "primary"); // Tagged derefs to DI<Db>
assert_eq!(repo.read.url, "replica");
```

* [`Tagged::new`](crate::Tagged::new) wraps a value, and
  [`Tagged::wrap`](crate::Tagged::wrap) wraps an existing `DI<T>`, for example
  `Tagged::wrap(di![Db on c])` to share an existing component under a tag.
* `Tagged<T, Tag>` derefs to `DI<T>`, so the inner methods can be called directly.
  [`.target()`](crate::Tagged::target) returns the `DI<T>` itself.

## Hiding providers behind a barrel module

The provider of each dependency must be in scope where the dependent struct is
declared, so a struct with many dependencies ends up importing many
implementation modules:

```rust
mod demo {
    mod service {
        use portaldi::*;

        pub trait FooI: DITarget {}
        pub trait BarI: DITarget {}
    }

    mod foo_impl {
        use portaldi::*;
        use super::service::*;

        #[derive(DIPortal)]
        #[provide(FooI)]
        pub struct Foo {}
        impl FooI for Foo {}
    }

    mod bar_impl {
        use portaldi::*;
        use super::service::*;

        #[derive(DIPortal)]
        #[provide(BarI)]
        pub struct Bar {}
        impl BarI for Bar {}
    }

    use portaldi::*;
    use foo_impl::FooIProvider; // must be in this scope
    use bar_impl::BarIProvider; // must be in this scope
    use service::*;

    #[derive(DIPortal)]
    struct Hoge {
        foo: DI<dyn FooI>,
        bar: DI<dyn BarI>,
    }
}
```

Add a *barrel* module that re-exports the providers, and import only that:

```rust
mod demo {
    mod service {
        use portaldi::*;

        pub trait FooI: DITarget {}
        pub trait BarI: DITarget {}
    }

    mod foo_impl {
        use portaldi::*;
        use super::service::*;

        #[derive(DIPortal)]
        #[provide(FooI)]
        pub struct Foo {}
        impl FooI for Foo {}
    }

    mod bar_impl {
        use portaldi::*;
        use super::service::*;

        #[derive(DIPortal)]
        #[provide(BarI)]
        pub struct Bar {}
        impl BarI for Bar {}
    }

    mod providers {
        pub use super::foo_impl::FooIProvider;
        pub use super::bar_impl::BarIProvider;
    }

    use portaldi::*;
    use providers::*;
    use service::*;

    #[derive(DIPortal)]
    struct Hoge {
        foo: DI<dyn FooI>,
        bar: DI<dyn BarI>,
    }
}
```

A barrel module is also the natural place to switch implementations, see
[Recipes](crate::docs::recipes).

Next: [Containers](crate::docs::containers)
