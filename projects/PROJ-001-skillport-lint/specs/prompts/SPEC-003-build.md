# SPEC-003 — BUILD prompt (Sonnet subagent)

You are the **implementer** for `SPEC-003: finding severity and sectioned report
model` in the skillport repo. You run as a metered subagent on branch
`feat/spec-003-report`, already created and checked out — **commit to the current
branch; do not create/switch branches, open a PR, or merge.** The spec is your
source of truth.

## Read first (in order)

1. `projects/PROJ-001-skillport-lint/specs/SPEC-003-finding-severity-and-sectioned-report-model.md`
   — the whole spec: **Acceptance Criteria**, **Failing Tests**, the fixed
   **Outputs** shape, the **rule_fn seam**, **Out of scope**, **Notes**.
2. `src/skill.rs` (SPEC-001 `Skill`) and `src/walk.rs` (SPEC-002 `Collection` /
   `CollectionItem`) — the report **consumes** a `Collection`.
3. `decisions/DEC-003` (severity → exit codes; no heuristic error), `DEC-004`
   (collection-first, N sections), `DEC-005` (deterministic; stable ids/shape).
4. `guidance/constraints.yaml` (`deterministic-stable-output`,
   `collection-first-substrate`, `no-heuristic-error`, `test-before-implementation`)
   and `guidance/toolchain-brief.md`.

## Your job

1. Implement `src/report.rs`: `Severity`, `Finding`, `Section`, `Summary`,
   `Report`, `Report::from_collection(collection, rule_fn)`, and
   `Report::exit_code(strict)` — exactly to the spec's **Outputs** shape.
   - `from_collection` maps each `CollectionItem::Unreadable` to ONE
     `file.unreadable` **Error** finding, and runs `rule_fn(&skill)` for each
     `Skill` item; one `Section` per item; sections **sorted by path**; findings
     within a section deterministically ordered (severity-desc, then rule id).
   - `exit_code`: any Error → 1; strict && any Warning → 1; else 0.
2. Wire the module into `src/lib.rs`.
3. Write **every** test in the spec's **Failing Tests** (in `#[cfg(test)] mod
   tests` in `src/report.rs`) and make them pass. Construct `CollectionItem`s
   in-memory (no filesystem needed) and trivial `rule_fn` closures.
4. **Stay in scope:** NO open-spec rules (`name.*`, `frontmatter.missing`, …), NO
   heuristics, NO emitters/serde, NO CLI. The only rule id this module contains is
   `"file.unreadable"`. Rules arrive via `rule_fn` (STAGE-002).

## Definition of done

- Every **Acceptance Criterion** met; every **Failing Test** passes.
- `cargo test` green · `cargo clippy --all-targets -- -D warnings` clean ·
  `cargo fmt --check` clean.
- **No new dependency** (pure std + existing crate). If you think one is needed,
  STOP and report instead of adding it.
- Fill the spec's **## Build Completion** (branch, AC met?, deviations, follow-ups),
  append a **build** cost session to `cost.sessions` with **null** numerics (per
  `projects/_templates/prompts/cost-snippet.md` — orchestrator fills real tokens at
  ship), set `agents.implementer` to the model you ran as, and commit to
  `feat/spec-003-report` (`feat(SPEC-003): …`). Do **not** advance cycle, PR, or merge.

## Return (your final message = data for the orchestrator)

Concise + factual: files changed, all ACs/tests pass with exact `cargo test` /
`clippy` / `fmt` result lines, any deviations from the spec and why, and any
follow-ups noticed. Confirm you added no dependency and no rule logic.
