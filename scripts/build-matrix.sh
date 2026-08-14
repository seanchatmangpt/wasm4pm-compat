#!/usr/bin/env bash
set -euo pipefail

# Complete public Cargo-feature lattice. The crate deliberately has exactly three
# public features, so these builds cover the empty set, default profile, each
# singleton capability, and the all-feature closure.
cargo build --locked --no-default-features
cargo build --locked
cargo build --locked --no-default-features --features formats
cargo build --locked --no-default-features --features strict
cargo build --locked --no-default-features --features wasm4pm
cargo build --locked --all-features

# Exercise the public connector/macro surface as an external crate consumer.
cargo test --locked --all-features --test basic_surfaces

# Packaging is part of the repository verification ladder. Dry-run publication
# validates registry packaging without mutating the registry.
cargo package --locked
cargo publish --dry-run --locked
