use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use regex::Regex;
use syn::{
    parse::{Parse, ParseStream},
    parse2,
    punctuated::Punctuated,
    token::Comma,
    Attribute, Data, DeriveInput, GenericArgument, Ident, Meta, Path, PathArguments, Token, Type,
    TypeParamBound, TypePath, TypeTuple, Visibility,
};

// syn::Generics では unit を解決できなかったので自前で実装
#[derive(Debug, Default)]
pub struct Generics_ {
    pub lt: Option<Token![<]>,
    pub params: Punctuated<Type, Comma>,
    pub gt: Option<Token![>]>,
}

impl Parse for Generics_ {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(if input.peek(Token![<]) {
            let lt: Token![<] = input.parse()?;
            let mut params: Punctuated<Type, Comma> = Punctuated::new();
            let gt: Token![>];
            loop {
                let t: Type = input.parse()?;
                params.push(t);
                if input.peek(Comma) {
                    let _: Comma = input.parse()?;
                }
                if input.peek(Token![>]) {
                    gt = input.parse()?;
                    break;
                }
            }
            Self {
                lt: Some(lt),
                params,
                gt: Some(gt),
            }
        } else {
            Self {
                lt: None,
                params: Punctuated::new(),
                gt: None,
            }
        })
    }
}

impl ToTokens for Generics_ {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.lt.to_tokens(tokens);
        self.params.to_tokens(tokens);
        self.gt.to_tokens(tokens);
    }
}

impl Generics_ {
    pub fn type_params_str(&self) -> String {
        self.params
            .iter()
            .flat_map(|p| match p {
                Type::Path(TypePath { path, .. }) => Some(path.to_token_stream().to_string()),
                Type::Tuple(TypeTuple { elems, .. }) if elems.is_empty() => {
                    Some("Unit".to_string())
                }
                _ => None,
            })
            .collect::<Vec<_>>()
            .concat()
    }
}

pub fn async_trait_attr() -> proc_macro2::TokenStream {
    if cfg!(feature = "multi-thread") {
        quote! {
            #[async_trait::async_trait]
        }
    } else {
        quote! {
            #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
            #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
        }
    }
}

pub fn attr_of<'a>(attrs: &'a Vec<Attribute>, name: &str) -> Option<&'a Attribute> {
    attrs.iter().find(|&a| {
        a.path()
            .get_ident()
            .filter(|i| i.to_string() == name)
            .is_some()
    })
}
