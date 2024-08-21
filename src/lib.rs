//! # `kovi-macros`
//!
//! `kovi-macros` is an auxiliary library for the [`kovi`](https://crates.io/crates/kovi) crate.


use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn plugin(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let is_async = input.sig.asyncness.is_some();
    let fn_name = &input.sig.ident;

    let expanded = if is_async {
        quote! {
            #input

            pub fn __kovi__get_plugin_info() -> (&'static str, &'static str) {
                let name = env!("CARGO_PKG_NAME");
                let version = env!("CARGO_PKG_VERSION");
                (name, version)
            }

            pub fn __kovi__run_async_plugin(p: PluginBuilder) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
                Box::pin(async move {
                    #fn_name(p).await;
                })
            }
        }
    } else {
        quote! {
            #input

            pub fn __kovi__get_plugin_info() -> (&'static str, &'static str) {
                let name = env!("CARGO_PKG_NAME");
                let version = env!("CARGO_PKG_VERSION");
                (name, version)
            }
        }
    };

    TokenStream::from(expanded)
}
