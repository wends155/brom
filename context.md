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
> * **Security:** Production code verified secure. `OsRng` is correctly used for all production token generation; SQL identifier vectors are hardened via runtime `validate_sql_identifier()` checks in the persistence layer as defense-in-depth.

> 📝 **Context Update:**
> * **Feature:** SQL Identifier Safety Hardening (brom-core review findings)
> * **Changes:** Added `validate_sql_identifier()` to `brom-core` for runtime defense-in-depth against SQL identifier injection. Hardened `SqliteRepository` to validate all table and column names before `format!()` interpolation. Fixed `SchemaRegistry` silent-failure on poisoned `RwLock` by recovering data via `unwrap_or_else`. Corrected the stale `context.md` security claim about `&'static str` enforcement — `FieldInfo::name` is `String`, not `&'static str`, and relies on runtime validation.
> * **New Constraints:** Any code interpolating schema metadata into SQL MUST call `validate_sql_identifier()` first.
> * **Pruned:** The incorrect claim that `FieldInfo::name` is bounded by `&'static str`.
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

> 📝 **Context Update:**
> * **Feature:** API Architectural Design (Phase 3B Preparation)
> * **Changes:** Formally captured the API architecture decisions in a new S-Tier design document (`api.md`). This solidifies the "Split API Surfaces" (admin vs. public), the `AuthPolicy` matrix, URL-based versioning (`/api/v1/`), hidden field stripping behavior, pagination/filtering strategies, and macro dual-route expansion design.
> * **New Constraints:** Any implementation regarding the REST API routes and endpoints MUST align with the design formalized in `api.md`.
> * **Pruned:** (None)

> 📝 **Context Update:**
> * **Feature:** TARS Zero-Exit Verification Pipeline
> * **Changes:** Implemented `just verify` consolidated recipe. Updated `architecture.md` §7 to mandate `just verify` for all TARS quality gates. 
> * **Lints:** Remediated 20+ clippy lints across `brom-macros` and `brom-server`. Suppressed macro-generated lints in `openapi.rs`. Fixed `BromEntity` macro to strip `brom` attributes from derived `Public` structs.
> * **Verification:** Achieved Zero-Exit state across `fmt`, `clippy`, `test`, and `sg scan`.

> 📝 **Context Update:**
> * **Feature:** Phase 3B Audit Remediation & Regression Fixes
> * **Changes:** Remediated outstanding audit findings and regression issues in `brom-macros`.
> * **Audit Fixes:** Replaced non-compliant `.unwrap()` with documented `.expect()` in `entity.rs`. Updated `STUB` markers in CLI to align with roadmap.
> * **Macro Fixes:** Implemented robust error accumulation in `BromEntity` to prevent swallowed syn-errors. Fixed regression in `pass_web.rs` by updating `router` -> `public_router`.

> 📝 **Context Update:**
> * **Feature:** Roadmap Restructuring (Phase 3 Expansion)
> * **Changes:** Formally split the monolithic Phase 3B into Phase 3B (API Architecture & Server Core), Phase 3C (REST Route Generation), and Phase 3D (OpenAPI & Swagger UI). Updated `roadmap.md` Gantt chart, dependency graph, and feature-to-phase mapping to reflect this new granularity.
> * **New Constraints:** Phase 3D is now the mandatory gateway (🔒) for Phase 4 (Tooling) and Phase 5 (UI).
> * **Pruned:** The "Phase 3B: REST API & Codegen" catch-all phase is replaced with bounded, verifiable milestones.
> * **Verification:** Clean audit. Zero-exit gate (fmt, clippy, test, sg scan) passed workspace-wide. Fidelity to Plan: 100%.
> * **Status:** Session closed after roadmap restructuring. Phase 3B implementation is planned and pending approval.

