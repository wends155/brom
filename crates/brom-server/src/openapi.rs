use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// Base OpenAPI specification for the brom framework.
/// 
/// This includes the core administrative APIs. Entity-specific routes
/// are dynamically discovered in the generated Swagger UI configuration.
#[derive(OpenApi)]
#[openapi(
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
    SwaggerUi::new("/docs")
        .url("/api-docs/openapi.json", ApiDoc::openapi())
}
