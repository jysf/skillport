# SPEC-010 — VERIFY prompt (Opus subagent)

You are an **independent verifier** for `SPEC-010: body.size via a real tokenizer`,
run as a metered subagent. A separate Sonnet build session implemented it and
committed to branch `feat/spec-010-tokenizer`. You did NOT build it. Disprove
"done." **Do not modify code, merge, or advance the cycle** — return a verdict.

## Review the diff

```bash
git diff main...HEAD
```

## Read (in order)

1. `projects/PROJ-001-skillport-lint/specs/SPEC-010-body-size-rule-via-real-tokenizer.md`
   — the Rule (id/severity/threshold), tokenizer guidance, ACs, Failing Tests, Out
   of scope, Build Completion/deviations.
2. `src/rules.rs`, `decisions/DEC-010`, `Cargo.toml`; `DEC-002`/`DEC-003`/`DEC-005`.

## Verify — run it, don't trust names

```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo build
command -v cargo-deny >/dev/null && cargo deny check licenses || echo "cargo-deny absent — read DEC-010 + Cargo tree for license"
./target/debug/skillport lint lint-fixtures/good ; echo good=$?    # 0, no findings
```

Adversarially probe:
- **Real tokenizer, not a heuristic:** find `body_token_count` and confirm it calls
  the tokenizer (tiktoken-rs BPE), not `chars()/4` or a word count. The pin test's
  expected number must be the tokenizer's actual output (recompute it if you can) —
  and must differ from `sample.chars().count()/4`.
- **`body.size` id + severity:** exactly `"body.size"`, severity **Info** (never
  error/warning). No other rule's severity changed (grep the severities).
- **Threshold + gating:** fires only when tokens `> 5000`; a body just under →
  none; runs inside `check_body` (Present-gated), consistent with body.empty/lines.
- **Determinism:** BPE built once (a `OnceLock`/static, not per-skill); same body →
  same count. No `HashMap`-order leakage in output.
- **Good fixture 0/0/0:** its small body must not trip `body.size`.
- **DEC-010 + licenses:** the crate choice is justified (proxy rationale), the dep
  + transitive licenses are permissive (cargo-deny `licenses` passes, or reason from
  the tree). Note the binary-size growth if relevant.
- **Scope:** no CLI/emitter change; no `--target`; no other new rule; `cargo test`
  green.

## Return a verdict (final message = data for the orchestrator)

**✅ APPROVED** / **⚠ PUNCH LIST** (numbered, file:line + failing case) /
**❌ REJECTED** (which criterion, concrete input → observed vs expected). Include
gate results, whether cargo-deny ran, the tokenizer-pin check, a per-AC pass/fail
summary, and judge DEC-010 + any deviations. Every flag needs a concrete
input→observed/expected. Don't touch code.
