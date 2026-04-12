# Implementation Plan Rules (IPR)

> Loaded by `/plan-making` workflow. Defines plan format, revision protocol, and handoff rules.

## 1. The Planning Gate

- **Trigger:** "Plan", "Draft", "Propose", "Design" → Agent is Language-Locked (Markdown only).
- **Prohibited:** Editing source code or core documentation (except to read).
- **Allowed:** Creating/Editing implementation plans in an artifacts directory.
- **Exit:** Agent **MUST** pause and request user approval. Unlock only after "Proceed"/"Approved".
- **Output:** Always an artifact, never code changes.

## 2. Plan Format

### Scaling Tiers

| Tier | Files | Required Sections |
|------|-------|-------------------|
| **S** (patch) | 1-3 | Header, Problem Statement, Global Execution Order, Verification Plan, Plan Summary |
| **M** (feature) | 4-10 | + Builder Context, Interface Contracts, Blast Radius Table, Deprecation Schedule *(if triggered)*, Test Plan, Negative Scope, Phase Context *(if multi-phase)* |
| **L** (refactor) | 10+ | + Module Boundaries, Cross-Module Handshakes, Architecture Diagram, Dependency Chain, Blast Radius Table, Deprecation Schedule *(if triggered)*, Phase Manifest *(if multi-phase)* |

### Header

| Field | Value |
|-------|-------|
| **Role** | Architect / Builder |
| **Date** | Current date |
| **Scope** | One-line summary of what changes |
| **Tier** | S / M / L |

> All code produced must comply with `.agent/rules/coding-standard.md`.

### Builder Context *(M/L tier)*

List exact files and line ranges the Builder must read before starting:

```markdown
## Builder Context
Read before starting:
- `src/lib.rs` L1-30 (module structure)
- `src/error.rs` (current error types)
- `architecture.md § Error Handling` (project convention)
```

### Phase Context *(M/L tier, multi-phase only)*

For plans that are part of a multi-phase project (see `phase-rules.md`):

```markdown
## Phase Context
- **Phase:** 2 of 5 (Core Feature)
- **Prior phase:** Phase 1 delivered foundation (config, errors, DB, tracing)
- **Stubs for this phase:** STUB(Phase 2) items from prior Phase Manifest:
  - `MockPaymentGateway` in src/infra/payment.rs → replace with Stripe integration
  - `NoOpNotifier` in src/infra/notify.rs → replace with webhook dispatcher
- **Reference:** See `phase-rules.md` for full conventions
```

> [!NOTE]
> Phase Context is only required for multi-phase plans. Single-phase plans omit this section.
> The Architect determines if a plan is multi-phase during scope analysis in `/plan-making`.

### Problem Statement

- What is the problem or feature request?
- Why does it need to be solved now?
- Any relevant context from `context.md` or prior conversations.
- **Constraints**: Technical limitations, environment restrictions, performance budgets, or scope boundaries.
- **Dependencies**: What existing libraries, crates, or packages can be leveraged? Check the ecosystem before proposing custom solutions.

> [!NOTE]
> If an input report exists from `/issue`, `/audit`, or `/feature`,
> cross-reference each proposed change against the report's findings.
> Every finding should map to a proposed change.

### Negative Scope *(M/L tier)*

Explicitly list what the Builder must NOT touch:

```markdown
## Out of Scope
- Do NOT modify `src/config.rs` (separate plan)
- Do NOT add new dependencies
- Do NOT refactor existing tests
```

### Interface Contracts *(M/L tier)*

For every new or changed public function, struct, or trait:
- Exact signature (name, params, return type, error type).
- Invariants (preconditions, postconditions).
- Error conditions (what can fail and what the caller gets back).

### Blast Radius Table *(M/L tier)*

For every public function, struct, trait, or interface being modified, document
the downstream impact:

| Symbol | File | Direct Callers | Indirect Deps | Test-Only | Cross-Package? |
|--------|------|---------------|---------------|-----------|----------------|
| `fn_name()` | `src/mod.rs` | 3 | 1 | 2 | Yes → `package-b` |

> [!TIP]
> Use Narsil `get_callers` (upstream blast radius), `get_callees` (downstream
> dependencies), and `get_call_graph` (full structural view) to populate this
> table when available. Fall back to `find_references` or `rg` when call-graph
> is unavailable.

If any row has `Cross-Package? = Yes`, the Deprecation Protocol applies (see
Deprecation Schedule section).

