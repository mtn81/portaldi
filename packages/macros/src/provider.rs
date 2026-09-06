macro_rules! define {
    () => {
        /// Generate a [`DIProvider`] or [`AsyncDIProvider`] implementation. (attribute macro)
        ///
        /// This attribute must be on [`DIPortal`] or [`AsyncDIPortal`] impl block.
        ///
        /// ```ignore
        /// trait HogeI {}
        ///
        /// struct Hoge {}
        ///
        /// // When you needs manual creation logic, define DIPortal implementation.
        /// #[portaldi::provider(HogeI)] // HogeIProvider will be generated.
        /// impl DIPortal for Hoge {
        ///   ...
        /// }
        /// ```
        ///
        /// For a trait with generics,
        /// ```ignore
        /// trait HogeI<A> {}
        ///
        /// struct Hoge {}
        ///
        /// // When you needs manual creation logic, define DIPortal implementation.
        /// #[portaldi::provider(HogeI<A>)] // HogeIAProvider will be generated.
        /// impl DIPortal for Hoge {
        ///   ...
        /// }
        /// ```
        ///
        /// You can also generate [`DIProvider`] for Self type.
        /// ```ignore
        /// struct Hoge {}
        ///
        /// // When you needs manual creation logic, define DIPortal implementation.
        /// #[portaldi::provider(Self)] // HogeProvider will be generated.
        /// impl DIPortal for Hoge {
        ///   ...
        /// }
        /// ```
        ///
        #[proc_macro_error]
        #[proc_macro_attribute]
        pub fn provider(attr: TokenStream, item: TokenStream) -> TokenStream {
            provider::exec(attr.into(), item.into()).into()
        }
    };
}
pub(crate) use define;

use proc_macro_error::abort;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    Ident, ImplItem, ItemImpl, Token, Type, TypePath,
    parse::{Parse, ParseStream},
    parse_quote, parse2,
    spanned::Spanned,
};

use crate::helper::{ProvideTarget, build_provider, build_provider_by_env};

pub fn exec(attr: TokenStream2, item: TokenStream2) -> TokenStream2 {
    let span = item.span();
    let item_impl = parse2::<ItemImpl>(item)
        .unwrap_or_else(|e| abort!(span, format!("[provider] must be on impl block: {e}")));

    let args = parse2::<ProviderArgs>(attr.clone()).unwrap_or_else(move |_| {
        abort!(
            attr,
            "invalid attribute args";
            help = "must be `Self`, <TargetTrait> or empty (for env)"
        );
    });

    // dbg!(&item_impl.trait_);
    let is_portal_impl = match &item_impl.trait_ {
        Some((_, p, _)) => p
            .segments
            .last()
            .is_some_and(|s| s.ident == "DIPortal" || s.ident == "AsyncDIPortal"),
        _ => false,
    };
    if !is_portal_impl {
        abort!(
            &item_impl,
            "must be on impl DIPortal or AsyncDIPortal block"
        )
    }

    // dbg!(&item_impl.self_ty);
    let (ident, path_args) = (match *item_impl.self_ty {
        Type::Path(TypePath { ref path, .. }) => {
            path.segments.last().map(|s| (&s.ident, &s.arguments))
        }
        _ => None,
    })
    .unwrap_or_else(|| {
        abort!(item_impl.self_ty, "self type name not found.");
    });

    let di_method = item_impl
        .items
        .iter()
        .find_map(|item| match item {
            ImplItem::Fn(m) if m.sig.ident == "create_for_di" => Some(m),
            _ => None,
        })
        .unwrap_or_else(|| abort!(item_impl, "'create_for_di' method must be defined."));

    let is_async = di_method.sig.asyncness.is_some();

    let provider_quote = match args {
        ProviderArgs::Target(target) => build_provider(ident, &target, is_async, true, None),
        ProviderArgs::Env => build_provider_by_env(ident, is_async),
        ProviderArgs::Same => build_provider(
            ident,
            &ProvideTarget {
                ident: ident.clone(),
                generics: parse_quote!(#path_args),
            },
            is_async,
            false,
            None,
        ),
    };

    // let q = quote! {
    //     #item_impl
    //     #provider_quote
    // };
    // println!("check !!!! {:}", q.to_string());

    quote! {
        #item_impl
        #provider_quote
    }
}

#[derive(Debug)]
enum ProviderArgs {
    Same,
    Target(ProvideTarget),
    Env,
}

impl Parse for ProviderArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(if input.peek(Token![Self]) {
            let _: Token![Self] = input.parse()?;
            ProviderArgs::Same
        } else {
            let ident_: Option<Ident> = input.parse()?;
            if let Some(ident) = ident_ {
                let generics = input.parse()?;
                ProviderArgs::Target(ProvideTarget { ident, generics })
            } else {
                ProviderArgs::Env
            }
        })
    }
}
