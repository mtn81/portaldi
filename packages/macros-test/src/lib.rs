#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use portaldi::{AsyncDIPortal, AsyncDIProvider, DIContainer, DIPortal, DIProvider, DI};
    use tokio;

    fn ptr_eq<T: ?Sized>(ref1: &T, ref2: &T) -> bool {
        std::ptr::eq(ref1 as *const _, ref2 as *const _)
    }

    mod di_for_trait {
        use super::*;
        mod sync_test {
            use super::*;

            use bar::*;
            use foo::*;
            #[derive(DIPortal)]
            struct Hoge {
                // di by "provide" & Ident
                foo1: DI<dyn Foo>,
                // di by "provide" & Path
                foo2: DI<dyn foo::Foo>,
                // di by implicit Provider (BarImpl)
                bar1: DI<dyn Bar>,
                // di by explicit type in "inject"
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

            mod bar {
                use super::*;

                pub trait Bar: Sync + Send {}

                #[derive(DIPortal)]
                struct BarImpl {}
                impl Bar for BarImpl {}

                #[derive(DIPortal)]
                pub struct BarImpl2 {}
                impl Bar for BarImpl2 {}
            }

            #[test]
            fn test_di() {
                let hoge = Hoge::di();
                assert!(ptr_eq(hoge.foo1.as_ref(), hoge.foo2.as_ref()));
                assert!(!ptr_eq(hoge.bar1.as_ref(), hoge.bar2.as_ref()));
            }
        }

        mod async_test {
            use super::*;

            use bar::*;
            use foo::*;
            #[derive(DIPortal)]
            struct Hoge {
                // async di by Provider
                #[inject(async)]
                foo1: DI<dyn Foo>,
                // async di by explicit type
                #[inject(FooImpl2, async)]
                foo2: DI<dyn Foo>,
                // async di by implicit Provider
                #[inject(async)]
                bar1: DI<dyn Bar>,
            }

            mod foo {
                use super::*;
                pub trait Foo: Sync + Send {}

                struct FooImpl {}
                impl Foo for FooImpl {}

                #[portaldi::provider(Foo)]
                #[async_trait]
                impl AsyncDIPortal for FooImpl {
                    async fn create_for_di(_container: &DIContainer) -> Self {
                        FooImpl {}
                    }
                }

                pub struct FooImpl2 {}
                impl Foo for FooImpl2 {}

                #[async_trait]
                impl AsyncDIPortal for FooImpl2 {
                    async fn create_for_di(_container: &DIContainer) -> Self {
                        FooImpl2 {}
                    }
                }
            }

            mod bar {
                use super::*;
                pub trait Bar: Sync + Send {}

                struct BarImpl {}
                impl Bar for BarImpl {}

                #[portaldi::provider]
                #[async_trait]
                impl AsyncDIPortal for BarImpl {
                    async fn create_for_di(_container: &DIContainer) -> Self {
                        BarImpl {}
                    }
                }
            }

            #[tokio::test]
            async fn test_di() {
                let hoge = Hoge::di().await;
                assert!(!ptr_eq(hoge.foo1.as_ref(), hoge.foo2.as_ref()));
            }
        }
    }

    mod di_for_concrete_type {
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

    mod di_for_complex_component_creation {
        use super::*;

        #[derive(PartialEq)]
        struct Hoge {
            foo: DI<Foo>,
        }
        #[derive(DIPortal, PartialEq)]
        struct Foo {}

        #[async_trait]
        impl AsyncDIPortal for Hoge {
            async fn create_for_di(container: &DIContainer) -> Self {
                HogeFactory::di_on(container).create().await
            }
        }

        #[derive(DIPortal)]
        struct HogeFactory {
            foo: DI<Foo>,
        }
        impl HogeFactory {
            async fn create(&self) -> Hoge {
                Hoge {
                    foo: self.foo.clone(),
                }
            }
        }

        #[tokio::test]
        async fn test_di() {
            assert!(Hoge::di().await == Hoge::di().await)
        }
    }
}
