//! A code-first headless CMS framework for Rust.
//!
//! `brom` is an ergonomic, macro-driven framework that transforms standard Rust structs into a fully functional headless CMS. With a single `#[derive(BromEntity)]` annotation, `brom` handles the boilerplate of creating database schemas, generating a complete REST API, documenting it with `OpenAPI`, and scaffolding an interactive admin dashboard—all compiled into a single, dependency-free binary.
//!
//! # Features
//!
//! *   **Macro-Driven**: Define your data models as Rust structs and let `brom` do the rest.
//! *   **Auto-Migration**: Automatic database schema generation and migrations.
//! *   **REST API**: Complete, type-safe API with search, filtering, and pagination.
//! *   **`OpenAPI` Documentation**: Automatically generated API documentation with `OpenAPI`.
//! *   **Admin Dashboard**: A beautiful, built-in dashboard for managing your content.

#[doc(hidden)]
pub mod __private {
    pub use brom_core;
    pub use brom_db;
    pub use brom_server;
    pub use serde;
    pub use tokio;
    pub use tracing;
    pub use utoipa;
}

mod app;

pub use app::BromApp;
pub use brom_auth::{ApiKeyStore, AuthError, SessionStore};
pub use brom_core::*;
pub use brom_db::{DbError, DbPool};
pub use brom_macros::*;
pub use brom_server::axum;
pub use brom_server::{AppState, ServerConfig, ServerError, create_router};
