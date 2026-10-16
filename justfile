# funkalization justfile — local/CI workflow (ecosystem §1: `just ci` is the gate)

set shell := ["bash", "-uc"]

# Default: build and test
default: build test

# === Build ===

build:
    cargo build

build-release:
    cargo build --release

install:
    cargo install --path .

# === Test ===

test:
    cargo test

# === Lint ===

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

clippy:
    cargo clippy --locked --all-targets -- -D warnings

# === CI gate (ecosystem §1: one way to evaluate a tool) ===

ci:
    cargo fmt --all -- --check
    cargo clippy -- -D warnings
    cargo test

# === Docs (ecosystem §2) ===

docs-build:
    mdbook build

docs-clean:
    rm -rf book