> 📝 **Context Update:**
> * **Feature:** TARS Workflow Hardening (Narsil Call-Graph Integration)
> * **Changes:** Replaced manual structural investigation with automated Narsil call-graph analysis. (1) Updated `ipr.md` to mandate topological GEO ordering and automated blast-radius calculations. (2) Integrated call-graph verification gates into `plan-making.md`, `build.md`, and `spec.md`. (3) Implemented post-implementation structural drift detection in `audit.md`. (4) Enhanced `issue.md` with callout-based diagnostic tools (`get_callers`, `get_callees`, `get_complexity`).
> * **New Constraints:** All M-Tier and S-Tier plans MUST include a topological Global Execution Order derived from `get_callers`/`get_callees` analysis. Builders MUST STOP and escalate if `READ` phase calls reveal count drift since plan approval.
> * **Verification:** Clean `just verify` (fmt, clippy, test, sg scan) passed 100%. All core TARS workflows now utilize structural code analysis as the primary investigation lens. Fidelity to Plan: 100%.

> 📝 **Context Update:**
> * **Feature:** Phase 5B (Admin SPA Functional Wiring)
> * **Changes:** Wired the Admin SPA to the backend REST API core. Transitions internal fetches to `/api/v1/*`. Implemented schema-driven dynamic form logic in `form_editor.rs` using the shared `schema_ctx`. Created standardized Forge Dark inputs in `admin/src/components/inputs.rs`. Enabled functional Create (`POST`) and Update (`PUT`) operations for all registered entities.
> * **New Constraints:** Any additions to `FieldType` in `brom-core` require corresponding mapping in the `form_editor.rs` match block.
> * **Verification:** Full `just verify` (fmt, clippy, test, sg scan) passed with exit code 0. Zero-Exit gate satisfied. Fidelity to Plan: 100%.

> 📝 **Context Update:**
> * **Feature:** Operations Toolcheck Hardening
> * **Changes:** Migrated git credential operations out of `.agent/workflows/toolcheck.md` and into `justfile` recipe `disable-git-prompts` to prevent IDE constraint violations.
> * **Verification:** Zero-exit gate (fmt, clippy, test, sg scan) passed workspace-wide.

> 📝 **Context Update:**
> * **Feature:** Documentation and Behavioral Spec Synchronization
> * **Changes:** Applied comprehensive `///` documentation to public endpoints, generic data envelopes, and entity metadata structures. Synchronized `spec.md` with `brom-server` contracts and API behavioral scenarios.
> * **New Constraints:** Any new public traits, structs, or route handlers MUST include proper `///` doc comments.
> * **Pruned:** The `missing_docs`, `clippy::doc_markdown`, and `clippy::unwrap_used` violations are fully resolved.
> * **Verification:** Clean audit. Zero-exit gate (fmt, clippy, test, sg scan) passed. Fidelity to Plan: 100%.

> 📝 **Context Update:**
> * **Feature:** Path Traversal (CWE-22) Remediation
> * **Changes:** Hardened the migration runner in `brom-db` by replacing dynamic path construction with OS-provided `DirEntry` paths, mandatory `canonicalize()` resolution, and explicit `starts_with()` containment validation. Added `narsil-ignore` to suppress scanner bias on the verified secure path.
> * **New Constraints:** Any file system reads from dynamic directory paths MUST follow the `entry.path().canonicalize()` pattern followed by a base-path `starts_with()` check.
> * **Pruned:** Removed the `// narsil-ignore: CWE-22` suppression from the insecure code version; replaced it in the secure implementation.
> * **Verification:** Clean audit. Zero-exit gate (fmt, clippy, test, doc-tests) passed workspace-wide. Security check `narsil check-cwe-top25` verified for the target crate.

> 📝 **Context Update:**
> * **Feature:** Phase 3B/C/D Hardening & Close-out
> * **Changes:** Expanded Axum security headers with `X-Frame-Options: DENY` and `Referrer-Policy: strict-origin-when-cross-origin`. Synchronized `architecture.md` with implementation reality (Extractors moved to `brom-server`). Formally closed out Phases 3B, 3C, and 3D in `roadmap.md` and marked "Network Edge Instrumentation" in `todos.md`.
> * **New Constraints:** Security headers are now distributed across three separate Tower layers in `middleware.rs` for modularity. Any new top-level router layers must preserve this defense-in-depth ordering.
> * **Pruned:** Removed the stale "Status: Next" markers from Phase 3 documentation.
> * **Verification:** Full `just verify` (clippy, fmt, test, doc-test, sg scan) passed with exit code 0. Zero-Exit gate satisfied.

