# Task List — Authentication Security Remediation

Execute the approved implementation plan to remediate 6 security findings.

## Baseline
- **Baseline Test Count:** 79 (Captured 2026-04-13)
- **Zero-Exit Gate:** `just verify`

## Global Execution Order

### Component A: Permission Type Safety (F5)
- [/] Step 1: [TEST] `crates/brom-auth/src/api_key.rs` — [+] `test_permission_display_and_parse()`
- [ ] Step 2: [MODIFY] `crates/brom-auth/src/api_key.rs` — [+] `Permission` enum + [~] `ApiKeyRecord`
- [ ] Step 3: [MODIFY] `crates/brom-db/src/api_key_store.rs` — [~] `create()`, [~] `validate()`
- [ ] Step 4: [MODIFY] `crates/brom-server/src/api_keys.rs` — [~] `CreateApiKeyRequest`, [~] `ApiKeyRecordDto`, [~] `create_key()`
- [ ] Step 5: [TEST] `crates/brom-db/src/api_key_store.rs` — [~] `test_api_key_lifecycle()`
- [ ] Step 6: [MODIFY] `crates/brom-server/src/extractor.rs` — [~] `test_require_api_key_valid()`
- [ ] 🔒 **CHECKPOINT A** (Commit: "auth: implementation of type-safe permissions")

### Component B: Session Token Hashing (F2) + Mass Invalidation (F4)
- [ ] Step 7: [TEST] `crates/brom-db/src/session_store.rs` — [+] `test_session_token_is_hashed_in_db()`
- [ ] Step 8: [MODIFY] `crates/brom-auth/src/session.rs` — [+] `destroy_all_for_user()`
- [ ] Step 9: [MODIFY] `crates/brom-db/src/session_store.rs` — [~] `create()`, [~] `validate()`, [~] `destroy()`, [+] `destroy_all_for_user()`
- [ ] 🔒 **CHECKPOINT B** (Commit: "auth: secure session hashing and mass invalidation")

### Component C: Cookie-Based Admin Auth (F1 + F6)
- [ ] Step 10: [MODIFY] `crates/brom-server/Cargo.toml` — [+] `axum-extra` dependency
- [ ] Step 11: [TEST] `crates/brom-server/src/extractor.rs` — [+] `test_require_admin_cookie()`
- [ ] Step 12: [MODIFY] `crates/brom-server/src/extractor.rs` — [~] `RequireAdmin`
- [ ] Step 13: [MODIFY] `crates/brom-server/src/router.rs` — [~] `login()`, [~] `logout()`, [~] `LoginResponse`
- [ ] Step 14: [MODIFY] `crates/brom-server/src/extractor.rs` — [~] `test_require_admin_valid_session()`
- [ ] Step 15: [MODIFY] `admin/src/auth.rs` — [-] `save_token_to_storage()`, [-] `get_token_from_storage()`, [-] `AuthContext`, [~] `auth_fetch()`
- [ ] Step 16: [MODIFY] `admin/src/pages/login.rs` — [~] login handler
- [ ] 🔒 **CHECKPOINT C** (Commit: "auth: migrate admin sessions to HttpOnly cookies")

### Component D: Decouple last_used_at (F3)
- [ ] Step 17: [TEST] `crates/brom-db/src/api_key_store.rs` — [~] `test_api_key_lifecycle()`
- [ ] Step 18: [MODIFY] `crates/brom-db/src/api_key_store.rs` — [~] `validate()`
- [ ] 🔒 **CHECKPOINT D** (Commit: "auth: decouple sync metadata updates from hot paths")

## Builder Notes
- (Log observations here)
