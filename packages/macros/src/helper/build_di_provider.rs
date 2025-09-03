use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use regex::Regex;
use syn::{
    parse::{Parse, ParseStream},
    Ident, Visibility,
};

use crate::helper::{async_trait_attr, Generics_};

pub fn build_provider(
    ident: &Ident,
    provide_target: &ProvideTarget,
    is_async: bool,
    for_trait: bool,
    vis: Option<&Visibility>,
) -> TokenStream {
    let type_params_str = provide_target.generics.type_params_str();
    let provider_type = format_ident!("{}{}Provider", provide_target.ident, type_params_str);
    let provide_target_ident = &provide_target.ident;
    let provide_target_generics = &provide_target.generics;
    let dyn_keyword = if for_trait { Some(quote!(dyn)) } else { None };
    let vis = vis.map(|vis| quote!(#vis)).unwrap_or(quote!(pub));
    if is_async {
        let asyn_trait_attr = async_trait_attr();
        quote! {
            #vis struct #provider_type;

            #asyn_trait_attr
            impl portaldi::AsyncDIProvider for #provider_type {
                type Output = #dyn_keyword #provide_target_ident #provide_target_generics;
                async fn di_on(container: &portaldi::DIContainer) -> portaldi::DI<Self::Output> {
                    #ident::di_on(container).await
                }
            }
        }
    } else {
        quote! {
            #vis struct #provider_type;

            impl portaldi::DIProvider for #provider_type {
                type Output = #dyn_keyword #provide_target_ident #provide_target_generics;
                fn di_on(container: &portaldi::DIContainer) -> portaldi::DI<Self::Output> {
                    #ident::di_on(container)
                }
            }
        }
    }
}

pub fn build_provider_by_env(ident: &Ident, is_async: bool) -> TokenStream {
    let ident_str = &ident.to_string();
    let provider_target_cap = std::env::var("PORTALDI_PROVIDER_PATTERN")
        .ok()
        .as_ref()
        .and_then(|pattern| {
            let re = Regex::new(pattern).unwrap();
            re.captures(ident_str)
        });

    if let Some(cap) = provider_target_cap {
        let provide_target = ProvideTarget {
            ident: quote::format_ident!("{}", &cap[1]),
            generics: Generics_::default(),
        };
        build_provider(&ident, &provide_target, is_async, true, None)
    } else {
        quote! {}
    }
}

#[derive(Debug)]
pub struct ProvideTarget {
    pub ident: Ident,
    pub generics: Generics_,
}

impl Parse for ProvideTarget {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let generics = input.parse()?;
        Ok(Self { ident, generics })
    }
}
