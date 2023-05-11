use std::vec;

#[deny(unsafe_code)]
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse::Parse, parse::ParseStream, parse_macro_input, Ident, Result};

#[derive(Debug)]
struct LayoutRandomizeArgs {
    debug: Option<Ident>,
}

impl Parse for LayoutRandomizeArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let debug = if input.peek(syn::Ident) {
            // TODO: parse multiple args
            Some(input.parse()?)
        } else {
            None
        };
        Ok(LayoutRandomizeArgs { debug })
    }
}

#[proc_macro_attribute]
pub fn layout_randomize(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as LayoutRandomizeArgs);
    let mut input = parse_macro_input!(input as syn::ItemStruct);

    let struct_name = &input.ident;

    let punctuated = if let syn::Fields::Named(ref mut field) = input.fields {
        // punctuated has field name.
        &mut field.named
    } else {
        panic!("layout_randomize only supports named struct");
    };

    let mut members: Vec<syn::Field> = Vec::new();
    while let Some(pair) = punctuated.pop() {
        members.push(pair.into_tuple().0);
    }

    let debug_impl = if let Some(debug_ident) = args.debug {
        let mut debug_impl_field_method_chain = quote! {};
        for v in members.iter() {
            let id = v.ident.clone().unwrap(); // since punctuated has fields name, this unwrap will
                                               // never panic.
            debug_impl_field_method_chain = quote! {
                .field(stringify!(#id), &self.#id)
                #debug_impl_field_method_chain
            };
        }

        if debug_ident == "Debug" {
            Some(quote! {
                impl std::fmt::Debug for #struct_name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.debug_struct(stringify!(#struct_name))
                            #debug_impl_field_method_chain
                            .finish()
                    }
                }
            })
        } else {
            None
        }
    } else {
        None
    };

    let dst_indeies: Vec<usize> = members
        .iter_mut()
        .enumerate()
        .filter_map(|(idx, v)| {
            let dst_attribute_idx = v.attrs.iter().position(|attr| attr.path.is_ident("dst"))?;
            v.attrs.remove(dst_attribute_idx); // remove #[dst]

            Some(idx)
        })
        .collect();

    assert!(dst_indeies.len() <= 1, "dst attribute too many exists.");

    let member_num = members.len();
    let shuffle_last_idx = if let Some(dst_idx) = dst_indeies.get(0) {
        members.swap(*dst_idx, member_num - 1);
        // dst. move to last
    
        *dst_idx + 1
    } else {
        0
    };

    use rand::{rngs::OsRng, seq::SliceRandom};
    // members[..shuffle_last_idx].shuffle(&mut OsRng);
    members[..(member_num - shuffle_last_idx)].shuffle(&mut OsRng);

    for v in members {
        punctuated.push(v);
    }

    let output = quote! {
        #[repr(C)] // TODO: add to input.attrs
        #input
        #debug_impl
    };

    output.into()
}
