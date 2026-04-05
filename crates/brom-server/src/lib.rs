//! Axum REST API and Server components for the brom headless CMS framework.

pub mod error;
pub mod extractor;
pub mod middleware;
pub mod openapi;
pub mod router;
pub mod schema_api;
pub mod state;

pub use error::ServerError;
pub use extractor::{RequireAdmin, RequireApiKey};
pub use state::AppState;

/// Re-export axum for use by generated macro code.
pub use axum;
pub use utoipa;

/// Creates the API router for a set of registered schemas.
pub fn create_router(state: AppState) -> axum::Router {
    router::build_router(state)
}
