# 🌐 Unified Project Workflow: The TAR-S Cycle

> The operating manual for AI-agent-driven development.
> **Project Truth:** See `architecture.md` for system design and `spec.md` for behavioral contracts.

---

## §1 Philosophy & The TAR-S Protocol

**Rule:** Every task must cycle through four mandatory phases. No phase may be skipped.

**Think** → **Act** → **Reflect** → **Summarize** → *(next task)*

1. **Think (Audit):** Architect analyzes scope, impact, creates Blueprint. Must end with: "🛑 Think Phase Complete. Reply with **Proceed** to Act."
2. **Act (Implementation):** Builder follows Blueprint exactly. Verify pipeline. Self-correct until exit `0`.
3. **Reflect (Review):** Architect validates via Fidelity Matrix. Check verification results + logic.
4. **Summarize:** Compress interaction to `context.md`.

---

## §2 Documentation Ecosystem

| File | Purpose | Ownership |
|:---|:---|:---|
| `GEMINI.md` | **Rules & Workflows** (How we build) | Operational Source of Truth |
| `architecture.md` | **Design & Patterns** (What we are building) | Technical Source of Truth |
| `design/design-spec.md` | **UI/UX Design** (What it looks like) | Design Source of Truth |
| `context.md` | **History & Decisions** (Why we built it) | Contextual Source of Truth |
| `spec.md` | **Behavioral Contracts** (What it should do) | Behavioral Source of Truth |
| `.agents/workflows/` | **TARS Implementation** (How we execute phases) | Workflow Source of Truth |
| `.agents/scripts/` | **Automation** (Companion scripts for workflows) | Automation Source of Truth |
| `.agents/rules/` | **Coding Standards** (Language-specific rules, e.g. `coding-standard.md`) | Standards Source of Truth |
| `.gemini/skills/` | **Agent Skills** (On-demand procedures and language patterns) | Skills Source of Truth |

> [!NOTE]
> At session start, agents **MUST** read all files in `.agents/workflows/` to load workflow definitions.
> Files in `.agents/rules/` are loaded **on demand** by each workflow's prerequisites.
> Skills in `.gemini/skills/` are **auto-discovered** at session start (frontmatter only) and loaded on-demand when invoked by workflows.

> 🛑 **Restricted Access**: Only **the Architect** may edit `GEMINI.md`, `architecture.md`, `context.md`, and `spec.md`. The Builder is **Read-Only**. If a plan contradicts `architecture.md`, STOP and request re-audit.

---

## §3 Model Roles

**Architect** (high-reasoning): Triggers = "Plan/Audit/Analyze/Review/Investigate." Analyzes, plans, reflects, summarizes. Must NEVER execute code in same turn as planning.

**Builder** (fast/efficient): Triggers = "Proceed/Implement/Fix/Write/Execute." Follows plan exactly per `/build` workflow and `builder-rules.md`. Output = production code + verification logs.

**Builder Act Flow:** Approved Plan → Git Checkpoint → Code → Update task.md → Formatter → Linter → Tests → (loop if exit≠0) → Validate task.md → Final Reflection.

**Execution Standards:** Zero-exit required (fmt + lint + test). Commands from `architecture.md § Toolchain`. Git checkpoints before/after functional blocks.

**Subagent Delegation (v2):** When running in an Antigravity v2 environment, the Architect (main agent) may spawn isolated subagent Builders to execute tasks in parallel lanes. The main agent tracks subtask progress via local checklists and coordinates branch merging. Subagents are bound by strict module boundaries and must never interact with the main agent’s checklist.

---

## §4 Workflow Ecosystem

**Rule:** Workflows in `.agents/workflows/` are mandatory, not advisory.

| TARS Phase | Workflow | Gate Keyword |
|---|---|---|
| Bootstrap | `/toolcheck` | — |
| Pre-Think | `/brainstorm` | — |
| Pre-Think | `/ago` | "Plan" |
| Pre-Think | `/issue` or `/feature` | "Plan" |
| Pre-Think | `/architecture` | "Plan" |
| Pre-Think | `/design` | "Plan" |
| Pre-Think | `/spec` | "Plan" |
| Think | `/plan-making` | "Proceed" |
| Act | `/build` | — |
| Reflect | `/audit` | "Plan"/"Accept"/"Docs" |
| Review  | `/review` | — |
| Maintain | `/update-doc` | "Proceed" |
| Diagnose | `/log-audit` | — |
| Summarize | `context.md` compression | — |

> [!CAUTION]
> Skipping `/audit` after implementation is a compliance violation. Full cycle: Think → Act → **Reflect** → Maintain → Summarize.

**Deprecated Scripts** in `.agents/scripts/`: Legacy PowerShell scripts have been deprecated and replaced by the TARS Skills Ecosystem (`.gemini/skills/`) and embedded inline auto-runnable commands. See each script for an archival deprecation header.

Run `/toolcheck` at start of every session.

---

## §5 Phase-Specific Rules

