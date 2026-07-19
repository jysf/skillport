# SPEC-013 timeline

Architect appends as cycles are designed. Executors update status as
they go. Status markers: `[ ]` not started · `[~]` in progress · `[x]` complete · `[?]` blocked.

Cycle prompts live in `prompts/SPEC-013-<cycle>.md`.

## Instructions

- [x] **design** — completed 2026-07-18 (architect: claude-opus-4-8) · incl. design-time probe: crates.io name check (`skillport` free/404), identity-inconsistency scan, current LICENSE = Apache-2.0
- [ ] **build** — prompt: `prompts/SPEC-013-build.md` (runs as a **Sonnet subagent** on branch `feat/spec-013-release-prep`)
- [ ] **verify** — prompt: pending (waiting on build) — **Opus subagent**
- [ ] **ship** — prompt: pending (waiting on verify) — first STAGE-004 spec
