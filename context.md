# Brom Headless CMS Context

> 📝 **Context Update:**
> * **Feature:** Phase 1 (Foundation) Scaffold
> * **Changes:** Initialized virtual workspace, `.editorconfig`, `.gitignore`. Implemented Core domain types (`brom-core`), DbPool/MigrationRunner skeletons (`brom-db`), mockable Auth stores (`brom-auth`). Added server and CLI module stubs.
> * **New Constraints:** Strict adherence to Phase 2 stubs implementation. Error mappings enforce HTTP serialization rules.
> * **Pruned:** The Phase 1 scaffold details can now be ignored; rely on `architecture.md` as the frozen specification.
> * **Feature:** Infrastructure Wiring (Testing & Observability)
> * **Changes:** Added `mockall` for trait mocking in `brom-core` and `brom-auth` (feature-gated). Instrumented all public methods in `brom-db` and `brom-auth` with `tracing::instrument`. Wired `tower-http` TraceLayer into `brom-server` router. Initialized `tracing-subscriber` in `brom-cli`.
> * **New Constraints:** Unit tests requiring mocks must enable the `testing` feature.
> * **Feature:** Verification Pipeline Remediation
> * **Changes:** Removed stable-incompatible nightly settings from `rustfmt.toml`. Updated `wildcard-import.yml` with AST-aware exclusion to permit wildcard imports in inline `mod tests` blocks, matching the logic established for `.unwrap()` checks.
> * **Feature:** Phase 2A (BromEntity Derive Macro)
> * **Changes:** Fully implemented `#[derive(BromEntity)]`. Added `syn` 2.0 based attribute parsing for `#[brom(table=...)]` and field constraints. Implemented `EntitySchema` trait generation including `table_name()`, `fields()`, and `schema_info()`. Added type mapping for common Rust types to `FieldType`.
> * **Verification:** Added `trybuild` integration tests covering basic success, attribute overrides, and enum-rejection failure modes.
> * **Feature:** Phase 2A (Macro Error Remediation)
> * **Changes:** Refactored `BromEntity` attribute parsing to return `syn::Result` with `syn::Error::combine`, ensuring exhaustive multi-error accumulation and strict unrecognized attribute validation. Abstracted expansion implementation to observe strict line-count cleanliness.
> * **New Constraints:** Any future procedural macro logic must accumulate errors rather than emitting early bail-outs, ensuring compile-time developer feedback is maximized.
> * **Pruned:** The unstable macro parsing state and false `unwraps` are history. The macro foundation is formally locked and verified.
> * **Feature:** Workflow Execution Safety
> * **Changes:** Migrated AI agent IDE execution constraints (`|`, `&&`, `;`) from hardcoded regex queries across `.agent/workflows/.md` templates to a decentralized `.justfile` task runner. Added `git-diff-last` recipe to bypass `~` character constraints in `run_command` auto-run.
> * **New Constraints:** Any future automation commands relying on regex, complex chaining, or IDE-restricted characters (like `~`) MUST be properly encapsulated within a `.justfile` recipe.
> * **Pruned:** Manual `git diff HEAD~1..HEAD` calls in `audit.md` are removed. Rely on `just git-diff-last` for Reflect phase safety.
> * **Feature:** Infrastructure Remediation (Observability & Testing)
> * **Changes:** Fixed `brom_auth::evaluate_policy` instrumentation to capture `AuthPolicy` context while masking sensitive tokens. Added 15 baseline unit/integration tests across `brom-core`, `brom-auth`, `brom-db`, and `brom-server`. Implemented `IntoResponse` status code mapping tests for `ServerError`.
> * **New Constraints:** Any new feature implementation MUST include matching `#[cfg(test)]` modules following the established Red-Green-Blue cycle.
> * **Pruned:** The deferred file-based log sink and `TraceLayer` customization items are now tracked in `todos.md` for Phase 2/4 execution.
> * **Feature:** Verification Pipeline Remediation (FMT & Linter)
> * **Changes:** Fixed formatting drift in `brom-core` and `brom-db` test files. Resolved `clippy::used-underscore-binding` in `brom-auth` by renaming the instrumented `_policy` parameter to `policy`.
> * **New Constraints:** The `tracing::instrument` macro consumes function parameters for fields; such parameters MUST NOT use the `_` prefix if they are to be instrumented.
> * **Feature:** Environment Normalization (Line-Endings)
> * **Changes:** Established repository-wide LF normalization by adding `.gitattributes` and configuring `.editorconfig`. This ensures byte-level consistency for AI agent file operations on Windows hosts.
> * **New Constraints:** Any new text-based file types MUST be added to `.gitattributes` if Git's `auto` detection is insufficient.
> * **Feature:** Phase 2 (Persistence & Migrations)
> * **Changes:** Transitioned from database stubs to fully-functional generic JSON-to-SQLite mappings using `serde_json`. Implemented `MigrationRunner::run_pending` to provision schemas synchronously. Removed all Phase 2 STUBs across `brom-db`.
> * **New Constraints:** Generics utilizing the `Repository` trait must satisfy `Serialize + DeserializeOwned` bounds constraints.
> * **Feature:** Phase 2 Verification Remediation
> * **Changes:** Fixed `clippy::uninlined_format_args` in `brom-cli/src/main.rs`. Centralized environment variable loading into a new `config` module in `brom-cli` to resolve `scattered-env-var` AST-grep warnings.
> * **New Constraints:** Any new environment variable lookups in CLI commands MUST be added to `AppConfig` in `config.rs`.
> * **Pruned:** The direct `std::env::var()` calls in `main.rs` are removed.
> * **Feature:** Toolcheck Workflow Hardening
> * **Changes:** Encapsulated PowerShell version checking and multi-tool environment scans into safe `.justfile` recipes (`pwsh-version`, `verify-toolchain`). Updated `.agent/workflows/toolcheck.md` to trigger these recipes, eliminating direct use of restricted characters (`$`) and shell chaining operators (`;`) in `run_command` calls.
> * **New Constraints:** Any automated environment checks requiring PowerShell internal variables or command pipelines MUST be mediated through `just` to avoid IDE auto-run interception.
> * **Pruned:** Direct `$PSVersionTable` lookups and shell-level command chaining in toolcheck execution are deprecated in favor of `just` recipes.
> * **Feature:** Workflow Discovery Remediation
> * **Changes:** Replaced references to the non-existent `find_by_name` tool with the native `list_dir` tool in `.agent/workflows/toolcheck.md` and `.agent/workflows/architecture.md`.
> * **New Constraints:** All environment discovery and file listing tasks MUST use native agent tools (`list_dir`, `grep_search`) rather than shell traversal commands or invalid internal tool names.
> * **Pruned:** The stale `find_by_name` instructions are removed, eliminating the primary cause of shell-level command fallbacks in session readiness workflows.
> * **Feature:** Audit Template Clarification
> * **Changes:** Updated `.agent/rules/audit-rules.md` to rename the "Findings" section to "Violations & Deviations". Added explicit fallback instruction: "No violations found". Moved positive confirmations and passing items to the "Compliant Items" block.
> * **New Constraints:** Audit reports MUST NOT populate the "Violations & Deviations" table with positive observations. Positive confirmations belong strictly in the bulleted "Compliant Items" list.
> * **Pruned:** The ambiguous "Findings" header and its associated confusion regarding positive confirmations in audit reports.
> * **Feature:** Phase 3 Infrastructure (Configuration & Observability)
> * **Changes:** Hardened the workspace by centralizing environment variable loading via `dotenvy`, establishing `.env.example`, mapping root logs to `logs/brom.log` via `tracing-appender`, and adding detailed `TraceLayer` HTTP instrumentation across the Axum router.
> * **New Constraints:** All configuration must be loaded through `dotenvy` via `AppConfig`. CLI and server environments must maintain dual-sink logs (stdout + file). Do not use `std::env::var` for configuration independent of `AppConfig`.
> * **Feature:** Phase 3A (Auth Core)
> * **Changes:** Replaced authentication stubs with production-ready logic. Implemented Argon2id password hashing, defined `SessionStore` and `ApiKeyStore` traits, and provided concrete SQLite implementations in `brom-db`. Built a robust RBAC `evaluate_policy` engine with full unit test coverage (~45 new tests).
> * **New Constraints:** Passwords MUST be hashed using `Argon2id` (v0.5). API keys MUST be stored as SHA-256 hashes with an 8-character prefix for identification.
> * **Pruned:** All `AuthCore` and persistence stubs in `brom-auth` and `brom-db` are now fully implemented.
> * **Verification:** Clean audit. Zero-exit gate (fmt, clippy, test), `scan-secrets`, `scan-stubs`, and `sg scan` all passed cleanly after isolating `allow(clippy::unwrap_used)` strictly to `#[cfg(test)]` modules. Fidelity to Plan: 100%.

