#!/usr/bin/env bash
set -euo pipefail

# DFCM feature lattice: the crate deliberately has exactly three public feature
# axes, so execute every explicit cell of the 2^3 powerset. Keep the default
# profile as a separate contract because changing `default` is itself a public
# compatibility change even when it currently aliases the `formats` singleton.
cargo build --locked --no-default-features
cargo build --locked --no-default-features --features formats
cargo build --locked --no-default-features --features strict
cargo build --locked --no-default-features --features wasm4pm
cargo build --locked --no-default-features --features formats,strict
cargo build --locked --no-default-features --features formats,wasm4pm
cargo build --locked --no-default-features --features strict,wasm4pm
cargo build --locked --all-features
cargo build --locked

# Exercise the public connector/macro surface as an external crate consumer in
# the default profile and every explicit powerset cell containing `formats`.
cargo test --locked --test basic_surfaces
cargo test --locked --no-default-features --features formats --test basic_surfaces
cargo test --locked --no-default-features --features formats,strict --test basic_surfaces
cargo test --locked --no-default-features --features formats,wasm4pm --test basic_surfaces
cargo test --locked --all-features --test basic_surfaces

# Packaging is part of the repository verification ladder. Dry-run publication
# validates registry packaging without mutating the registry.
cargo package --locked
cargo publish --dry-run --locked
