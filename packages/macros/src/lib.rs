//! proc-macros for generate trait implementations.

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use regex::Regex;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Attribute, Data, DeriveInput, GenericArgument, Ident, ImplItem, ItemImpl, Meta, Path,
    PathArguments, Token, Type, TypeParamBound,
};

/// Generate a [`DIPortal`] or [`AsyncDIPortal`] implementation. (derive macro)
///
/// * `provide`: generate [`DIProvider`] implementation for a specified trait.
///   ```ignore
///   #[derive(DIPortal)]
///   #[provide(HogeI)] // HogeIProvider will be generated.
///   struct Hoge {
///     foo: DI<dyn FooI>  // needs FooIProvider in the current scope.
///   }
///   ```
///
/// * `inject`: specify DI settings for a field.
///   ```ignore
///   #[derive(DIPortal)]
///   struct Hoge {
///     #[inject(Foo)]  // specify concrete type
///     foo: DI<dyn FooI>,
///     #[inject(AsyncFoo, async)]  // specify concrete type that needs async creation,
///                                 // and consequently AsyncDIPortal for Hoge will be generated.
///     foo2: DI<dyn FooI>,
///     #[inject(async)]  // Bar nedds async creation,
///                       // and consequently AsyncDIPortal for Hoge will be generated.
///     bar: DI<Bar>,
///     #[inject(MyBazProvider)] // specify DI provider for a another crate concrete type.
///     baz: DI<Baz>,
///     #[inject(with_provider)] // specify DI provider for a another crate concrete type (short hand notation).
///     baz2: DI<Baz>,
///   }
///   ```
///
#[proc_macro_derive(DIPortal, attributes(provide, inject))]
pub fn derive_di_portal(input: TokenStream) -> TokenStream {
    let is_always_async = std::env::var("PORTALDI_ALWAYS_ASYNC")
        .ok()
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(false);

    let DeriveInput {
        data, ident, attrs, ..
    } = parse_macro_input!(input);

    match data {
        Data::Struct(s) => {
            let field_dis: Vec<_> = s
                .fields
                .iter()
                .map(|f| {
                    let inject_attr = parse_inject_attr(&f.attrs);
                    let is_async = is_always_async
                        || inject_attr.as_ref().map(|a| a.is_async).unwrap_or(false);
                    let with_provider = inject_attr
                        .as_ref()
                        .map(|a| a.with_provider)
                        .unwrap_or(false);
                    let inject_path = inject_attr.as_ref().and_then(|a| a.path.as_ref());
                    let di_expr = build_field_di(&f, inject_path, with_provider);
                    let field_ident = f.ident.as_ref().unwrap().clone();
                    FieldDI {
                        field_ident,
                        is_async,
                        di_expr,
                    }
                })
                .collect();

            let is_totally_async = is_always_async || field_dis.iter().any(|f| f.is_async);
            let di_portal_quote = build_portal(&ident, field_dis, is_totally_async);

            let provider_quote = if let Some(provide_attr) = attr_of(&attrs, "provide") {
                let provide_target = provide_attr.parse_args::<syn::Ident>().unwrap();
                build_provider(&ident, &provide_target, is_totally_async)
            } else {
                build_provider_by_env(&ident, is_totally_async)
            };

            let result = quote! {
                #provider_quote
                #di_portal_quote
            };

            // println!("check !!!! {:}", result);

            result.into()
        }
        _ => syn::Error::new_spanned(&ident, "Must be struct type")
            .to_compile_error()
            .into(),
    }
}

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
///
/// ```
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
    let ident = match *item_impl.self_ty {
        Type::Path(ref p) => p.path.segments.first().map(|s| &s.ident),
        _ => None,
    }
    .expect("impl type name not found.");

    let provider_target = args.ident;

    let di_method = item_impl
        .items
        .iter()
        .find_map(|item| match item {
            ImplItem::Fn(m) if m.sig.ident == "create_for_di" => Some(m),
            _ => None,
        })
        .expect("'di' method must be defined.");

    let is_async = di_method.sig.asyncness.is_some();

    let provider_quote = provider_target.map_or_else(
        || build_provider_by_env(&ident, is_async),
        |target| build_provider(&ident, &target, is_async),
    );

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
/// di_provider!(Hoge, |c| {
///     // some creation logic
/// });
///
/// // Also you can define provider for a trait.
/// di_provider!(dyn HogeI, |c| {
///     // some creation logic
/// });
///
/// ```
#[proc_macro]
pub fn di_provider(input: TokenStream) -> TokenStream {
    let DefDiProviderInput {
        kw_dyn,
        target_ident,
        create_fn,
        ..
    } = parse_macro_input!(input as DefDiProviderInput);
    let provider_ident = format_ident!("{}Provider", target_ident);

    let result = quote! {
        pub struct #provider_ident;
        impl portaldi::DIProvider for #provider_ident {
            type Output = #kw_dyn #target_ident;

            fn di_on(c: &potaldi::DIContainer) -> portaldi::DI<Self::Output> {
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
/// async_di_provider!(Hoge, |c| async {
///     // some asynchronous creation logic
/// });
///
/// // Also you can define provider for a trait.
/// async_di_provider!(dyn HogeI, |c| {
///     // some creation logic
/// });
///
/// ```
#[proc_macro]
pub fn async_di_provider(input: TokenStream) -> TokenStream {
    let DefDiProviderInput {
        kw_dyn,
        target_ident,
        create_fn,
        ..
    } = parse_macro_input!(input as DefDiProviderInput);
    let provider_ident = format_ident!("{}Provider", target_ident);
    let async_trait_attr = async_trait_attr();

    let result = quote! {
        pub struct #provider_ident;

        #async_trait_attr
        impl portaldi::AsyncDIProvider for #provider_ident {
            type Output = #kw_dyn #target_ident;

            async fn di_on(c: &potaldi::DIContainer) -> portaldi::DI<Self::Output> {
                c.get_or_init_async(|| (#create_fn)(c)).await
            }
        }
    };

    result.into()
}

struct FieldDI {
    field_ident: syn::Ident,
    is_async: bool,
    di_expr: proc_macro2::TokenStream,
}

struct DefDiProviderInput {
    kw_dyn: Option<Token![dyn]>,
    target_ident: syn::Ident,
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
        let _comma = input.parse()?;
        let create_fn = input.parse()?;

        Ok(DefDiProviderInput {
            kw_dyn,
            target_ident,
            _comma,
            create_fn,
        })
    }
}

fn attr_of<'a>(attrs: &'a Vec<Attribute>, name: &str) -> Option<&'a Attribute> {
    attrs.iter().find(|&a| {
        a.path()
            .get_ident()
            .filter(|i| i.to_string() == name)
            .is_some()
    })
}

enum DIType<'a> {
    Trait { type_ident: &'a Ident },
    Concrete { path: &'a Path },
}

fn get_di_type(ty: &Type) -> Option<DIType<'_>> {
    if let Type::Path(x) = ty {
        let last_path_segment = x.path.segments.last().unwrap();

        if last_path_segment.ident != "DI" {
            return None;
        }

        if let PathArguments::AngleBracketed(x) = &last_path_segment.arguments {
            match x.args.first().unwrap() {
                GenericArgument::Type(Type::TraitObject(x)) => {
                    if let TypeParamBound::Trait(x) = x.bounds.first().unwrap() {
                        return Some(DIType::Trait {
                            type_ident: &x.path.segments.last().unwrap().ident,
                        });
                    }
                }
                GenericArgument::Type(Type::Path(x)) => {
                    return Some(DIType::Concrete { path: &x.path });
                }
                _ => return None,
            }
        }
    }
    return None;
}

fn async_trait_attr() -> proc_macro2::TokenStream {
    quote! {
        #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
        #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
    }
}

fn build_provider(
    ident: &Ident,
    provide_target: &Ident,
    is_async: bool,
) -> proc_macro2::TokenStream {
    let provider_type = quote::format_ident!("{}Provider", provide_target);
    if is_async {
        let asyn_trait_attr = async_trait_attr();
        quote! {
            pub struct #provider_type;

            #asyn_trait_attr
            impl portaldi::AsyncDIProvider for #provider_type {
                type Output = dyn #provide_target;
                async fn di_on(container: &portaldi::DIContainer) -> portaldi::DI<Self::Output> {
                    #ident::di_on(container).await
                }
            }
        }
    } else {
        quote! {
            pub struct #provider_type;

            impl portaldi::DIProvider for #provider_type {
                type Output = dyn #provide_target;
                fn di_on(container: &portaldi::DIContainer) -> portaldi::DI<dyn #provide_target> {
                    #ident::di_on(container)
                }
            }
        }
    }
}

