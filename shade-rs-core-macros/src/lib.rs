#![feature(iter_intersperse)]
#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_quote;

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

struct VectorValues {
    zeros: Vec<syn::Expr>,
    non_zeros: Vec<syn::Expr>,
}

fn values_for(element_name: &str, elements_count: usize) -> VectorValues {
    let zeros;
    let non_zeros;

    match element_name {
        "Float" => {
            zeros = (0..elements_count).map(|_| parse_quote!(0.0f32)).collect();
            non_zeros = (0..elements_count)
                .map(|idx| parse_quote!((#idx as f32) + 1.0f32))
                .collect();
        }

        "Int" => {
            zeros = (0..elements_count).map(|_| parse_quote!(0i32)).collect();
            non_zeros = (0..elements_count)
                .map(|idx| parse_quote!((#idx as i32) + 1))
                .collect();
        }

        "UInt" => {
            zeros = (0..elements_count).map(|_| parse_quote!(0.0f32)).collect();
            non_zeros = (0..elements_count)
                .map(|idx| parse_quote!((#idx as u32) + 1))
                .collect();
        }

        "Bool" => {
            zeros = (0..elements_count).map(|_| parse_quote!(0.0f32)).collect();
            non_zeros = (0..elements_count).map(|_| parse_quote!(true)).collect();
        }

        _ => unreachable!(),
    }

    VectorValues { zeros, non_zeros }
}

fn impl_color_permutations_inner(struct_item: &syn::ItemStruct, colors: &str) -> TokenStream {
    let mut impl_streams = vec![];
    let mut test_streams = vec![];

    let struct_name = struct_item.ident.to_string();
    let element_type_name = {
        let mut name = struct_name.clone();
        name.pop(); // pop vector length to get vec. element type name
        name
    };
    let elements_count = colors.len();

    let struct_type = format_ident!("{}", struct_name);
    let element_type = format_ident!("{}", element_type_name);

    let colors: Vec<syn::Ident> = colors
        .chars()
        .into_iter()
        .map(|s| format_ident!("{}", s))
        .collect();

    let vector_values = values_for(&element_type_name, elements_count);

    let default_vector_value = {
        let zeros = &vector_values.zeros;

        quote! {
            [#(#zeros),*].into();
        }
    };

    colors.iter().enumerate().for_each(|(idx, color)| {
        let set_color = format_ident!("set_{}", color);

        let stream = quote! {
            pub fn #color(&self) -> #element_type {
                self[#idx]
            }

            pub fn #set_color(&mut self, value: #element_type) {
                self[#idx] = value;
            }
        };
        impl_streams.push(stream);

        let test_color = format_ident!("test_{}", color);
        let new_color_value = &vector_values.non_zeros[0];

        let stream = quote! {
            #[test]
            fn #test_color() {
                let mut vector: #struct_type = #default_vector_value;

                vector.#set_color(#new_color_value);
                assert_eq!(vector.#color(), #new_color_value);
            }
        };
        test_streams.push(stream);
    });

    let combinations: Vec<Vec<(usize, &syn::Ident)>> = (2..=elements_count)
        .flat_map(|combination_len| {
            cartesian_power(colors.iter().enumerate().collect(), combination_len)
        })
        .collect();

    combinations.into_iter().for_each(|combination| {
        let colors_permutation = combination.iter().map(|(_, color)| color);
        let indices_permutation: Vec<usize> = combination.iter().map(|(index, _)| *index).collect();
        let indices_ordered = 0..indices_permutation.len();

        let slice_name: String = colors_permutation.map(|ident| ident.to_string()).collect();
        let slice_type = format_ident!("{}{}", element_type_name, format!("{}", combination.len()));

        let slice = format_ident!("{}", slice_name);
        let set_slice = format_ident!("set_{}", slice);

        let stream = quote! {
            pub fn #slice(&self) -> #slice_type {
                [#(self[#indices_permutation]),*].into()
            }

            pub fn #set_slice(&mut self, value: #slice_type) {
                #(self[#indices_permutation] = value[#indices_ordered];)*
            }
        };
        impl_streams.push(stream);

        let test_slice = format_ident!("test_{}", slice);
        let slice_values: Vec<syn::Expr> = indices_permutation
            .iter()
            .map(|&idx| vector_values.non_zeros[idx].clone())
            .collect();
        let slice_value_expr = quote! { [#(#slice_values),*].into() };

        let mut vector_values_after = vector_values.zeros.clone();
        for idx in indices_permutation {
            vector_values_after[idx] = vector_values.non_zeros[idx].clone();
        }
        let vector_values_after_expr = quote! { [#(#vector_values_after),*].into() };

        let stream = quote! {
            #[test]
            fn #test_slice() {
                let mut vector: #struct_type = #default_vector_value;

                vector.#set_slice(#slice_value_expr);
                assert_eq!(vector.#slice(), #slice_value_expr);
                assert_eq!(vector, #vector_values_after_expr);
            }
        };
        test_streams.push(stream);
    });

    let struct_name = &struct_item.ident;
    quote! {
        #struct_item

        impl #struct_name {
            #(#impl_streams)*
        }

        #[cfg(test)]
        mod test_color_permutations {
            use super::*;
            #(#test_streams)*
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn impl_color_permutations(attr: TokenStream, item: TokenStream) -> TokenStream {
    let colors = syn::parse_macro_input!(attr as syn::Ident).to_string();
    let struct_def = syn::parse::<syn::ItemStruct>(item.clone()).unwrap();

    impl_color_permutations_inner(&struct_def, &colors)
}
