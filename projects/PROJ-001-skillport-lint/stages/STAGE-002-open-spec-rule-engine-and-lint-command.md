---
# Maps to ContextCore epic-level conventions.
# A Stage is a coherent chunk of work within a Project.
# It has a spec backlog and ships as a unit when the backlog is done.

stage:
  id: STAGE-002                     # stable, zero-padded, continuous across the repo
  status: shipped                   # proposed | active | shipped | cancelled | on_hold
  priority: high                    # critical | high | medium | low
  target_complete: null             # optional: YYYY-MM-DD

project:
  id: PROJ-001                      # parent project
repo:
  id: skillport

created_at: 2026-07-17
shipped_at: 2026-07-18

value_contribution:
  advances: "The 'genuinely useful lint' half of the project thesis — the crisp, citable validator teams put in CI."
  delivers:
    - "the `lint` command over a file, folder, or tree"
    - "the open-spec rule catalog with correct severities"
    - "human + --json output and CI exit codes (+ --strict)"
  explicitly_does_not:
    - "verify or encode any per-platform constraint (STAGE-003)"
    - "emit SARIF or ship a GitHub Action (STAGE-003)"
    - "use a real tokenizer for body.size (STAGE-003)"
---

# STAGE-002: Open-spec rule engine + `lint` command

## What This Stage Is

The first user-facing capability: a `lint` command that runs the open-spec rule
catalog over a single skill, a skill folder, or a whole tree, and reports
findings at three severities with correct CI exit codes. It plugs the rule engine
into STAGE-001's model, walker, and sectioned report. This is table stakes (it
overlaps the official `skills-ref validate`) — implement it correctly and move
the differentiation into STAGE-003 and PROJ-002.

## Why Now

The substrate (STAGE-001) is inert without rules and a command. This stage makes
skillport *do something* and is the credible entry point for adoption. It must
land before the per-platform/DX polish (STAGE-003), which widens and dresses up a
working validator rather than creating one.

## Success Criteria

- `lint <path>` works for a single skill, a folder, and a tree in one pass.
- Every rule in the catalog below is implemented with the **exact seeded
  severity** and a **stable rule id**.
- Human-readable output and `--json` (stable schema, DEC-005).
- Exit codes: non-zero on any **error**; non-zero on any **warning** under
  `--strict`; zero otherwise.
- A malformed skill in a bulk run is reported as a per-file finding and the run
  continues (never aborts — DEC-005).
- Results are path-sorted and deterministic.
- No heuristic/soft rule is error-level (DEC-003).

## Scope

### In scope
- The rule engine + the catalog below (open-spec layer only).
- `lint` subcommand (clap): single skill / folder / tree.
- Human + `--json` emitters over STAGE-001's sectioned report.
- Exit-code logic + `--strict`.

### Explicitly out of scope
- `--target` recognized-field widening beyond the open `SPEC_KEYS` set — the
  `frontmatter.unknown` rule ships here against the **open** field set; the
  platform-specific widening is STAGE-003 (DEC-002).
- SARIF output, GitHub Action (STAGE-003).
- Real tokenizer — `body.size` may ship here as a **placeholder/deferred** check
  or be deferred wholesale to STAGE-003 where the tokenizer lands (design call).

## Open-spec rule catalog (implement exactly; source: agentskills.io)

Severity: **error** = spec violation (gates CI); **warning** =
recommended/likely-wrong; **info** = advisory. Per DEC-002 only these
open-spec-backed rules are firm; per DEC-003 nothing heuristic is error-level.

| Rule id | Sev | Check |
|---|---|---|
| `frontmatter.missing` | error | frontmatter block present |
| `name.required` / `name.type` | error | present; is a string |
| `name.length` | error | 1–64 chars |
| `name.charset` | error | lowercase letters, digits, hyphens only |
| `name.hyphen-edges` | error | no leading/trailing hyphen |
| `name.hyphen-consecutive` | error | no `--` |
| `name.dir-match` | warning | equals parent directory name |
| `description.required` / `description.type` | error | present; is a string |
| `description.length` | error | 1–1024 chars, non-empty |
| `description.detail` | info | too terse to convey *when* to use (soft; tune) |
| `compatibility.length` | error | ≤500 chars if present |
| `metadata.type` | warning | is a key→value map |
| `metadata.values` | info | values are strings (spec is string→string) |
| `allowed-tools.format` | warning* | space-separated string, not a list (*info where a platform is confirmed to accept a list — that downgrade is STAGE-003 / DEC-002) |
| `body.empty` | warning | body non-empty |
| `body.lines` | warning | ≤500 lines recommended |
| `body.size` | warning | ~<5000 tokens recommended (real tokenizer lands STAGE-003; info-level per the answered Frame question) |
| `frontmatter.unknown` | info | key recognized against the open spec field set (widened per `--target` in STAGE-003) |

> Two severity seams that STAGE-003 / DEC-002 govern: `allowed-tools.format` is
> `warning` for the open target and downgrades to `info` only where a platform is
> *confirmed* to accept a list; `frontmatter.unknown` runs against the open
> `SPEC_KEYS` here and widens per verified `--target`.

## Spec Backlog

> Proposed decomposition — the Design cycle turns these into specs via
> `just new-spec "<title>" STAGE-002`. Not yet scaffolded.

