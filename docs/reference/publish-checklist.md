# Reference: Publish Readiness Checklist

This document is the **crates.io-only** checklist for a `wasm4pm-compat` release. npm artifacts (`@wasm4pm/compat-ts`, `@wasm4pm/compat-wasm`) are out of scope here — they are gated and published by `.github/workflows/npm-release.yml` (fires on `v*` tags); see that workflow for the npm gate list.

All commands run on the pinned toolchain `nightly-2026-06-22` (`rust-toolchain.toml`). Status columns are left blank on purpose: re-verify each row on the exact release candidate before sign-off — a previously PASSing row is not evidence for this release.

---

## 1. Metadata and Crate Registry Audits

| Check Category | Description / Verification Method | Status | Notes / Known Constraints |
|---|---|---|---|
| **Crate Version** | Root `Cargo.toml` version matches the target release (currently `26.10.12`). |  | Workspace subcrates (`c8-*`) and derived ggen manifests stay at `26.6.x` — workspace-internal, excluded from the published package; do not gate on them. |
| **Registry Auth** | `cargo login <token>` completed (crates.io API token; stored by your credential provider). |  | `cargo publish --dry-run` contacts the registry and fails without it. |
| **Repository URL** | Cargo.toml `repository` field points to `https://github.com/seanchatmangpt/wasm4pm-compat`. |  | Matches actual repository URL. |
| **Readme Metadata** | Cargo.toml `readme` points to `README.md`. |  | Configured in root manifest. |
| **License Files** | `LICENSE-MIT` and `LICENSE-APACHE` present in the package root. |  | Both files exist and are packaged. |
| **Feature Model** | Exactly **three** features: `formats`, `strict`, `wasm4pm`. |  | `ts` and `wasm` are retired. |

---

## 2. Compilation and Code Quality Checks

| Check Category | Command / Verification Method | Status | Notes |
|---|---|---|---|
| **Styling & Fmt** | `cargo fmt --check` (`cargo make fmt`) |  | Re-run on the release candidate. |
| **Static Analysis** | `cargo clippy --all-features -- -D warnings` (`cargo make clippy`) |  | Warnings as errors. |
| **Library Build** | `cargo build --all-features` |  | Compiles successfully. |

---

## 3. Test Suite and Boundary Verification

| Check Category | Command / Verification Method | Status | Notes |
|---|---|---|---|
| **Unit & Integration** | `cargo test --all-features --tests` |  | Re-run on the release candidate. |
| **Doctests** | `cargo test --doc --all-features` (explicit opt-in; disabled in the default run) |  | `cargo make doc-test`. |
| **Trybuild (UI)** | `cargo test --test ui_tests -- --ignored` (the ALIVE gate, `cargo make alive`) |  | Compile-fail fixtures must fail for the intended named law, not accidentally. |

---

## 4. Packaging and Dry-Run Verification

| Check Category | Command / Verification Method | Status | Notes |
|---|---|---|---|
| **File Exclusions** | `cargo package --list --allow-dirty` |  | No `.DS_Store`, `target/`, `.agents/`, `ggen/`, or `c8-*` paths in the listing. |
| **Dry-Run Publish** | `cargo publish --dry-run --allow-dirty` |  | Ends with `warning: aborting upload due to dry run`; requires registry login (see §1). |

---

## 5. npm Release (cross-reference, out of scope)

| Step | Where | Status | Requirement |
|---|---|---|---|
| **npm packages** | `.github/workflows/npm-release.yml` |  | `@wasm4pm/compat-ts` and `@wasm4pm/compat-wasm` publish only on `v*` tags, after the workflow's own build/test/`npm publish --dry-run` gates. Do not run `npm publish` locally. |

---

## 6. Deployment Sign-off

| Step | Responsible Role | Status | Requirement |
|---|---|---|---|
| **Actual Publish** | Human / Orchestrator | 🛑 | **DO NOT AUTOMATE**. A human operator must manually run the final `cargo publish` from a clean tagged commit after verifying this checklist. |

---

## Related Documentation

- Back to [README](../../README.md)
- [Preparing for a Crates.io Release](../how-to/prepare-crates-io-publish.md)
- [NPM Release Workflow](../../.github/workflows/npm-release.yml)
- [Version Policy](../VERSION_POLICY.md)
- [Feature Model](feature-model.md)