fn build_provider_by_env(ident: &Ident, is_async: bool) -> proc_macro2::TokenStream {
    let ident_str = &ident.to_string();
    let provider_target_cap = std::env::var("PORTALDI_PROVIDER_PATTERN")
        .ok()
        .as_ref()
        .and_then(|pattern| {
            let re = Regex::new(pattern).unwrap();
            re.captures(ident_str)
        });

    if let Some(cap) = provider_target_cap {
        let provide_target = quote::format_ident!("{}", &cap[1]);
        build_provider(&ident, &provide_target, is_async)
    } else {
        quote! {}
    }
}

fn build_portal(
    ident: &Ident,
    field_dis: Vec<FieldDI>,
    is_totally_async: bool,
) -> proc_macro2::TokenStream {
    let to_var_name = |s: &syn::Ident| format_ident!("__di_{}", &s);
    let di_var_quotes = if cfg!(feature = "futures-join") {
        let (async_field_dis, sync_field_dis): (Vec<_>, Vec<_>) =
            field_dis.iter().partition(|f| f.is_async);

        let async_di_exprs = async_field_dis.iter().map(|f| &f.di_expr);
        let async_var_names = async_field_dis.iter().map(|f| to_var_name(&f.field_ident));
        let async_quote = if async_field_dis.len() > 1 {
            quote! {
                let (#(#async_var_names),*) = futures::join!(#(#async_di_exprs),*);
            }
        } else if async_field_dis.len() == 1 {
            quote! {
                let #(#async_var_names)* = #(#async_di_exprs)*.await;
            }
        } else {
            quote! {}
        };
        let mut sync_quotes = sync_field_dis
            .iter()
            .map(|f| {
                let ident = &f.field_ident;
                let var_name = to_var_name(&ident);
                let expr = &f.di_expr;
                quote! {
                    let #var_name = #expr;
                }
            })
            .collect::<Vec<_>>();
        let mut result: Vec<proc_macro2::TokenStream> = vec![];
        result.append(&mut sync_quotes);
        result.push(async_quote);
        result
    } else {
        field_dis
            .iter()
            .map(|f| {
                let ident = &f.field_ident;
                let var_name = to_var_name(&ident);
                let expr = &f.di_expr;
                if f.is_async {
                    quote! {
                        let #var_name = #expr.await;
                    }
                } else {
                    quote! {
                        let #var_name = #expr;
                    }
                }
            })
            .collect::<Vec<_>>()
    };

    let field_idents = field_dis.iter().map(|f| {
        let ident = &f.field_ident;
        let var_name = to_var_name(&f.field_ident);
        quote! {
            #ident: #var_name
        }
    });
    if is_totally_async {
        let async_trait_attr = async_trait_attr();

        quote! {
            #async_trait_attr
            impl portaldi::AsyncDIPortal for #ident {
                async fn create_for_di(container: &portaldi::DIContainer) -> Self {
                    #(#di_var_quotes)*
                    #ident { #(#field_idents),* }
                }
            }
        }
    } else {
        quote! {
            impl portaldi::DIPortal for #ident {
                fn create_for_di(container: &portaldi::DIContainer) -> Self {
                    #(#di_var_quotes)*
                    #ident { #(#field_idents),* }
                }
            }
        }
    }
}

