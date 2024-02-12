use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ReadableRegister)]
pub fn derive_readable_register(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics regiface::ReadableRegister for #name #ty_generics #where_clause {}
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(WritableRegister)]
pub fn derive_writable_register(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics regiface::WritableRegister for #name #ty_generics #where_clause {}
    };

    proc_macro::TokenStream::from(expanded)
}
