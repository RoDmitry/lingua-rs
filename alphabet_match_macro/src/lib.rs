use ahash::{AHashMap, AHashSet};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ExprArray, ExprTuple, Lit};

#[proc_macro]
pub fn alphabet_match(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ExprArray);

    // Create a map to collect the result of value -> keys associations
    let mut value_to_keys = AHashMap::new();

    let mut keys_all = Vec::new();
    // Parse the input array of tuples
    for tuple in input.elems {
        if let Expr::Tuple(ExprTuple { elems, .. }) = tuple {
            // We assume the key can be any expression, so we just clone it
            let key = &elems[0];
            keys_all.push(key.clone());

            // Handle the array of values associated with the key
            if let Expr::Array(ExprArray { elems: values, .. }) = &elems[1] {
                #[cfg(debug_assertions)]
                let mut values_set: AHashSet<char> = AHashSet::new();

                for val in values {
                    let value = match val {
                        Expr::Lit(expr_lit) => match &expr_lit.lit {
                            Lit::Char(lit_char) => lit_char.value(),
                            _ => panic!("Expected char literal for value"),
                        },
                        _ => panic!("Expected char literal for value"),
                    };

                    #[cfg(debug_assertions)]
                    if let Expr::Path(ep) = key {
                        let ki = &ep.path.segments[1].ident;
                        assert!(
                            values_set.insert(value),
                            "Char literal: {} was already added for key: {}",
                            value,
                            ki
                        );
                    }

                    value_to_keys
                        .entry(value)
                        .or_insert_with(Vec::new)
                        .push(key.clone()); // Clone the key to store it in the map
                }
            } else {
                panic!("Expected an array of values");
            }
        } else {
            panic!("Expected a tuple");
        }
    }

    // Generate match arms
    let arms = value_to_keys
        .iter()
        .filter(|(_, k)| k.len() < keys_all.len())
        .map(|(value, keys)| {
            quote! {
                #value => &[#(#keys),*],
            }
        });

    // Generate the entire match block
    let expanded = quote! {
        match ch {
            #(#arms)*
            _ => &[#(#keys_all),*],
        }
    };

    // Return the generated code as TokenStream
    TokenStream::from(expanded)
}

/// only to use rust-analyzer expansion
#[test]
fn expand() {
    #[cfg(not(test))]
    alphabet_match!([(SomeEnum::A, ['a', 'b', 'c']), (SomeEnum::B, ['a', 'c']),]);
}
