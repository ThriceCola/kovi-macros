//! # `kovi-macros`
//!
//! `kovi-macros` is an auxiliary library for the [`kovi`](https://crates.io/crates/kovi) crate.


use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn plugin(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let expanded = quote! {
        #input

        pub fn __kovi__get_crate_name() -> (&'static str, &'static str) {
            let name = env!("CARGO_PKG_NAME");
            let version = env!("CARGO_PKG_VERSION");
            (name, version)
        }
    };

    TokenStream::from(expanded)
}
