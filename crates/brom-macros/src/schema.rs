use quote::ToTokens;
use syn::{DeriveInput, Field, Type};

pub struct BromStructAttrs {
    pub table_name: Option<String>,
}

pub struct BromFieldAttrs {
    pub unique: bool,
    pub not_null: bool,
    pub default: Option<String>,
    pub hidden: bool,
    pub ui_widget: Option<String>,
    pub link_target: Option<String>,
    pub many_many_target: Option<String>,
    pub many_many_junction: Option<String>,
}

impl BromStructAttrs {
    pub fn parse(input: &DeriveInput) -> syn::Result<Self> {
        let mut table_name = None;
        let mut errors: Option<syn::Error> = None;

        for attr in &input.attrs {
            if !attr.path().is_ident("brom") {
                continue;
            }

            if let Err(e) = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("table") {
                    let value = meta.value()?;
                    let s: syn::LitStr = value.parse()?;
                    table_name = Some(s.value());
                    Ok(())
                } else {
                    Err(meta.error("unrecognized brom attribute"))
                }
            }) {
                if let Some(errs) = &mut errors {
                    errs.combine(e);
                } else {
                    errors = Some(e);
                }
            }
        }

        if let Some(errs) = errors {
            return Err(errs);
        }

        Ok(Self { table_name })
    }
}

impl BromFieldAttrs {
    pub fn parse(field: &Field) -> syn::Result<Self> {
        let mut unique = false;
        let mut not_null = false;
        let mut default = None;
        let mut hidden = false;
        let mut ui_widget = None;
        let mut link_target = None;
        let mut many_many_target = None;
        let mut many_many_junction = None;
        let mut errors: Option<syn::Error> = None;

        for attr in &field.attrs {
            if !attr.path().is_ident("brom") {
                continue;
            }

            if let Err(e) = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("unique") {
                    unique = true;
                    Ok(())
                } else if meta.path.is_ident("not_null") {
                    not_null = true;
                    Ok(())
                } else if meta.path.is_ident("hidden") {
                    hidden = true;
                    Ok(())
                } else if meta.path.is_ident("default") {
                    let value = meta.value()?;
                    let s: syn::LitStr = value.parse()?;
                    default = Some(s.value());
                    Ok(())
                } else if meta.path.is_ident("widget") {
                    let value = meta.value()?;
                    let s: syn::LitStr = value.parse()?;
                    ui_widget = Some(s.value());
                    Ok(())
                } else if meta.path.is_ident("link") {
                    let value = meta.value()?;
                    let s: syn::LitStr = value.parse()?;
                    link_target = Some(s.value());
                    Ok(())
                } else if meta.path.is_ident("many_many") {
                    let value = meta.value()?;
                    let s: syn::LitStr = value.parse()?;
                    many_many_target = Some(s.value());
                    Ok(())
                } else if meta.path.is_ident("junction") {
                    let value = meta.value()?;
                    let s: syn::LitStr = value.parse()?;
                    many_many_junction = Some(s.value());
                    Ok(())
                } else {
                    Err(meta.error("unrecognized brom attribute"))
                }
            }) {
                if let Some(ref mut errs) = errors {
                    errs.combine(e);
                } else {
                    errors = Some(e);
                }
            }
        }

        if let Some(errs) = errors {
            return Err(errs);
        }

        Ok(Self {
            unique,
            not_null,
            default,
            hidden,
            ui_widget,
            link_target,
            many_many_target,
            many_many_junction,
        })
    }
}

pub fn map_type_to_field_type(ty: &Type, attrs: &BromFieldAttrs) -> proc_macro2::TokenStream {
    if let Some(target) = &attrs.many_many_target {
        let junction = attrs.many_many_junction.as_deref().unwrap_or("");
        return quote::quote!(FieldType::ManyToMany { target: #target.to_string(), junction_table: #junction.to_string() });
    }

    if let Some(target) = &attrs.link_target {
        return quote::quote!(FieldType::Link { target: #target.to_string() });
    }

    let ty_str = ty.to_token_stream().to_string().replace(' ', "");

    if ty_str.contains("String") {
        quote::quote!(FieldType::String)
    } else if ty_str == "i64" || ty_str == "u64" || ty_str == "i32" || ty_str == "u32" {
        quote::quote!(FieldType::Integer)
    } else if ty_str == "f64" || ty_str == "f32" {
        quote::quote!(FieldType::Float)
    } else if ty_str == "bool" {
        quote::quote!(FieldType::Boolean)
    } else if ty_str.contains("DateTime") {
        quote::quote!(FieldType::DateTime)
    } else {
        quote::quote!(FieldType::String)
    }
}
