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

# Copy deployed specs into the book source (same step as docs.yml — keep
# the two in sync)
specs-docs:
    mkdir -p docs/src/specs
    for dir in openspec/specs/*/; do \
        name=$(basename "$dir"); \
        cp "$dir/spec.md" "docs/src/specs/$name.md"; \
    done

# §6: version claims come from Cargo.toml, not hand-typed text — build artifact
release-status:
    VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1); \
    DATE=$(date +%Y-%m-%d); \
    printf '# Release %s\n\n- **Version:** %s (from Cargo.toml)\n- **Build ref:** %s\n- **Generated:** %s\n' \
        "$VERSION" "$VERSION" "$(git rev-parse --short=7 HEAD)" "$DATE" > docs/src/release.md

# Build the mdBook with specs and release page rendered, then place the
# publication and research trees inside the artifact (pattern:
# charly-vibes/kaya justfile:docs @ v0.4.1)
docs-build: specs-docs release-status
    mdbook build
    python3 scripts/build-publications.py

# Fail when the built book contains a relative link the artifact cannot resolve
links: docs-build
    python3 scripts/check-links.py

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

# Full record gate: verify, lag (mode warn|fail)
record-check mode="warn": record-verify
    python3 scripts/record-lag.py --mode {{mode}} --bound 60
