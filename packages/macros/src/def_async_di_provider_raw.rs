macro_rules! define {
    () => {
        /// Generate an [`AsyncDIProvider`] implementation whose closure returns `DI<T>` directly.
        ///
        /// Unlike [`def_async_di_provider!`](macro@def_async_di_provider), the closure is responsible for
        /// getting the component from the container (e.g. with `c.get_or_init_async(...).await`).
        /// This allows switching the implementation of a trait by a condition.
        ///
        /// ```ignore
        /// pub trait HogeI {}
        ///
        /// def_async_di_provider_raw!(dyn HogeI, |c| async move {
        ///     if some_condition {
        ///         c.get_or_init_async(|| async { HogeA {} }).await
        ///     } else {
        ///         c.get_or_init_async(|| async { HogeB {} }).await
        ///     }
        /// });
        /// ```
        ///
        /// [`AsyncDIProvider`]: https://docs.rs/portaldi/latest/portaldi/trait.AsyncDIProvider.html
        #[proc_macro_error]
        #[proc_macro]
        pub fn def_async_di_provider_raw(input: TokenStream) -> TokenStream {
            def_async_di_provider_raw::exec(input.into()).into()
        }
    };
}
pub(crate) use define;

use proc_macro_error::abort;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse2, spanned::Spanned as _};

use crate::helper::{DefDiProviderInput, async_trait_attr};

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
            help = "usage: def_async_di_provider_raw!([dyn] <TargetIdent>[<Generics>], |c| async { <expression returning DI<Target>> })";
        );
    });

    let ty_params_str = generics.type_params_str();
    let provider_ident = format_ident!("{}{}Provider", target_ident, ty_params_str);

    // The closure is expanded inline instead of being called, because a closure returning
    // a future that borrows its argument can not be typed without higher-ranked lifetime issues.
    let mut inputs = create_fn.inputs.iter();
    let (Some(arg), None) = (inputs.next(), inputs.next()) else {
        abort!(
            create_fn.span(),
            "the closure must take exactly one argument (the container)"
        );
    };
    // If the body is an async block, expand its statements directly so that
    // each branch coerces to `DI<Self::Output>`.
    let create_expr = match create_fn.body.as_ref() {
        syn::Expr::Async(syn::ExprAsync { block, .. }) => {
            let stmts = &block.stmts;
            quote! { { #(#stmts)* } }
        }
        body => quote! { (#body).await },
    };

    let async_trait_attr = async_trait_attr();

    quote! {
        pub struct #provider_ident;

        #async_trait_attr
        impl portaldi::AsyncDIProvider for #provider_ident {
            type Output = #kw_dyn #target_ident #generics;

            async fn di_on(c: &portaldi::DIContainer) -> portaldi::DI<Self::Output> {
                let #arg = c;
                let component: portaldi::DI<Self::Output> = #create_expr;
                component
            }
        }
    }
}
