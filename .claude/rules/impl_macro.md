---
paths:
  - "packages/macros/**"
---

# portaldi-macros の実装規約

可読性の観点から、1マクロが1ファイルになるように分割して、書き方を以下の例の通りに揃える。

`lib.rs`

```rust
mod di;
di::define!();
```

`di.rs`

```rust
macro_rules! define {
    () => {
        /// ...
        #[proc_macro_error]
        #[proc_macro]
        pub fn di(input: TokenStream) -> TokenStream {
            di::exec(input.into()).into()
        }
    };
}
pub(crate) use define;

// use statements...

pub fn exec(input: TokenStream2) -> TokenStream2 {
    // implementation here...
}
```