> [!NOTE]
> Adapt column names to your language ecosystem (e.g., "Cross-Crate?" for Rust,
> "Cross-Module?" for Go, "Cross-Package?" for npm/TypeScript).

### Deprecation Schedule *(when Deprecation Protocol triggers)*

Required when the Blast Radius Table shows cross-package impact, the change
affects a published package, or >5 call sites are affected (even internally).

| Old Symbol | New Symbol | Introduced | Removal Target | Migration |
|-----------|-----------|-----------|---------------|-----------|
| `old_fn()` | `new_fn()` | v0.5.0 | v0.7.0 | See doc comment |

**Deprecation Protocol Rules:**

1. Create the new function with the improved signature.
2. Mark the old function with the language's standard deprecation mechanism
   (e.g., Rust: `#[deprecated(since, note)]`, TypeScript: `@deprecated` JSDoc,
   Go: `// Deprecated:` comment, Python: `warnings.warn(DeprecationWarning)`).
3. Add a migration guide in the new function's doc comment showing before/after.
4. Add `// TODO(deprecation): remove old_fn in v<target>` marker.
5. Update all internal callers in the same PR when feasible.

**Threshold Table:**

| Scenario | Strategy |
|----------|----------|
| Internal-only, ≤5 call sites | Update all callers in same plan |
| Published package or external consumers | Deprecate + migration window |
| >5 call sites (even internal) | Deprecate for incremental migration |
| Interface/trait method with multiple implementations | Always deprecate |

### Module Boundaries *(L tier only)*

For each component group:
- **Owns**: What this module is responsible for.
- **Does NOT own**: What's delegated to other modules.

### Cross-Module Handshakes *(L tier only)*

When a change affects callers/callees across modules:
- Caller → Callee with the exact function/method.
- Data format exchanged (types, ownership).
- Error propagation path across the boundary.

### Global Execution Order *(all tiers)*

> [!IMPORTANT]
> Number ALL steps globally across ALL files. The Builder follows steps 1, 2, 3... linearly.
> No per-file numbering. No jumping.
>
> **Call-Graph-Driven Ordering (M/L tier):** When Narsil call-graph is available,
> use `get_callers`/`get_callees` on each seed function being modified to discover
> the complete modification set. Order steps topologically:
> - **Signature narrowing** (removing params, tightening types): bottom-up
>   (callees first, then callers)
> - **Signature widening** (adding params, new error variants): top-down
>   (callers first, then callees)
> - **New functions**: insert at the dependency layer where they'll be consumed.
>
> Use `find_call_path(A, B)` to verify no indirect paths are missed.
>
> Line ranges in `(L##-##)` are advisory hints for initial orientation. The
> **Target name** (function, struct, module) is the binding anchor. If prior
> steps shifted line numbers, the Builder locates the target by name using
> `grep_search` or Narsil `get_symbol_definition` — not by stale line range.

Each step is verification-oriented:

```markdown
Step N: [NEW/MODIFY/DELETE/TEST] file_path — [+/~/−] function_name() (L##-##)
- Pre: CHECK
- Target: function/struct name + line range
- Action: what to change (code snippet or description)
- Post: CHECK, no anyhow in file
- 🔒 CHECKPOINT (only on steps requiring ALL)
```

**Tags:**
- `[NEW]` — create a new file
- `[MODIFY]` — change existing code
- `[DELETE]` — remove a file or code block
- `[TEST]` — write or update a test (TDD Red step)

**Function sub-tags** *(optional modifier after the `—` separator)*:
- `[+]` — **create** a new symbol (function, struct, trait, enum, impl block)
- `[~]` — **modify** an existing symbol (edit body, signature, or attributes)
- `[-]` — **remove** a symbol from the file

**Applicability:**

| Tier | Sub-Tags | Rationale |
|------|----------|-----------|
| **S** | Optional | 1–3 files, context is obvious from reading the Action body |
| **M** | Recommended | 4–10 files, scanability matters for Builder orientation |
| **L** | Required | 10+ files, function-level intent must be unambiguous |

> [!NOTE]
> When sub-tags are omitted, the step behaves exactly as today — no breaking change.

**Valid combinations:**

| File Tag | Sub-Tag | Meaning |
|----------|---------|---------|
| `[NEW]` | `[+]` | New file, new function (redundant but explicit) |
| `[MODIFY]` | `[+]` | Existing file, **new** function being added |
| `[MODIFY]` | `[~]` | Existing file, **existing** function being changed |
| `[MODIFY]` | `[-]` | Existing file, function being **removed** |
| `[DELETE]` | *(none)* | Entire file removal — no function sub-tag needed |
| `[TEST]` | `[+]` | New test function |
| `[TEST]` | `[~]` | Modifying an existing test |

