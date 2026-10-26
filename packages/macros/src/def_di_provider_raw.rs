macro_rules! define {
    () => {
        /// Generate a [`DIProvider`] implementation whose closure returns `DI<T>` directly.
        ///
        /// Unlike [`def_di_provider!`](macro@def_di_provider), the closure is responsible for
        /// getting the component from the container (e.g. with `c.get_or_init(...)`).
        /// This allows switching the implementation of a trait by a condition.
        ///
        /// ```ignore
        /// pub trait HogeI {}
        ///
        /// def_di_provider_raw!(dyn HogeI, |c| {
        ///     if some_condition {
        ///         c.get_or_init(|| HogeA {})
        ///     } else {
        ///         c.get_or_init(|| HogeB {})
        ///     }
        /// });
        /// ```
        ///
        /// [`DIProvider`]: https://docs.rs/portaldi/latest/portaldi/trait.DIProvider.html
        #[proc_macro_error]
        #[proc_macro]
        pub fn def_di_provider_raw(input: TokenStream) -> TokenStream {
            def_di_provider_raw::exec(input.into()).into()
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
            help = "usage: def_di_provider_raw!([dyn] <TargetIdent>[<Generics>], |c| { <expression returning DI<Target>> })";
        );
    });

    let ty_params_str = generics.type_params_str();
    let provider_ident = format_ident!("{}{}Provider", target_ident, ty_params_str);

    quote! {
        pub struct #provider_ident;
        impl portaldi::DIProvider for #provider_ident {
            type Output = #kw_dyn #target_ident #generics;

            fn di_on(c: &portaldi::DIContainer) -> portaldi::DI<Self::Output> {
                // Give the closure an expected signature so that each branch coerces to `DI<Self::Output>`.
                let create: &dyn Fn(&portaldi::DIContainer) -> portaldi::DI<Self::Output> = &(#create_fn);
                create(c)
            }
        }
    }
}
