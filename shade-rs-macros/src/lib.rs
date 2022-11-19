#![feature(iter_intersperse)]
#![feature(proc_macro_diagnostic)]

mod code_utils;
mod convenience_wrap;
mod mapper;
mod parser;

use parser::ShaderFnParser;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::visit::Visit;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn fragment_shader(_attr: TokenStream, function: TokenStream) -> TokenStream {
    let function = syn::parse::<ItemFn>(function).unwrap();

    let fun_name = format_ident!("{}", function.sig.ident.to_string());

    let mut visitor = ShaderFnParser::new();
    visitor.visit_item_fn(&function);
    let code = visitor.code;

    let block = &function.block;

    let expanded = quote! {
        fn #fun_name() -> &'static str {
            fn _orig_func_just_for_compiler_checks()
            #block

            #code
        }
    };

    TokenStream::from(expanded)
}
