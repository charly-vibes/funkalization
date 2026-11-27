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

# === Research record (kaya) ===

# Capture a snapshot: observe wai and git, then rebuild the record.
# Serialized against concurrent captures via a lockfile; skipped when the
# tree is dirty (checked under the lock, so a queued capture re-checks),
# so a capture never appends over half-finished work.
record-capture:
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p .kaya
    exec 9>.kaya/capture.lock
    flock 9
    if [ -n "$(git status --porcelain)" ]; then
        echo "record-capture: SKIP - working tree is dirty; commit or stash first"
        exit 0
    fi
    kaya import wai --project funkalization
    kaya import git --project funkalization --exclude-message-prefix "chore(record):"
    kaya build

# Verify the committed record against its projections without writing
record-verify:
    kaya check
