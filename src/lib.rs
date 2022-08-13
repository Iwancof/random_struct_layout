#[deny(unsafe_code)]
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input};

#[proc_macro_attribute]
pub fn layout_randomize(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as syn::ItemStruct);

    let punctuated = if let syn::Fields::Named(ref mut name) = input.fields {
        &mut name.named
    } else if let syn::Fields::Unnamed(ref mut unnamed) = input.fields {
        &mut unnamed.unnamed
    } else {
        return (quote! { #input }).into();
    };

    let mut vector: Vec<syn::Field> = Vec::new();
    while let Some(pair) = punctuated.pop() {
        vector.push(pair.into_tuple().0);
    }

    use rand::{rngs::OsRng, seq::SliceRandom};
    vector.shuffle(&mut OsRng);

    for v in vector {
        punctuated.push(v);
    }

    return (quote! {
        #[repr(C)] // TODO: add to input.attrs
        #input
    })
    .into();
}
