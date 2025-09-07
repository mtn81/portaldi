//! proc-macros for generate trait implementations.

pub(crate) mod helper;

use proc_macro::TokenStream;

mod def_async_di_provider;
def_async_di_provider::define!();

mod def_di_provider;
def_di_provider::define!();

mod derive_di_portal;
derive_di_portal::define!();

mod di;
di::define!();

mod provider;
provider::define!();