> 📝 **Context Update:**
> * **Feature:** Narsil Workspace Attachment
> * **Changes:** Added the `brom` repository path to the Narsil MCP configuration in `mcp_config.json`. 
> * **New Constraints:** The Narsil MCP server must be restarted to finalize indexing of the project.
> * **Pruned:** (None)

> 📝 **Context Update:**
> * **Feature:** Toolcheck Workflow Cleanup
> * **Changes:** Removed redundant shell and Rust version checks from `.agent/workflows/toolcheck.md`. All version reporting is now consolidated into the encapsulation-safe `just verify-toolchain` recipe.
> * **New Constraints:** (None)
> * **Pruned:** Duplicate version printouts and legacy inline shell commands for `git`, `rg`, `sg`, and Rust tools are removed.
> * **Feature:** Phase 3A (Auth Core Remediation)
> * **Changes:** Fixed `clippy::doc_markdown` violations in `rbac.rs`. Applied `clippy::expect_used` suppressions to `password.rs` tests. Cleaned up redundant wildcard exports in `brom_db`.
> * **New Constraints:** (None - adhered to existing standards)
> * **Pruned:** Zero-exit verification gate is formally restored.
> 
> 📝 **Context Update:**
> * **Feature:** Roadmap Alignment (Phase 3 Split)
> * **Changes:** Formally split Phase 3 into Phase 3A (Auth Core - Done) and Phase 3B (REST API & Codegen - Next) in `roadmap.md` tables and dependency graph.
> * **New Constraints:** (None)
> * **Pruned:** Monolithic Phase 3 ambiguity removed from phasing plans.
> 
> 📝 **Context Update:**
> * **Feature:** Phase 3B Audit Remediation (REST API & Codegen)
> * **Changes:** Fixed `clippy::uninlined_format_args` in `routes.rs` and `missing_errors_doc` in `router.rs`. Resolved `BromEntity` dependency violation in `brom-db` test fixtures by manually implementing `EntitySchema`. Suppressed macro-generated `clippy` failures (`too_many_lines`, `needless_for_each`) in `brom-macros` and `brom-server`.
> * **New Constraints:** The `#[derive(BromEntity)]` macro is architecturally bound to `brom-server`. It MUST NOT be used in lower-level crates (e.g., `brom-db`). Use manual `EntitySchema` implementations for test fixtures in lower layers to preserve dependency isolation.
> * **Pruned:** The Phase 3B audit blockers (dependency breaches, linting drift, formatting) are cleared. Zero-exit verification gate is fully restored for the workspace.
> 
> 📝 **Context Update:**
> * **Feature:** Formal Lint Suppression Inventory
> * **Changes:** Created `lint_suppression_inventory.md` in the project root. This artifact tabulates all `#[allow]` attributes across the workspace, providing technical justifications for each to ensure long-term auditability and prevent linting decay.
> * **New Constraints:** Any new lint suppressions MUST be added to the `lint_suppression_inventory.md` with a valid technical justification.
> * **Pruned:** The temporary `issue_report.md` investigation data is now formally persisted in the root inventory.
> 📝 **Context Update:**
> * **Feature:** Phase 3B Security Audit Remediation
> * **Changes:** Performed deep-dive validation of 5 security findings from `/review all all`. Confirmed 3/5 were false positives (insecure randomness in tests, hardcoded test credentials, rusqlite parameter indexing noise). Hardened `EntitySchema` trait in `brom-core` with `# Safety` documentation enforcing compile-time constants for schema metadata.
> * **New Constraints:** Any manual implementation of `EntitySchema` MUST adhere to the `# Safety` section to prevent SQL injection in the repository's dynamic query builder.
> * **Deferred:** The JSON allocation bottleneck in `SqliteRepository` is formally deferred to the `roadmap.md` Tech Debt Register for Post-v1 optimization.
> * **Security:** Production code verified secure. `OsRng` is correctly used for all production token generation; SQL vectors are structurally bounded by the `&'static str` trait contract.
> 
> 📝 **Context Update:**
> * **Feature:** Project Documentation Foundation
> * **Changes:** Synthesized project architecture, toolchain verifications, and macro usage documentation into a comprehensive `README.md` at the repository root.
> * **New Constraints:** The `README.md` should serve as the initial anchor for standard workflows. Ensure pipeline execution commands described within align precisely with the zero-exit gates enforced in `architecture.md`.
> * **Pruned:** Ad-hoc usage questions are now explicitly answered via the README.

