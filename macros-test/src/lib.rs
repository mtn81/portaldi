#[cfg(test)]
mod tests {
    use portaldi::{DIPortal, DI};

    #[derive(DIPortal)]
    struct Hoge {
        foo: DI<Foo>,
        bar: DI<Bar>,
    }

    impl PartialEq for Hoge {
        fn eq(&self, other: &Self) -> bool {
            std::ptr::eq(self as *const _, other as *const _)
                && std::ptr::eq(
                    self.foo.as_ref() as *const _,
                    other.foo.as_ref() as *const _,
                )
                && std::ptr::eq(
                    self.bar.as_ref() as *const _,
                    other.bar.as_ref() as *const _,
                )
        }
    }

    #[derive(DIPortal)]
    struct Foo {}

    struct Bar {}
    // implements manually
    impl DIPortal for Bar {
        fn create_for_di(_container: &portaldi::DIContainer) -> Self {
            Bar {}
        }
    }

    #[test]
    fn test_di_concrete_type() {
        assert!(Hoge::di() == Hoge::di())
    }
}
