#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use portaldi::{AsyncDIPortal, DIPortal, DI};
    use tokio;

    mod concrete_type {
        use super::*;

        mod sync_test {
            use super::*;

            #[derive(DIPortal)]
            struct Hoge {
                foo: DI<Foo>,
                bar: DI<Bar>,
            }

            impl PartialEq for Hoge {
                fn eq(&self, other: &Self) -> bool {
                    fn ptr_eq<T>(ref1: &T, ref2: &T) -> bool {
                        std::ptr::eq(ref1 as *const _, ref2 as *const _)
                    }
                    ptr_eq(self, other)
                        && ptr_eq(self.foo.as_ref(), other.foo.as_ref())
                        && ptr_eq(self.bar.as_ref(), other.bar.as_ref())
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

            #[test]
            fn test_di() {
                assert!(Hoge::di() == Hoge::di())
            }
        }

        mod async_test {
            use super::*;
            #[derive(DIPortal, PartialEq)]
            struct AHoge {
                foo: DI<AFoo>,
                #[inject(async)]
                bar: DI<ABar>,
            }

            #[derive(DIPortal, PartialEq)]
            struct AFoo {}

            #[derive(PartialEq)]
            struct ABar {}

            #[async_trait]
            impl AsyncDIPortal for ABar {
                async fn create_for_di(_container: &portaldi::DIContainer) -> Self {
                    ABar {}
                }
            }

            #[tokio::test]
            async fn test_di() {
                assert!(AHoge::di().await == AHoge::di().await)
            }
        }
    }
}
