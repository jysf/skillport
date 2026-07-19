---
insight:
  id: DEC-009
  type: decision
  confidence: 0.85
  audience:
    - developer
    - agent

agent:
  id: claude-opus-4-8
  session_id: null

project:
  id: PROJ-001
repo:
  id: skillport

created_at: 2026-07-18
supersedes: null
superseded_by: null

affected_scope:
  - ".github/workflows/**"
  - "Cargo.toml"
  - "action.yml"

tags:
  - distribution
  - release
  - packaging
---

# DEC-009: Distribution strategy — GitHub Releases + crates.io first; Homebrew deferred

## Decision

skillport ships to users via **GitHub Releases (prebuilt cross-platform binaries)**
and **crates.io** (`cargo install skillport`) to start. A **Homebrew tap** is
**deferred** until an Apple Developer key is available (for signing/notarizing the
macOS binaries). The release is cut **after STAGE-003 completes** (as a new
**STAGE-004: Release & distribution**), not before. Canonical repo identity is
**github.com/jysf/skillport**; the crate is **dual-licensed MIT OR Apache-2.0**.

## Context

skillport (the `lint` tool) is complete and CI-green but not distributed — the only
way to get it is to build from source. GitHub Releases with prebuilt binaries is the
foundation every other channel leans on (Homebrew points at the release tarballs,
the shipped Action can download the binary instead of `cargo install --git`,
cargo-binstall works for free). crates.io is cheap, independent, and right for the
Rust/CI audience. Homebrew is the best CLI UX but pairs naturally with macOS code
signing, so it waits for the Apple Developer key.

## Alternatives Considered

- **"Everything" (brew + cargo + releases + scoop + nix + AUR + deb)**
  - Why rejected: the long tail isn't worth the maintenance pre-traction.
- **Homebrew-first**
  - Why rejected: a tap needs the release binaries anyway, and signing/notarization
    wants the Apple key that isn't available yet.
- **Chosen: Releases + crates.io now, Homebrew after the Apple key**
  - Why selected: covers ~all CLI + Rust/CI users with modest effort; each channel
    builds on the GitHub Release foundation; defers only the signing-coupled channel.

## Consequences

- **Positive:** one foundation (Releases) unlocks the rest; users get `cargo install`
  immediately and prebuilt binaries per tag.
- **Negative:** macOS release binaries are unsigned until the Apple key lands (minor
  Gatekeeper friction; documented). Homebrew UX waits.
- **Neutral / human-only steps** (Claude prepares; the human triggers — these hit the
  publish/credential/account guardrails): `cargo publish` (crates.io token,
  irreversible), pushing the `v*` release tag, creating the `homebrew-tap` repo.

## Phase-0 pre-flight (resolve before any release)

- **Canonical identity = github.com/jysf/skillport** (repo renamed from `skillport0`).
  Make README, `.repo-context.yaml`, `action.yml`, and `Cargo.toml` `repository`
  consistent. (Git remote + `Cargo.toml repository` fixed in the commit that adds
  this DEC.)
- **License = MIT OR Apache-2.0** (Rust-idiomatic dual). Add `LICENSE-MIT` +
  `LICENSE-APACHE` (the release-prep spec does this) so the files match `Cargo.toml`.
- **Confirm `skillport` is free on crates.io** (human/network check) before publish.
- **Fill crates.io metadata:** `readme`, `keywords`, `categories`, `homepage`,
  `authors`.

## Attack plan — STAGE-004 (after STAGE-003)

1. **Release-prep / Phase-0** — identity + dual-license files + Cargo metadata.
2. **Release workflow** (`.github/workflows/release.yml`) — on `v*` tag, cross-compile
   a matrix (macOS arm64+x86_64, Linux x86_64+aarch64-musl, Windows x86_64), strip,
   archive, sha256, attach to the GitHub Release; stamp `just build-info` provenance.
3. **crates.io publish** — tag-triggered `cargo publish` job (or first publish manual).
4. **Action speedup** — `action.yml` downloads the release binary instead of
   `cargo install --git`.
5. **Cut v0.1.0** — `just new-release-spec` / `just next-version`; the human pushes the
   tag; verify each channel installs; README install matrix + CHANGELOG.

## Validation

Right if a user can `cargo install skillport` and download a working binary from a
GitHub Release for their OS. Revisit Homebrew once the Apple Developer key exists.

## References

- Related: DEC-005 (stable CLI/exit-code/SARIF contract consumers pin),
  DEC-006 (release-spec lane), `docs/versioning.md` (semver, `just next-version`).
- Shipped: SPEC-009 (the Action that will download the release binary).
