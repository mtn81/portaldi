use std::marker::PhantomData;

use crate::common::*;

#[test]
fn test_di() {
    assert!(Hoge::di() == Hoge::di())
}

#[derive(DIPortal)]
struct Hoge {
    foo: DI<Foo>,
    #[inject(Bar)]
    bar: DI<Bar>,
    #[inject(BazProvider)]
    baz: DI<Baz>,
    yah: DI<Yah>,
    _yah3: DI<Yah3<String, u8>>,
}

impl PartialEq for Hoge {
    fn eq(&self, other: &Self) -> bool {
        ptr_eq(self, other)
            && ptr_eq(self.foo.as_ref(), other.foo.as_ref())
            && ptr_eq(self.bar.as_ref(), other.bar.as_ref())
            && ptr_eq(self.baz.as_ref(), other.baz.as_ref())
            && ptr_eq(self.yah.as_ref(), other.yah.as_ref())
            && ptr_eq(self.foo.bar.as_ref(), other.foo.bar.as_ref())
            && ptr_eq(self.foo.bar.as_ref(), self.bar.as_ref())
            && ptr_eq(other.foo.bar.as_ref(), other.bar.as_ref())
    }
}

#[derive(DIPortal)]
struct Foo {
    #[inject(Bar)]
    bar: DI<Bar>,
}

struct Bar {}
// implements manually
impl DIPortal for Bar {
    fn create_for_di(_container: &portaldi::DIContainer) -> Self {
        Bar {}
    }
}

pub struct Baz {}

// implements provider manually
struct BazProvider {}
impl DIProvider for BazProvider {
    type Output = Baz;

    fn di_on(container: &DIContainer) -> DI<Self::Output> {
        container.get_or_init(|| Baz {})
    }
}

pub struct Yah {}

// implements provider manually
di_provider!(Yah, |_c| { Yah {} });

pub struct Yah2<A, B> {
    a: PhantomData<A>,
    b: PhantomData<B>,
}

// implements provider manually
di_provider!(Yah2<String, bool>, |_c| { Yah2 { a: PhantomData::<String>, b: PhantomData::<bool>} });

pub struct Yah3<A, B> {
    a: PhantomData<A>,
    b: PhantomData<B>,
}

// implements manually & self provider attribute
#[provider(self)]
impl DIPortal for Yah3<String, u8> {
    fn create_for_di(_container: &portaldi::DIContainer) -> Self {
        Yah3 {
            a: PhantomData,
            b: PhantomData,
        }
    }
}
