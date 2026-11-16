# Version Policy

## Versioning scheme

`wasm4pm-compat` uses calendar versioning: `YY.MM.PATCH`.

The current version is `26.10.12`.

## Scope of a version bump

A `26.10.x` bump of the root version covers exactly these release units, bumped together:

- the root crate `wasm4pm-compat` (`Cargo.toml`)
- the two npm packages `@wasm4pm/compat-ts` and `@wasm4pm/compat-wasm`
- `bindings/elixir` (`mix.exs`)
- the versioned ggen manifests under `ggen/`

The following are **intentionally out of scope** and remain on their own cadence (currently
`26.6.x`) by design:

- the `c8-*` sidecar crates (`c8-receipts`, and the other workspace-excluded `c8-*` dirs) —
  they are excluded from the published package and are independent release units with
  independent cadence
- `wasm4pm-compat-lsp` — likewise packaging-excluded, independent release unit

Being on a different version than the root is not version drift for these units; it is the
declared policy.

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

The release gates below are the real, currently-defined gates (`Makefile.toml`, `scripts/audit/`, pinned toolchain `nightly-2026-06-22`):

1. Bump the root `Cargo.toml` version (`wasm4pm-compat`). This is the only published Rust crate — workspace subcrates (`c8-*`) and the derived ggen manifests stay at their own versions and are excluded from the published package.
2. Run the full crown audit: `./scripts/audit/audit_crown_gate_all.sh`
3. Run the fast verification ladder (`cargo make ci` = `check-all` + `test-all` + clippy + fmt + ALIVE gate): `cargo test --all-features --tests`, `cargo test --test ui_tests -- --ignored` (trybuild ALIVE gate, `cargo make alive`), and explicitly if doc surfaces changed: `cargo test --doc --all-features`
4. Packaging check: `cargo package --list --allow-dirty`, then `cargo publish --dry-run --allow-dirty` (requires `cargo login`; ends with `warning: aborting upload due to dry run`).
5. Update `CHANGELOG.md`
6. Tag the ALIVE milestone: `git tag wasm4pm-compat-paperlaw-crown-alive-NNN`. If npm artifacts ship in this release, additionally push a `v*` tag (e.g. `v26.10.12`) — that tag is the trigger for `.github/workflows/npm-release.yml`, which builds, tests, dry-run-gates, and publishes `@wasm4pm/compat-ts` and `@wasm4pm/compat-wasm`.
7. Push tag(s) to remote. The final `cargo publish` itself is human-executed only (see `docs/reference/publish-checklist.md` §6).
