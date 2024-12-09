use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, DeriveInput, Ident, LitInt};

struct RegisterAttr {
    value: LitInt,
    ty: Ident,
}

impl Parse for RegisterAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse the entire input as a single LitInt first
        let lit = input.parse::<LitInt>()?;

        // Extract the type suffix from the literal
        let suffix = lit.suffix();
        if suffix.is_empty() {
            return Err(syn::Error::new(
                lit.span(),
                "Expected type suffix (e.g., u8, u16)",
            ));
        }

        // Create an Ident from the suffix
        let ty = Ident::new(suffix, lit.span());

        Ok(RegisterAttr { value: lit, ty })
    }
}

#[proc_macro_attribute]
pub fn register(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as RegisterAttr);
    let input = parse_macro_input!(item as DeriveInput);

    let name = &input.ident;
    let value = &attr.value;
    let ty = &attr.ty;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        #input

        impl #impl_generics regiface::Register for #name #ty_generics #where_clause {
            type IdType = #ty;

            fn id() -> Self::IdType {
                #value
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(ReadableRegister)]
pub fn derive_readable_register(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics regiface::ReadableRegister for #name #ty_generics #where_clause {}
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(WritableRegister)]
pub fn derive_writable_register(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics regiface::WritableRegister for #name #ty_generics #where_clause {}
    };

    TokenStream::from(expanded)
}
