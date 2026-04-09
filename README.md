# brom

**A Code-First Headless CMS Framework for Rust**

`brom` is an ergonomic, macro-driven framework that transforms standard Rust structs into a fully functional headless CMS. With a single `#[derive(BromEntity)]` annotation, `brom` handles the boilerplate of creating database schemas, generating a complete REST API, documenting it with OpenAPI, and scaffolding an interactive admin dashboard—all compiled into a single, dependency-free binary.

## Why brom?

Traditional CMS setups require you to navigate layers of frameworks, databases, Docker containers, and external runtime dependencies (like Node.js). `brom` optimizes for simplicity and performance:

*   **Zero External Dependencies**: Ships as a static binary with SQLite compiled in and the admin SPA embedded (targeted for Phase 5).
*   **Compile-Time Safety**: Your schema is defined in Rust code, ensuring complete type safety and structurally bounds SQL injections at compile time.
*   **Built-in Auth**: Argon2 session cookies and Bearer token API keys are standard, meaning you don't need a third-party directory for authentication.
*   **High Performance**: Designed to deliver sub-millisecond response latency for CMS operations.

## Quick Start

### 1. Define Your Data Models

Simply define your content structures as standard Rust structs and annotate them. The macro handles the heavy lifting:

```rust
use brom_core::entity::BromEntity;

#[derive(BromEntity)]
pub struct BlogPost {
    pub title: String,
    
    // You can customize the Admin UI with #[brom] attributes 
    #[brom(ui(widget = "markdown"))]
    pub content: String,
}
```

This single definition expands to generate:
*   A `CREATE TABLE` SQLite schema.
*   Full CRUD endpoints mounted on `/admin/api/entities`.
*   A Swagger OpenAPI interface reflecting the route payload logic.
*   Dynamic JSON schemas instructing the embedded Admin SPA on how to render inputs (Phase 5 target).

### 2. Manage Migrations

Because the schema is defined in your Rust code, changes to the database constraints are controlled through the built-in `brom-cli` tool.

1.  Compare your Rust structs against the live database to generate timestamped SQL migration files:
    ```bash
    cargo run --bin brom-cli -- diff
    ```
2.  Review the `.sql` migration file output in the `migrations/` directory.
3.  Apply pending migrations to your SQLite database:
    ```bash
    cargo run --bin brom-cli -- migrate
    ```

### 3. Run the Server

Start the application using:
```bash
cargo run --bin brom-server
```

When you launch the server, it seamlessly mounts:
*   **API Layer**: The auto-generated REST API (`/admin/api/entities/*`).
*   **API Docs**: The Swagger OpenAPI documentation endpoints.
*   **Admin Panel**: The embedded Admin UI accessible at `/admin`. Log in to handle content using built-in session mechanics (Phase 5 target).

## Architecture Highlights

`brom` separates its domain responsibilities across explicitly bounded crates to provide flexible execution and clear design rules:

*   `brom-core`: Traits and foundational elements like `EntitySchema` and Relationship wrappers (`Link<T>`, `ManyToMany<T>`).
*   `brom-macros`: Proc-macros driving `#[derive(BromEntity)]` to synthesize endpoints and database instructions.
*   `brom-db`: The persistence layer, managing the SQLite connection pool (`r2d2`) and the auto-migration runner.
*   `brom-server`: Handshakes the Axum HTTP routes to SQLite execution scopes, hosting the web layer natively.
*   `brom-cli`: Project companion for evaluating database diffs, seeding, and triggering migrations.
*   `admin`: The self-hosted Leptos frontend SPA, compiled in and served natively (Phase 5 targeted).

## Development & Verification Pipeline

`brom` projects adhere to strict pipeline verifications to maintain quality and correctness before any commit. Standard development workflows should execute:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
sg scan  # Used to lint specific AST conditions
```
*Note: We enforce a zero-exit policy across all these checks.*
