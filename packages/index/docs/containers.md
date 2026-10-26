# Containers

Components live in a [`DIContainer`](crate::DIContainer). This chapter explains
the global container used by `di()`, how to use your own containers, and how
long components live.

## The global container

[`di()`](crate::DIPortal::di) resolves against a process-global container. Each
component is created on its first request and kept for the rest of the process,
so every caller shares the same instance.

To look up a component that has already been created, without creating it, use
[`global::get`](crate::global::get):

```rust
use portaldi::*;

#[derive(DIPortal)]
struct Foo {}

assert!(global::get::<Foo>().is_none()); // not created yet

let foo = Foo::di();
assert!(DI::ptr_eq(&global::get::<Foo>().unwrap(), &foo));
```

## Scoped containers

For tests, or to keep several independent object graphs, create a local
[`DIContainer`](crate::DIContainer) and resolve with
[`di_on`](crate::DIPortal::di_on) (or `di![T on c]`):

```rust
use portaldi::*;

#[derive(DIPortal)]
struct Hoge {
    foo: DI<Foo>,
}

#[derive(DIPortal)]
struct Foo {}

let container = DIContainer::new();
let hoge = Hoge::di_on(&container); // this graph is isolated from the global one

assert!(DI::ptr_eq(&hoge.foo, &Foo::di_on(&container)));
assert!(!DI::ptr_eq(&hoge.foo, &Foo::di()));
```

* Each component is a singleton within a given container.
* A new container starts empty.
* The whole graph is resolved on the container you pass, including the
  dependencies of dependencies.

## Putting a component in advance

[`DIContainer::put_if_absent`](crate::DIContainer::put_if_absent) stores a
component before the graph is built, so it is used instead of the provider. This
is the basis for replacing a dependency with a stub in tests
(see [Recipes](crate::docs::recipes)):

```rust
use portaldi::*;

#[derive(DIPortal)]
struct Hoge {
    foo: DI<Foo>,
}

#[derive(DIPortal)]
struct Foo {}

let container = DIContainer::new();
let foo = container.put_if_absent(&DI::new(Foo {}));

assert!(DI::ptr_eq(&Hoge::di_on(&container).foo, &foo));
```

If a component of that type is already present, `put_if_absent` keeps it and
returns the existing one.

## Circular dependencies

A component is fully constructed before it is stored in the container, so a
dependency cycle (`A` needs `B`, `B` needs `A`) can not be resolved and leads to
unbounded recursion (stack overflow) the first time the graph is built.
Break the cycle at design time, or let one side look the other up on demand
inside a method via `di![Other on container]` instead of holding it as a field.

Next: [Configuration](crate::docs::configuration)
