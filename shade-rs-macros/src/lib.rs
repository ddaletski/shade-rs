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
            use std::ops::{Add, AddAssign};
            use shade_rs::fragment_shader;

            type Scalar<T, const N: usize> = [T; N];

            #[derive(Debug, Clone, Copy)]
            pub struct Float4(Scalar<f32, 4>);
            impl From<Scalar<f32, 4>> for Float4 {
                fn from(val: Scalar<f32, 4>) -> Self {
                    Self(val)
                }
            }

            pub type Float = f32;
            pub type Int = i32;

            // TODO: better cast alternative
            // TODO: generate this kind of functions
            fn int_to_float(i: Int) -> Float {
                i as Float
            }

            pub fn float4(x: f32, y: f32, z: f32, w: f32) -> Float4 {
                [x, y, z, w].into()
            }

            pub fn float(x: f32) -> Float {
                x
            }


            pub fn sin(val: Float) -> Float {
                val
            }

            impl AddAssign for Float4 {
                fn add_assign(&mut self, rhs: Self) {
                    self.0[0] += rhs.0[0];
                    self.0[1] += rhs.0[1];
                    self.0[2] += rhs.0[2];
                    self.0[3] += rhs.0[3];
                }
            }

            impl Add for Float4 {
                type Output = Self;

                fn add(self, rhs: Self) -> Self::Output {
                    let mut result = self;
                    result += rhs;
                    result
                }
            }

            #(#mod_items)*
        }
    };

    TokenStream::from(expanded)
}
