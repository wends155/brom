use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

use crate::schema::{BromFieldAttrs, BromStructAttrs, map_type_to_field_type};

#[allow(clippy::too_many_lines)]
pub fn expand_brom_entity(input: &DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;
    let mut errors: Option<syn::Error> = None;

    let struct_attrs = match BromStructAttrs::parse(input) {
        Ok(attrs) => Some(attrs),
        Err(e) => {
            errors = Some(e);
            None
        }
    };

    let table_name = struct_attrs
        .as_ref()
        .and_then(|a| a.table_name.clone())
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

    let mut field_infos = Vec::new();
    let mut public_fields = Vec::new();
    let mut admin_fields = Vec::new();
    let mut public_field_idents = Vec::new();
    let mut admin_field_idents = Vec::new();
    for f in &fields.named {
        let Some((info, attrs)) = expand_field(f, &mut errors) else {
            continue;
        };

        field_infos.push(info);

        if !attrs.hidden {
            let mut pub_f = f.clone();
            // Strip the #[brom(...)] helper attributes from the public struct
            pub_f.attrs.retain(|attr| !attr.path().is_ident("brom"));

            public_fields.push(pub_f);
            #[allow(clippy::expect_used)] // Infallible: BromEntity only derives on named structs
            public_field_idents.push(f.ident.clone().expect("Struct fields must be named"));
        }

        let mut admin_f = f.clone();
        admin_f.attrs.retain(|attr| !attr.path().is_ident("brom"));
        admin_fields.push(admin_f);
        #[allow(clippy::expect_used)]
        admin_field_idents.push(f.ident.clone().expect("Struct fields must be named"));
    }

    if let Some(errs) = errors {
        return Err(errs);
    }

    let public_struct_name = syn::Ident::new(&format!("{struct_name}Public"), struct_name.span());
    let admin_struct_name = syn::Ident::new(&format!("{struct_name}Admin"), struct_name.span());

    let policy_str = struct_attrs.as_ref().and_then(|a| a.auth_policy.as_deref());
    let policy_token = match policy_str {
        Some("AdminOnly") => quote!(::brom::__private::brom_core::AuthPolicy::AdminOnly),
        Some("ApiKey") => quote!(::brom::__private::brom_core::AuthPolicy::ApiKey),
        _ => quote!(::brom::__private::brom_core::AuthPolicy::Public), // Default
    };

    let routes = crate::routes::expand_routes(struct_name, policy_str);
    let openapi = crate::openapi::expand_openapi(struct_name);

    let auto_mod_name = syn::Ident::new(
        &format!("__brom_hygiene_{}", struct_name.to_string().to_lowercase()),
        struct_name.span(),
    );

    let expanded = quote! {
        #[automatically_derived]
        impl ::brom::__private::brom_core::EntitySchema for #struct_name {
            fn table_name() -> &'static str {
                #table_name
            }

            fn fields() -> Vec<::brom::__private::brom_core::FieldInfo> {
                vec![
                    #(#field_infos),*
                ]
            }

            fn schema_info() -> ::brom::__private::brom_core::SchemaInfo {
                ::brom::__private::brom_core::SchemaInfo {
                    table_name: Self::table_name().to_string(),
                    fields: Self::fields(),
                    auth_policy: #policy_token,
                }
            }
        }

        mod #auto_mod_name {
            use super::*;
            use ::brom::__private::utoipa as utoipa;
            use ::brom::__private::serde as serde;

            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
            pub struct #public_struct_name {
                #(#public_fields),*
            }

            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
            pub struct #admin_struct_name {
                #(#admin_fields),*
            }
        }
        pub use #auto_mod_name::{#public_struct_name, #admin_struct_name};

        impl From<#struct_name> for #public_struct_name {
            fn from(item: #struct_name) -> Self {
                Self {
                    #(#public_field_idents: item.#public_field_idents),*
                }
            }
        }

        impl From<#struct_name> for #admin_struct_name {
            fn from(item: #struct_name) -> Self {
                Self {
                    #(#admin_field_idents: item.#admin_field_idents),*
                }
            }
        }

        impl From<#admin_struct_name> for #struct_name {
            fn from(item: #admin_struct_name) -> Self {
                Self {
                    #(#admin_field_idents: item.#admin_field_idents),*
                }
            }
        }

        #routes
        #openapi
    };

    Ok(expanded)
}

