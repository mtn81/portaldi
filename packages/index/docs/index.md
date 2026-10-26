# User guide

This guide walks you through PortalDI, from a first injected struct to async
components, scoped containers and testing. Read the chapters in order if you
are new to PortalDI, or jump to the one that matches what you want to do.

For the complete syntax of the `#[provide]` and `#[inject]` attributes, see the
[`DIPortal`](derive@crate::DIPortal) derive macro reference.

## Chapters

1. [Getting started](crate::docs::getting_started) — inject a struct into another struct, and the concepts behind it.
2. [Trait objects](crate::docs::traits) — depend on a trait instead of a concrete type.
3. [Custom creation](crate::docs::custom_creation) — create a component with your own code.
4. [Async components](crate::docs::async_components) — components that need async creation.
5. [Resolving dependencies](crate::docs::resolving) — how providers are found, and how to override them.
6. [Containers](crate::docs::containers) — the global container, scoped containers and component lifetimes.
7. [Configuration](crate::docs::configuration) — feature flags, environment variables, thread-safety and Wasm.
8. [Errors](crate::docs::errors) — common compile errors and how to fix them.
9. [Recipes](crate::docs::recipes) — practical patterns for testing and structuring an application.
