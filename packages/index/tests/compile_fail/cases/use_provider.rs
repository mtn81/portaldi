use portaldi::*;

#[provider(Self)]
struct Hoge {}

#[provider(1+1)]
impl Hoge {}

struct Hoge2;

#[provider(Self)]
impl Hoge2 {}

fn main() {}
