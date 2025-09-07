macro_rules! define {
    () => {
        /// Generate a [`DIPortal`] and [`DIProvider`] or [`AsyncDIPortal`] and [`AsyncDIProvider`] implementation.
        ///
        /// * `provide`: generate [`DIProvider`] implementation for a specified trait.
        ///   ```ignore
        ///   #[derive(DIPortal)]
        ///   #[provide(HogeI)] // HogeIProvider will be generated.
        ///   struct Hoge {
        ///     foo: DI<dyn FooI>  // needs FooIProvider in the current scope.
        ///   }
        ///   ```
        ///   For a trait with generics,
        ///   ```ignore
        ///   #[derive(DIPortal)]
        ///   #[provide(HogeI<A>)] // HogeIAProvider will be generated.
        ///   struct Hoge {
        ///     foo: DI<dyn FooI<B>>  // needs FooIBProvider in the current scope.
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
        ///                       // implicitly BarProvider is used.
        ///     bar: DI<Bar>,
        ///     #[inject(MyBazProvider)] // specify DI provider for a another crate concrete type.
        ///     baz: DI<Baz>,
        ///     baz2: DI<Baz>,            // implicitly BarProvider is used.
        ///     piyo: DI<dyn IPiyo>,      // implicitly IPiyoProvider is used.
        ///     piyo2: DI<dyn IPiyo2<A>>, // implicitly IPiyo2AProvider is used.
        ///   }
        ///   ```
        ///
        #[proc_macro_derive(DIPortal, attributes(provide, inject))]
        pub fn derive_di_portal(input: TokenStream) -> TokenStream {
            derive_di_portal::exec(input.into()).into()
        }
    };
}
pub(crate) use define;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse2,
    punctuated::Punctuated,
    Attribute, Data, DeriveInput, GenericArgument, Ident, Meta, Path, PathArguments, Token, Type,
    TypeParamBound, TypeTuple,
};

use crate::helper::{
    async_trait_attr, attr_of, build_provider, build_provider_by_env, Generics_, ProvideTarget,
};

pub fn exec(input: TokenStream) -> TokenStream {
    let is_always_async = std::env::var("PORTALDI_ALWAYS_ASYNC")
        .ok()
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(false);

    let DeriveInput {
        data,
        vis,
        ident,
        generics,
        attrs,
        ..
    } = parse2(input).unwrap();

    match data {
        Data::Struct(s) => {
            let field_dis: Vec<_> = s
                .fields
                .iter()
                .map(|f| {
                    let inject_attr = parse_inject_attr(&f.attrs);
                    let is_async = is_always_async
                        || inject_attr.as_ref().map(|a| a.is_async).unwrap_or(false);
                    let inject_path = inject_attr.as_ref().and_then(|a| a.path.as_ref());
                    let di_expr = build_field_di(&f, inject_path);
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
                let provide_target = provide_attr.parse_args::<ProvideTarget>().unwrap();
                build_provider(&ident, &provide_target, is_totally_async, true, None)
            } else {
                build_provider_by_env(&ident, is_totally_async)
            };

            let self_provider_quote = build_provider(
                &ident,
                &ProvideTarget {
                    ident: ident.clone(),
                    generics: Generics_ {
                        lt: generics.lt_token,
                        params: generics
                            .params
                            .iter()
                            .map(|p| syn::parse2::<Type>(quote!(#p)).unwrap())
                            .collect(),
                        gt: generics.gt_token,
                    },
                },
                is_totally_async,
                false,
                Some(&vis),
            );

            let result = quote! {
                #provider_quote
                #self_provider_quote
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

struct InjectAttr {
    path: Option<Path>,
    is_async: bool,
}

fn parse_inject_attr(attrs: &Vec<Attribute>) -> Option<InjectAttr> {
    attr_of(attrs, "inject").and_then(|attr| match &attr.meta {
        Meta::List(metas) => {
            let args = metas
                .parse_args_with(Punctuated::<InjectAttrPart, Token![,]>::parse_terminated)
                .unwrap();

            let is_async = args.iter().any(|arg| arg == &InjectAttrPart::Async);
            let path = args.iter().find_map(|arg| match arg {
                InjectAttrPart::Path(p) => Some(p.clone()),
                _ => None,
            });

            Some(InjectAttr { path, is_async })
        }
        _ => None,
    })
}

fn build_field_di(f: &syn::Field, inject_path: Option<&Path>) -> proc_macro2::TokenStream {
    inject_path.map_or_else(
        || {
            let DIType {
                type_ident: di_type,
                type_params,
            } = get_di_type(&f.ty)
                .expect(format!("{:?} is not DI type", &f.ident.as_ref()).as_str());

            let type_params_str = type_params
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .concat();
            let di_provider_type = quote::format_ident!("{}{}Provider", di_type, type_params_str);
            quote! {
                #di_provider_type::di_on(container)
            }
        },
        |path| {
            quote! {
                #path::di_on(container)
            }
        },
    )
}

struct DIType<'a> {
    type_ident: &'a Ident,
    type_params: Vec<Ident>,
}

fn get_di_type(ty: &Type) -> Option<DIType<'_>> {
    if let Type::Path(x) = ty {
        let last_path_segment = x.path.segments.last().unwrap();

        if last_path_segment.ident != "DI" {
            return None;
        }

        if let PathArguments::AngleBracketed(x) = &last_path_segment.arguments {
            let path = match x.args.first().unwrap() {
                GenericArgument::Type(Type::TraitObject(x)) => {
                    if let TypeParamBound::Trait(x) = x.bounds.first().unwrap() {
                        Some(&x.path)
                    } else {
                        None
                    }
                }
                GenericArgument::Type(Type::Path(x)) => Some(&x.path),
                _ => None,
            };
            if let Some(path) = path {
                let last_seg = path.segments.last().unwrap();
                let type_params = match &last_seg.arguments {
                    PathArguments::AngleBracketed(x) => x
                        .args
                        .iter()
                        .flat_map(|arg| match arg {
                            GenericArgument::Type(Type::Path(p)) => {
                                p.path.segments.last().map(|s| s.ident.clone())
                            }
                            GenericArgument::Type(Type::Tuple(TypeTuple { elems, .. }))
                                if elems.is_empty() =>
                            {
                                Some(syn::parse2::<Ident>(quote!(Unit)).unwrap())
                            }
                            _ => None,
                        })
                        .collect(),
                    _ => vec![],
                };

                return Some(DIType {
                    type_ident: &last_seg.ident,
                    type_params,
                });
            }
        }
    }
    return None;
}

struct FieldDI {
    field_ident: syn::Ident,
    is_async: bool,
    di_expr: proc_macro2::TokenStream,
}

fn build_portal(
    ident: &Ident,
    field_dis: Vec<FieldDI>,
    is_totally_async: bool,
) -> proc_macro2::TokenStream {
    let to_var_name = |s: &syn::Ident| format_ident!("__di{}", &s);
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
