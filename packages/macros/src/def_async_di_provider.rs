macro_rules! define {
    () => {
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
            def_async_di_provider::exec(input.into()).into()
        }
    };
}
pub(crate) use define;

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::parse2;

use crate::helper::{async_trait_attr, DefDiProviderInput};

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

    let async_trait_attr = async_trait_attr();

    quote! {
        pub struct #provider_ident;

        #async_trait_attr
        impl portaldi::AsyncDIProvider for #provider_ident {
            type Output = #kw_dyn #target_ident #generics;

            async fn di_on(c: &portaldi::DIContainer) -> portaldi::DI<Self::Output> {
                c.get_or_init_async(|| (#create_fn)(c)).await
            }
        }
    }
}