# Lint Suppression Inventory — brom

This document provides a formal inventory of all compiler and lint suppressions (`#[allow(...)]`) currently in effect across the `brom` workspace. These suppressions were added to unblock the zero-exit verification gate while maintaining strict global enforcement elsewhere.

## 📋 Status Summary

| Metric | Value |
|--------|-------|
| **Total Suppressions** | 13 Locations |
| **Strictness Policy** | `-D warnings` (Clippy Warnings = Errors) |
| **Last Audit** | 2026-04-05 |

---

## 🔍 Detailed Inventory

### 1. Macro Expansion Infrastructure
These suppressions are localized to procedural macro implementation where complexity or expansion artifacts trigger non-idiomatic patterns.

| File | Lint | Justification |
|------|------|---------------|
| `crates/brom-macros/src/routes.rs` | `clippy::too_many_lines` | The `expand_routes` function generates complex Axum handler logic. Splitting the expansion logic would increase fragmentation and reduce maintainability of the generated router structure. |
| `crates/brom-macros/src/entity.rs` | `clippy::single_match_else` | Used in attribute parsing (syn 2.0). Matching on optional attributes often results in more readable code than nested combinators when generating compile-time errors. |
| `crates/brom-server/src/openapi.rs` | `clippy::needless_for_each` | Artifact of the `utoipa` macro expansion and local configuration. Added to unblock the gate where external macro generation conflicts with local strictness. |

### 2. Test Infrastructure (Internal)
Multiple suppressions are used within `#[cfg(test)]` modules or dedicated test crates. The justification is to allow fail-fast assertions (`unwrap()`, `expect()`) in test code while strictly prohibiting them in production.

| File | Lint | Context | Justification |
|------|------|---------|---------------|
| `crates/brom-server/src/extractor.rs` | `clippy::unwrap_used` | Unit Tests | Permit clean setup in mock request assertions. |
| `crates/brom-db/src/api_key_store.rs` | `clippy::unwrap_used`, `clippy::expect_used` | Unit Tests | Permit fail-fast database setup in in-memory tests. |
| `crates/brom-db/src/session_store.rs` | `clippy::unwrap_used`, `clippy::expect_used` | Unit Tests | Permit fail-fast session lifecycle assertions. |
| `crates/brom-db/tests/pool_test.rs` | `clippy::expect_used` | Integration | Module-level suppression for setup/teardown boilerplate reduction. |
| `crates/brom-db/tests/migration_test.rs` | `clippy::expect_used` | Integration | Module-level suppression for migration state verification. |
| `crates/brom-db/src/tests/repository_test.rs` | `clippy::expect_used` | Unit Tests | Module-level suppression for manual mock schema setup. |
| `crates/brom-auth/src/password.rs` | `clippy::expect_used` | Unit Tests | Allow setup failures to panic early during hashing tests. |
| `crates/brom-core/src/entity.rs` | `clippy::unwrap_used` | Unit Tests | Used in schema validation tests where panic on setup failure is desired. |

---

## 🛡️ Governance

- **New Suppressions**: Must be documented in this inventory and justified by the Architect during the `/plan-making` phase.
- **Review Cycle**: This inventory should be audited at every major Phase milestone to assess if suppressions can be removed through refactoring.
- **Negative Scope**: Do NOT globally disable any of these lints in `Cargo.toml`. They must remain localized to specific items.