> [!NOTE]
> Renames use `[~]` with the Action field clarifying the rename mapping
> (e.g., `Action: Rename old_fn() → new_fn(). Update all callers.`).

**Action specificity rules:**

| Step Type | Size | Architect Must Provide |
|-----------|------|----------------------|
| `[NEW]` | ≤ 30 lines | Full code body |
| `[NEW]` | 31–80 lines | All public signatures + core logic paths; internal helpers in prose with structural guidance |
| `[NEW]` | > 80 lines | **Decompose into multiple steps** (each ≤ 30 lines targeting a specific function/block) — *unless* the file is purely mechanical (schema DDL, config, test fixtures), in which case full body is permitted |
| `[TEST]` | Any | Full test body (tests are the contract — never prose) |
| `[MODIFY]` | ≤ 20 changed lines | Code snippet showing the replacement |
| `[MODIFY]` | > 20 changed lines | Prose + **all new/changed function signatures** (see Control Flow Override below) |
| `[DELETE]` | Any | Prose — name the target to remove |

**Sub-tag specificity override** *(when function sub-tags are present)*:

The function sub-tag **overrides** the file-level tag for determining action specificity:

| Step Pattern | Effective Specificity | What Architect Provides |
|-------------|----------------------|------------------------|
| `[MODIFY] — [+] new_fn()` | Follows **`[NEW]`** rules | Full code body (≤30 lines) or signatures + core logic (31–80) |
| `[MODIFY] — [~] old_fn()` | Follows **`[MODIFY]`** rules | Code snippet (≤20 lines) or prose + signatures (>20) |
| `[MODIFY] — [-] dead_fn()` | Follows **`[DELETE]`** rules | Prose — name the target to remove |
| `[TEST] — [+] test_fn()` | Follows **`[TEST]`** rules | Full test body (always) |
| `[TEST] — [~] test_fn()` | Follows **`[TEST]`** rules | Full test body (always) |

**Placeholder convention:** Skeleton code uses `// STUB(Phase N): description`
markers per `phase-rules.md`. Do NOT use `todo!()` or `unimplemented!()`.

**Control Flow Override:**

Whether a large `[MODIFY]` step needs code depends on what kind of change it
makes. **If the change alters the number or meaning of code paths through the
function, include a code snippet regardless of size.**

**Code required** (alters code paths):

| Change Type | Why code is needed |
|-------------|-------------------|
| Adding/removing `match` arms | Pattern matching is precise — arm order and exhaustiveness matter |
| Adding/removing `if`/`else` branches | Branch conditions are design decisions |
| Adding/removing `?` error propagation | Error paths are interface contracts |
| Changing function return type | Signature is a contract — every call site is affected |
| Adding `async`/`await` | Concurrency model is architectural |
| Loop refactoring (changing iteration) | Iteration boundaries are logic, not style |

**Prose sufficient** (structural/mechanical):

| Change Type | Why prose works |
|-------------|----------------|
| Renaming symbols | Find-replace with a rename mapping |
| Moving code between modules | Source path → destination path |
| Adding doc comments | Documentation content in prose |
| Adding `#[derive(...)]` / attributes | List the additions |
| Deleting dead code | Name the targets to remove |
| Adding imports | List the imports |

> Quick test for Architects: **"Does this change alter the number or meaning of
> code paths through the function?"** If yes → include code. If no → prose is fine.

**Pre/Post Vocabulary:**

| Shorthand | Meaning | Default Command |
|-----------|---------|-----------------|
| `CHECK` | Type-check compiles | `cargo check` |
| `FMT` | Format passes | `cargo fmt --check` |
| `CLIPPY` | Lint passes | `cargo clippy -- -D warnings` |
| `TEST` | Tests pass | `cargo test` |
| `BUILD` | Full build | `cargo build` |
| `ALL` | FMT + CLIPPY + TEST | Full pipeline |
| `RED` | Named test compiles but fails | *Per `architecture.md § Toolchain`* |
| `GREEN` | Previously-RED test now passes | *Per `architecture.md § Toolchain`* |

> [!NOTE]
> Default commands shown. Projects override in `architecture.md § Toolchain`.

**`RED` / `GREEN` semantics:**

- `RED(test_name)`: The named test **compiles** but **fails** (exit non-zero).
  The Pre condition (typically `ALL`) already guarantees the rest of the suite
  is green. If the test doesn't compile, that's a bug in the test code —
  Builder fixes it before proceeding.
