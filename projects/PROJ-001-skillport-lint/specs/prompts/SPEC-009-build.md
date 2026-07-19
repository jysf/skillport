# SPEC-009 — BUILD prompt (Sonnet subagent)

You are the **implementer** for `SPEC-009: GitHub Action + Rust CI`. You run as a
metered subagent on branch `feat/spec-009-action`, already created and checked out
— **commit to the current branch; do not create/switch branches, open a PR, or
merge.** The spec is your source of truth.

## Read first (in order)

1. `projects/PROJ-001-skillport-lint/specs/SPEC-009-github-action-and-rust-ci.md`
   — the Outputs (`action.yml`, `ci.yml` jobs, README section), the **Testability
   note** (you CANNOT run GitHub Actions locally — do not claim a workflow "passed
   CI"), Acceptance Criteria, Failing Tests (local YAML/command checks), Notes.
2. `.github/workflows/ci.yml` (extend — keep the `cost-data` job), `app.just` (the
   gate commands), `README.md` (add a "Use in CI" section), `docs/api-contract.md`
   (exit codes), `docs/license-policy.md` (optional cargo-deny job).
3. `decisions/DEC-005`; `guidance/constraints.yaml` (`license-policy`).

## Your job

1. `action.yml` (repo root): a **composite** Action (`runs.using: "composite"`)
   with inputs `path` (default `.`), `strict` (default `false`), `upload-sarif`
   (default `true`). Steps: ensure Rust toolchain → install skillport (`cargo
   install --git https://github.com/jysf/skillport skillport --locked`, with a
   comment that a released binary will be faster later) → run `skillport lint
   <path> [--strict] --sarif > skillport.sarif` (do NOT swallow the exit code) →
   upload via `github/codeql-action/upload-sarif@v3` (use `if: always()` so results
   upload even on findings). Each `run` step needs a `shell`.
2. `.github/workflows/ci.yml`: KEEP `cost-data`; ADD `rust` (checkout →
   `dtolnay/rust-toolchain@stable` → `cargo fmt --check` → `cargo clippy
   --all-targets -- -D warnings` → `cargo test`) and `dogfood` (checkout → build →
   `skillport lint lint-fixtures/good` expecting exit 0 — do NOT gate on the bad
   fixture). Optionally a `license` job (`cargo-deny check licenses` + a minimal
   permissive-only `deny.toml`) if trivial; else note it as a follow-up.
3. `.github/workflows/example-usage.yml` (or a README snippet): a minimal consumer
   workflow using `uses: jysf/skillport@v0 with: path: skills`, with
   `permissions: { contents: read, security-events: write }`.
4. `README.md`: a "Use in CI" section with the `uses:` snippet + that findings
   surface in code-scanning.
5. Pin every referenced action to a major version; require no secret beyond
   `GITHUB_TOKEN`.

## Definition of done

- Every **Acceptance Criterion** met. Run the spec's **Failing Tests** locally:
  YAML parse (`python3 -c "import yaml,sys;[yaml.safe_load(open(p)) for p in
  sys.argv[1:]]" action.yml .github/workflows/*.yml`), composite-schema check,
  and the command-correctness checks (`skillport lint lint-fixtures/good` → 0,
  `... bad` → 1, `... bad --sarif | python3 -m json.tool`). If `actionlint` is
  installed, run it; if not, say so.
- `cargo test` still green · `cargo clippy --all-targets -- -D warnings` clean ·
  `cargo fmt --check` clean (you add NO Rust — this should be unchanged).
- **No Rust source change; no new crate dependency.** Do NOT claim CI passed on
  GitHub — only that YAML is valid and the commands work locally.
- Fill the spec's **## Build Completion**, append a **build** cost session (null
  numerics, per `projects/_templates/prompts/cost-snippet.md`), set
  `agents.implementer` to your model, commit to `feat/spec-009-action`
  (`feat(SPEC-009): …`). Do **not** advance cycle, PR, or merge.

## Return (final message = data for the orchestrator)

Concise + factual: files created/changed, the local validity/command checks you
ran and their results (paste the YAML-parse + exit-code checks), whether
`actionlint` was available, confirm no Rust/dep change and `cargo test` green, any
deviations (e.g. license job included or deferred), follow-ups.
