//! proc-macros for generate trait implementations.

mod derive_di_portal;
mod provider;
mod def_di_provider_sync;
mod def_async_di_provider;

pub(crate) mod helper;

derive_di_portal::define!();
provider::define!();
def_di_provider_sync::define!();
def_async_di_provider::define!();

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, Token,
};

use crate::helper::Generics_;



/// Generate a di method call with a target provider type name.
///
/// ```ignore
/// di![Hoge]               // => HogeProvider::di()
/// di![Hoge<String, bool>] // => HogeStringboolProvider::di()
/// di![Hoge<String, ()>]   // => HogeStringUnitProvider::di()
/// di![Hoge on c]          // => HogeProvider::di_on(c)
/// ```
#[proc_macro]
#[allow(non_snake_case)]
pub fn di(input: TokenStream) -> TokenStream {
    let DiInput {
        target_ident,
        generics,
        arg,
    } = parse_macro_input!(input as DiInput);

    let type_params_str = generics.type_params_str();
    let provider_type_name = format_ident!("{}{}Provider", target_ident, type_params_str);

    if let Some(arg) = arg.as_ref() {
        quote!(#provider_type_name :: di_on(#arg)).into()
    } else {
        quote!(#provider_type_name :: di()).into()
    }
}

struct DiInput {
    target_ident: syn::Ident,
    generics: Generics_,
    arg: Option<syn::Expr>,
}
impl Parse for DiInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let target_ident = input.parse()?;
        let generics = input.parse()?;
        let arg = if input.peek(kw::on) {
            let _: kw::on = input.parse()?;
            let arg = input.parse()?;
            Some(arg)
        } else {
            None
        };

        Ok(DiInput {
            target_ident,
            generics,
            arg,
        })
    }
}

mod kw {
    syn::custom_keyword!(on);
}