- `GREEN(test_name)`: The named test **passes** (exit 0). Semantically
  equivalent to `TEST` but communicates the TDD intent — this is the
  Red→Green transition, not a generic test run.

Pre/Post can combine shorthand with conditions: `Post: CHECK, no anyhow imports in file`.

Free-form Post conditions **MUST** include the verification command with an
`expects:` clause in parentheses.

| Notation | Meaning |
|----------|---------|
| `expects: 0 matches` | Search returns no results |
| `expects: ≥1 match` | Search returns at least one result |
| `expects: exit 0` | Command succeeds |
| `expects: exit non-zero` | Command fails (expected failure) |

**Examples:**

✅ `Post: CHECK, no anyhow usage (rg "anyhow" src/config.rs expects: 0 matches)`
✅ `Post: CHECK, uses thiserror (rg "thiserror" src/error.rs expects: ≥1 match)`
❌ `Post: CHECK, no anyhow in file`
❌ `Post: CHECK, error handling looks correct`

**🔒 CHECKPOINT** marks where the Builder runs `ALL` and commits.

**Checkpoint frequency:** At minimum, place `🔒` after each component group and after every `[TEST]` step. For S-tier plans, one `🔒` at the end suffices.

> A **component group** is all contiguous steps targeting the same crate (Rust),
> package (TS/Go), or top-level module. When the plan's execution order crosses
> a crate/package boundary, place a 🔒 CHECKPOINT before entering the next
> boundary.

### Dependency Chain *(L tier)*

Show which steps depend on which:

```
1 → 2 → 3
         ↘
     4 → 5 → 6 🔒
```

### Architecture Diagram *(if applicable, M/L tier)*

Include a Mermaid diagram for any structural or data-flow changes.

### Edge Cases & Risks

List edge cases the implementation must handle. Document risks or trade-offs.

### Test Plan (TDD) *(M/L tier)*

> [!IMPORTANT]
> Plans **must** specify tests **before** implementation code. The Builder writes
> tests first, verifies they fail (Red), then implements until they pass (Green).

For each proposed change, define:

1. **Test cases**: Function signatures and assertions — written as executable code, not prose.
2. **Test type**: Unit, integration, property-based, or doc-test.
3. **Expected failures**: What the test asserts when run *before* implementation.
4. **Test file location**: Co-located `#[cfg(test)]` module or dedicated test file.

**Code snippets as executable tests:** Instead of describing expected output in prose,
express verification as a test assertion. The plan's code should be testable, not illustrative.

**Test Regression Guard:**
The total test count must be monotonically non-decreasing. If a plan requires deleting or disabling tests (e.g., removing a deprecated feature), it MUST include an explicit **Test Removal Justification** section within the Test Plan.
- List the exact tests to be removed.
- Provide the rationale for why they are no longer valid.
Without this explicit authorization, the Builder is forbidden from removing any tests.

### Verification Plan *(all tiers)*

| Type | Required? | Details |
|------|-----------|---------|
| **Automated tests** | Yes | Exact command (e.g., `cargo test`, `npm test`) |
| **Lint / Format** | Yes | Exact command (e.g., `cargo fmt --check`) |
| **Manual testing** | If applicable | Step-by-step instructions |
| **Browser testing** | If applicable | Specific pages/flows |

> [!IMPORTANT]
> Do NOT invent test commands. Refer to `architecture.md § Toolchain`.

### Plan Summary *(all tiers)*

| Metric | Value |
|--------|-------|
| Tier | S / M / L |
| Files | N |
| Steps | N |
| Checkpoints | N |
| Estimated effort | Low / Medium / High |

## 3. Revision Protocol

When revising an approved or in-progress plan:

1. **Targeted edits only** — Use `replace_file_content` or `multi_replace_file_content`.
   Do NOT rewrite the entire plan. Unchanged sections must be preserved verbatim.
2. **Mark revisions** — Tag changed sections with `[REVISED]` so the diff is visible.
3. **No summarization** — Never condense unchanged content. If a section wasn't
   discussed in the revision, do not touch it.
4. **Re-sync task.md** — After any revision, re-run the **Validate task.md** procedure defined in `plan-making.md`.

## 4. Decision Resolution

When a plan presents options and the user decides:

1. **Delete rejected options entirely** — Do not leave them as "rejected" or "not chosen."
2. **State the chosen option as fact** — No "we chose X over Y"; just state X.
3. **Log the rationale** — Record what was decided and why in the plan's Problem Statement
   or in `context.md` so the reasoning isn't lost.