fn build_field_di(
    f: &syn::Field,
    inject_path: Option<&Path>,
    with_provider: bool,
) -> proc_macro2::TokenStream {
    let di_type =
        get_di_type(&f.ty).expect(format!("{:?} is not DI type", &f.ident.as_ref()).as_str());
    inject_path.map_or_else(
        || match di_type {
            DIType::Trait {
                type_ident: di_type,
            } => {
                let di_provider_type = quote::format_ident!("{}Provider", di_type);
                quote! {
                    #di_provider_type::di_on(container)
                }
            }
            DIType::Concrete {
                path: di_concrete_type,
            } => {
                let di_type = if with_provider {
                    let concrete_type_ident = &di_concrete_type.segments.last().unwrap().ident;
                    let di_provider_type = format_ident!("{}Provider", concrete_type_ident);
                    quote! { #di_provider_type }
                } else {
                    quote! { #di_concrete_type }
                };
                quote! {
                    #di_type::di_on(container)
                }
            }
        },
        |path| match di_type {
            DIType::Trait { type_ident: _ } => {
                quote! {
                    #path::di_on(container)
                }
            }
            DIType::Concrete { path: _ } => {
                let di_provider_type = path.get_ident().unwrap();
                quote! {
                    #di_provider_type::di_on(container)
                }
            }
        },
    )
}

#[derive(PartialEq)]
enum InjectAttrPart {
    Path(Path),
    Async,
    WithProvider,
}

impl Parse for InjectAttrPart {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(if input.peek(Token![async]) {
            input.parse::<Token![async]>()?;
            InjectAttrPart::Async
        } else if input.peek(kw::with_provider) {
            input.parse::<kw::with_provider>()?;
            InjectAttrPart::WithProvider
        } else {
            InjectAttrPart::Path(input.parse()?)
        })
    }
}

mod kw {
    syn::custom_keyword!(with_provider);
}

struct InjectAttr {
    path: Option<Path>,
    is_async: bool,
    with_provider: bool,
}

fn parse_inject_attr(attrs: &Vec<Attribute>) -> Option<InjectAttr> {
    attr_of(attrs, "inject").and_then(|attr| match &attr.meta {
        Meta::List(metas) => {
            let args = metas
                .parse_args_with(Punctuated::<InjectAttrPart, Token![,]>::parse_terminated)
                .unwrap();

            let is_async = args.iter().any(|arg| arg == &InjectAttrPart::Async);
            let with_provider = args.iter().any(|arg| arg == &InjectAttrPart::WithProvider);
            let path = args.iter().find_map(|arg| match arg {
                InjectAttrPart::Path(p) => Some(p.clone()),
                _ => None,
            });

            Some(InjectAttr {
                path,
                is_async,
                with_provider,
            })
        }
        _ => None,
    })
}

#[derive(Debug)]
struct ProviderArgs {
    ident: Option<syn::Ident>,
}

impl Parse for ProviderArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ProviderArgs {
            ident: input.parse()?,
        })
    }
}