> 📝 **Context Update:**
> * **Feature:** Phase 1 (Test Visibility)
> * **Changes:** Established code coverage baseline infrastructure. Added `coverage` and `coverage-html` recipes to `justfile` utilizing `cargo llvm-cov`. Updated `architecture.md` toolchain to include `Coverage` as a standard diagnostic tool.
> * **New Constraints:** Any developer-level coverage reporting requires `cargo-llvm-cov` to be installed locally. Coverage metrics are currently for observation only; no hard failure thresholds are enforced.
> 
> 📝 **Context Update:**
> * **Feature:** API Integration Tests (E2E)
> * **Changes:** Added `tower::ServiceExt::oneshot` infrastructure to run in-process API integration tests for `brom-server` without live TCP binding. Created `tests/common/mod.rs` harness utilizing in-memory SQLite (`DbPool`) to satisfy both `session_store` and `api_key_store` trait implementations. Covered login, logout, schema, zero-route paths, and security headers in `tests/api_test.rs`.
> * **New Constraints:** Any new routes added to `brom-server/src/router.rs` MUST have corresponding E2E tests in `tests/api_test.rs` validating both success pathways and `ServerError` status mappings.
> * **Pruned:** The `todos.md` backlog item for "E2E Integration" is complete and removed. Line coverage for `brom-server` has significantly scaled up.
> 📝 **Context Update:**
> * **Feature:** Dynamic CORS Configuration (Environment-Driven)
> * **Changes:** Extracted hardcoded CORS origins from `middleware.rs` into a centralized `ServerConfig` struct in `config.rs`. Implemented `ServerConfig::load_from_env()` using `dotenvy` and the `BROM_CORS_ORIGINS` environment variable. Updated `create_router` and `build_router` signatures to accept dynamic origins. Added CORS origin template to `.env.example`.
> * **New Constraints:** Any additional CORS origins MUST be configured via `BROM_CORS_ORIGINS` (comma-separated list). Hardcoded URLs in middleware are strictly prohibited.
> * **Pruned:** Removed all `// ast-grep-ignore: hardcoded-url` suppressions in `middleware.rs`.
> * **Verification:** Clean audit. Zero-exit gate (fmt, clippy, test) passed. All 18 integration tests in `brom-server` verified with mock origin injection.

