//! proc-macros for generate trait implementations.

use proc_macro::TokenStream;

mod derive_di_portal;
mod provider;
mod def_di_provider_sync;
mod def_async_di_provider;
mod di_macro;

pub(crate) mod helper;

derive_di_portal::define!();
provider::define!();
def_di_provider_sync::define!();
def_async_di_provider::define!();
di_macro::define!();

