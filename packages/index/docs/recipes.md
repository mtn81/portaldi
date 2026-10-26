# Recipes

Practical patterns built on the previous chapters.

## Creating components at startup

Components are created lazily on their first request. If a component should be
ready before the application starts serving, for example to fail fast on a bad
connection, request it explicitly at startup:

```rust
use portaldi::*;

#[derive(DIPortal)]
struct App {
    repo: DI<Repository>,
}

#[derive(DIPortal)]
struct Repository {}

fn main() {
    let app = App::di(); // the whole graph is created here
    // ... serve requests with `app`
#   let _ = app;
}
```

## Replacing a concrete dependency in tests

Build the graph on a scoped container, and put the replacement there first with
[`put_if_absent`](crate::DIContainer::put_if_absent)
(see [Containers](crate::docs::containers)):

```rust
use portaldi::*;

pub struct Config {
    pub url: String,
}

def_di_provider!(Config, |_c| Config {
    url: std::env::var("DATABASE_URL").unwrap_or_default(),
});

#[derive(DIPortal)]
struct Repository {
    config: DI<Config>,
}

// in a test
let container = DIContainer::new();
container.put_if_absent(&DI::new(Config { url: "memory://".into() }));

let repo = Repository::di_on(&container);
assert_eq!(repo.config.url, "memory://");
```

The container stores components by their concrete type, so this works for
`DI<Foo>` fields. To replace the implementation behind a `DI<dyn FooI>`, switch
the provider instead, as shown next.

## Switching implementations with `#[cfg]`

Re-export the providers from a barrel module
(see [Resolving dependencies](crate::docs::resolving)), and choose the module
with `#[cfg(...)]`. The dependent code does not change:

```rust
mod demo {
    mod service {
        use portaldi::*;

        pub trait MailerI: DITarget {
            fn send(&self, to: &str) -> String;
        }
    }

    mod smtp {
        use portaldi::*;
        use super::service::*;

        #[derive(DIPortal)]
        #[provide(MailerI)]
        pub struct SmtpMailer {}
        impl MailerI for SmtpMailer {
            fn send(&self, to: &str) -> String { format!("smtp: {to}") }
        }
    }

    mod fake {
        use portaldi::*;
        use super::service::*;

        #[derive(DIPortal)]
        #[provide(MailerI)]
        pub struct FakeMailer {}
        impl MailerI for FakeMailer {
            fn send(&self, to: &str) -> String { format!("fake: {to}") }
        }
    }

    #[cfg(not(test))]
    mod providers {
        pub use super::smtp::MailerIProvider;
    }

    #[cfg(test)]
    mod providers {
        pub use super::fake::MailerIProvider;
    }

    use portaldi::*;
    use providers::*;
    use service::*;

    #[derive(DIPortal)]
    pub struct SignUp {
        mailer: DI<dyn MailerI>,
    }
}
```

The same technique works with Cargo features, for example
`#[cfg(feature = "aws")]`, to pick an implementation per deployment.
For a decision at runtime, use [`def_di_provider_raw!`](crate::def_di_provider_raw)
(see [Custom creation](crate::docs::custom_creation)).

## Isolating tests from each other

The global container is shared by the whole test binary, so components created
by one test are visible to the others. Resolve with
[`di_on`](crate::DIPortal::di_on) on a fresh [`DIContainer`](crate::DIContainer)
in each test to keep them independent.