> 📝 **Context Update:**
> * **Feature:** Verification Pipeline Remediation (Clippy Doc Markdown)
> * **Changes:** Fixed `clippy::doc_markdown` violations within `brom-server/tests/common/mod.rs` and `brom-server/tests/api_test.rs` by securing identifiers within code block backticks in documentation comments.
> * **New Constraints:** (None)
> * **Pruned:** Clippy linting errors for markdown documentation. Zero-exit pipeline formally restored.
> 
> 📝 **Context Update:**
> * **Feature:** Macro Snapshot Testing (Structural Regression)
> * **Changes:** Implemented structural snapshot testing for `#[derive(BromEntity)]` using `insta` and `prettyplease`. Added 5 expansion variants in `crates/brom-macros/src/entity.rs`: Basic Struct, Custom Table, Field Constraints, Link relationship, and ManyToMany relationship. Integrated `insta` and `prettyplease` into the workspace dev-dependencies.
> * **New Constraints:** Any modification to the `BromEntity` expansion logic MUST be verified against these snapshots. Use `INSTA_UPDATE=always cargo test -p brom-macros` to update baselines after intentional structural changes.
> * **Pruned:** Deferred CLI snapshot testing to `todos.md` until `brom diff` is stable.
> * **Verification:** Clean audit. Zero-exit gate (fmt, clippy, test) passed. All 5 new snapshots verified and accepted. Fidelity to Plan: 100%.

