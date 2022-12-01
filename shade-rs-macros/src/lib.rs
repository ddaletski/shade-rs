#![feature(iter_intersperse)]
#![feature(proc_macro_diagnostic)]

mod code_utils;
mod convenience_wrap;
mod global_variables;
mod mapper;
mod parser;
mod types;

use convenience_wrap::ConvenienceWrap;
use parser::ShaderFnParser;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use shade_rs_core::parsing_error;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::ItemFn;
use syn::ItemMod;

#[proc_macro_attribute]
pub fn fragment_shader(_attr: TokenStream, function: TokenStream) -> TokenStream {
    let mut function = syn::parse::<ItemFn>(function).unwrap();

    let fun_name = format_ident!("{}", function.sig.ident.to_string());
    let block = &function.block;

    let mut code = String::new();

    for arg in &mut function.sig.inputs {
        let syn::FnArg::Typed(arg) = arg else {
            parsing_error!(arg, "unsupported function argument kind");
        };

        if let Some(attr) = arg.attrs.first() {
            let glob_var = global_variables::parse_global_var(attr);

            if let Some(location) = glob_var.location {
                code += &format!("layout(location = {location}) ");
            }

            let kind_str = match glob_var.kind {
                global_variables::GlobalVariableKind::Uniform => "uniform",
                global_variables::GlobalVariableKind::Input => "in",
                global_variables::GlobalVariableKind::Output => "out",
            };

            let var_name = ShaderFnParser::new()
                .apply(|parser| parser.visit_pat(&arg.pat))
                .code;

            let type_str = ShaderFnParser::new()
                .apply(|parser| parser.visit_type(&arg.ty))
                .code;

            code += &format!("{kind_str} {type_str} {var_name};\n");
        }

        arg.attrs.clear();
    }

    code += "\n";
    code += &ShaderFnParser::new()
        .apply(|parser| parser.visit_item_fn(&function))
        .code;

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
