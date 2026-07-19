# SPEC-017 timeline

Architect appends as cycles are designed. Executors update status as
they go. Status markers: `[ ]` not started · `[~]` in progress · `[x]` complete · `[?]` blocked.

Cycle prompts live in `prompts/SPEC-017-<cycle>.md`.

## Instructions

- [x] **design** — completed 2026-07-19 (architect: claude-opus-4-8) · probed `just next-version` (v0.1.0, no bump), confirmed root CHANGELOG.md is template-owned (app notes → GitHub Release), read SPEC-014 asset names + the release.yml notes line
- [ ] **build** — prompt: `prompts/SPEC-017-build.md` (runs as a **Sonnet subagent** on branch `feat/spec-017-cut-v0-1-0`)
- [x] **verify** — completed 2026-07-19 (Opus subagent, 64,627 tok/~$0.43/~5 min) — ✅ APPROVED, 0 punch-list; all 5 install-matrix asset names cross-check vs release.yml (no 404s), rule-table drift guard intact, release.yml diff = notes flag only, Cargo.toml still 0.1.0
- [x] **ship** — completed 2026-07-19 (PR #17 squash-merged 21f3fa5) — **last PROJ-001 spec.** Backlog code-complete; the v0.1.0 release itself (tag + publish) is the human's, then STAGE-004 + PROJ-001 close.
