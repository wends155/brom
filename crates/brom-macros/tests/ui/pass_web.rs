use brom_macros::BromEntity;
use serde::{Deserialize, Serialize};

// Use the re-exported utoipa from brom-server
pub use brom_server::utoipa::{self, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema, BromEntity)]
#[brom(table = "posts")]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
}

fn main() {
    // The macro should generate Post::router() and PostApi struct.
    let _router = Post::router();
    
    // To call .openapi(), the trait must be in scope.
    let _doc = <PostApi as brom_server::utoipa::OpenApi>::openapi();
}