> 📝 **Context Update:**
> * **Feature:** Documentation sync for Phase 3B/C/D
> * **Changes:** Synchronized `spec.md` with current `brom-server` contracts (API response envelopes, security middleware, problem details). Applied comprehensive `///` documentation to `brom-server`, `brom-db`, `brom-auth`, and `brom-macros` resolving all `missing_docs` warnings. Corrected `doc-markdown` backtick violations in crates.
> * **New Constraints:** All future public items must include the "What, Why, Arguments, Returns, Errors" doc-comment sections per `doc-rules.md`.
> * **Pruned:** Stale verification hash (`83c78be`) and all Phase 3 documentation warnings.

> 📝 **Context Update:**
> * **Feature:** Verification Gate Remediation (Clippy Doc Markdown)
> * **Changes:** Resolved `clippy::doc_markdown` failures in `brom-macros` and `brom-db` by wrapping technical terms (`SQLite`, `OpenAPI`, `Axum`) in backticks within module-level doc comments.
> * **New Constraints:** None. Adheres to established `Architecture.md §11` standards.
> * **Pruned:** The blocking lint warnings that were preventing zero-exit on `just verify`.
> * **Verification:** Full `just verify` (clippy, fmt, test, doc-test, sg scan) passed workspace-wide. Zero-Exit gate restored.


> 📝 **Context Update:**
> * **Feature:** Phase 4 Planning (Schema Diffing)
> * **Changes:** Finalized the implementation plan for `brom diff`. Resolved a major architectural contradiction in `architecture.md` regarding modern SQLite native `ALTER TABLE DROP COLUMN` support (available since 3.45 via `rusqlite 0.32`). Established the `.brom-schema.json` strategy for CLI schema ingestion and implemented topological sorting requirements for handling foreign key dependencies.
> * **New Constraints:** The `brom diff` command will read expected schema metadata from `.brom-schema.json`. Migration files MUST include `-- DOWN` sections for rollback support.
> * **Pruned:** Removed stale Phase 4 "STUB" assumptions from the implementation roadmap.
> 
> 📝 **Context Update:**
> * **Feature:** Phase 4 Tech Debt Tracking (Migrations)
> * **Changes:** Formally documented SQLite rollback limitations for `DropColumn` and `DropTable` operations. These operations currently generate SQL comment placeholders in `-- DOWN` migrations, requiring manual intervention for data preservation during rollbacks.
> * **Technical Debt:**
>   - `diff.rs:247` (`DropColumn`): Rollback is non-trivial in SQLite without full table recreation patterns; currently requires manual SQL authoring.
>   - `diff.rs:254` (`DropTable`): Data-preserving rollback is impossible without historical schema snapshots or shadow tables; currently deferred.
> * **Status:** Tracked as accepted Phase 4 tech debt.
> 
> 📝 **Context Update:**
> * **Feature:** Phase 4 Documentation Sync & Completion
> * **Changes:** Applied comprehensive `///` documentation to `brom-db/src/introspect.rs` and `brom-cli/src/diff.rs`, resolving all `missing_docs` and `clippy::doc_markdown` warnings. Updated `spec.md` with behavioral contracts for database introspection and schema diffing (topological sorting, destructive rollback restrictions). Finalized Phase 4 with a workspace commitment.
> * **New Constraints:** (None - adhered to existing `doc-rules.md`)
> * **Verification:** Clean audit. Zero-exit gate (fmt, clippy, test, doc-test, sg scan) passed. Commit: `146a759`.
> * **Status:** Phase 4 Complete. Ready for Phase 5.
> 📝 **Context Update:**
> * **Feature:** Structured Error Propagation Hardening
> * **Changes:** Finalized the mapping of SQLite `UNIQUE` constraint violations to `brom_core::Error::UniqueViolation` in `brom-db`, using dynamic message parsing to extract entity and field names. Updated `Error::UniqueViolation` to use `String` for dynamic table context. Integrated HTTP 409 Conflict mapping in `brom-server` for these violations.
> * **New Constraints:** The `entity` field in `UniqueViolation` is now a `String`. All server error tests must use `.into()` for string fields.
> * **Verification:** Full `just verify` (clippy, fmt, test, doc-test, sg scan) passed with exit code 0. Zero-Exit gate satisfied.

