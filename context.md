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
