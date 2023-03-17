use crate::common::*;

#[test]
fn test_di() {
    assert!(Hoge::di() == Hoge::di())
}

#[derive(DIPortal)]
struct Hoge {
    foo: DI<Foo>,
    bar: DI<Bar>,
    #[inject(BazProvider)]
    baz: DI<Baz>,
}

impl PartialEq for Hoge {
    fn eq(&self, other: &Self) -> bool {
        ptr_eq(self, other)
            && ptr_eq(self.foo.as_ref(), other.foo.as_ref())
            && ptr_eq(self.bar.as_ref(), other.bar.as_ref())
            && ptr_eq(self.baz.as_ref(), other.baz.as_ref())
            && ptr_eq(self.foo.bar.as_ref(), other.foo.bar.as_ref())
            && ptr_eq(self.foo.bar.as_ref(), self.bar.as_ref())
            && ptr_eq(other.foo.bar.as_ref(), other.bar.as_ref())
    }
}

#[derive(DIPortal)]
struct Foo {
    bar: DI<Bar>,
}

struct Bar {}
// implements manually
impl DIPortal for Bar {
    fn create_for_di(_container: &portaldi::DIContainer) -> Self {
        Bar {}
    }
}

struct Baz {}

struct BazProvider {}
// implements provider manually
impl DIProvider for BazProvider {
    type Output = Baz;

    fn di_on(container: &DIContainer) -> DI<Self::Output> {
        container.get_or_init(|| Baz {})
    }
}
