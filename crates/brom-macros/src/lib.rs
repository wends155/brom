//! Procedural macros for the `brom` framework.

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

/// Code generation for entity schemas and SQLite metadata.
mod entity;
/// Synthesis of OpenAPI security and schema definitions.
mod openapi;
/// Generation of Axum route handlers and routing tables.
mod routes;
/// Logic for generating JSON schema definitions.
mod schema;

/// The primary macro used to transform a Rust struct into a `brom` entity.
///
/// It generates `EntitySchema` implementations, Axum routers, and SQLite migration SQL.
#[proc_macro_derive(BromEntity, attributes(brom))]
pub fn derive_brom_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    entity::expand_brom_entity(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