4. **Sweep for strays** — After resolving, use `rg` to catch lingering references:
   ```powershell
   rg "Option A|Option B|Alternative|vs\." <plan-file>
   ```

## 5. Handoff-Ready Requirements

Before the Architect can request "Proceed", the plan must satisfy:

| Requirement | Verification |
|-------------|-------------|
| Every file listed with `[NEW]`/`[MODIFY]`/`[DELETE]`/`[TEST]` tags | Manual review |
| Every change has a discrete, verifiable description | Manual review |
| Test cases pre-specified (TDD: Red → Green → Refactor) | Test Plan section exists |
| Verification commands sourced from `architecture.md § Toolchain` | Cross-reference check |
| Plan Summary filled in | Manual review |
| `task.md` aligned per §6 | `Sync-TaskList.ps1 -Mode validate` exits 0 |

## 6. The task.md Contract

`task.md` is the bridge between the Architect's plan and the Builder's execution:

1. **Generated** by `Sync-TaskList.ps1 -Mode generate` — writes `task.md` to the plan directory automatically.
2. **1:1 Mapping**: Each checklist item maps to exactly one plan item.
3. **Progress Tracking**: Builder marks `[ ]` → `[/]` (in-progress) → `[x]` (done).
4. **Validation Gate**: Before each commit, `Sync-TaskList.ps1 -Mode validate` must exit 0.
5. **Pre-flight**: Before plan approval, `Sync-TaskList.ps1 -Mode preflight` must exit 0.

## 7. Builder Obligations & STOP Conditions

**Obligations:**
1. Execute plan items in order — no deviations.
2. If a plan item is unclear or flawed → **STOP**, request re-audit.
3. Update `task.md` in the artifacts directory after each file modification:
   - Mark `[ ]` → `[/]` when starting a step.
   - Mark `[/]` → `[x]` when the step passes verification.
   - Antigravity reads this file for UI progress — stale markers hide progress from the user.
4. Run `ALL` at each 🔒 CHECKPOINT.
5. Use `Git-Checkpoint.ps1` for atomic commits tied to task.md progress.
6. 🛑 **Wait for user instruction** before pushing to remote repositories.

**STOP Conditions** — Builder must immediately halt and return to the Architect when:
- The plan contradicts `architecture.md`.
- A plan item is ambiguous or untestable.
- An unplanned dependency or breaking change is discovered.
- The second consecutive test failure occurs on the same item.

**On STOP:** Commit current progress with message `WIP: stopped at step N — [reason]`.
Do NOT revert completed steps. The Architect decides rollback scope during re-planning.
If a step broke prior work, note the regression in the STOP report.

## 8. Resumption Protocol

When resuming a plan in a new session:

1. Read `context.md` for prior session summary.
2. Read `task.md` — identify last `[x]` item and first `[ ]` item.
3. Verify partial state: run `ALL` to confirm prior work still passes.
4. Resume from the first `[ ]` step. Do not re-execute `[x]` steps.
5. If `ALL` fails on prior work → STOP, escalate (do not silently fix).

## 9. Example: S-Tier Plan

```markdown
**Role:** Architect · **Date:** 2026-02-26 · **Tier:** S

### Problem Statement
`parse_config()` panics on empty input. Should return `ConfigError::EmptyInput`.

### Global Execution Order

Step 1: [TEST] src/config.rs — test_empty_input()
- Pre: ALL
- Target: #[cfg(test)] mod tests
- Action: Add test asserting parse_config("") returns Err(ConfigError::EmptyInput)
- Post: RED(test_empty_input)

Step 2: [MODIFY] src/config.rs — parse_config() (L12-30)
- Pre: Step 1 test exists
- Target: parse_config() L12
- Action: Add early return: if input.is_empty() { return Err(ConfigError::EmptyInput); }
- Post: GREEN(test_empty_input), ALL 🔒

### Verification Plan
| Type | Command |
|------|---------|
| Tests | cargo test |
| Lint | cargo clippy -- -D warnings |

### Plan Summary
| Metric | Value |
|--------|-------|
| Tier | S |
| Files | 1 |
| Steps | 2 |
| Checkpoints | 1 |
| Estimated effort | Low |
```

> [!TIP]
> For S-tier plans, function sub-tags are **optional**. The example above omits
> them for brevity. An enhanced version would read:
> ```markdown
> Step 1: [TEST] src/config.rs — [+] test_empty_input()
> Step 2: [MODIFY] src/config.rs — [~] parse_config() (L12-30)
> ```
