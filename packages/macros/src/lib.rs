use proc_macro::TokenStream;
use quote::quote;
use regex::Regex;
use syn::{
    parse_macro_input, Attribute, AttributeArgs, Data, DeriveInput, GenericArgument, Ident,
    ImplItem, ItemImpl, Meta, NestedMeta, Path, PathArguments, Type, TypeParamBound,
};

#[proc_macro_derive(DIPortal, attributes(provide, inject))]
pub fn derive_di_portal(input: TokenStream) -> TokenStream {
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
                    let is_async = inject_attr.as_ref().map(|a| a.is_async).unwrap_or(false);
                    let inject_path = inject_attr.as_ref().and_then(|a| a.path.as_ref());
                    let field_di = build_field_di(&f, inject_path, is_async);
                    (field_di, is_async)
                })
                .collect();

            let field_di_quotes = field_dis.iter().map(|f| &f.0).collect::<Vec<_>>();
            let is_totally_async = field_dis.iter().any(|f| f.1);
            let di_portal_quote = build_portal(&ident, field_di_quotes, is_totally_async);

            let provider_quote = if let Some(provide_attr) = attr_of(&attrs, "provide") {
                let provide_target = provide_attr.parse_args::<syn::Ident>().unwrap();
                build_provider(&ident, &provide_target, is_totally_async)
            } else {
                build_provider_by_env(&ident, is_totally_async)
            };

            // let q = quote! {
            //     #provider_quote
            //     #di_portal_quote
            // };
            // println!("check !!!! {:}", q.to_string());

            quote! {
                #provider_quote
                #di_portal_quote
            }
            .into()
        }
        _ => syn::Error::new_spanned(&ident, "Must be struct type")
            .to_compile_error()
            .into(),
    }
}

#[proc_macro_attribute]
pub fn provider(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_impl = parse_macro_input!(item as ItemImpl);
    let attr_args = parse_macro_input!(attr as AttributeArgs);

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

    dbg!(&item_impl.self_ty);
    let ident = match *item_impl.self_ty {
        Type::Path(ref p) => p.path.segments.first().map(|s| &s.ident),
        _ => None,
    }
    .expect("impl type name not found.");

    let provider_target = attr_args.iter().find_map(|arg| match arg {
        NestedMeta::Meta(Meta::Path(p)) => p.get_ident(),
        _ => None,
    });

    let di_method = item_impl
        .items
        .iter()
        .find_map(|item| match item {
            ImplItem::Method(m) if m.sig.ident == "create_for_di" => Some(m),
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

fn attr_of<'a>(attrs: &'a Vec<Attribute>, name: &str) -> Option<&'a Attribute> {
    attrs.iter().find(|&a| {
        a.path
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
        let first_path_segment = x.path.segments.first().unwrap();

        if first_path_segment.ident != "DI" {
            return None;
        }

        if let PathArguments::AngleBracketed(x) = &first_path_segment.arguments {
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

fn build_provider(
    ident: &Ident,
    provide_target: &Ident,
    is_async: bool,
) -> proc_macro2::TokenStream {
    let provider_type = quote::format_ident!("{}Provider", provide_target);
    if is_async {
        quote! {
            pub struct #provider_type;

            #[async_trait::async_trait]
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
    field_di_quotes: Vec<&proc_macro2::TokenStream>,
    is_async: bool,
) -> proc_macro2::TokenStream {
    let di_target_quote = quote! {
        impl portaldi::DITarget for #ident {}
    };
    if is_async {
        quote! {
            // #di_target_quote

            #[async_trait::async_trait]
            impl portaldi::AsyncDIPortal for #ident {
                async fn create_for_di(container: &portaldi::DIContainer) -> Self {
                    #ident { #(#field_di_quotes)* }
                }
            }
        }
    } else {
        quote! {
            // #di_target_quote

            impl portaldi::DIPortal for #ident {
                fn create_for_di(container: &portaldi::DIContainer) -> Self {
                    #ident { #(#field_di_quotes)* }
                }
            }
        }
    }
}

fn build_field_di(
    f: &syn::Field,
    inject_path: Option<&Path>,
    is_async: bool,
) -> proc_macro2::TokenStream {
    let fname = f.ident.as_ref().unwrap();
    inject_path.map_or_else(
        || match get_di_type(&f.ty).expect(format!("{} is not DI type", fname.to_string()).as_str())
        {
            DIType::Trait {
                type_ident: di_type,
            } => {
                let di_provider_type = quote::format_ident!("{}Provider", di_type);
                if is_async {
                    quote! {
                        #fname: #di_provider_type::di_on(container).await,
                    }
                } else {
                    quote! {
                        #fname: #di_provider_type::di_on(container),
                    }
                }
            }
            DIType::Concrete { path: di_type } => {
                if is_async {
                    quote! {
                        #fname: #di_type::di_on(container).await,
                    }
                } else {
                    quote! {
                        #fname: #di_type::di_on(container),
                    }
                }
            }
        },
        |path| {
            if is_async {
                quote! {
                    #fname: #path::di_on(container).await,
                }
            } else {
                quote! {
                    #fname: #path::di_on(container),
                }
            }
        },
    )
}

struct InjectAttr {
    path: Option<Path>,
    is_async: bool,
}

fn parse_inject_attr(attrs: &Vec<Attribute>) -> Option<InjectAttr> {
    attr_of(attrs, "inject").and_then(|attr| match &attr.parse_meta() {
        Ok(Meta::List(x)) => {
            let is_async = x.nested.iter().any(|arg| match arg {
                NestedMeta::Meta(Meta::Path(p)) => p.is_ident("async"),
                _ => false,
            });

            let path = x.nested.iter().find_map(|arg| match arg {
                NestedMeta::Meta(Meta::Path(p)) if !p.is_ident("async") => Some(p.clone()),
                _ => None,
            });
            Some(InjectAttr { path, is_async })
        }
        _ => None,
    })
}
