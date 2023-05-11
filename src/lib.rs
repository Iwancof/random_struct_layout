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
        let debug = if input.peek(syn::Ident) { // TODO: parse multiple args
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

    let punctuated = if let syn::Fields::Named(ref mut name) = input.fields {
        // punctuated has field name.
        &mut name.named
    } else {
        panic!("layout_randomize only supports named struct");
    };

    let mut vector: Vec<syn::Field> = Vec::new();
    while let Some(pair) = punctuated.pop() {
        vector.push(pair.into_tuple().0);
    }


    let mut fields = quote! {};
    for v in vector.iter() {
        let id = v.ident.clone().unwrap(); // since punctuated has fields name, this unwrap will
                                           // never panic.
        fields = quote! {
            .field(stringify!(#id), &self.#id)
            #fields
        };
    }

    use rand::{rngs::OsRng, seq::SliceRandom};
    vector.shuffle(&mut OsRng);

    for v in vector {
        punctuated.push(v);
    }

    let debug_impl = if let Some(debug_ident) = args.debug {
        if debug_ident == "Debug" {
            Some(quote! {
                impl std::fmt::Debug for #struct_name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.debug_struct(stringify!(#struct_name))
                            #fields
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

    let output = quote! {
        #[repr(C)] // TODO: add to input.attrs
        #input
        #debug_impl
    };

    output.into()
}