📝 **Context Update:**
* **Feature:** Error Hardening & Pagination Remediation
* **Changes:** Remediated a zero-cost abstraction deviation by reverting `UniqueViolation.entity` to `&'static str`, eliminating unnecessary heap allocations during constraint violation mapping. Implemented pagination hardening in `brom-core` with a `MAX_PER_PAGE` (100) bound and a new `Pagination::new()` constructor that saturates bounds (page >= 1, 1 <= per_page <= 100).
* **New Constraints:** `Pagination` structs must now be initialized via `Pagination::new()` to ensure security bounds are enforced at the type level. `MAX_PER_PAGE` is the project-wide constant for `DoS` prevention.
* **Verification:** Clean `just verify` (fmt, clippy, test, doc-test, sg scan) passed 100% on Windows. Zero-Exit gate satisfied.

📝 **Context Update:**
* **Feature:** SchemaRegistry Deduplication Guard
* **Changes:** Implemented an idempotency guard in `SchemaRegistry::register()` using `any()` check on `table_name`. Added TDD test `register_duplicate_is_idempotent()`. Added `tracing` dependency to `brom-core` to enable structured warnings for duplicate registration attempts. Documented Findings 4 (deep clones) and 5 (sync repository) as accepted technical debt with specific re-evaluation triggers.
* **New Constraints:** `brom-core` now depends on `tracing`. Duplicate schema registrations will be silently ignored but logged at `WARN` level.
* **Verification:** Full `just verify` (fmt, clippy, test, doc-test, sg scan) passed with zero-exit. New TDD test confirms idempotency.
📝 **Context Update:**
* **Feature:** brom-core Design & Security Remediation
* **Changes:** Completed the `brom-core` architectural audit remediation. (1) Implemented `ManyToMany::new()` and `Default` for metadata markers. (2) Hardened `validate_sql_identifier` with a 35-word SQLite reserved keyword blocklist. (3) Refactored `MAX_PER_PAGE` from a global `const` to an associated constant on `Pagination`. (4) Replaced wildcard re-exports in `lib.rs` with explicit named exports. (5) Formally registered high-impact technical debt (SchemaRegistry cloning, field allocations, String-based field names) in `roadmap.md`.
* **New Constraints:** SQL identifiers are now validated against a reserved word blocklist; custom entities must avoid using SQL keywords like `select` or `table` as table names. `MAX_PER_PAGE` must be accessed via `Pagination::MAX_PER_PAGE`.
* **Verification:** Clean `just verify` (fmt, clippy, test, doc-test, sg scan) passed 100%. Total remediation coverage: 10/10 findings addressed or documented.

> 📝 **Context Update:**
> * **Feature:** Workflow IDE Constraint Lifting & Structural Diagnostics
> * **Changes:** Modernized executing environment constraints in `GEMINI.md` and across 14 workflow files to permit regex special characters within `rg` patterns strings (e.g. `rg "pub (?:struct|enum|trait)\s+[A-Z]"`). Implemented the "Structural Diagnostics Toolkit" in `issue.md`, `audit.md`, `plan-making.md`, and `review.md` to equip the Architect with auto-runnable `rg` tips for diagnosing public API surfaces, error propagation, and destructive operations safely without tool interception.
> * **New Constraints:** Advanced regex searches using `rg` are now fully authorized. However, shell chaining operators (`&&`, `||`, `;`, `>`, `|`) remain strictly prohibited in `run_command` calls.
> * **Pruned:** The outdated workarounds instructing agents to use `HEAD~1..HEAD` or specific Git log checks to avoid the `~` character have been cleaned up and replaced with native Git diff commands.

