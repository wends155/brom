//! A minimal blog example demonstrating the brom headless CMS framework.
//!
//! Run with: `cargo run -p simple-blog`
//!
//! Then visit:
//! - Admin UI: <http://localhost:3000/admin>
//! - API docs: <http://localhost:3000/swagger-ui>

use brom::BromApp;
use brom::BromEntity;
use serde::{Deserialize, Serialize};

/// A blog post entity.
#[derive(Debug, Serialize, Deserialize, BromEntity)]
#[brom(table = "posts")]
pub struct Post {
    /// The title of the blog post.
    pub title: String,
    /// The post body content.
    pub body: String,
    /// Whether the post is published.
    pub published: bool,
}

/// A content category for organizing posts.
#[derive(Debug, Serialize, Deserialize, BromEntity)]
#[brom(table = "categories")]
pub struct Category {
    /// The category display name.
    pub name: String,
    /// A short description of the category.
    pub description: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    BromApp::new()
        .entity::<Post>()
        .entity::<Category>()
        .serve("0.0.0.0:3000")
        .await
}
