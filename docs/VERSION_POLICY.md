# Version Policy

## Versioning scheme

`wasm4pm-compat` uses calendar versioning: `YYYY.MM.PATCH`.

The current version is `2026.05.0` (May 2026 crown release).

## What constitutes a breaking change

- Removing a public type, trait, or module
- Changing a public type's fields or generic parameters in a non-additive way
- Tightening a type bound (making previously-valid code invalid)
- Changing the set of public Cargo features
- Changing `rust-toolchain.toml` to a nightly that breaks existing compile-pass fixtures

## What does not constitute a breaking change

- Adding new types, modules, or impls
- Loosening a type bound (making previously-invalid code valid)
- Adding new variants to non-exhaustive enums
- Updating `.stderr` files when nightly changes compiler output
- Adding new benchmark targets
- Adding new examples

## Release process

1. Run full crown audit: `./scripts/audit/audit_crown_gate_all.sh`
2. Run `cargo test --all-features --tests` and `cargo test --test ui_tests -- --ignored`
3. Update `CHANGELOG.md`
4. Tag with ALIVE milestone: `git tag wasm4pm-compat-paperlaw-crown-alive-NNN`
5. Push tag to remote