📝 **Context Update:**
* **Feature:** Observability Sweep & CLI Contract Hardening
* **Changes:** Finalized Phase 4 remediation by implementing a comprehensive observability layer and contract testing suite. (1) Instrumentated critical boundaries in `brom-auth` (password hashing, RBAC evaluation), `brom-cli` (diff engine), and `brom-server` (route handlers) using `tracing::instrument` with selective field skipping for security. (2) Customized `brom-server`'s `TraceLayer` to emit structured spans with rich HTTP metadata (method, latency, status). (3) Integrated `insta` snapshot testing in `brom-cli` to establish a behavioral contract for the SQL generation engine, capturing `CREATE`, `ALTER`, `DROP` migrations for verification. (4) Updated `roadmap.md` to reflect Phase 2A, 2B, and 4 completion, and initiated Phase 5 (Admin SPA).
* **New Constraints:** `brom-cli` tests now depend on `insta`; new snapshots must be reviewed via `cargo insta review`. Structured logging is now the standard for all new `brom-server` routes.
* **Verification:** Full `just verify` (fmt, clippy, test, doc-test, sg scan) passed 100%. Snapshot tests pass with `INSTA_UPDATE=always` baseline. Zero-Exit gate satisfied.

📝 **Context Update:**
* **Feature:** Documentation Reframing of Shell Operator Constraints
* **Changes:** Act phase for the semantic reframing of shell operator constraints (`|`, `>`, `&&`, `||`, `;`) and `pwsh` scripts across the project ecosystem. Addressed factually obsolete warnings about IDE auto-run interception, replacing them with `TARS Execution Discipline` warnings in `GEMINI.md` and `.agent/rules/builder-rules.md`. Removed stale warnings about the `~` character in Git commit hashes.
* **New Constraints:** Despite IDE changes supporting raw shell operators, they remain strictly banned for agent usage. This ensures atomic verification checks, observability, and enforces the usage of native tools instead of obscure pipelines.
* **Verification:** Zero-exit gate validated with `just verify`. Scanned workspaces to ensure 0 remaining instances of "IDE interception" or similar claims.

> 📝 **Context Update:**
> * **Feature:** Phase 5 (Admin SPA) Planning & Spec Alignment
> * **Changes:** Ran `@[/spec]update` to document missing behavioral contracts for the upcoming Phase 5 Admin SPA and core internal Data Models (`User`, `Session`, `ApiKey`, `Migration`) in `spec.md` (Version 1.1.0). Drafted the multi-phase implementation plan and `task.md` for Phase 5, organizing the Leptos CSR setup and `rust-embed` Axum integration. 
> * **Next Milestone:** Awaiting "Proceed" approval to begin the Phase 5 Act cycle.

📝 **Context Update:**
* **Feature:** Phase 4 Architectural Documentation Sync
* **Changes:** Remediated `architecture.md` structural discrepancies identified in the Phase 4 audit. Synchronized the project layout mappings for `brom-cli`, `brom-db`, and `brom-server` by adding missing modules (e.g., `config.rs`, `introspect.rs`, `response.rs`, `state.rs`, `api_keys.rs`) and removing stale, unimplemented references (e.g., `migrate.rs`, `assets.rs`). Removed false `mockall` MockRepository claims from the module boundaries.
* **New Constraints:** The dependencies table explicitly marks `rust-embed`, `leptos`, and `trunk` as Phase 5 architectural targets.
* **Pruned:** The `architecture_recommendations_report.md` action items have been fully resolved.

> 📝 **Context Update:**
> * **Feature:** Phase 5 Context Alignment for README.md
> * **Changes:** Updated `README.md` to accurately flag the compilation of the embedded Admin SPA and JSON schemas as a Phase 5 target. This corrects documentation drift by accurately representing current functional gaps before the Admin UX development cycle.
> * **New Constraints:** Features slated for Phase 5 (Leptos CSR, endpoints, Admin UI) should not be described as "current functionality" in standard documentation until Phase 5 Act begins.
> * **Pruned:** Outdated README presentation of unimplemented capabilities.

> 📝 **Context Update:**
> * **Feature:** IPR Hardening & Anti-Hallucination Measures
> * **Changes:** Refined `.agent/rules/ipr.md` to harden the Global Execution Order against Builder hallucinations. Bound Target names to explicit blocks, formalized `RED`/`GREEN` keywords for structural TDD, defined Component Group boundaries, implemented tiered action specificity heuristics (Control Flow Override), and mandated `expects:` mechanical verification statements in Post conditions. 
> * **New Constraints:** All Implementation Plans MUST use `RED`/`GREEN` keywords in their Post-conditions, include explicit commands or grep checks via `expects:`, and avoid subjective prose for control flow actions. 
> * **Pruned:** Removed the ambiguity of the "component group" definition and loose prose guidelines in the previous iteration of the IPR.

