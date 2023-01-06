use portaldi::*;

#[derive(DIPortal)]
struct Hoge {
    foo: DI<dyn FooI>,
    bar: DI<Bar>,
}
impl Hoge {
    fn say_hello(&self) {
        println!("hello hoge < {}, {}", self.foo.hello(), self.bar.hello())
    }
}

pub trait FooI: DITarget {
    fn hello(&self) -> &str;
}
#[derive(DIPortal)]
#[provide(FooI)]
struct Foo {}
impl FooI for Foo {
    fn hello(&self) -> &str {
        "hello foo"
    }
}

#[derive(DIPortal)]
struct Bar {}

impl Bar {
    fn hello(&self) -> &str {
        "hello bar"
    }
}

fn main() {
    Hoge::di().say_hello();
}
