use portaldi::*;

#[provider(Self)]
struct Hoge {}

#[provider(1+1)]
impl Hoge {}

struct Hoge2;

#[provider(Self)]
impl Hoge2 {}

#[provider(Self)]
impl DIPortal for [Hoge2] {
    fn create_for_di(_: &DIContainer) -> Self {
        todo!()
    }
}

struct Hoge3;

#[provider(Self)]
impl DIPortal for Hoge3 {
    fn create() {}
}

fn main() {}