- [x] SPEC-004 (shipped 2026-07-18, PR #4) — Rule engine (`lint_skill` = the
  `rule_fn`) + frontmatter presence (three `frontmatter.*` ids) + `name.*` +
  `description.*` + `compatibility.length`. Locked the empty-`Present` decision.
- [x] SPEC-005 (shipped 2026-07-18, PR #5) — **`lint` command** (clap) + human &
  `--json` emitters + exit codes + `--strict`, implementing `docs/api-contract.md`.
  `skillport lint <path>` is runnable. Added clap + serde + serde_json (DEC-008).
- [x] SPEC-006 (shipped 2026-07-18, PR #6) — remaining open-spec rules into
  `lint_skill`: `metadata.type`/`.values`, `allowed-tools.format`/`.type`,
  `body.empty`/`.lines`, `frontmatter.unknown`, `compatibility.type`; tightened
  `name.charset` to ASCII (resolved `name-charset-ascii`). **Open-spec catalog
  complete.** `body.size` + `--target` widening are STAGE-003.
- [x] ~~`key.duplicate` rule~~ — **CLOSED as resolved-redundant (SPEC-007 design).**
  The parser does NOT take last-write-wins; `serde_yaml_ng` rejects duplicate keys,
  so a duplicate already surfaces as `frontmatter.invalid` (error, message names the
  key). A separate rule would add redundant public surface (DEC-005) — not added.
- [x] SPEC-007 (shipped 2026-07-18, PR #7) — perm-denied subtree → `dir.unreadable`
  **warning** finding (resolves signal `walk-unreadable-dirs`): the walk records an
  unreadable directory instead of silently skipping it. Extends `CollectionItem` +
  `Report::from_collection`.

**Count:** 4 shipped / 0 active / 0 pending — **stage backlog complete**
(`key.duplicate` closed as resolved-redundant; `body.size`/`--target` are STAGE-003).

## Design Notes

- The prototype's `lint.rs` already implements essentially this catalog with the
  right severities — it is the strongest reuse candidate. Port it onto STAGE-001's
  collection-first model + sectioned report (the prototype lints one skill at a
  time; adapt to N-with-sections). Reuse `lint-fixtures/good|bad`.
- Keep the open `SPEC_KEYS` set (`name`, `description`, `license`,
  `compatibility`, `metadata`, `allowed-tools`) here; STAGE-003 adds the verified
  `--target claude` widening.
- Firm constraints: `no-heuristic-error`, `deterministic-stable-output`,
  `only-verified-constraints-are-firm`; DEC-002, DEC-003, DEC-005.

## Dependencies

### Depends on
- STAGE-001 (model, walker, report, stable-id + severity types).

### Enables
- STAGE-003 (widens `frontmatter.unknown`/`allowed-tools.format`, adds emitters/DX).
- PROJ-002 (audit reuses the same finding model).

## Stage-Level Reflection

*Shipped 2026-07-18.*

- **Did we deliver the outcome in "What This Stage Is"?** Yes. `skillport lint
  <path> [--json] [--strict]` runs and enforces the **entire open-spec catalog**
  over a file/folder/tree with three severities, stable rule ids, a stable
  `--json` schema (`schema: 1`), correct CI exit codes (0/1/2), stdout/stderr
  separation, and never-abort bulk behavior. It even surfaces coverage gaps
  (`dir.unreadable`). This is the "genuinely useful lint" half of the project thesis.
- **How many specs did it actually take?** 4 (SPEC-004 rule engine + identity
  rules, SPEC-005 the CLI+emitters, SPEC-006 the rest of the catalog, SPEC-007
  unreadable-dir). `key.duplicate` was closed as resolved-redundant (no spec).
  ~95 tests; every spec APPROVED first pass by an independent Opus verifier.
- **What changed between starting and shipping?** We **pulled the CLI (SPEC-005)
  ahead** of the remaining rules (SPEC-006) at the user's request, so `skillport
  lint` became runnable sooner and the last rules dropped into `lint_skill`
  without touching the CLI — validating the SPEC-003 `rule_fn` seam.
- **Lessons that should update AGENTS.md, templates, or constraints?** None
  promoted to codified beyond `name-charset-ascii` (already codified in SPEC-006).
  The metered-subagent pipeline (Sonnet build / Opus verify) is now the settled
  workflow — captured in memory; the process-debt `cost-metering-manual-sessions`
  stays `accepted` pending the PROJ-001-close confirmation.
- **Signals dispositioned at this close?** All STAGE-002-owned signals walked (no
  silent carry):
  - `walk-unreadable-dirs` (lesson, watch) → **resolved/codified** — implemented as
    `dir.unreadable` in SPEC-007.
  - `spec-pin-edge-cases` (lesson, watch, N=1) → **kept watch**; its concrete carry
    (lock empty-`Present` frontmatter) was completed in SPEC-004; no new occurrence
    in STAGE-002. `last_touched` bumped.
  - `name-charset-ascii` → already `codified` (SPEC-006).
- **Should any spec-level reflections be promoted to stage-level lessons?** One
  worth keeping: *verify an assumption about existing behavior before designing a
  backlog item* (the `key.duplicate` investigation turned a "loose end" into a
  non-task). Recorded here; below the N=3 codify bar, so not codified.