> 📝 **Context Update:**
> * **Feature:** Verification Gate Remediation (Clippy & Snapshots)
> * **Changes:** Remediated a blocking clippy violation in `brom-db` by removing an unused `std::io::Read` import. Synchronized stale `brom-cli` migration snapshots caused by the `TODO` -> `ACCEPTED-DEBT` terminology change using `INSTA_UPDATE=always`.
> Zero-Exit gate satisfied.

> 📝 **Context Update:**
> * **Feature:** Admin SPA Migration Remediation (Audit Findings)
> * **Changes:** Scoped SPA fallback to `/admin` to resolve 404 hijacking in API and docs. Migrated integration tests in `api_test.rs` to the JSON-based Bearer token authentication contract, removing deprecated `Set-Cookie` assertions. Resolved `clippy::into_iter_on_ref` and `clippy::collapsible_if` violations in the `admin` crate.
> * **New Constraints:** Any authenticated API requests MUST use the `Authorization: Bearer <token>` header. The SPA fallback is strictly scoped to `/admin/*`; non-admin routes will correctly return 404.
> * **Verification:** Full `just verify` (fmt, clippy, test, doc-test, sg scan) passed with exit code 0. Zero-Exit gate fully restored.
> * **Status:** Workspace is clean and verified for Phase 5 development.

> 📝 **Context Update:**
> * **Feature:** IDOR & Auth Performance Remediation
> * **Changes:** Enforced `user_id` ownership scoping in `ApiKeyStore::revoke` and its SQLite implementation. Optimized `RequireAdmin` and `RequireApiKey` extractors to eliminate heap allocations by using `eq_ignore_ascii_case` for Bearer token validation.
> * **New Constraints:** Any API key lifecycle operations MUST be scoped to the authenticated `user_id`. Auth header parsing must prioritize allocation-free comparison.
> * **Verification:** Clean audit. Zero-exit gate satisfied after switching integration tests to Bearer token contract. Fidelity to Plan: 100%.

> 📝 **Context Update:**
> * **Feature:** Phase 5A (Forge Dark Admin SPA) Implementation
> * **Changes:** Successfully implemented the "Forge Dark" design system across the `brom` headless CMS Admin SPA. Reskinned core pages (Login, Collection List, Form Editor) and created new ones (Dashboard, Settings for API Keys). Extracted unified dark-themed components (`Breadcrumbs`, `DataTable`, `StatCard`) to enforce design consistency. Solved complex `location` cloning / ownership issues within Leptos functional layouts to ensure stable navigation state. Replaced standard Tailwind with strict `#111827` base and amber accents aligned with the Forge Dark Stitch spec.
> * **New Constraints:** Shared components MUST be used for standard interface elements (tables, data points) across the Admin SPA.
> * **Verification:** Clean audit. Zero-exit gate (fmt, clippy, test, sg scan) passed workspace-wide. The `wildcard-import` violations in `admin` were resolved by structurally excluding Leptos framework preludes in `.ast-grep/rules/wildcard-import.yml`. Fidelity to Plan: 100%.

> 📝 **Context Update:**
> * **Feature:** IPR Checkpoint Remediation (Cognitive Hardening)
> * **Changes:** Implemented a mandatory JIT IPR Alignment Checkpoint in `.agent/workflows/plan-making.md`. This requires the Architect to use `sequentialthinking` to map the specific Tier (S/M/L) schema before drafting an implementation plan, ensuring strict structural compliance and suppressing generic template drift.
> * **New Constraints:** Implementation plans MUST be preceded by a thought block retrieval of the relevant `ipr.md` schema. The `write_to_file` command for `implementation_plan.md` is now conditionally gated behind this internal validation step.
> * **Pruned:** The "Template Override" phenomenon where generic system prompts interfered with `ipr.md` formatting is remediated.

