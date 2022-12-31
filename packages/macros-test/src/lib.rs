#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use portaldi::{AsyncDIPortal, DIPortal, DIProvider, DI};
    use tokio;

    fn ptr_eq<T: ?Sized>(ref1: &T, ref2: &T) -> bool {
        std::ptr::eq(ref1 as *const _, ref2 as *const _)
    }

    mod provider {
        use super::*;
        mod sync_test {
            use super::*;

            use foo::*;
            #[derive(DIPortal)]
            struct Hoge {
                // di by "provide" & Ident
                foo1: DI<dyn Foo>,
                // di by "provide" & Path
                foo2: DI<dyn foo::Foo>,
                // di by implicit Provider (BarImpl)
                bar1: DI<dyn Bar>,
                // di by explicit "inject"
                #[inject(BarImpl2)]
                bar2: DI<dyn Bar>,
            }

            mod foo {
                use super::*;
                pub trait Foo: Sync + Send {}

                #[derive(DIPortal, PartialEq)]
                #[provide(Foo)]
                struct FooImpl {}

                impl Foo for FooImpl {}
            }

            pub trait Bar: Sync + Send {}

            #[derive(DIPortal)]
            struct BarImpl {}
            impl Bar for BarImpl {}

            #[derive(DIPortal)]
            struct BarImpl2 {}
            impl Bar for BarImpl2 {}

            #[test]
            fn test_di() {
                let hoge = Hoge::di();
                assert!(ptr_eq(hoge.foo1.as_ref(), hoge.foo2.as_ref()));
                assert!(!ptr_eq(hoge.bar1.as_ref(), hoge.bar2.as_ref()));
            }
        }
    }

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
                foo: DI<Foo>,
                #[inject(async)]
                bar: DI<ABar>,
            }

            #[derive(DIPortal, PartialEq)]
            struct Foo {}

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
