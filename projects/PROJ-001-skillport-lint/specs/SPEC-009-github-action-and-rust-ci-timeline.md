# SPEC-009 timeline

Architect appends as cycles are designed. Executors update status as
they go. Status markers: `[ ]` not started · `[~]` in progress · `[x]` complete · `[?]` blocked.

Cycle prompts live in `prompts/SPEC-009-<cycle>.md`.

## Instructions

- [x] **design** — completed 2026-07-18 (architect: claude-opus-4-8)
- [ ] **build** — prompt: `prompts/SPEC-009-build.md` (runs as a **Sonnet subagent** on branch `feat/spec-009-action`)
- [ ] **verify** — prompt: `prompts/SPEC-009-verify.md` (runs as an **Opus subagent**; waiting on build)
- [ ] **ship** — prompt: pending (waiting on verify)
