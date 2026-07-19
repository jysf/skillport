# SPEC-014 — BUILD prompt (Sonnet subagent)

You are the **implementer** for `SPEC-014: release workflow — cross-platform binaries
on tag`. You run as a metered subagent on branch `feat/spec-014-release-workflow`,
already created and checked out — **commit to the current branch; do not create/switch
branches, open a PR, or merge.** The spec is your source of truth.

> **Add the workflow; do not fire it.** Do NOT push a tag, create a real GitHub
> Release, or run `cargo publish`. Do NOT change `action.yml`, `ci.yml`, `src/`,
> `Cargo.toml`, or `Cargo.lock`. This spec is one file: `.github/workflows/release.yml`
> (+ the spec doc bookkeeping). DEC-005: the contract is frozen; release = packaging.

## Read first (in order)

1. `projects/PROJ-001-skillport-lint/specs/SPEC-014-release-workflow-cross-platform-binaries-on-tag.md`
   — the Outputs table (the 5-target matrix, runners, archive rules), Acceptance
   Criteria, Failing Tests, Notes for the Implementer, Out of scope.
2. `.github/workflows/ci.yml` (mirror `actions/checkout@v4` + `dtolnay/rust-toolchain@stable`),
   `scripts/build-info.sh` (provenance via `just build-info`), `Cargo.toml` (name/version),
   `decisions/DEC-009-distribution-strategy.md` (step 2 + matrix).

## Your job

Create `.github/workflows/release.yml`:
1. **Triggers:** `on: push: tags: ['v*']` AND `on: workflow_dispatch`.
2. **`build` matrix** over exactly these 5 targets (see the spec table for runners/exts):
   `aarch64-apple-darwin` + `x86_64-apple-darwin` (macos-14), `x86_64-unknown-linux-gnu`
   + `aarch64-unknown-linux-musl` (ubuntu-latest), `x86_64-pc-windows-msvc`
   (windows-latest). Per leg: checkout → toolchain **with the matrix target** →
   `cargo build --release --locked --target <triple>` → strip → archive
   `skillport-<version>-<triple>.<ext>` containing the binary + `README.md` +
   `LICENSE-MIT` + `LICENSE-APACHE` → write `…​.<ext>.sha256` → `actions/upload-artifact@v4`.
   `<version>` = tag minus leading `v` on a tag run, else `Cargo.toml` version — derive it
   deterministically (a step, not hand-typed per leg). The `aarch64-unknown-linux-musl`
   leg is the only true cross-compile — use `cross` (simplest) or a musl cross-linker;
   `RUSTFLAGS=-Cstrip=symbols` is fine if a cross `strip` is awkward.
3. **`release` job** — `needs: build`, `if: startsWith(github.ref, 'refs/tags/v')`,
   `permissions: contents: write`: download all artifacts; write `build-info.txt` from
   `./scripts/build-info.sh`; create/update the tag's Release and upload every archive +
   `.sha256` + `build-info.txt` using the **`gh` CLI** (`gh release create … || gh
   release upload … --clobber`) with the default `GITHUB_TOKEN`. On `workflow_dispatch`
   this job is skipped (dry path = artifacts only, no Release).
4. Pin actions to major (`@v4`); keep third-party surface to `dtolnay/rust-toolchain`
   only; everything else first-party + `gh`. No `softprops/*` / `taiki-e/*` release action.

## Definition of done

- Every **Acceptance Criterion** met; every **Failing Test** (static + local proof)
  passes. Run `actionlint .github/workflows/release.yml` (exit 0; install it if needed —
  `go install github.com/rhysd/actionlint/cmd/actionlint@latest` or brew; if truly
  unavailable, YAML-parse + document a manual review and say so). Do the **local archive
  round-trip proof**: `cargo build --release --locked` for the host, strip, `tar czf`
  the binary + 3 doc/license files, `sha256sum > …sha256`, `sha256sum -c …sha256` → exit 0.
- `git diff main -- src/ Cargo.toml Cargo.lock .github/workflows/ci.yml action.yml` is
  EMPTY. Existing `cargo test`/`clippy`/`fmt`/`cargo publish --dry-run` still pass.
- Fill the spec's **## Build Completion**, append a **build** cost session (null
  numerics, per `projects/_templates/prompts/cost-snippet.md`), set `agents.implementer`
  to your model, commit to `feat/spec-014-release-workflow` (`feat(SPEC-014): …`). Do
  **not** advance cycle, PR, merge, tag, or create a Release.

## Return (final message = data for the orchestrator)

Concise + factual: PASTE `.github/workflows/release.yml` in full (it's the deliverable);
the `actionlint` result; the local archive round-trip proof output (incl. the
`sha256sum -c` line); confirm the 5 targets + triggers + the tag-only release guard +
`gh`-only release step + no third-party release action; confirm `git diff main -- src/
Cargo.toml Cargo.lock ci.yml action.yml` empty and gates green; note how the musl
cross-compile is handled; any deviations/follow-ups.
