extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, FieldsNamed, Fields};

#[proc_macro_derive(FlattenF32)]
pub fn derive_flatten_f32(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident.clone();

    let fields = match get_struct_fields(&input) {
        Some(fields) => fields,
        None => return compile_error("Flatten can only be derived for structs with named fields."),
    };

    let flatten_calls = fields.named.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            self.#field_name.flatten()
        }
    });

    // Generate the final implementation
    let expanded = quote! {
        impl Flatten<f32> for #struct_name
        {
            fn flatten(&self) -> Vec<f32> {
                vec![#(#flatten_calls),*].concat()
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(FlattenF64)]
pub fn derive_flatten_f64(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident.clone();

    let fields = match get_struct_fields(&input) {
        Some(fields) => fields,
        None => return compile_error("Flatten can only be derived for structs with named fields."),
    };

    let flatten_calls = fields.named.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            self.#field_name.flatten()
        }
    });

    // Generate the final implementation
    let expanded = quote! {
        impl Flatten<f64> for #struct_name
        {
            fn flatten(&self) -> Vec<f64> {
                vec![#(#flatten_calls),*].concat()
            }
        }
    };

    TokenStream::from(expanded)
}


fn get_struct_fields(input: &DeriveInput) -> Option<&FieldsNamed> {
    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields) = &data_struct.fields {
            return Some(fields);
        }
    }
    None
}

fn compile_error(message: &str) -> TokenStream {
    TokenStream::from(quote! {
        compile_error!(#message);
    })
}

