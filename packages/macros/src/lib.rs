//! proc-macros for generate trait implementations.

mod derive_di_portal;

pub(crate) mod helper;

derive_di_portal::define!();

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote, Ident, ImplItem, ItemImpl, Path, Token, Type,
};

use crate::helper::{
    async_trait_attr, build_provider, build_provider_by_env, Generics_, ProvideTarget,
};

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
#[proc_macro_attribute]
pub fn provider(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_impl = parse_macro_input!(item as ItemImpl);
    let args = parse_macro_input!(attr as ProviderArgs);

    // dbg!(&item_impl.trait_);
    let is_portal_impl = match &item_impl.trait_ {
        Some((_, p, _)) => p
            .segments
            .iter()
            .any(|s| s.ident == "DIPortal" || s.ident == "AsyncDIPortal"),
        _ => false,
    };
    if !is_portal_impl {
        panic!("[provider] must be on DIPortal or AsyncDIPortal")
    }

    // dbg!(&item_impl.self_ty);
    let (ident, path_args) = match *item_impl.self_ty {
        Type::Path(ref p) => p.path.segments.last().map(|s| (&s.ident, &s.arguments)),
        _ => None,
    }
    .expect("impl type name not found.");

    let di_method = item_impl
        .items
        .iter()
        .find_map(|item| match item {
            ImplItem::Fn(m) if m.sig.ident == "create_for_di" => Some(m),
            _ => None,
        })
        .expect("'di' method must be defined.");

    let is_async = di_method.sig.asyncness.is_some();

    let provider_quote = match args {
        ProviderArgs::TargetProvider(target) => {
            build_provider(&ident, &target, is_async, true, None)
        }
        ProviderArgs::EnvProvider => build_provider_by_env(&ident, is_async),
        ProviderArgs::SelfProvider => build_provider(
            &ident,
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
    .into()
}

/// Generate a [`DIProvider`] implementation.
///
/// ```ignore
/// pub struct Hoge {}
///
/// // This macro is useful if you want to define the [`DIProvider`] manually.
/// def_di_provider!(Hoge, |c| {
///     // some creation logic
/// });
///
/// // Also you can define provider for a trait.
/// def_di_provider!(dyn HogeI, |c| {
///     // some creation logic
/// });
///
/// // Also you can define provider for a trait with generics.
/// def_di_provider!(dyn HogeI<A>, |c| {
///     // some creation logic
/// });
///
/// ```
#[proc_macro]
pub fn def_di_provider(input: TokenStream) -> TokenStream {
    let DefDiProviderInput {
        kw_dyn,
        target_ident,
        generics,
        create_fn,
        ..
    } = parse_macro_input!(input as DefDiProviderInput);

    let ty_params_str = generics.type_params_str();
    let provider_ident = format_ident!("{}{}Provider", target_ident, ty_params_str);

    let result = quote! {
        pub struct #provider_ident;
        impl portaldi::DIProvider for #provider_ident {
            type Output = #kw_dyn #target_ident #generics;

            fn di_on(c: &portaldi::DIContainer) -> portaldi::DI<Self::Output> {
                c.get_or_init(|| (#create_fn)(c))
            }
        }
    };

    result.into()
}

/// Generate a [`AsyncDIProvider`] implementation.
///
/// ```ignore
/// pub struct Hoge {}
///
/// // This macro is useful if you want to define a [`AsyncDIProvider`] manually.
/// def_async_di_provider!(Hoge, |c| async {
///     // some asynchronous creation logic
/// });
///
/// // Also you can define provider for a trait.
/// def_async_di_provider!(dyn HogeI, |c| {
///     // some creation logic
/// });
///
/// // Also you can define provider for a trait with generics.
/// def_async_di_provider!(dyn HogeI<A>, |c| {
///     // some creation logic
/// });
///
/// ```
#[proc_macro]
pub fn def_async_di_provider(input: TokenStream) -> TokenStream {
    let DefDiProviderInput {
        kw_dyn,
        target_ident,
        generics,
        create_fn,
        ..
    } = parse_macro_input!(input as DefDiProviderInput);

    let ty_params_str = generics.type_params_str();
    let provider_ident = format_ident!("{}{}Provider", target_ident, ty_params_str);

    let async_trait_attr = async_trait_attr();

    let result = quote! {
        pub struct #provider_ident;

        #async_trait_attr
        impl portaldi::AsyncDIProvider for #provider_ident {
            type Output = #kw_dyn #target_ident #generics;

            async fn di_on(c: &portaldi::DIContainer) -> portaldi::DI<Self::Output> {
                c.get_or_init_async(|| (#create_fn)(c)).await
            }
        }
    };

    result.into()
}

struct DefDiProviderInput {
    kw_dyn: Option<Token![dyn]>,
    target_ident: syn::Ident,
    generics: Generics_,
    _comma: Token![,],
    create_fn: syn::ExprClosure,
}
impl Parse for DefDiProviderInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let kw_dyn = if input.peek(Token![dyn]) {
            Some(input.parse()?)
        } else {
            None
        };
        let target_ident = input.parse()?;
        let generics = input.parse()?;
        let _comma = input.parse()?;
        let create_fn = input.parse()?;

        Ok(DefDiProviderInput {
            kw_dyn,
            target_ident,
            generics,
            _comma,
            create_fn,
        })
    }
}

#[derive(PartialEq)]
enum InjectAttrPart {
    Path(Path),
    Async,
}

impl Parse for InjectAttrPart {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(if input.peek(Token![async]) {
            input.parse::<Token![async]>()?;
            InjectAttrPart::Async
        } else {
            InjectAttrPart::Path(input.parse()?)
        })
    }
}

#[derive(Debug)]
enum ProviderArgs {
    SelfProvider,
    TargetProvider(ProvideTarget),
    EnvProvider,
}

impl Parse for ProviderArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(if input.peek(Token![Self]) {
            let _: Token![Self] = input.parse()?;
            ProviderArgs::SelfProvider
        } else {
            let ident_: Option<Ident> = input.parse()?;
            if let Some(ident) = ident_ {
                let generics = input.parse()?;
                ProviderArgs::TargetProvider(ProvideTarget { ident, generics })
            } else {
                ProviderArgs::EnvProvider
            }
        })
    }
}

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
