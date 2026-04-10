use crate::auth::auth_fetch;
use brom_core::entity::SchemaInfo;
use leptos::prelude::*;

#[derive(Clone)]
pub struct SchemaContext {
    pub schemas: LocalResource<Result<Vec<SchemaInfo>, String>>,
}

/// Provides the global SchemaContext to the application.
/// It fetches the schema metadata once on boot and makes it available to all components.
pub fn provide_schema_context() {
    let schemas = LocalResource::new(
        move || async move {
            let resp = auth_fetch("/admin/api/schema", "GET", None::<()>).await?;
            if resp.ok() {
                resp.json::<Vec<SchemaInfo>>()
                    .await
                    .map_err(|e| e.to_string())
            } else {
                Err(format!("Failed to fetch schema: {}", resp.status()))
            }
        },
    );

    provide_context(SchemaContext { schemas });
}

/// Helper hook to use the SchemaContext.
pub fn use_schema() -> SchemaContext {
    use_context::<SchemaContext>().expect("SchemaContext must be provided")
}
