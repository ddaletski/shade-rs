#![feature(iter_intersperse)]
#![feature(proc_macro_diagnostic)]

use std::collections::HashMap;

use itertools::Itertools;
use proc_macro::{token_stream, Ident, TokenStream};
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

fn cartesian_power<T: Clone>(vec: Vec<T>, power: usize) -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = vec![vec![]];

    for _ in 0..power {
        result = itertools::iproduct!(result, vec.clone())
            .map(|(mut res, next_vec)| {
                res.push(next_vec);
                res
            })
            .collect();
    }

    result
}

fn impl_color_permutations_inner(struct_item: &syn::ItemStruct, colors: &str) -> TokenStream {
    let mut impl_streams = vec![];

    let item_type_name = {
        let mut struct_name = struct_item.ident.to_string();
        struct_name.pop(); // pop vector length to get vec. element type name
        struct_name
    };

    let item_type = format_ident!("{}", item_type_name);

    let colors: Vec<syn::Ident> = colors
        .chars()
        .into_iter()
        .map(|s| format_ident!("{}", s))
        .collect();

    colors.iter().enumerate().for_each(|(idx, color)| {
        let set_color = format_ident!("set_{}", color);

        let stream = quote! {
            pub fn #color(&self) -> #item_type {
                self[#idx]
            }

            pub fn #set_color(&mut self, value: #item_type) {
                self[#idx] = value;
            }
        };
        impl_streams.push(stream);
    });

    let combinations: Vec<Vec<(usize, &syn::Ident)>> = (2..=colors.len())
        .flat_map(|combination_len| {
            cartesian_power(colors.iter().enumerate().collect(), combination_len)
        })
        .collect();

    combinations.into_iter().for_each(|combination| {
        let colors_permutation = combination.iter().map(|(_, color)| color);
        let indices_permutation: Vec<usize> = combination.iter().map(|(index, _)| *index).collect();
        let indices_ordered = 0..indices_permutation.len();

        let slice_name: String = colors_permutation.map(|ident| ident.to_string()).collect();
        let slice_type = format_ident!("{}{}", item_type_name, format!("{}", combination.len()));

        let slice_token = format_ident!("{}", slice_name);
        let set_slice_token = format_ident!("set_{}", slice_token);

        let stream = quote! {
            pub fn #slice_token(&self) -> #slice_type {
                [#(self[#indices_permutation]),*].into()
            }

            pub fn #set_slice_token(&mut self, value: #slice_type) {
                #(self[#indices_permutation] = value[#indices_ordered];)*
            }
        };
        impl_streams.push(stream);
    });

    let struct_name = &struct_item.ident;
    quote! {
        #struct_item

        impl #struct_name {
            #(#impl_streams)*
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn impl_color_permutations(attr: TokenStream, item: TokenStream) -> TokenStream {
    let colors = parse_macro_input!(attr as syn::Ident).to_string();
    let struct_def = syn::parse::<ItemStruct>(item.clone()).unwrap();

    impl_color_permutations_inner(&struct_def, &colors)
}
