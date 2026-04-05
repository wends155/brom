use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, LitStr};

#[allow(clippy::too_many_lines)]
pub fn expand_routes(struct_name: &Ident) -> TokenStream {
    let lower_name = struct_name.to_string().to_lowercase();
    let api_mod_name = format_ident!("{}_api", lower_name);
    let base_path = format!("/admin/api/entities/{lower_name}");

    // Axum 0.7+ and Utoipa 5.x BOTH require `{id}` for capturing path segments.
    // The previous `:id` syntax caused Axum runtime router panics.
    let id_path = format!("{base_path}/{{id}}");
    let tag_name = format!("Entity {struct_name}");

    let base_path_lit = LitStr::new(&base_path, proc_macro2::Span::call_site());
    let id_path_lit = LitStr::new(&id_path, proc_macro2::Span::call_site());

    quote! {
        #[automatically_derived]
        pub mod #api_mod_name {
            use super::*;
            use ::brom_server::axum::{
                extract::{State, Path},
                Json,
                http::StatusCode,
            };
            use ::brom_server::AppState;
            use ::brom_core::{Repository, Pagination};
            use ::brom_db::SqliteRepository;

            #[::brom_server::utoipa::path(
                get,
                path = #base_path_lit,
                responses(
                    (status = 200, description = "List all items", body = [#struct_name])
                ),
                tag = #tag_name
            )]
            pub async fn list_handler(
                state: State<AppState>,
            ) -> Result<Json<Vec<#struct_name>>, ::brom_server::ServerError> {
                let repo = SqliteRepository::<#struct_name>::new(state.db.clone());
                let items = repo.find_all(&Pagination::default())?;
                Ok(Json(items))
            }

            #[::brom_server::utoipa::path(
                get,
                path = #id_path_lit,
                params(
                    ("id" = i64, Path, description = "The ID of the entity to retrieve")
                ),
                responses(
                    (status = 200, description = "Get item by ID", body = #struct_name),
                    (status = 404, description = "Item not found")
                ),
                tag = #tag_name
            )]
            pub async fn get_handler(
                state: State<AppState>,
                id: Path<i64>,
            ) -> Result<Json<#struct_name>, ::brom_server::ServerError> {
                let id = id.0;
                let repo = SqliteRepository::<#struct_name>::new(state.db.clone());
                let item = repo.find_by_id(id)?
                    .ok_or(::brom_server::ServerError::Core(::brom_core::Error::NotFound { entity: #lower_name, id }))?;
                Ok(Json(item))
            }

            #[::brom_server::utoipa::path(
                post,
                path = #base_path_lit,
                request_body = #struct_name,
                responses(
                    (status = 201, description = "Item created", body = #struct_name)
                ),
                tag = #tag_name
            )]
            pub async fn create_handler(
                state: State<AppState>,
                payload: Json<#struct_name>,
            ) -> Result<Json<#struct_name>, ::brom_server::ServerError> {
                let repo = SqliteRepository::<#struct_name>::new(state.db.clone());
                let id = repo.create(&payload.0)?;
                let item = repo.find_by_id(id)?
                    .ok_or(::brom_server::ServerError::Core(::brom_core::Error::NotFound { entity: #lower_name, id }))?;
                Ok(Json(item))
            }

            #[::brom_server::utoipa::path(
                put,
                path = #id_path_lit,
                params(
                    ("id" = i64, Path, description = "The ID of the entity to update")
                ),
                request_body = #struct_name,
                responses(
                    (status = 200, description = "Item updated", body = #struct_name),
                    (status = 404, description = "Item not found")
                ),
                tag = #tag_name
            )]
            pub async fn update_handler(
                state: State<AppState>,
                id: Path<i64>,
                payload: Json<#struct_name>,
            ) -> Result<Json<#struct_name>, ::brom_server::ServerError> {
                let id = id.0;
                let repo = SqliteRepository::<#struct_name>::new(state.db.clone());
                repo.update(id, &payload.0)?;
                let item = repo.find_by_id(id)?
                    .ok_or(::brom_server::ServerError::Core(::brom_core::Error::NotFound { entity: #lower_name, id }))?;
                Ok(Json(item))
            }

            #[::brom_server::utoipa::path(
                delete,
                path = #id_path_lit,
                params(
                    ("id" = i64, Path, description = "The ID of the entity to delete")
                ),
                responses(
                    (status = 204, description = "Item deleted"),
                    (status = 404, description = "Item not found")
                ),
                tag = #tag_name
            )]
            pub async fn delete_handler(
                state: State<AppState>,
                id: Path<i64>,
            ) -> Result<StatusCode, ::brom_server::ServerError> {
                let id = id.0;
                let repo = SqliteRepository::<#struct_name>::new(state.db.clone());
                repo.delete(id)?;
                Ok(StatusCode::NO_CONTENT)
            }
        }

        #[automatically_derived]
        impl #struct_name {
            /// Generated Axum router for this entity.
            pub fn router() -> ::brom_server::axum::Router<::brom_server::AppState> {
                use ::brom_server::axum::routing::get;
                ::brom_server::axum::Router::<::brom_server::AppState>::new()
                    .route(#base_path_lit, get(#api_mod_name::list_handler).post(#api_mod_name::create_handler))
                    .route(#id_path_lit, get(#api_mod_name::get_handler).put(#api_mod_name::update_handler).delete(#api_mod_name::delete_handler))
            }
        }
    }
}
