# How-To: Preparing for a Crates.io Release

This guide outlines the step-by-step procedure required to prepare the `wasm4pm-compat` crate for publishing to [crates.io](https://crates.io). It addresses version management, local verification routines, and release procedures.

---

## 1. Context & Manifest Version Alignment

Only the root crate is published to crates.io. Current versions (verified on 2026-10-12):
- **Root Crate (`wasm4pm-compat`)**: `26.10.12` in the root `Cargo.toml` — this is the version that gets published.
- **npm packages (`@wasm4pm/compat-ts`, `@wasm4pm/compat-wasm`)**: `26.10.12` in `wasm4pm-compat-ts/package.json` and `wasm4pm-compat-wasm/package.json`. These publish via `.github/workflows/npm-release.yml` on `v*` tags, not via `cargo publish`.
- **Workspace subcrates (`c8-time`, `c8-receipts`, `c8-instruments`, `c8-market`, `c8-adversary`)** and **derived ggen manifests (`ggen-witness.toml`, `ggen/ggen.toml`)** carry older versions (`26.6.x`); they are workspace-internal and excluded from the published package. They are not part of the crates.io release and do not gate it.

When preparing the release, the publisher bumps the root crate version — and the two npm package versions if npm artifacts ship in the same release.

---

## 2. Step-by-Step Release Preparation

Follow these steps in sequence to guarantee publish readiness:

### Step 2.1: Confirm a Clean Working Tree
Ensure the workspace is clean. Never discard uncommitted work with `git checkout --` — commit or stash deliberately through the normal workflow instead.
```bash
git status
```
*Verification: No unexpected modified files under `src/` or `tests/`.*

### Step 2.2: Verify Code Styling
Formatting must strictly follow the standard Rust configuration (equivalent to `cargo make fmt`):
```bash
cargo fmt --check
```
*Expected: Exit code 0 with no stdout/stderr output.*

### Step 2.3: Run Clippy Lints
Run the lints on all features, treating all warnings as hard errors (equivalent to `cargo make clippy`):
```bash
cargo clippy --all-features -- -D warnings
```
*Expected: Clean compilation with no warnings or errors.*

### Step 2.4: Execute the Test Suites
Run the core tests, UI (trybuild) tests, and documentation tests (`cargo make test-all` covers #1; the trybuild ALIVE gate and doctests are explicit opt-in):

```bash
# 1. Main Unit and Integration Tests
cargo test --all-features --tests

# 2. Trybuild Compile-time UI Tests (the ALIVE gate; `cargo make alive`)
cargo test --test ui_tests -- --ignored

# 3. Documentation Examples (explicit opt-in; doctests are disabled in the
#    default run — `cargo make doc-test`)
cargo test --doc --all-features
```

All of these run on the pinned toolchain from `rust-toolchain.toml` (`nightly-2026-06-22`). Trybuild tests and doc-tests depend on nightly features and strict compiler outputs; any output diffs or failures must be repaired (including `.stderr` fixture updates when nightly output legitimately changed) before publishing.

### Step 2.5: Verify the Package Manifest
List the files that will be packaged to ensure no junk files, tests, or agent metadata files are included:
```bash
cargo package --list --allow-dirty
```
*Verification (verified working on the current manifest, 2026-10-12): only source files, LICENSE files, README, benches, and examples should be listed. `.agents/`, `.DS_Store`, `target/`, `ggen/`, and the `c8-*` workspace subcrates must be excluded (the `exclude` block in `Cargo.toml` handles this).*

### Step 2.6: Run a Dry-Run Publish
Test the actual packaging and upload process without actually publishing:
```bash
cargo publish --dry-run --allow-dirty
```
*Verified current semantics (2026-10-12): this packages (~1257 files, ~3.0 MiB), updates the crates.io index, requires a logged-in registry token, prints `Uploading wasm4pm-compat v<version>`, then `warning: aborting upload due to dry run`. If you are not logged in, run `cargo login <token>` first (a crates.io API token, scoped to `publish-new`/`publish-update`; the token is stored by your credential provider and `cargo login` itself never publishes anything). `--allow-dirty` is needed because a release checkout normally carries in-flight doc/status churn; the real `cargo publish` from a clean tagged commit does not need it.*

### Step 2.7: Cross-Reference the npm Release (non-cargo artifacts)
The two npm packages (`@wasm4pm/compat-ts`, `@wasm4pm/compat-wasm`) are NOT published by `cargo publish`. They publish through `.github/workflows/npm-release.yml`, which fires on `v*` tags (e.g. `v26.10.12`) and runs its own build/test/`npm publish --dry-run` gates before `npm publish --access public`. Pushing a `v*` tag is the only action required here — see the workflow file for the full npm gate list.

---

## 3. Human Approval Requirement

**CRITICAL REQUIREMENT**: The final step of actually publishing to crates.io (`cargo publish`) **MUST NOT** be automated. It requires explicit human approval and verification from the orchestrator.
Do not execute the final publish command unless you have received clear, written human sign-off to proceed.

---

## Related Documentation

- Back to [README](../../README.md)
- [Publish Readiness Checklist](../reference/publish-checklist.md)
- [Verification Report](../DEFINITION_OF_DONE.md)
