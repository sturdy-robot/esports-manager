# Every quality gate, runnable locally exactly as CI runs it (ADR 0008).
# `just` lists the recipes; `just check` runs them all.

set shell := ["bash", "-euo", "pipefail", "-c"]

# Branch that `mutants` compares against.
base := "origin/develop"

default:
    @just --list

# Run every gate. CI runs this same recipe.
check: fmt-check clippy arch vocab trace coverage deny machete reuse mutants

# Format all Rust code.
fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all --check

clippy:
    cargo clippy --workspace --all-targets --locked -- -D warnings

# Fast test run without coverage instrumentation.
test:
    cargo test --workspace --locked

# Crate graph matches ADR 0001.
arch:
    cargo xtask arch

# No genre vocabulary in the core crate (ADR 0002).
vocab:
    cargo xtask vocab

# Every scenario in an accepted spec has a test (ADR 0009).
trace:
    cargo xtask trace

# Runs the tests with coverage and checks each crate's minimum (quality.toml).
coverage:
    cargo llvm-cov --workspace --exclude xtask --locked --json --summary-only --output-path target/llvm-cov.json
    cargo xtask coverage target/llvm-cov.json

# HTML coverage report in target/llvm-cov/html.
coverage-html:
    cargo llvm-cov --workspace --exclude xtask --locked --html

# Licences, bans and sources. Advisories run weekly in CI (`just advisories`).
deny:
    cargo deny --locked check bans licenses sources

advisories:
    cargo deny --locked check advisories

machete:
    cargo machete

reuse:
    reuse lint

# Mutation testing on the code changed since `base` (ADR 0008).
mutants:
    mkdir -p target
    git diff "$(git merge-base {{base}} HEAD)" > target/mutants.diff
    cargo mutants --workspace --in-diff target/mutants.diff

# Full mutation run (weekly in CI).
mutants-all:
    cargo mutants --workspace
