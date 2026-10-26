use portaldi::*;

def_async_di_provider_raw!(Hoge);

pub struct Foo {}

def_async_di_provider_raw!(Foo, || async { DI::new(Foo {}) });

fn main() {}
