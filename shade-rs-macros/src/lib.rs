#![feature(iter_intersperse)]
#![feature(proc_macro_diagnostic)]

mod code_utils;
mod convenience_wrap;
mod mapper;
mod parser;
mod types;

use parser::ShaderFnParser;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::visit::Visit;
use syn::ItemFn;
use syn::ItemMod;

#[proc_macro_attribute]
pub fn fragment_shader(_attr: TokenStream, function: TokenStream) -> TokenStream {
    let mut function = syn::parse::<ItemFn>(function).unwrap();

    let fun_name = format_ident!("{}", function.sig.ident.to_string());
    let block = &function.block;

    let mut code_parser = ShaderFnParser::new();
    code_parser.visit_item_fn(&function);
    let code = code_parser.code;

    // TODO: process uniforms
    for arg in &mut function.sig.inputs {
        let syn::FnArg::Typed(arg) = arg else {
            panic!();
        };

        arg.attrs.clear();
    }

    let args = function.sig.inputs;

    let expanded = quote! {
        pub struct #fun_name {
        }

        impl #fun_name {
            pub fn call(#args) {
                #block
            }


            pub fn code() -> &'static str {
                #code
            }

        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn module(_attr: TokenStream, program: TokenStream) -> TokenStream {
    let module = syn::parse::<ItemMod>(program).unwrap();

    let mod_name = format_ident!("{}", module.ident.to_string());
    let mod_items = module.content.map(|(_, items)| items).unwrap_or(vec![]);

    let expanded = quote! {
        mod #mod_name {
            use shade_rs::*;
            #(#mod_items)*
        }
    };

    TokenStream::from(expanded)
}
