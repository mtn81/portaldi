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
        #[proc_macro]
        pub fn def_di_provider(input: TokenStream) -> TokenStream {
            def_di_provider_sync::exec(input.into()).into()
        }
    };
}
pub(crate) use define;

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::parse2;

use crate::helper::DefDiProviderInput;

pub fn exec(input: TokenStream2) -> TokenStream2 {
    let DefDiProviderInput {
        kw_dyn,
        target_ident,
        generics,
        create_fn,
        ..
    } = parse2::<DefDiProviderInput>(input).unwrap();

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