> 📝 **Context Update:**
> * **Feature:** Narsil Performance Remediation (Indexing Optimization)
> * **Changes:** Updated root `.gitignore` to explicitly exclude `node_modules/`, resolving a performance bottleneck where Narsil would hang while indexing thousands of administrative dependency files. Remediated secondary clippy warnings (`collapsible_str_replace` and `unnecessary_first_then_check`) in the `admin` crate discovered during verification.
> * **New Constraints:** The root `.gitignore` MUST remain configured to ignore build artifacts and dependencies to maintain semantic indexing performance.
> * **Verification:** Full `just verify` (fmt, clippy, test, sg scan) passed with exit code 0. Zero-Exit gate fully restored. TOOLING STATUS: STABLE.

> 📝 **Context Update:**
> * **Feature:** Phase 5B (WASM Navigation & Real API Integration)
> * **Changes:** Completed the high-fidelity implementation of the Admin SPA Phase 5B. (1) Implemented dynamic form widget resolution in `form_editor.rs` (textarea, boolean, and server-side select for links). (2) Wired "Create New" collection navigation via the Leptos `A` component. (3) Replaced mock data in `settings.rs` with real API key management endpoints (List, Create, Revoke) using `LocalResource` and `auth_fetch`. (4) Hardened move/borrow checker safety within async Leptos view closures.
> * **New Constraints:** Any new `FieldType` variants or `ui_widget` hints MUST be mapped within the `form_editor.rs` resolution logic. Asynchronous data fetching in the SPA must use the `auth_fetch` / `LocalResource` pattern to ensure session stability.
> * **Verification:** Full `just verify` (fmt, clippy, test, sg scan) passed with exit code 0. Zero-Exit gate satisfied. Fidelity to Plan: 100%. Phase 5 Officially Closed.

> 📝 **Context Update:**
> * **Feature:** Narsil Call-Graph Integration (Workflow Hardening)
> * **Changes:** Integrated Narsil's structural analysis tools (`get_callers`, `get_callees`, `get_call_graph`, `get_complexity`, `check_cwe_top25`) across core TARS workflows (`ipr.md`, `plan-making.md`, `build.md`, `audit.md`, `issue.md`, `spec.md`). Enforced **Topological Execution Order** for M/L-Tier plans. Implemented Builder-level caller-count guardrails (Drift Guard) and post-implementation call-graph drift detection to ensure high plan fidelity.
> * **New Constraints:** All M/L-Tier implementation plans MUST be topologically ordered. The Builder MUST halt and request re-audit if the caller count of a target function diverges from the approved Blast Radius table during execution.
> * **Pruned:** Legacy, manual, `rg`-based blast-radius guessing is fully deprecated in favor of Narsil's structural MCP analysis.

> 📝 **Context Update:**
> * **Feature:** Phase 6 Security Hardening (CWE-22 Remediation)
> * **Changes:** Remediated high-severity path traversal vulnerabilities in `brom-db::MigrationRunner::run_rollback()` by implementing mandatory `canonicalize()` and `starts_with()` bounds checking. Added regression test `test_rollback_rejects_path_traversal()` in `crates/brom-db/tests/rollback_test.rs`. Applied `// narsil-ignore: CWE-22` annotations to verified secure paths in `run_pending()` and the CLI entry point.
> * **New Constraints:** Any dynamic path construction involving migration versions MUST be canonicalized and validated against the migration directory base path before I/O operations.
> * **Pruned:** The insecure rollback logic and potential traversal vectors are fully neutralized.
> * **Verification:** Clean audit. Security scan `narsil check-cwe-top25` confirmed zero high-severity findings in `brom-db`.
> 
> 📝 **Context Update:**
> * **Feature:** Documentation Sync & Baseline Establishment
> * **Changes:** Formalized documentation ownership via the `/update-doc` workflow. Unified the project overview across `README.md`, `Cargo.toml [description]`, and `crates/brom/src/lib.rs` using primary sentiment sentinels. Removed legacy "Phase 5 target" annotations as the system is now production-aligned.
> * **New Constraints:** `README.md` is now the single source of truth for high-level marketing/overview copy. Secondary sinks (Cargo.toml, lib.rs) MUST be synchronized via the `/update-doc` workflow.
> * **Pruned:** Stale mentions of Phase 5 implementation targets; technical descriptions now reflect production state.
> 📝 **Context Update:**
> * **Feature:** Macro Hardening & Observability (brom-macros)
> * **Changes:** Hardened the `#[derive(BromEntity)]` macro suite. (1) Implemented robust type mapping in `schema.rs` with `syn::Path` segment inspection. (2) Instrumented all generated Axum handlers with `#[tracing::instrument]`. (3) Refactored list handlers to use optional query parameters, resolving 400 Bad Request errors for clients omitting pagination. (4) Centralized macro dependencies via re-exports in `brom-server`. (5) Synchronized structural snapshots across the macro test surface.
> * **New Constraints:** Any new macro-generated handlers MUST be instrumented via the centralized `tracing` re-export in `brom-server`. Optional fields in macro-generated structs must use the `Option<T>` pattern to ensure deserialization stability.
> * **Verification:** Full `just verify` (fmt, clippy, test, doc-test, trybuild, sg scan) passed with exit code 0. Zero-Exit gate satisfied. Fidelity to Plan: 100%.