fn expand_field(
    f: &syn::Field,
    errors: &mut Option<syn::Error>,
) -> Option<(TokenStream, BromFieldAttrs)> {
    let Some(ident) = f.ident.as_ref() else {
        let e = syn::Error::new_spanned(f, "Field must have an identifier");
        if let Some(errs) = errors {
            errs.combine(e);
        } else {
            *errors = Some(e);
        }
        return None;
    };
    let name = ident.to_string();

    #[allow(clippy::single_match_else)]
    let attrs = match BromFieldAttrs::parse(f) {
        Ok(a) => a,
        Err(e) => {
            if let Some(errs) = errors {
                errs.combine(e);
            } else {
                *errors = Some(e);
            }
            return None;
        }
    };

    let field_type = map_type_to_field_type(&f.ty, &attrs);

    let mut constraints = Vec::new();
    if attrs.unique {
        constraints.push(quote!(::brom::__private::brom_core::Constraint::Unique));
    }
    if attrs.not_null {
        constraints.push(quote!(::brom::__private::brom_core::Constraint::NotNull));
    }
    if let Some(default) = &attrs.default {
        constraints
            .push(quote!(::brom::__private::brom_core::Constraint::Default(#default.to_string())));
    }

    let hidden = attrs.hidden;
    let ui_widget = if let Some(w) = &attrs.ui_widget {
        quote!(Some(#w.to_string()))
    } else {
        quote!(None)
    };

    Some((
        quote! {
            ::brom::__private::brom_core::FieldInfo {
                name: #name.to_string(),
                field_type: #field_type,
                constraints: vec![#(#constraints),*],
                ui_widget: #ui_widget,
                hidden: #hidden,
            }
        },
        attrs,
    ))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use insta::assert_snapshot;
    use syn::parse_quote;

    fn expand_and_format(input: &syn::DeriveInput) -> String {
        let tokens = expand_brom_entity(input).unwrap();
        let file: syn::File = match syn::parse2(tokens.clone()) {
            Ok(f) => f,
            Err(e) => panic!("Parse failed: {e}\nTokens:\n{tokens}"),
        };
        prettyplease::unparse(&file)
    }

    #[test]
    fn snapshot_basic_struct() {
        let input: syn::DeriveInput = parse_quote! {
            pub struct Post {
                pub title: String,
                pub body: String,
                pub published: bool,
            }
        };
        assert_snapshot!(expand_and_format(&input));
    }

    #[test]
    fn snapshot_custom_table_name() {
        let input: syn::DeriveInput = parse_quote! {
            #[brom(table = "blog_posts")]
            pub struct Post {
                pub title: String,
            }
        };
        assert_snapshot!(expand_and_format(&input));
    }

    #[test]
    fn snapshot_field_constraints() {
        let input: syn::DeriveInput = parse_quote! {
            pub struct User {
                #[brom(unique)]
                pub email: String,
                #[brom(not_null, default = "Anonymous")]
                pub display_name: String,
                #[brom(hidden)]
                pub password_hash: String,
            }
        };
        assert_snapshot!(expand_and_format(&input));
    }

    #[test]
    fn snapshot_link_relationship() {
        let input: syn::DeriveInput = parse_quote! {
            pub struct Comment {
                pub body: String,
                #[brom(link = "post")]
                pub post_id: i64,
            }
        };
        assert_snapshot!(expand_and_format(&input));
    }

    #[test]
    fn snapshot_many_to_many_relationship() {
        let input: syn::DeriveInput = parse_quote! {
            pub struct Article {
                pub title: String,
                #[brom(many_many = "tag", junction = "article_tag")]
                pub tags: String,
            }
        };
        assert_snapshot!(expand_and_format(&input));
    }
}
