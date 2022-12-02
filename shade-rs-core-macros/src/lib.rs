#![feature(iter_intersperse)]
#![feature(proc_macro_diagnostic)]

use std::collections::HashMap;

use proc_macro::{token_stream, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    spanned::Spanned,
    ItemMod, ItemStruct,
};

macro_rules! parsing_error {
    ($tree:expr, $($fmt_args:expr),+) => {
        {
            let msg = format!($($fmt_args),*);
            let span = $tree.span().unwrap();
            span.error(msg).emit();
            unreachable!(); // for unstructuring let to see that this function always panics
        }
    };
}

#[derive(Debug)]
struct VectorImplMetadata {
    elem_type_token: syn::TypePath,
    vec_len_token: syn::LitInt,
}

impl VectorImplMetadata {
    fn len(&self) -> usize {
        self.vec_len_token.base10_parse().unwrap()
    }

    fn type_name_base(&self) -> &'static str {
        let type_name = self
            .elem_type_token
            .path
            .segments
            .last()
            .unwrap()
            .ident
            .to_string();

        match type_name.as_str() {
            "f32" => "Float",
            "i32" => "Int",
            "u32" => "UInt",
            "bool" => "Bool",
            _ => parsing_error!(
                self.elem_type_token,
                "unknown item type for vector: {}",
                type_name
            ),
        }
    }
}

impl Parse for VectorImplMetadata {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let elem_type: syn::TypePath = input.parse()?;
        let _: syn::Token![,] = input.parse()?;
        let size: syn::LitInt = input.parse()?;

        Ok(VectorImplMetadata {
            elem_type_token: elem_type,
            vec_len_token: size,
        })
    }
}

#[proc_macro_attribute]
pub fn impl_rgba(attr: TokenStream, item: TokenStream) -> TokenStream {
    let vector_type = parse_macro_input!(attr as VectorImplMetadata);
    let vector_type_name_base = vector_type.type_name_base();

    let elem_type = &vector_type.elem_type_token;
    let colors_count = vector_type.len();

    let struct_def = syn::parse::<ItemStruct>(item).unwrap();

    let struct_name = &struct_def.ident;

    let colors: Vec<syn::Ident> = ["r", "g", "b", "a"][..colors_count]
        .into_iter()
        .map(|s| format_ident!("{}", s))
        .collect();

    let mut impl_streams = vec![];

    let color_iter = || colors.iter().enumerate();

    color_iter().for_each(|(idx, color)| {
        let stream: syn::ItemFn = parse_quote! {
            pub fn #color(&self) -> #elem_type {
                self[#idx]
            }
        };
        impl_streams.push(stream);
    });

    if colors_count >= 2 {
        itertools::iproduct!(color_iter(), color_iter()).for_each(
            |((idx1, color1), (idx2, color2))| {
                let color_composed = format_ident!("{}{}", color1, color2);
                let view_type = format_ident!("{}{}", vector_type_name_base, "2");

                let stream: syn::ItemFn = parse_quote! {
                    pub fn #color_composed(&self) -> #view_type {
                        [self[#idx1], self[#idx2]].into()
                    }
                };
                impl_streams.push(stream);
            },
        );
    }

    if colors_count >= 3 {
        itertools::iproduct!(color_iter(), color_iter(), color_iter()).for_each(
            |((idx1, color1), (idx2, color2), (idx3, color3))| {
                let color_composed = format_ident!("{}{}{}", color1, color2, color3);
                let view_type = format_ident!("{}{}", vector_type_name_base, "3");

                let stream: syn::ItemFn = parse_quote! {
                    pub fn #color_composed(&self) -> #view_type {
                        [self[#idx1], self[#idx2], self[#idx3]].into()
                    }
                };
                impl_streams.push(stream);
            },
        );
    }

    if colors_count >= 4 {
        itertools::iproduct!(color_iter(), color_iter(), color_iter(), color_iter()).for_each(
            |((idx1, color1), (idx2, color2), (idx3, color3), (idx4, color4))| {
                let color_composed = format_ident!("{}{}{}{}", color1, color2, color3, color4);
                let view_type = format_ident!("{}{}", vector_type_name_base, "4");

                let stream: syn::ItemFn = parse_quote! {
                    pub fn #color_composed(&self) -> #view_type {
                        [self[#idx1], self[#idx2], self[#idx3], self[#idx4]].into()
                    }
                };
                impl_streams.push(stream);
            },
        );
    }

    quote! {
        #struct_def

        impl #struct_name {
            #(#impl_streams)*
        }
    }
    .into()
}
