# SPEC-015 — BUILD prompt (Sonnet subagent)

You are the **implementer** for `SPEC-015: crates.io publish (tag-triggered) + RELEASING
doc`. You run as a metered subagent on branch `feat/spec-015-crates-publish`, already
created and checked out — **commit to the current branch; do not create/switch branches,
open a PR, or merge.** The spec is your source of truth.

> **Prepare the publish; do not publish.** Do NOT run `cargo publish` for real, do NOT
> add a crates.io token, do NOT push a tag. Do NOT change `action.yml`, `ci.yml`, `src/`,
> `Cargo.toml`, or `Cargo.lock` (DEC-005). The deliverable is a `publish` job added to
> `.github/workflows/release.yml` + a new `RELEASING.md` (+ spec bookkeeping).

## Read first (in order)

1. `projects/PROJ-001-skillport-lint/specs/SPEC-015-crates-io-publish-tag-triggered-plus-releasing-doc.md`
   — Outputs, Acceptance Criteria, Failing Tests, Notes, Out of scope.
2. `.github/workflows/release.yml` (the `version`/`build`/`release` jobs — mirror their
   style and the `if: startsWith(github.ref, 'refs/tags/v')` tag guard), `Cargo.toml`
   (name/version), `decisions/DEC-009` (step 3 + the human-only guardrail).

## Your job

1. **Add a `publish` job to `.github/workflows/release.yml`:**
   - `if: startsWith(github.ref, 'refs/tags/v')` (tag-only; skipped on
     `workflow_dispatch`), `needs: [version, build]`, `runs-on: ubuntu-latest`.
   - Steps: `actions/checkout@v4`; `dtolnay/rust-toolchain@stable`; a **version-match
     guard** step that derives the `Cargo.toml` version (awk, like the `version` job) and
     `exit 1`s with a clear message if it ≠ `needs.version.outputs.version`; then
     `cargo publish --locked` with `env: CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}`.
   - Reference the token ONLY via `${{ secrets.CARGO_REGISTRY_TOKEN }}` (never a literal).
     Pin actions to major. No extra `permissions`.
2. **Create `RELEASING.md`** at repo root — a concise numbered human runbook:
   - One-time setup: crates.io account → API token → add as the `CARGO_REGISTRY_TOKEN`
     GitHub Actions secret; re-confirm `skillport` is free; **first `cargo publish
     --locked` manually** to establish ownership.
   - Per-release: bump version + CHANGELOG (SPEC-017 / `just next-version`), commit, push
     `vX.Y.Z` → `release.yml` builds binaries (SPEC-014) and the `publish` job publishes.
     Optionally `workflow_dispatch` first (builds artifacts, no Release, no publish).
   - Guardrails: tag version must equal `Cargo.toml` version (job enforces it); a version
     already on crates.io can't be republished; macOS binaries unsigned until an Apple
     key (Homebrew deferred, DEC-009). Mark the human-only steps; link DEC-009.

## Definition of done

- Every **Acceptance Criterion** met; every **Failing Test** passes. Run
  `actionlint .github/workflows/release.yml` (exit 0). Confirm the crate is still
  unpublished: `curl -s -o /dev/null -w '%{http_code}' -H 'User-Agent: x'
  https://crates.io/api/v1/crates/skillport` → `404`.
- `git diff main -- src/ Cargo.toml Cargo.lock .github/workflows/ci.yml action.yml` is
  EMPTY. Existing `cargo test`/`clippy`/`fmt`/`cargo publish --dry-run` still pass.
- Fill the spec's **## Build Completion**, append a **build** cost session (null
  numerics, per `projects/_templates/prompts/cost-snippet.md`), set `agents.implementer`
  to your model, commit to `feat/spec-015-crates-publish` (`feat(SPEC-015): …`). Do
  **not** advance cycle, PR, merge, publish, add a secret, or tag.

## Return (final message = data for the orchestrator)

Concise + factual: PASTE the new `publish` job YAML and the `RELEASING.md`; the
`actionlint` result; the crates.io `404` check; confirm the tag-only guard + secret
reference (no literal) + version-match step + `workflow_dispatch`-skips-publish; confirm
`git diff main -- src/ Cargo.toml Cargo.lock ci.yml action.yml` empty and gates green;
any deviations/follow-ups.
