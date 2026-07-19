# SPEC-010 — BUILD prompt (Sonnet subagent)

You are the **implementer** for `SPEC-010: body.size via a real tokenizer`. You run
as a metered subagent on branch `feat/spec-010-tokenizer`, already created and
checked out — **commit to the current branch; do not create/switch branches, open
a PR, or merge.** The spec is your source of truth.

## Read first (in order)

1. `projects/PROJ-001-skillport-lint/specs/SPEC-010-body-size-rule-via-real-tokenizer.md`
   — the Rule (exact id `body.size` / severity **info** / 5000-token threshold),
   the tokenizer-choice guidance, Acceptance Criteria, Failing Tests, Out of scope,
   Notes.
2. `src/rules.rs` (`check_body` — the deferral comment marks where `body.size`
   goes; the `Present`-only gating), `src/skill.rs` (`Skill.body`), `src/report.rs`.
3. `decisions/DEC-002`, `DEC-003`, `DEC-005`; `guidance/constraints.yaml`
   (`no-heuristic-error`, `no-new-top-level-deps-without-decision`, `license-policy`).

## Your job

1. Add the tokenizer dep and author **`decisions/DEC-010`** in the same pass:
   recommended **`tiktoken-rs`** (`cl100k_base` or `o200k_base`, embedded BPE —
   offline). Record the **proxy rationale** (no public Anthropic tokenizer; modern
   BPE within ~10–20% for prose; the rule is info) and the **license** (permissive;
   the `licenses` CI job / cargo-deny must pass). Note the ~1–2 MB binary growth.
2. `src/rules.rs`: a `body_token_count(&str) -> usize` helper — build the BPE **once**
   (`std::sync::OnceLock`), encode the body (ordinary/content tokens), return the
   count. Add a `body.size` **info** finding in `check_body` when the count exceeds
   `const BODY_TOKENS_THRESHOLD: usize = 5000` (`>`), with the count in the message
   (use "~" since it's a proxy). Keep it inside `check_body` (Present-gated).
3. Write **every** Failing Test in the spec, including the **tokenizer-pin** test
   (hardcode the real token count of a chosen sample for your encoding — run it once
   locally to get the number — and assert it differs from `chars/4`), and confirm
   `lint-fixtures/good` stays zero findings.

## Definition of done

- Every **Acceptance Criterion** met; every **Failing Test** passes.
- `cargo test` green · `cargo clippy --all-targets -- -D warnings` clean ·
  `cargo fmt --check` clean.
- `body.size` is **info**; no other rule's severity changes; the good fixture stays
  0/0/0.
- `DEC-010` written; the dep + transitive licenses are permissive (run `cargo deny
  check licenses` if available, else confirm by reading — note which).
- **No CLI/emitter change.** Fill the spec's **## Build Completion**, append a
  **build** cost session (null numerics, per `projects/_templates/prompts/cost-snippet.md`),
  set `agents.implementer` to your model, commit to `feat/spec-010-tokenizer`
  (`feat(SPEC-010): …`). Do **not** advance cycle, PR, or merge.

## Return (final message = data for the orchestrator)

Concise + factual: files changed, the tokenizer crate + DEC-010, all ACs/tests
pass with exact `cargo test`/`clippy`/`fmt` lines, the pinned token count you used,
confirm good fixture 0/0/0 + no CLI change + licenses permissive, any deviations,
follow-ups.
