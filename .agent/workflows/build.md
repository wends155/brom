---
description: Formalized Act phase — execute an approved implementation plan (Act Phase)
---

# /build — Act Phase Workflow

Execute an approved implementation plan with full execution discipline,
scope fencing, and self-verification.

> [!IMPORTANT]
> This workflow is the **Builder's operating procedure**. It is loaded
> automatically when the user says "Proceed" after plan approval.

## Trigger

- `/build` — explicit invocation
- "Proceed" / "Implement" / "Execute" — after plan approval in `/plan-making`

## Prerequisites

> [!IMPORTANT]
> **Execution Discipline:** You **MUST** use the `view_file` tool to read all listed rule files (e.g., `.agent/rules/...`) before starting Step 1. Do not rely on internal memory.

> [!TIP]
> These are loaded in sequence. Do not skip any.

1. Read `.agent/rules/builder-rules.md` for execution discipline rules.
2. Read `.agent/rules/coding-standard.md` for language-specific coding standards.
3. Read the approved plan's **Builder Context** section — files and line ranges to read before starting.
4. Read the approved plan's **Negative Scope** section — files and areas NOT to touch.
5. Read `architecture.md` (if present) for project-specific design and toolchain commands.
6. If this is a multi-phase plan, read the prior phase's Phase Manifest (Deferred table in `context.md` or the prior plan). Verify that any `STUB(Phase N)` items where N = current phase are addressed as plan steps.
7. Confirm you are operating as the **Builder** (fast/efficient model).

## Steps

### 1. Pre-Flight

Verify the environment is ready:

Verify task.md is aligned with the plan (Agent Procedure):

1. Read both `task.md` and the plan file with `view_file`.
2. Check that every `[NEW|MODIFY|DELETE|TEST] filename` in the plan appears in `task.md`.
3. Check that every such entry in `task.md` appears in the plan.
4. If mismatches → report them and STOP. Fix alignment before starting.

> [!WARNING]
> with a misaligned task.md.

**Capture Test Baseline (Regression Guard):**
Before writing any code, run the project's test suite to capture the baseline test count (e.g., `cargo test` and note the total).
Record this baseline in `task.md` or a local scratchpad.

### 2. Git Checkpoint

Ensure a clean working tree:

// turbo
```
git status --short
```

If the tree is dirty, commit or stash before starting. The first checkpoint
establishes the "before" state for the audit's `git diff`.

### 3. Step Execution Loop

For each step in the plan's Global Execution Order, follow the protocol
from `builder-rules.md §2`:

```
For each Step N:
  1. PARSE  — Extract file tag, file, function sub-tag (if present), function, line range from step header
  2. READ   — Read the target file/function. Verify Pre-condition holds.
  3. CODE   — Execute the Action. For [TEST] steps, write test first (TDD Red).
  4. VERIFY — Re-read changed code. Run Post check. Confirm match to plan.
  5. UPDATE — Mark task.md: [ ] → [/] → [x]
  6. REPEAT — Next step
```

**When replacing a STUB:** Verify the new implementation satisfies the original contract
comment from the `// STUB(Phase N)` marker. The replacement must pass all existing tests
that exercised the stub.

#### MCP-Enhanced Implementation *(when available)*

During the Step Execution Loop, the Builder **SHOULD** use Narsil call-graph
tools for precision verification:

| Phase | Tool | Purpose |
|-------|------|---------|
| **READ** | `get_callers` | Verify the plan's stated caller count still holds. If new callers appeared since plan approval, STOP — the blast radius changed. |
| **CODE** | `get_callees` | When modifying a function, verify downstream contracts aren't violated by the change. |
| **TEST** (TDD) | `get_callees` | Ensure test cases cover all downstream call paths of the target function. |
| **VERIFY** | `get_complexity` | Confirm the modified function's complexity hasn't exceeded project thresholds. |

**At 🔒 CHECKPOINT markers:**

1. *(Optional)* Run the **Validate task.md** procedure from Step 1 to confirm alignment.
2. Stage and commit (separate commands — no chaining):

   ```
   git add <changed-files>
   ```
   ```
   git commit -m "step N-M: <description>"
   ```

> [!CAUTION]
> Do NOT skip checkpoints. They create atomic commits for the audit trail.
> All verification (fmt + clippy + test) must pass before committing.
> **BANNED OPERATORS:** Never chain git commands with `&&`. You MUST run `git add` and `git commit` as separate `run_command` tool calls to prevent IDE auto-run blocking.

### 4. Final Verification

After all steps are complete, run the full verification pipeline one last time:

```
FMT + LINT + TEST
```

Use the exact commands from `architecture.md § Toolchain`. Confirm zero-exit on all gates.

**Test Regression Guard Verification:**
Compare the final test count from the verification logs against the baseline test count captured in Pre-Flight.
- The final count MUST be ≥ the baseline count.
- If the count decreased, verify the plan explicitly authorized the removal in a **Test Removal Justification** section.
- If tests were removed without authorization, **STOP**, revert the removal, and escalate to the Architect. Do not complete the Act Phase.

> [!CAUTION]
> **BANNED OPERATORS:** Do NOT chain these checks together with `&&` or `;`. You MUST run `fmt`, `clippy`, and `test` sequentially as *separate* `run_command` tool calls.

> [!NOTE]
> If `sgconfig.yml` exists in the project root, also run `sg scan` as Gate 4 (AST Linting) and confirm zero findings.

### 5. Builder Notes Review

If you logged any observations or suggestions during execution, verify
the Builder Notes section exists in `task.md` (per `builder-rules.md §7`):

```markdown
## Builder Notes
- 💡 Step N: [suggestion]
- ⚠️ Step M: [observation]
```

These will be reviewed by the Architect during `/audit`.

### 6. Completion

End the build with:

> ✅ **Act Phase Complete.** Reply with `/audit` for Reflect.

Do NOT proceed to audit yourself — the Architect role handles Reflect.

## Rules

1. **Follow `builder-rules.md`** — it defines scope discipline, fidelity hierarchy, and STOP conditions.
2. **TDD is non-negotiable** — tests first, even for minor changes. Every code change needs a test.
3. **Scope fence** — touch only files/functions listed in the current step. Minor additions per §4 are pre-approved.
4. **STOP on ambiguity** — if a step requires a design decision, halt and escalate to the Architect.
5. **No creative additions** — if you notice an improvement, write a Builder Note, don't write code.
6. **Update task.md** — Antigravity reads this for UI progress. Stale markers hide progress from the user.
7. **Git checkpoint at 🔒** — every checkpoint is a commit. No skipping.
8. **Wait for user instruction** before pushing to remote repositories.
9. **Command Execution Constraints** — NEVER use shell chaining (`&&`, `||`, `;`), redirects (`>`, `2>&1`), or shell pipes (`cmd1 | cmd2`) in `run_command` calls. Regex special characters inside `rg` pattern strings (e.g., `rg "pub (struct|enum)"`) are permitted. One standalone command per `run_command` call. See GEMINI.md §6.
