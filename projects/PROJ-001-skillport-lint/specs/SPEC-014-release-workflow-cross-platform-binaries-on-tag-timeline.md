# SPEC-014 timeline

Architect appends as cycles are designed. Executors update status as
they go. Status markers: `[ ]` not started · `[~]` in progress · `[x]` complete · `[?]` blocked.

Cycle prompts live in `prompts/SPEC-014-<cycle>.md`.

## Instructions

- [x] **design** — completed 2026-07-18 (architect: claude-opus-4-8) · probed build-info.sh (provenance), ci.yml action patterns, the DEC-009 5-target matrix; designed workflow_dispatch dry path so it's CI-testable without a tag
- [ ] **build** — prompt: `prompts/SPEC-014-build.md` (runs as a **Sonnet subagent** on branch `feat/spec-014-release-workflow`)
- [ ] **verify** — prompt: pending (waiting on build) — **Opus subagent**
- [ ] **ship** — prompt: pending (waiting on verify) — STAGE-004 step 2 (release workflow)
