#![feature(iter_intersperse)]
#![feature(proc_macro_diagnostic)]

use itertools::Itertools;
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
    // [0; N] for int, [0.0; N], for float [false; N] for bool etc.
    zeros: Vec<syn::Expr>,
    // [1, 2, ...] for int, [1.0, 2.0, ...] for float, [true; N] for bool etc.
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
            zeros = (0..elements_count).map(|_| parse_quote!(0u32)).collect();
            non_zeros = (0..elements_count)
                .map(|idx| parse_quote!((#idx as u32) + 1))
                .collect();
        }

        "Bool" => {
            zeros = (0..elements_count).map(|_| parse_quote!(false)).collect();
            non_zeros = (0..elements_count).map(|_| parse_quote!(true)).collect();
        }

        _ => unreachable!(),
    }

    VectorValues { zeros, non_zeros }
}

type Streams = Vec<proc_macro2::TokenStream>;

/// generate getters and setters for single colors (e.g. vector.r(), vector.set_r(...))
fn generate_single_color(
    struct_name: &syn::Ident,
    colors: &[syn::Ident],
    element_type: &syn::Ident,
    elements_count: usize,
) -> (Streams, Streams) {
    let mut impl_streams = vec![];
    let mut test_streams = vec![];

    let vector_values = values_for(&element_type.to_string(), elements_count);
    let default_vector_value_expr = {
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
                let mut vector: #struct_name = #default_vector_value_expr;

                vector.#set_color(#new_color_value);
                assert_eq!(vector.#color(), #new_color_value);
            }
        };
        test_streams.push(stream);
    });

    (impl_streams, test_streams)
}

/// generate getters and setters for color permutations (e.g. vector.rg(), vector.set_rg(...))
fn generate_color_combinations(
    struct_name: &syn::Ident,
    colors: &[syn::Ident],
    element_type: &syn::Ident,
    elements_count: usize,
) -> (Streams, Streams) {
    let mut impl_streams = vec![];
    let mut test_streams = vec![];

    let vector_values = values_for(&element_type.to_string(), elements_count);
    let default_vector_value_expr = {
        let zeros = &vector_values.zeros;

        quote! {
            [#(#zeros),*].into();
        }
    };

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
        let slice_type = format_ident!("{}{}", element_type, format!("{}", combination.len()));

        let slice = format_ident!("{}", slice_name);
        let set_slice = format_ident!("set_{}", slice);
        let test_slice = format_ident!("test_{}", slice);

        impl_streams.push(quote! {
            pub fn #slice(&self) -> #slice_type {
                [#(self[#indices_permutation]),*].into()
            }
        });

        let setter_is_needed = slice_name.chars().all_unique();

        if setter_is_needed {
            impl_streams.push(quote! {
                pub fn #set_slice(&mut self, value: #slice_type) {
                    #(self[#indices_permutation] = value[#indices_ordered];)*
                }
            });
        }

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

        let test_stream = if setter_is_needed {
            quote! {
                #[test]
                fn #test_slice() {
                    let mut vector: #struct_name = #default_vector_value_expr;

                    vector.#set_slice(#slice_value_expr);
                    assert_eq!(vector.#slice(), #slice_value_expr);
                    assert_eq!(vector, #vector_values_after_expr);
                }
            }
        } else {
            quote! {
                #[test]
                fn #test_slice() {
                    let mut vector: #struct_name = #vector_values_after_expr;
                    assert_eq!(vector.#slice(), #slice_value_expr);
                }
            }
        };
        test_streams.push(test_stream);
    });

    (impl_streams, test_streams)
}


#[proc_macro_attribute]
pub fn impl_color_permutations(attr: TokenStream, item: TokenStream) -> TokenStream {
    let colors_string = syn::parse_macro_input!(attr as syn::Ident).to_string();
    let struct_item = syn::parse::<syn::ItemStruct>(item.clone()).unwrap();

    let struct_type = &struct_item.ident;

    let element_type_name = {
        let mut name = struct_type.to_string();
        name.pop(); // pop vector length to get vec. element type name
        name
    };
    let element_type = format_ident!("{}", element_type_name);

    let elements_count = colors_string.len();

    let colors: Vec<syn::Ident> = colors_string
        .chars()
        .into_iter()
        .map(|s| format_ident!("{}", s))
        .collect();

    let mut impl_streams = vec![];
    let mut test_streams = vec![];

    let (mut s1, mut s2) =
        generate_single_color(&struct_type, &colors, &element_type, elements_count);
    impl_streams.append(&mut s1);
    test_streams.append(&mut s2);

    let (mut s1, mut s2) =
        generate_color_combinations(&struct_type, &colors, &element_type, elements_count);
    impl_streams.append(&mut s1);
    test_streams.append(&mut s2);

    let struct_name = &struct_item.ident;
    let test_mod_name = format_ident!(
        "test_{}_{}",
        struct_type.to_string().to_lowercase(),
        colors_string
    );

    quote! {
        #struct_item

        impl #struct_name {
            #(#impl_streams)*
        }

        #[cfg(test)]
        mod #test_mod_name {
            use super::*;
            #(#test_streams)*
        }
    }
    .into()
}
