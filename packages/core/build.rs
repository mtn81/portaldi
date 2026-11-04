// Aliases only apply inside this crate: macro-generated code and Cargo.toml
// can not use them, so write the full condition there.
fn main() {
    cfg_aliases::cfg_aliases! {
        single_thread: { all(target_arch = "wasm32", not(feature = "multi-thread")) },
    }
}
