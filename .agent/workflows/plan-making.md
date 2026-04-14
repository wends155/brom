---
description: How to create a high-quality implementation plan (Think Phase)
---

# Plan-Making Workflow

This workflow defines the standard process for creating implementation plans.
It enforces the Planning Gate and Think Phase of the TARS protocol.

## Prerequisites

> [!IMPORTANT]
> **Execution Discipline:** You **MUST** use the `view_file` tool to read all listed rule files (e.g., `.agent/rules/...`) before starting Step 1. Do not rely on internal memory.

> [!TIP]
> Load context using native agent tools (zero-prompt):
> 1. Read `architecture.md`, `context.md`, `.agent/rules/coding-standard.md`, `.agent/rules/ipr.md`, and `.agent/rules/phase-rules.md` (if multi-phase) with `view_file` (if they exist).
> 2. Run these auto-runnable commands:
// turbo
>    - `git log -n 20 --oneline`
// turbo
>    - `rg -n -e "TODO" -e "FIXME" -e "HACK" --glob "*.rs" --glob "*.go" --glob "*.ts" --glob "*.js" --glob "*.svelte" --glob "*.py" .`
>
> If a Report was produced by `/issue`, `/audit`, or `/feature`, use it as the **primary input**. Confirm you are operating in **Planning mode** (no code edits allowed).

## Phases

### Phase 1: Scope & Impact Analysis 

Investigate the request before writing anything:
- **Identify affected files**: List every file/module that will be touched.
- **Map dependencies**: What depends on those files? What do they depend on?
- **Flag risks**: Security concerns, breaking changes, performance impacts.
- **Check for existing tests**: Search for test files related to the affected code.

> [!TIP]
> **Structural scanning** for blast radius estimation:
// turbo
> - `rg "pub (?:struct|enum|trait|type)\s+[A-Z]" <affected-path>` — map public types in scope
// turbo
> - `rg "use .*<crate>::" <project-root>` — find cross-crate import consumers
// turbo
> - `rg "^impl.*for" <affected-path>` — locate trait implementations that may need updating

**Assess phase scope**: Determine if this is a single-phase or multi-phase plan. Multi-phase indicators: scope touches >3 modules, requires infrastructure setup before features, or depends on stubs from prior phases. If multi-phase, load `phase-rules.md` and include Phase Context / Phase Manifest in the plan.

#### Pre-Draft Review Lenses
Before drafting, apply selected `/review` lenses on the **existing code**. Outcomes should be baked into the plan:
- 🏗️ **Design:** (M/L tier) Find coupling risks → Document in the plan's Edge Cases & Risks.
- 📐 **API:** (When modifying public interfaces) Find breaking changes → Trigger Deprecation Protocol (`ipr.md`).
- 🔒 **Security:** (When modifying taint-carrying code) Mandate sanitization in the plan's proposed changes.

#### MCP-Enhanced Analysis *(when available)*
If **Narsil MCP** is available, actively use it throughout planning. Primary use cases include:
- **Blast Radius & Contracts (`get_callers`, `get_callees`, `get_call_graph`):** Trace upstream/downstream impact to populate Blast Radius table and ensure strict execution ordering without breaking downstream consumers.
- **Security & Complexity (`check_dependencies`, `check_owasp_top10`, `get_complexity`):** Find risks early and flag highly complex targets for potential refactor before feature integration.
- **Interfaces (`find_symbols`, `get_symbol_definition`):** Understand signatures prior to modifying them and find existing usage patterns (`find_similar_code`).

### Phase 2: Draft the Plan

For **M/L tier** plans, the Architect **MUST** use `sequentialthinking` to reason through root cause validation, change ordering, blast radius analysis (populating the Blast Radius Table), and interface contract risks before drafting.

> [!IMPORTANT]
> Follow `.agent/rules/ipr.md` strictly — do NOT use default system templates.

Write the implementation plan to `<artifacts>/implementation_plan.md` using the `write_to_file` tool (`IsArtifact: true`). Follow the plan format, revision protocol, and handoff rules defined in `.agent/rules/ipr.md`.

> [!NOTE]
> Once the artifact is written, you **MUST** provide a clickable markdown link to it in your final chat response (e.g., `[Implementation Plan](file:///absolute/path/to/implementation_plan.md)`).

### Phase 3: Sync task.md (Agent Procedure)

Generate `task.md` from the plan:

1. Read the plan file with `view_file`.
2. Extract all headings matching `### ComponentName` and `#### [NEW|MODIFY|DELETE|TEST] filename`.
3. Write `task.md` to the same directory as the plan:

   ```markdown
   # Task: <plan title from first # heading>

   ## Objectives
   - [ ] <Component 1>
     - [ ] [ACTION] <filename>
   - [ ] <Component 2>
     - [ ] [ACTION] <filename>
   - [ ] Run verification pipeline
   - [ ] Update docs
   - [ ] Update context.md
   - [ ] Commit
   ```

> [!WARNING]
> task.md must be aligned with the plan before requesting approval.

### Phase 4: Pre-Flight Gate (Agent Procedure)

Before requesting approval, run the **Validate task.md** inline procedure:
1. Read both `task.md` and the plan file with `view_file`.
2. Check that every `[NEW|MODIFY|DELETE|TEST] filename` in the plan appears 1:1 in `task.md` and vice-versa. 
3. Verify `task.md` contains checklist markers (`[ ]`, `[x]`, `[/]`).

Verify the 6 Critical Fail-Paths (M/L criteria marked with 🧠):
- [ ] No code was edited (Planning Gate enforced).
- [ ] All touched files are listed in the plan (check upstream/downstream blast radius).
- [ ] 🧠 Deprecation Protocol invoked if cross-package blast radius exists.
- [ ] Test cases specified *before* implementation steps (TDD ordering).
- [ ] `task.md` matches the drafted plan 1:1.
- [ ] 🧠 Global Execution Order is topologically sorted (`ipr.md`).

> [!CAUTION]
> The pre-flight gate MUST pass before requesting approval. If it fails, fix the issues and re-check.

> [!NOTE]
> **Post-Approval Handoff:** Once approved, follow **GEMINI.md §6 Handoff Protocol** for the full Act cycle. After `/audit` passes, run `/update-doc` scoped to affected files, then summarize in `context.md`.

End the plan with:
> 🛑 **Think Phase Complete.** Reply with **"Proceed"** to Act.

Do NOT proceed to implementation until the user explicitly approves.

## Rules


