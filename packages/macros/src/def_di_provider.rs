macro_rules! define {
    () => {
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
        #[proc_macro_error]
        #[proc_macro]
        pub fn def_di_provider(input: TokenStream) -> TokenStream {
            def_di_provider::exec(input.into()).into()
        }
    };
}
pub(crate) use define;

use proc_macro_error::abort;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse2, spanned::Spanned as _};

use crate::helper::DefDiProviderInput;

pub fn exec(input: TokenStream2) -> TokenStream2 {
    let span = input.span();
    let DefDiProviderInput {
        kw_dyn,
        target_ident,
        generics,
        create_fn,
        ..
    } = parse2::<DefDiProviderInput>(input).unwrap_or_else(move |_| {
        abort!(
            span,
            "invalid input format";
            help = "usage: def_di_provider!([dyn] <TargetIdent>[<Generics>], |c| { <create expression> })";
        );
    });

    let ty_params_str = generics.type_params_str();
    let provider_ident = format_ident!("{}{}Provider", target_ident, ty_params_str);

    quote! {
        pub struct #provider_ident;
        impl portaldi::DIProvider for #provider_ident {
            type Output = #kw_dyn #target_ident #generics;

            fn di_on(c: &portaldi::DIContainer) -> portaldi::DI<Self::Output> {
                c.get_or_init(|| (#create_fn)(c))
            }
        }
    }
}
