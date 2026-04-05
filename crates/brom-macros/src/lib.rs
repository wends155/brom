use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod entity;
mod routes;
mod openapi;
mod schema;

#[proc_macro_derive(BromEntity, attributes(brom))]
pub fn derive_brom_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    entity::expand_brom_entity(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
