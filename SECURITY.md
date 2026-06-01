# Security Policy

## Scope

`wasm4pm-compat` is a structure-only, compile-time type-law crate with no runtime dependencies,
no network access, no file I/O, and `#![forbid(unsafe_code)]`. The attack surface is limited to
the Rust compiler and the nightly toolchain.

## Supported Versions

Only the current `main` branch is supported. The crate is nightly-only and pins a specific
toolchain via `rust-toolchain.toml`. No stable or LTS release line exists.

## Reporting a Vulnerability

If you discover a security issue (e.g. a soundness hole in the type law, an unsafe pattern
introduced by a dependency, or a supply-chain concern), please open a GitHub issue tagged
`security`. For sensitive disclosures, email the maintainer directly.

## Known Non-Issues

- `generic_const_exprs` and `adt_const_params` are nightly features under active development.
  Compiler bugs in these features (e.g. E0391 cycles) are tracked as known nightly regressions,
  not security vulnerabilities.
- The crate has zero runtime dependencies. `cargo audit` will always report a clean bill.
