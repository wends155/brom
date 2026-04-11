use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, LitStr};

#[allow(clippy::too_many_lines)]
pub fn expand_routes(struct_name: &Ident, policy: Option<&str>) -> TokenStream {
    let lower_name = struct_name.to_string().to_lowercase();
    let public_struct_name = format_ident!("{}Public", struct_name);

    let admin_api_mod_name = format_ident!("{}_admin_api", lower_name);
    let public_api_mod_name = format_ident!("{}_public_api", lower_name);

    let admin_base_path = format!("/admin/api/entities/{lower_name}");
    let admin_id_path = format!("{admin_base_path}/{{id}}");
    let public_base_path = format!("/api/v1/entities/{lower_name}");
    let public_id_path = format!("{public_base_path}/{{id}}");

    let admin_tag_name = format!("Admin {struct_name}");
    #[allow(unused_variables)]
    let public_tag_name = format!("Public {struct_name}");

    let admin_base_lit = LitStr::new(&admin_base_path, proc_macro2::Span::call_site());
    let admin_id_lit = LitStr::new(&admin_id_path, proc_macro2::Span::call_site());
    let public_base_lit = LitStr::new(&public_base_path, proc_macro2::Span::call_site());
    let public_id_lit = LitStr::new(&public_id_path, proc_macro2::Span::call_site());

    let admin_module = quote! {
        pub mod #admin_api_mod_name {
            use super::*;
            use ::brom_server::axum::{extract::{State, Path}, Json, http::StatusCode};
            use ::brom_server::AppState;
            use ::brom_core::{Repository, Pagination};
            use ::brom_db::SqliteRepository;
            use ::brom_server::{tracing, RequireAdmin};

            #[derive(::brom_server::serde::Deserialize)]
            pub struct PaginationParams {
                pub page: Option<u64>,
                pub per_page: Option<u64>,
            }

            #[::brom_server::utoipa::path(
                get,
                path = #admin_base_lit,
                responses((status = 200, description = "List all items", body = [#struct_name])),
                tag = #admin_tag_name
            )]
            #[tracing::instrument(skip_all)]
            pub async fn list_handler(
                state: State<AppState>,
                query: ::brom_server::axum::extract::Query<PaginationParams>,
                _: RequireAdmin,
            ) -> Result<Json<Vec<#struct_name>>, ::brom_server::ServerError> {
                let pagination = ::brom_core::Pagination::new(
                    query.0.page.unwrap_or(1),
                    query.0.per_page.unwrap_or(25),
                );
                let repo = SqliteRepository::<#struct_name>::new(state.db.clone());
                let items = ::brom_core::Repository::find_all(&repo, &pagination)?;
                Ok(Json(items))
            }

            #[::brom_server::utoipa::path(
                get,
                path = #admin_id_lit,
                params(("id" = i64, Path, description = "ID")),
                responses((status = 200, description = "Get item by ID", body = #struct_name), (status = 404, description = "Not found")),
                tag = #admin_tag_name
            )]
            #[tracing::instrument(skip_all)]
            pub async fn get_handler(state: State<AppState>, id: Path<i64>) -> Result<Json<#struct_name>, ::brom_server::ServerError> {
                let id = id.0;
                let repo = SqliteRepository::<#struct_name>::new(state.db.clone());
                let item = ::brom_core::Repository::find_by_id(&repo, id)?.ok_or(::brom_core::Error::NotFound { entity: #lower_name, id })?;
                Ok(Json(item))
            }

            #[::brom_server::utoipa::path(
                post,
                path = #admin_base_lit,
                responses((status = 201, description = "Item created", body = #struct_name)),
                tag = #admin_tag_name
            )]
            #[tracing::instrument(skip_all)]
            pub async fn create_handler(state: State<AppState>, payload: Json<#struct_name>) -> Result<Json<#struct_name>, ::brom_server::ServerError> {
                let repo = SqliteRepository::<#struct_name>::new(state.db.clone());
                let id = repo.create(&payload.0)?;
                let item = repo.find_by_id(id)?.ok_or(::brom_core::Error::NotFound { entity: #lower_name, id })?;
                Ok(Json(item))
            }

            #[::brom_server::utoipa::path(
                put,
                path = #admin_id_lit,
                params(("id" = i64, Path, description = "ID")),
                responses((status = 200, description = "Item updated", body = #struct_name), (status = 404, description = "Not found")),
                tag = #admin_tag_name
            )]
            #[tracing::instrument(skip_all)]
            pub async fn update_handler(state: State<AppState>, id: Path<i64>, payload: Json<#struct_name>) -> Result<Json<#struct_name>, ::brom_server::ServerError> {
                let id = id.0;
                let repo = SqliteRepository::<#struct_name>::new(state.db.clone());
                repo.update(id, &payload.0)?;
                let item = repo.find_by_id(id)?.ok_or(::brom_core::Error::NotFound { entity: #lower_name, id })?;
                Ok(Json(item))
            }

            #[::brom_server::utoipa::path(
                delete,
                path = #admin_id_lit,
                params(("id" = i64, Path, description = "ID")),
                responses((status = 204, description = "Item deleted"), (status = 404, description = "Not found")),
                tag = #admin_tag_name
            )]
            #[tracing::instrument(skip_all)]
            pub async fn delete_handler(state: State<AppState>, id: Path<i64>) -> Result<StatusCode, ::brom_server::ServerError> {
                let id = id.0;
                let repo = SqliteRepository::<#struct_name>::new(state.db.clone());
                repo.delete(id)?;
                Ok(StatusCode::NO_CONTENT)
            }
        }
    };

    #[allow(clippy::single_match_else)]
    let handlers = match policy {
        Some("ApiKey") => quote! {
            #[derive(::brom_server::serde::Deserialize)]
            pub struct PaginationParams {
                pub page: Option<u64>,
                pub per_page: Option<u64>,
            }

            use ::brom_server::tracing;

            #[::brom_server::utoipa::path(get, path = #public_base_lit, responses((status = 200, description = "List all items", body = PaginatedResponse<#public_struct_name>)), tag = #public_tag_name)]
            #[tracing::instrument(skip_all)]
            pub async fn list_handler(
                _: ::brom_server::RequireApiKey,
                state: ::brom_server::axum::extract::State<::brom_server::AppState>,
                query: ::brom_server::axum::extract::Query<PaginationParams>,
            ) -> Result<::brom_server::axum::Json<::brom_server::PaginatedResponse<#public_struct_name>>, ::brom_server::ServerError> {
                let pagination = ::brom_core::Pagination::new(
                    query.0.page.unwrap_or(1),
                    query.0.per_page.unwrap_or(25),
                );
                let repo = ::brom_db::SqliteRepository::<#struct_name>::new(state.db.clone());
                let total_items = ::brom_core::Repository::count(&repo)?;
                let total_pages = (total_items + i64::from(pagination.per_page as i32) - 1) / i64::from(pagination.per_page as i32);
                let items = ::brom_core::Repository::find_all(&repo, &pagination)?;
                let pub_items = items.into_iter().map(Into::into).collect();
                Ok(::brom_server::axum::Json(::brom_server::PaginatedResponse::new(pub_items, total_items, total_pages, pagination.page, pagination.per_page)))
            }
            #[::brom_server::utoipa::path(get, path = #public_id_lit, params(("id" = i64, Path, description = "ID")), responses((status = 200, description = "Get item by ID", body = DataEnvelope<#public_struct_name>), (status = 404, description = "Not found")), tag = #public_tag_name)]
            #[tracing::instrument(skip_all)]
            pub async fn get_handler(
                _: ::brom_server::RequireApiKey,
                state: ::brom_server::axum::extract::State<::brom_server::AppState>,
                id: ::brom_server::axum::extract::Path<i64>
            ) -> Result<::brom_server::axum::Json<::brom_server::DataEnvelope<#public_struct_name>>, ::brom_server::ServerError> {
                let id = id.0;
                let repo = ::brom_db::SqliteRepository::<#struct_name>::new(state.db.clone());
                let item = ::brom_core::Repository::find_by_id(&repo, id)?.ok_or(::brom_server::ServerError::Core(::brom_core::Error::NotFound { entity: #lower_name, id }))?;
                Ok(::brom_server::axum::Json(::brom_server::DataEnvelope::new(item.into())))
            }
        },
        _ => quote! {
            #[derive(::brom_server::serde::Deserialize)]
            pub struct PaginationParams {
                pub page: Option<u64>,
                pub per_page: Option<u64>,
            }

            use ::brom_server::tracing;

            #[::brom_server::utoipa::path(get, path = #public_base_lit, responses((status = 200, description = "List all items", body = PaginatedResponse<#public_struct_name>)), tag = #public_tag_name)]
            #[tracing::instrument(skip_all)]
            pub async fn list_handler(
                state: ::brom_server::axum::extract::State<::brom_server::AppState>,
                query: ::brom_server::axum::extract::Query<PaginationParams>,
            ) -> Result<::brom_server::axum::Json<::brom_server::PaginatedResponse<#public_struct_name>>, ::brom_server::ServerError> {
                let pagination = ::brom_core::Pagination::new(
                    query.0.page.unwrap_or(1),
                    query.0.per_page.unwrap_or(25),
                );
                let repo = ::brom_db::SqliteRepository::<#struct_name>::new(state.db.clone());
                let total_items = ::brom_core::Repository::count(&repo)?;
                let total_pages = (total_items + i64::from(pagination.per_page as i32) - 1) / i64::from(pagination.per_page as i32);
                let items = ::brom_core::Repository::find_all(&repo, &pagination)?;
                let pub_items = items.into_iter().map(Into::into).collect();
                Ok(::brom_server::axum::Json(::brom_server::PaginatedResponse::new(pub_items, total_items, total_pages, pagination.page, pagination.per_page)))
            }
            #[::brom_server::utoipa::path(get, path = #public_id_lit, params(("id" = i64, Path, description = "ID")), responses((status = 200, description = "Get item by ID", body = DataEnvelope<#public_struct_name>), (status = 404, description = "Not found")), tag = #public_tag_name)]
            #[tracing::instrument(skip_all)]
            pub async fn get_handler(
                state: ::brom_server::axum::extract::State<::brom_server::AppState>,
                id: ::brom_server::axum::extract::Path<i64>
            ) -> Result<::brom_server::axum::Json<::brom_server::DataEnvelope<#public_struct_name>>, ::brom_server::ServerError> {
                let id = id.0;
                let repo = ::brom_db::SqliteRepository::<#struct_name>::new(state.db.clone());
                let item = ::brom_core::Repository::find_by_id(&repo, id)?.ok_or(::brom_server::ServerError::Core(::brom_core::Error::NotFound { entity: #lower_name, id }))?;
                Ok(::brom_server::axum::Json(::brom_server::DataEnvelope::new(item.into())))
            }
        },
    };

    let public_module = quote! {
        pub mod #public_api_mod_name {
            use super::*;
            use ::brom_server::axum::{extract::{State, Path}, Json};
            use ::brom_server::AppState;
            use ::brom_core::{Repository, Pagination};
            use ::brom_db::SqliteRepository;
            use ::brom_server::{DataEnvelope, PaginatedResponse};

            #handlers
        }
    };

    #[allow(clippy::single_match_else)]
    let public_router_stmt = match policy {
        Some("AdminOnly") => {
            quote! { ::brom_server::axum::Router::<::brom_server::AppState>::new() }
        }
        _ => quote! {
            ::brom_server::axum::Router::<::brom_server::AppState>::new()
                .route(#public_base_lit, get(#public_api_mod_name::list_handler))
                .route(#public_id_lit, get(#public_api_mod_name::get_handler))
        },
    };

    quote! {
        #[automatically_derived]
        #admin_module

        #[automatically_derived]
        #public_module

        #[automatically_derived]
        impl #struct_name {
            pub fn admin_router() -> ::brom_server::axum::Router<::brom_server::AppState> {
                use ::brom_server::axum::routing::get;
                ::brom_server::axum::Router::<::brom_server::AppState>::new()
                    .route(#admin_base_lit, get(#admin_api_mod_name::list_handler).post(#admin_api_mod_name::create_handler))
                    .route(#admin_id_lit, get(#admin_api_mod_name::get_handler).put(#admin_api_mod_name::update_handler).delete(#admin_api_mod_name::delete_handler))
            }

            pub fn public_router() -> ::brom_server::axum::Router<::brom_server::AppState> {
                use ::brom_server::axum::routing::get;
                #public_router_stmt
            }
        }
    }
}
