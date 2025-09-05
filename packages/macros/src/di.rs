macro_rules! define {
    () => {
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
            di::exec(input.into()).into()
        }
    };
}
pub(crate) use define;

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse2,
};

use crate::helper::kw;
use crate::helper::Generics_;

pub fn exec(input: TokenStream2) -> TokenStream2 {
    let DiInput {
        target_ident,
        generics,
        arg,
    } = parse2::<DiInput>(input).unwrap();

    let type_params_str = generics.type_params_str();
    let provider_type_name = format_ident!("{}{}Provider", target_ident, type_params_str);

    if let Some(arg) = arg.as_ref() {
        quote!(#provider_type_name :: di_on(#arg))
    } else {
        quote!(#provider_type_name :: di())
    }
}

#[derive(Debug)]
pub struct DiInput {
    pub target_ident: syn::Ident,
    pub generics: Generics_,
    pub arg: Option<syn::Expr>,
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
