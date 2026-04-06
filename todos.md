# Project TODOs & Technical Strategy

This document captures forward-looking architectural choices and technical tasks agreed upon during the brainstorming phases.

## 1. Hybrid Testing Strategy
*   **[ ] Keep `mockall` at the Core:** Ensure traits like `Repository<T>`, `SessionStore`, and `ApiKeyStore` use `#[automock]`. This allows higher-level crates (`brom-server`, `brom-auth`) to remain decoupled and fast during unit tests.
*   **[ ] SQLite `:memory:` for Data Layers:** Implement tests for `brom-db` concrete implementations (`SqliteRepository`, migrations) exclusively against real, in-memory SQLite instances to verify SQL dialects and bindings perfectly.

## 2. Pervasive Observability (Tracing)
*   **[ ] Implement File Sink (Phase 2+):** Integrate `tracing-appender` to support `just logs-*` recipes once actual runtime traffic generation begins.
*   **[ ] Customize Route Tracing (Phase 4):** Refine `tower_http::trace::TraceLayer` to extract unified CMS contexts and trim generic header noise.
*   **[ ] Network Edge Instrumentation:** Implement `tower_http::trace::TraceLayer` on the Axum router to guarantee span creation, method, URI, and latency logging for *every* request.
*   **[ ] Deep Macro Instrumentation:** Use `#[tracing::instrument(skip_all)]` aggressively on core functions throughout the stack (e.g., `Repository::create`, `evaluate_policy`, `run_pending`).
*   **[ ] Span Propagation Check:** Ensure that `tracing` context is correctly carried down into `brom-db` so that database operations inherit the parent `request_id`.

## 3. Toolchain & Quality Gates
*   **[ ] Retain AST-Grep (`sg scan`):** Maintain AST-Grep in the verification pipeline.
*   **[ ] Tune AST-Grep Rules:** Refine `.ast-grep` rules to correctly ignore `mod tests` and specific test module macros to prevent pipeline friction when mocking or unwrap testing. 

## 4. Advanced Testing & Coverage
*   **[x] E2E Integration (oneshot):** Utilize `tower::ServiceExt::oneshot` to perform end-to-end API integration tests without needing live TCP ports.
*   **[/] Snapshot Testing:**
    *   **[x] Macro Expansion Snapshots:** Wire `insta` and `prettyplease` for `BromEntity` expansion verification (Structural Regression Safety).
    *   **[ ] CLI Output Snapshots (Deferred):** Implement `insta` for `brom diff` and complex CLI outputs once subcommands are fully stabilized.
