use portaldi::*;

#[derive(DIPortal)]
enum Hoge1 {}

#[derive(DIPortal)]
struct Hoge2 {
    foo: Foo,
}

#[derive(DIPortal)]
struct Hoge3 {
    #[inject(1 + 1)]
    foo: DI<Foo>,
}

#[derive(DIPortal)]
#[provide(1 + 1)]
struct Hoge4 {}

#[derive(DIPortal)]
struct Hoge5<'a> {
    foo: DI<Foo>,
}

#[derive(DIPortal)]
struct Hoge6 {
    foo: DI<[Foo]>,
}

#[derive(DIPortal)]
struct Hoge7 {
    foo: DI<dyn 'static + Send>,
}

mod hoge8 {
    use super::*;

    // Shadows portaldi::DI so that only the macro error is reported.
    type DI = std::sync::Arc<Foo>;

    #[derive(DIPortal)]
    struct Hoge8 {
        foo: DI,
    }
}

#[derive(DIPortal)]
struct Foo {}

fn main() {}
