# SPEC-009 — VERIFY prompt (Opus subagent)

You are an **independent verifier** for `SPEC-009: GitHub Action + Rust CI`, run
as a metered subagent. A separate Sonnet build session implemented it and
committed to branch `feat/spec-009-action`. You did NOT build it. Disprove "done."
**Do not modify code, merge, or advance the cycle** — return a verdict.

Remember: **GitHub Actions run on GitHub, not locally** — neither you nor the build
can prove a workflow "passes CI". Verify what's locally checkable: YAML validity,
composite-action schema conformance, correct pinned actions, exit-code
propagation logic (by reading), and that the **commands** the workflows invoke
actually work against the local binary.

## Review the diff

```bash
git diff main...HEAD
```

## Read (in order)

1. `projects/PROJ-001-skillport-lint/specs/SPEC-009-github-action-and-rust-ci.md`
   — Outputs, Testability note, ACs, Failing Tests, Out of scope, Build Completion.
2. `action.yml`, `.github/workflows/*.yml`, `README.md` (the CI section),
   `decisions/DEC-005`.

## Verify — run the local checks yourself

```bash
cargo test        # must still be green (no Rust added)
cargo clippy --all-targets -- -D warnings
cargo fmt --check
python3 -c "import yaml,sys; [yaml.safe_load(open(p)) for p in sys.argv[1:]]; print('YAML_OK')" action.yml .github/workflows/*.yml
cargo build
./target/debug/skillport lint lint-fixtures/good ; echo good_exit=$?     # 0
./target/debug/skillport lint lint-fixtures/bad  ; echo bad_exit=$?      # 1
./target/debug/skillport lint lint-fixtures/bad --sarif | python3 -m json.tool >/dev/null && echo SARIF_OK
command -v actionlint >/dev/null && actionlint || echo "actionlint not installed (note it)"
```

Adversarially check:
- **`action.yml`:** `runs.using == "composite"`; every `run` step has a `shell`;
  inputs `path`/`strict`/`upload-sarif` declared with defaults; it invokes
  `skillport lint` with those inputs and produces + uploads SARIF via
  `github/codeql-action/upload-sarif@vN`.
- **Exit-code propagation:** the lint step does NOT swallow a non-zero exit
  (no `|| true`); the job fails on findings; the upload still runs (`if: always()`
  or ordered before the gate).
- **`ci.yml`:** `cost-data` job still present; `rust` job runs fmt-check + clippy
  `-D warnings` + test; `dogfood` lints `lint-fixtures/good` (NOT the bad fixture)
  expecting exit 0. All actions pinned to a major version.
- **Commands match reality:** the exact `skillport lint …` strings in the YAML are
  the ones that work locally (good→0, bad→1, `--sarif` valid). Flag any mismatch.
- **Permissions:** SARIF upload has `security-events: write`; no secret beyond
  `GITHUB_TOKEN`.
- **No Rust/dep change:** `git diff -- src/ Cargo.toml Cargo.lock` is empty;
  `cargo test` green.
- Judge whether the `license`/cargo-deny job was included or deferred — reasonable?

## Return a verdict (final message = data for the orchestrator)

**✅ APPROVED** / **⚠ PUNCH LIST** (numbered, file:line + failing case) /
**❌ REJECTED** (which criterion, concrete check → observed vs expected). Include
your local-check outputs (YAML_OK, exit codes, actionlint availability), a per-AC
pass/fail summary, and judge any deviations. Do not claim CI passed on GitHub.
Every flag needs a concrete observed/expected. Don't touch code.
