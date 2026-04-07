#![allow(clippy::needless_for_each)]
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// Base `OpenAPI` specification for the `brom` framework.
///
/// This includes the core administrative APIs. Entity-specific routes
/// are dynamically discovered in the generated Swagger UI configuration.
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::router::login,
        crate::router::logout,
        crate::api_keys::list_keys,
        crate::api_keys::create_key,
        crate::api_keys::revoke_key,
    ),
    components(
        schemas(
            crate::router::LoginRequest,
            crate::router::LoginResponse,
            crate::api_keys::ApiKeyRecordDto,
            crate::api_keys::CreateApiKeyRequest,
            crate::api_keys::CreateApiKeyResponse,
        )
    ),
    info(
        title = "brom API",
        version = "0.1.0",
        description = "Automated REST API and CRUD interface for brom entities."
    ),
    tags(
        (name = "admin", description = "Framework administrative operations")
    )
)]
pub struct ApiDoc;

/// Returns a Swagger UI instance configured for the /docs endpoint.
pub fn swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi())
}
