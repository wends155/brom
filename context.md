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
