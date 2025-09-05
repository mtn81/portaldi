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

pub fn attr_of<'a>(attrs: &'a Vec<Attribute>, name: &str) -> Option<&'a Attribute> {
    attrs.iter().find(|&a| {
        a.path()
            .get_ident()
            .filter(|i| i.to_string() == name)
            .is_some()
    })
}

#[derive(Debug)]
pub struct DefDiProviderInput {
    pub kw_dyn: Option<Token![dyn]>,
    pub target_ident: syn::Ident,
    pub generics: Generics_,
    pub _comma: Token![,],
    pub create_fn: syn::ExprClosure,
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