📝 **Context Update:**
* **Feature:** Phase 6 Security Hardening (RBAC Remediation)
* **Changes:** Remediated a critical authentication bypass in `brom-macros` by injecting the `RequireAdmin` extractor into all generated admin route handlers (`list`, `get`, `create`, `update`, `delete`). Fixed the Axum extractor ordering to ensure the `Json` body extractor is always positioned as the final argument. Added a dedicated regression test `test_auth_policy_admin_only_access` to `brom-server`.
* **New Constraints:** Any additions to the `admin_module` expansion in `brom-macros/src/routes.rs` MUST include the `RequireAdmin` guard. The `RequireAdmin` guard MUST be positioned before any `FromRequest` (body-consuming) extractors in the handler signature.
* **Pruned:** The insecure publicly-exposed admin CRUD endpoints are now fully protected.
* **Verification:** Clean audit. Zero-exit gate (fmt, clippy, test, trybuild, sg scan) passed workspace-wide. Regression test successfully validates `401 Unauthorized` for anonymous admin access. Fidelity to Plan: 100%.

> 📝 **Context Update:**
> * **Feature:** Phase 6 Security Scanner Baseline
> * **Changes:** Hardened the security posture by systematically suppressing verified false-positive findings via `// narsil-ignore` markers. Treated dynamic SQL formatting in `repository.rs` (CWE-89) guarded by `validate_sql_identifier()`, isolated hardcoded test credentials in `password.rs` (CWE-798), and logically-safe unwraps in `entity.rs` and tests (RUST-002). Added checklist updates to `phase6.md`.
> * **New Constraints:** Any new findings from `sg scan` must be resolved by refactoring, unless definitively proven safe through logical bounds, in which case a strictly scoped `// narsil-ignore: <RULEID>` boundary is permitted.
> * **Pruned:** Narsil scanner noise from safe/tested contexts.
> * **Verification:** Clean audit. Zero-exit gate (`just verify`) passed workspace-wide. `sg scan` outputs zero high-severity findings.

> 📝 **Context Update:**
> * **Feature:** Phase 6 Behavioral Specification Restitution
> * **Changes:** Formalized API tables and BDD scenarios across `brom-core`, `brom-db`, `brom-auth`, `brom-server`, and `brom-cli` modules in `spec.md`. Aligned behavioral contracts with Phase 3/4 implementations, addressing data model relationships, SQLite DB introspection, API Key generation workflows, Axum REST Extractor behaviors, and differential CLI operations.
> * **New Constraints:** Any modifications to public API signatures or behaviors in these crates MUST be systematically updated in `spec.md`.
> * **Verification:** Clean audit. Zero-exit gate (`cargo check`) passed workspace-wide. Fidelity to Plan: 100%.

> ?? **Context Update:**
> * **Feature:** Phase 6 Architecture Documentation Sync
> * **Changes:** Remediated minor discrepancies found during the rchitecture.md audit. Added Section 16 for Environment Configuration, appended dmin_ui.rs to the Project Layout, and updated Mock Availability for Repository<T> to reflect mockall usage.
> * **New Constraints:** None.
> * **Verification:** Checked document formatting. Zero-exit gate satisfied.
