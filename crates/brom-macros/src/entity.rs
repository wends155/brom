use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

use crate::schema::{BromFieldAttrs, BromStructAttrs, map_type_to_field_type};

pub fn expand_brom_entity(input: &DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;
    let struct_attrs = BromStructAttrs::parse(input);

    let table_name = struct_attrs
        .table_name
        .unwrap_or_else(|| struct_name.to_string().to_lowercase());

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields,
            _ => {
                return Err(syn::Error::new_spanned(
                    &input.ident,
                    "BromEntity can only be derived on structs with named fields",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                &input.ident,
                "BromEntity can only be derived on structs with named fields",
            ));
        }
    };

    let field_infos = fields
        .named
        .iter()
        .map(|f| {
            let name = f
                .ident
                .as_ref()
                .ok_or_else(|| syn::Error::new_spanned(f, "Field must have an identifier"))?
                .to_string();
            let attrs = BromFieldAttrs::parse(f);
            let field_type = map_type_to_field_type(&f.ty, &attrs);

            let mut constraints = Vec::new();
            if attrs.unique {
                constraints.push(quote!(Constraint::Unique));
            }
            if attrs.not_null {
                constraints.push(quote!(Constraint::NotNull));
            }
            if let Some(default) = &attrs.default {
                constraints.push(quote!(Constraint::Default(#default.to_string())));
            }

            let hidden = attrs.hidden;
            let ui_widget = if let Some(w) = &attrs.ui_widget {
                quote!(Some(#w.to_string()))
            } else {
                quote!(None)
            };

            Ok(quote! {
                FieldInfo {
                    name: #name.to_string(),
                    field_type: #field_type,
                    constraints: vec![#(#constraints),*],
                    ui_widget: #ui_widget,
                    hidden: #hidden,
                }
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;

    let expanded = quote! {
        #[automatically_derived]
        impl ::brom_core::EntitySchema for #struct_name {
            fn table_name() -> &'static str {
                #table_name
            }

            fn fields() -> Vec<::brom_core::FieldInfo> {
                use ::brom_core::{FieldInfo, FieldType, Constraint};
                vec![
                    #(#field_infos),*
                ]
            }

            fn schema_info() -> ::brom_core::SchemaInfo {
                ::brom_core::SchemaInfo {
                    table_name: Self::table_name().to_string(),
                    fields: Self::fields(),
                    auth_policy: ::brom_core::AuthPolicy::Public, // Default for now
                }
            }
        }
    };

    Ok(expanded)
}
