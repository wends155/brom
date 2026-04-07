use serde::Serialize;
use utoipa::ToSchema;

/// Standard envelope for returning a single entity or collection without pagination.
#[derive(Debug, Serialize, ToSchema)]
pub struct DataEnvelope<T: Serialize> {
    pub data: T,
}

impl<T: Serialize> DataEnvelope<T> {
    /// Creates a new `DataEnvelope` containing the given data.
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

/// Paginated response envelope conforming to the API spec.
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}

impl<T: Serialize> PaginatedResponse<T> {
    /// Creates a new `PaginatedResponse`.
    pub fn new(
        data: Vec<T>,
        total_items: i64,
        total_pages: i64,
        current_page: u64,
        per_page: u64,
    ) -> Self {
        Self {
            data,
            meta: PaginationMeta {
                total_items,
                total_pages,
                current_page,
                per_page,
            },
        }
    }
}

/// Metadata for paginated responses.
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginationMeta {
    pub total_items: i64,
    pub total_pages: i64,
    pub current_page: u64,
    pub per_page: u64,
}
