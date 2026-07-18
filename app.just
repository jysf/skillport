# app.just — skillport's project commands (build, test, run, lint).
#
# PROJECT-OWNED. The template-managed root `justfile` imports this
# (`import? 'app.just'`), so these run as `just build`, `just test`, … alongside
# the workflow commands (`status`, `new-spec`, …). Keep app recipes HERE so a
# template update never clobbers them. Mirrors AGENTS.md §6.
#
# NOTE: skillport is mid-build. The library substrate (parse/walk/rules/report)
# works and is tested; the real `lint` CLI is a pending STAGE-002 spec, so
# `main.rs` is still a stub. Until then, use `just demo` to see the library
# validate real skills end-to-end.

# Dev build
build:
    cargo build

# Release binary -> target/release/skillport
build-release:
    cargo build --release

# Run the binary (currently a stub until the lint CLI spec lands)
run *ARGS:
    cargo run -- {{ARGS}}

# Run the whole test suite (unit + integration + fixtures)
test:
    cargo test

# Run a single test by name substring, e.g. `just test-one name.charset`
test-one NAME:
    cargo test {{NAME}}

# Clippy with warnings denied (matches CI)
clippy:
    cargo clippy --all-targets -- -D warnings

# Format the code
fmt:
    cargo fmt

# Check formatting without writing (matches CI)
fmt-check:
    cargo fmt --check

# Fast type/borrow check, no codegen
check:
    cargo check --all-targets

# Build + open the API docs for eyeballing the substrate
doc:
    cargo doc --no-deps --open

# The gates a spec must pass before ship (fmt + clippy + test)
verify-all: fmt-check clippy test

# DEMO: exercise the shipped library against a path (default: the good/bad
# fixtures) and print findings — a stand-in until the real `lint` CLI ships.
# Usage: `just demo`  or  `just demo path/to/skills`
demo PATH="lint-fixtures":
    cargo run --quiet --example lint_demo -- {{PATH}}
