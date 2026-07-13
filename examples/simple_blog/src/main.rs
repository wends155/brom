//! A minimal blog example demonstrating the brom headless CMS framework.
//!
//! Run with: `cargo run -p simple-blog` (from `examples/simple_blog/` is recommended
//! so the default `SQLite` file `brom.db` is created next to the example).
//!
//! Then visit:
//! - Admin UI: <http://localhost:3000/admin>
//! - API docs (Swagger UI): <http://localhost:3000/docs>

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
    const ADDR: &str = "0.0.0.0:3000";

    // If you see `ERR_CONNECTION_REFUSED` in the browser, the server is not running.
    // Start this process and keep the terminal open, then open the URLs below.
    eprintln!();
    eprintln!("simple_blog listening on {ADDR}");
    // ast-grep-ignore
    eprintln!("  Admin UI:  http://127.0.0.1:3000/admin");
    // ast-grep-ignore
    eprintln!("  API docs:  http://127.0.0.1:3000/docs");
    eprintln!("  (Stop with Ctrl+C.)");
    eprintln!();

    BromApp::new()
        .entity::<Post>()
        .entity::<Category>()
        .serve(ADDR)
        .await
}
