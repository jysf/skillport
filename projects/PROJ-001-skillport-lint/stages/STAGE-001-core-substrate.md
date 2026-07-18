---
# Maps to ContextCore epic-level conventions.
# A Stage is a coherent chunk of work within a Project.
# It has a spec backlog and ships as a unit when the backlog is done.

stage:
  id: STAGE-001                     # stable, zero-padded, continuous across the repo
  status: active                    # proposed | active | shipped | cancelled | on_hold
  priority: high                    # critical | high | medium | low
  target_complete: null             # optional: YYYY-MM-DD

project:
  id: PROJ-001                      # parent project
repo:
  id: skillport

created_at: 2026-07-17
shipped_at: null

# What part of the project's value thesis this stage advances.
# If you can't articulate value_contribution, the stage may be
# infrastructure-only — acceptable but flag it.
value_contribution:
  advances: "The 'built for PROJ-002 reuse' half of the project thesis — the collection-first substrate every later stage and the audit build on."
  delivers:
    - "a parsed, canonical model of any SKILL.md (even malformed ones)"
    - "a walk of a path into a collection of skills"
    - "a sectioned N-skill report + finding model with stable ids"
  explicitly_does_not:
    - "implement any rules (STAGE-002)"
    - "expose a CLI command (STAGE-002)"
    - "do per-platform / --target work (STAGE-003)"
---

# STAGE-001: Core substrate

## What This Stage Is

The shared foundation the rest of skillport stands on: a **tolerant, lossless,
order-preserving** `SKILL.md` parser; a canonical `Skill` model; a **tree-walker
that returns a collection** of skills; and a **finding + report model that
already takes N skills with sections and stable ids**. When this ships, the rule
engine (STAGE-002) and the PROJ-002 audit both plug into the same substrate
instead of re-deciding it. This is deliberately designed here as the reuse base
(DEC-004) — it is *not* a single-file linter with a folder loop.

## Why Now

Everything in PROJ-001 and PROJ-002 depends on it. Built single-file-first, it
would force a rewrite when the audit arrives (DEC-004). The parser's tolerance
and losslessness are load-bearing: bulk runs must survive malformed skills
(DEC-005), and future normalization/round-trip work needs nothing dropped on
parse.

## Success Criteria

- Any `SKILL.md` parses into a canonical `Skill` (frontmatter + body), preserving
  frontmatter key order and losing nothing.
- Tolerant of real-world messiness: BOM, leading blank lines, missing frontmatter,
  unclosed frontmatter, CRLF — each handled gracefully (surfaced as a finding,
  never a panic).
- Walking a path yields a **collection**: a single file → 1 skill; a folder / tree
  → all `SKILL.md` under it, skipping `.git`, `node_modules`, `target`.
- The report model represents **N skills, each with a section of findings**; a
  finding carries `{ severity, rule (stable id), message, location }`.
- Output ordering is deterministic (sorted by path) — the substrate guarantees it
  even before any emitter exists (DEC-005).

## Scope

### In scope
- `parse`: split YAML frontmatter from Markdown body; tolerant of the edge cases above.
- `Skill` model: order-preserving frontmatter map + raw body + source path.
- `walk`: path → `Vec<Skill-or-parse-error>` collection, with directory skips.
- `Finding` + `Severity` (error/warning/info) + a sectioned report type.
- Stable rule-id convention (the ids are a public contract per DEC-005).

### Explicitly out of scope
- Any rule logic or the `lint` command (STAGE-002).
- Human/JSON/SARIF emitters (STAGE-002/003 — the report *model* lives here; the
  *rendering* comes later).
- `--target` / per-platform recognized fields (STAGE-003).
- Tokenizer (STAGE-003, used by `body.size`).

## Spec Backlog

> Proposed decomposition — the Design cycle turns these into specs via
> `just new-spec "<title>" STAGE-001`. Not yet scaffolded.

- [x] SPEC-001 (shipped 2026-07-18, PR #1) — Tolerant, lossless, order-preserving
  `SKILL.md` parser + canonical `Skill` model (BOM / leading blanks / missing /
  unclosed / invalid frontmatter / CRLF; total `parse`, never aborts). Model
  folded in. Emitted DEC-007 (`serde_yaml_ng` + `indexmap`).
- [x] SPEC-002 (shipped 2026-07-18, PR #2) — Collection tree-walker: `walk(root) ->
  Collection` (skips `.git`/`node_modules`/`target`; single file & tree both yield
  a collection; unreadable/non-UTF-8 file → `Unreadable` item, never aborts;
  path-sorted). Reuses SPEC-001's `parse`. `tempfile` dev-dep only.
- [~] SPEC-003 (design) — Finding + `Severity` + sectioned N-skill `Report` model
  with stable rule ids, path-sorted sections, `Report::from_collection(collection,
  rule_fn)` assembly (Unreadable → `file.unreadable` error; `rule_fn` seam for
  STAGE-002), and `exit_code(strict)`. **Closes STAGE-001.** (`FrontmatterStatus` →
  findings and the perm-denied-subtree question are deferred to the rules that
  consume `rule_fn`, per the spec's Out-of-scope + signal `walk-unreadable-dirs`.)

**Count:** 2 shipped / 1 in design / 0 pending

## Design Notes

- Pick a **current, maintained** YAML crate (the prototype's `serde_yaml` is now
  deprecated; its `=`-pins were a Rust-1.75 artifact — drop them). Adding it is a
  runtime dep → author a DEC in the same pass (`no-new-top-level-deps-without-decision`).
- Order preservation implies an order-preserving map (e.g. an index-map style
  structure) rather than a plain `HashMap`.
- The prototype's `parse.rs` / `skill.rs` are a reasonable reference for the split,
  but the collection-first walker and the sectioned/stable-id report are the parts
  the prototype does *not* fully have — design them here.
- Firm constraints this stage must honor: `collection-first-substrate`,
  `deterministic-stable-output` (see `guidance/constraints.yaml`); DEC-004, DEC-005.

## Dependencies

### Depends on
- None (foundational stage).

### Enables
- STAGE-002 (rule engine plugs into the model + report).
- STAGE-003 (emitters, tokenizer, `--target`).
- PROJ-002 (`audit` reuses walker + model + report).

## Stage-Level Reflection

*Filled in when status moves to shipped.*

- **Did we deliver the outcome in "What This Stage Is"?** <not yet>
- **How many specs did it actually take?** <not yet>
- **What changed between starting and shipping?** <not yet>
- **Lessons that should update AGENTS.md, templates, or constraints?** <not yet>
- **Signals dispositioned at this close?** <not yet>
- **Should any spec-level reflections be promoted to stage-level lessons?** <not yet>