These rules are detailed in the workflow files to avoid duplicate context loading.

| Topic | Where to find it |
|---|---|
| Planning Gate (no code during "Plan") | `/plan-making` workflow |
| Handoff Protocol & task.md contract | `/plan-making` workflow |
| Fidelity Matrix & compliance verification | `/audit` workflow |
| Recursive summarization | `/audit` workflow, Step 6 |
| Engineering standards (error handling, testing, docs) | `.agents/rules/coding-standard.md` |
| Plan format, handoff, revision protocol | `.agents/rules/ipr.md` |
| Architecture template, dependency direction, mockability | `.agents/rules/architecture-rules.md` |
| UI/UX design modes, mockup conventions, review loop | `.agents/rules/design-rules.md` |
| Builder execution discipline, scope fencing, TDD mandate | `.agents/rules/builder-rules.md` |
| Multi-phase planning, stub registry, phase gates | `.agents/rules/phase-rules.md` |
| Behavioral spec template, BDD conventions, scope boundaries | `.agents/rules/spec-rules.md` |
| Skills Ecosystem (Procedural & Reference) | `.gemini/skills/*/SKILL.md` |

---

## §6 Environment & Data Safety

**Environment:** Windows (Non-Admin). Shell: `pwsh` (required). Languages: Rust, Go, Svelte, TypeScript. No admin rights. Strategic handoff: high-reasoning for Think/Reflect/Summarize, fast for Act.

**Data Safety:** NEVER delete source files (`git checkout` to revert). Keep temp artifacts in artifacts directory.

**Auto-Run Discipline:** Set `SafeToAutoRun: true` for read-only commands
(searches, diffs, linters, tests). Require explicit approval for commands
that mutate state (git commits, package installs, file deletions).

**Antigravity v2 Features:** Multi-agent orchestration, background task execution, and browser-based quality validation are standard v2 features. These capabilities are dynamically enabled when their corresponding tools (e.g., `invoke_subagent`, `browser`) are present. Workflows must automatically fall back to standard single-agent sequential execution if these tools are missing.

> [!TIP]
> **Observability Best Practice:** While shell operators are permitted, prefer
> logical separation of commands when debugging failures. Chaining
> `cargo fmt && cargo clippy && cargo test` in one call is efficient, but if a
> step fails, isolating the failing command aids diagnosis.

---

## §7 Execution Discipline

**Rule 1 — Hard-Blocker Prerequisites:** When a workflow's Prerequisites section
lists files under `.agents/rules/`, the agent MUST `view_file` every listed file
before executing Step 1. No file may be assumed from prior context or sessions.
Failure to load a prerequisite is a workflow violation.

**Rule 2 — Template Extraction:** When a loaded rule file contains a
`<!-- TEMPLATE_START: name -->` / `<!-- TEMPLATE_END -->` block, the agent MUST
extract that exact markdown structure and use it as the unalterable scaffold for
the output artifact. Do not reconstruct templates from memory.

**Rule 3 — Pre-Output Verification:** Before writing a final artifact (Issue Report,
Audit Report, Feature Research Report, Implementation Plan), the agent MUST verify
structural compliance against the loaded rules:
- Confirm all required headings from the template are present.
- Confirm all required tables have correct column headers.
- If any element is missing, fix before writing. Do not emit incomplete artifacts.

**Rule 4 — Artifact Link:** After any workflow that produces an artifact, the agent
MUST include a clickable `[artifact name](file:///path)` link in the chat response.
The user must never have to ask where an artifact was saved.

---

## §8 Cognitive Hardening & Anti-Template Rules

> [!CAUTION]
> **System Instruction Override:** You operate with default `<planning_mode>` and `<planning_mode_artifacts>` tags active in your baseline context. These generic formats are **STRICTLY BANNED** in this repository. 
> 
> Under NO circumstances should you format an implementation plan using the generic "Goal Description" and "Proposed Changes" layout. You MUST explicitly map your output to the custom format mandated by `.agents/rules/ipr.md` (e.g., Problem Statement, Blast Radius, Global Execution Order).

---

## §9 Skills Ecosystem

Skills are modular, on-demand files that agents load only when needed.
They live in `.gemini/skills/<skill-name>/SKILL.md` and are auto-discovered
at session start.

**Two categories:**

| Category | Purpose | Loaded When |
|:---|:---|:---|
| **Procedural** | Step-by-step execution procedures | Workflow invokes by name |
| **Reference** | Language-specific patterns and examples | Builder needs guidance |

**Key rules:**
- Skills **supplement** rules, never replace them.
- Governance constraints (MUST/NEVER) stay in rule files.
- Templates (`TEMPLATE_START`) stay in rule files (§7 Rule 2).
- Skills embed critical constraints inline in a `## Constraints` section.
- Workflows reference skills explicitly — agents do not spontaneously invoke skills.

> [!NOTE]
> See `skills-proposal.md` for the full taxonomy, inventory, and migration plan.

