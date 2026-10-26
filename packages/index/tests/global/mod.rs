use crate::helper::*;

#[test]
fn test_get_from_global() {
    assert!(global::get::<Hoge>().is_none());

    let hoge = Hoge::di();
    assert!(ptr_eq(
        global::get::<Hoge>().unwrap().as_ref(),
        hoge.as_ref()
    ));
    assert!(ptr_eq(
        global::get::<Foo>().unwrap().as_ref(),
        hoge.foo.as_ref()
    ));
}

#[derive(DIPortal)]
struct Hoge {
    foo: DI<Foo>,
}

#[derive(DIPortal)]
struct Foo {}
