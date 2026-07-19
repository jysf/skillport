# SPEC-013 — BUILD prompt (Sonnet subagent)

You are the **implementer** for `SPEC-013: release phase-0 prep — dual license +
crates.io metadata`. You run as a metered subagent on branch
`feat/spec-013-release-prep`, already created and checked out — **commit to the
current branch; do not create/switch branches, open a PR, or merge.** The spec is your
source of truth.

> **This is packaging/config only.** No `src/` change, no dependency change, no
> behavior/`--json`/SARIF/exit-code/rule-id change (DEC-005). **Do NOT run
> `cargo publish` for real** — dry-run only; publishing is human-only (SPEC-015). Do
> NOT add a release workflow, change `action.yml`, bump the version, tag, or write a
> CHANGELOG — those are later STAGE-004 specs.

## Read first (in order)

1. `projects/PROJ-001-skillport-lint/specs/SPEC-013-release-phase-0-prep-dual-license-and-crates-metadata.md`
   — Outputs, Acceptance Criteria, Failing Tests, Out of scope, Notes.
2. `decisions/DEC-009-distribution-strategy.md` (Phase-0 pre-flight + metadata list),
   `Cargo.toml`, `LICENSE`, `README.md` (`## License` section).

## Your job

1. **Dual license files:** `git mv LICENSE LICENSE-APACHE` (preserve the Apache-2.0
   text unchanged). Create `LICENSE-MIT` with the standard MIT License text, copyright
   line exactly `Copyright (c) 2026 jysf`.
2. **`Cargo.toml` `[package]` metadata** — add:
   - `authors = ["jysf <jyashinsky@gmail.com>"]`
   - `readme = "README.md"`
   - `homepage = "https://github.com/jysf/skillport"`
   - `keywords = ["skills", "linter", "agent", "validation", "cli"]` (≤ 5, ≤ 20 chars each)
   - `categories = ["command-line-utilities", "development-tools"]` (valid crates.io
     slugs — do not invent slugs)
   - keep `license = "MIT OR Apache-2.0"` and `repository`; do NOT add `license-file`.
3. **README `## License`** — replace the "Apache-2.0 … call to confirm / inherited from
   the template" paragraph with a dual-license statement: skillport is licensed under
   **either** MIT (`LICENSE-MIT`) **or** Apache-2.0 (`LICENSE-APACHE`) at the user's
   option, linking both files. Remove the "call to confirm" / "inherited from the
   template" language.
4. **(Recommended, keep small)** add a CI guard to `.github/workflows/ci.yml` that runs
   `cargo publish --dry-run` so packaging can't silently regress. If it bloats scope,
   skip it and note it as a follow-up in Build Completion instead.
5. **Prove it:** run `cargo publish --dry-run` (must exit 0) and `cargo package --list`
   (must include `LICENSE-MIT`, `LICENSE-APACHE`, `README.md`, `Cargo.toml`).

## Definition of done

- Every **Acceptance Criterion** met; every **Failing Test** (command check) passes —
  paste the real output of `cargo publish --dry-run`, `cargo package --list`, and
  `cargo metadata --no-deps --format-version 1 | jq '.packages[0] | {authors,keywords,categories,homepage,readme,license}'`.
- `cargo test` green · `cargo clippy --all-targets -- -D warnings` clean ·
  `cargo fmt --check` clean. No `src/` change, no new dependency, no contract change.
- Fill the spec's **## Build Completion**, append a **build** cost session (null
  numerics, per `projects/_templates/prompts/cost-snippet.md`), set `agents.implementer`
  to your model, commit to `feat/spec-013-release-prep` (`feat(SPEC-013): …` or
  `chore(SPEC-013): …`). Do **not** advance cycle, PR, or merge, and do **not**
  `cargo publish`.

## Return (final message = data for the orchestrator)

Concise + factual: files changed (incl. the `LICENSE`→`LICENSE-APACHE` rename and new
`LICENSE-MIT`); PASTE the `cargo publish --dry-run` result, the `cargo package --list`
output, and the `cargo metadata` field dump; confirm no `src/`/dep/contract change and
gates green; note whether you added the CI guard; any deviations/follow-ups.
