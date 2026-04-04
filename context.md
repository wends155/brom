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
> * **Verification:** Clean audit. Zero-exit gate (fmt, clippy, test) passed cleanly after isolating `allow(clippy::unwrap_used)` strictly to `#[cfg(test)]` modules.
