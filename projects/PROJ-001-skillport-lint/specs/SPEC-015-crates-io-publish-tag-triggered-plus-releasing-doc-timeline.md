# SPEC-015 timeline

Architect appends as cycles are designed. Executors update status as
they go. Status markers: `[ ]` not started · `[~]` in progress · `[x]` complete · `[?]` blocked.

Cycle prompts live in `prompts/SPEC-015-<cycle>.md`.

## Instructions

- [x] **design** — completed 2026-07-18 (architect: claude-opus-4-8) · re-confirmed crates.io name `skillport` free (404); designed a tag-gated publish job on release.yml + a RELEASING runbook (publish/token/tag are human-only per DEC-009)
- [ ] **build** — prompt: `prompts/SPEC-015-build.md` (runs as a **Sonnet subagent** on branch `feat/spec-015-crates-publish`)
- [ ] **verify** — prompt: pending (waiting on build) — **Opus subagent**
- [ ] **ship** — prompt: pending (waiting on verify) — STAGE-004 step 3 (crates.io publish)
