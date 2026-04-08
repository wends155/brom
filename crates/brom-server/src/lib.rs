//! Axum REST API and Server components for the brom headless CMS framework.

/// API key lifecycle and management records.
pub mod api_keys;
/// Server configuration and settings mapping.
pub mod config;
/// Error definitions and standard conversions into HTTP responses.
pub mod error;
/// Authentication and authorization Axum extractors.
pub mod extractor;
/// Tower layers and standard Axum middlewares.
pub mod middleware;
/// `OpenAPI` integration and Swagger UI serving.
pub mod openapi;
/// Standardized JSON response formatting (e.g. `DataEnvelope`).
pub mod response;
/// Main router assembly defining routing tables.
pub mod router;
/// Catch-all endpoints for entity CRUD operations.
pub mod schema_api;
/// Shared application state containers passed across requests.
pub mod state;

pub use error::ServerError;
pub use extractor::{RequireAdmin, RequireApiKey};
pub use response::{DataEnvelope, PaginatedResponse};
pub use state::AppState;

/// Re-export axum for use by generated macro code.
pub use axum;
pub use utoipa;

/// Re-export `ServerConfig` for use by consumers.
pub use config::ServerConfig;

/// Creates the API router for a set of registered schemas.
pub fn create_router(state: AppState, cors_origins: Vec<axum::http::HeaderValue>) -> axum::Router {
    router::build_router(state, cors_origins)
}
