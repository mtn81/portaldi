use crate::common::*;

#[test]
fn test_di() {}

pub struct foo;

#[derive(DIPortal)]
struct Hoge {
    foo: DI<Foo>, // unit struct と 同じフィールド名だと コンパイルエラーになっていたのを修正
}

#[derive(DIPortal)]
struct Foo {}