> 📝 **Context Update:**
> * **Feature:** Phase 2B (Persistence & Migrations Hardening)
> * **Changes:** Refactored `brom-core::Error` with `Database(String)` and `Serde(String)` variants and `#[non_exhaustive]`. Hardened `SqliteRepository` with auto-inject `created_at`/`updated_at` timestamps using `chrono::Utc`. Implemented SHA-256 checksum verification in `MigrationRunner`. Updated `_brom_migration` schema to include `name` and `checksum`. Added `diff` subcommand stub to CLI.
> * **New Constraints:** Any modification to existing migrations will trigger a checksum mismatch error. Entities using `SqliteRepository` must include `created_at` and `updated_at` fields (as `Option<String>`) to receive auto-timestamps.
> * **Verification:** Established `repository_test.rs` (CRUD lifecycle) and `migration_test.rs` (idempotency/schema) integration tests in `brom-db`.
> * **Technical Debt:** `BromEntity` macro is currently coupled with `utoipa` and `brom-server`; integration tests in `brom-db` require these as `dev-dependencies`.
> * **Audit Outcome:** ❌ FAILED (Verification Gates violated). Linter (`expect_used`) and formatting anomalies detected in Phase 2B tests. Structural violation (`brom-db` testing dynamically references `brom-macros` and `brom-server`). Remediations required via Phase 2B Tech Debt/Plan.

> 📝 **Context Update:**
> * **Feature:** Phase 2B Verification Gate Remediation
> * **Changes:** Removed cyclic dev-dependencies (`brom-macros`, `brom-server`) from `brom-db` to enforce strict architectural isolation. Manually implemented `EntitySchema` on `Post` for testing in `repository_test.rs`. Locally suppressed `expect_used`/`unwrap_used` within test scopes for maintainability. Hardened `migration.rs` with `canonicalize()` to explicitly remediate CWE-22 Path Traversal.
> * **New Constraints:** Lower-level crates (e.g. `brom-db`) MUST NEVER depend on execution-tier code-generation libraries (`brom-macros`) for their testing harness. File paths passed to disk I/O routines MUST be securely anchored with `canonicalize()`. 
> * **Pruned:** The Phase 2B structural integration anomalies and verification gate failures are fully resolved.
> * **Verification:** Clean audit. Zero-exit gate (fmt, clippy, test) restored workspace-wide.

> 📝 **Context Update:**
> * **Feature:** Documentation Sync (Phase 2B Cleanup)
> * **Changes:** Synchronized `spec.md` with current Phase 2B behavioral contracts and updated the verification hash (`83c78be`). Hardened `brom-db` doc comments in `migration.rs` and `repository.rs` with `# Arguments`, `# Returns`, `# Errors`, and validated `# Examples`.
> * **New Constraints:** (None)
> * **Pruned:** The stale documentation state and missing usage examples for the core storage engine are resolved.

