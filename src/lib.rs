use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[cfg(feature = "dylib")]
#[proc_macro_attribute]
pub fn plugin(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;

    let expanded = quote! {
        #input

        pub fn __kovi_get_plugin_info() -> (&'static str, &'static str) {
            let name = env!("CARGO_PKG_NAME");
            let version = env!("CARGO_PKG_VERSION");
            (name, version)
        }

        pub fn __kovi_run_async_plugin() -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
        {
            Box::pin(async {
                #fn_name().await;
            })
        }

        #[unsafe(no_mangle)]
        pub fn __kovi_build_plugin() -> kovi::plugin::Plugin {
            let (name, version) = crate::__kovi_get_plugin_info();

            kovi::plugin::Plugin::new(
                name,
                version,
                std::sync::Arc::new(crate::__kovi_run_async_plugin),
            )
        }

        #[unsafe(no_mangle)]
        pub fn __kovi_dylib_init(
            rt: kovi::runtime::Runtime,
            task_manager: kovi::task::TaskManager,
            plugin_builder: kovi::PluginBuilder,
        ) {
            kovi::runtime::RUNTIME.set(rt).unwrap();
            kovi::task::TASK_MANAGER.set(task_manager).unwrap();
            kovi::plugin::PLUGIN_BUILDER.set(plugin_builder).unwrap();
            kovi::plugin::PLUGIN_NAME
                .set(env!("CARGO_PKG_NAME").to_string())
                .unwrap();
        }

    };

    TokenStream::from(expanded)
}

#[cfg(not(feature = "dylib"))]
#[proc_macro_attribute]
pub fn plugin(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;

    let expanded = quote! {
        #input

        pub fn __kovi_get_plugin_info() -> (&'static str, &'static str) {
            let name = env!("CARGO_PKG_NAME");
            let version = env!("CARGO_PKG_VERSION");
            (name, version)
        }

        pub fn __kovi_run_async_plugin() -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
        {
            Box::pin(async {
                #fn_name().await;
            })
        }

        #[unsafe(no_mangle)]
        pub fn __kovi_build_plugin() -> kovi::plugin::Plugin {
            let (name, version) = crate::__kovi_get_plugin_info();

            kovi::plugin::Plugin::new(
                name,
                version,
                std::sync::Arc::new(crate::__kovi_run_async_plugin),
            )
        }
    };

    TokenStream::from(expanded)
}
