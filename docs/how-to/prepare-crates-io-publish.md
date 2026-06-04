# How-To: Preparing for a Crates.io Release

This guide outlines the step-by-step procedure required to prepare the `wasm4pm-compat` crate for publishing to [crates.io](https://crates.io). It addresses version management, local verification routines, and release procedures.

---

## 1. Context & Manifest Version Discrepancies

A known constraint in the `wasm4pm-compat` workspace is the **version discrepancy** between the root crate and the workspace subcrates:
- **Root Crate (`wasm4pm-compat`)**: Configured at version `26.6.4` in the root `Cargo.toml`.
- **Subcrates (`c8-time`, `c8-receipts`, `c8-instruments`, `c8-market`, `c8-adversary`)**: Frozen at version `0.1.0` due to historical integration constraints.
- **Derived Manifests (`ggen-witness.toml`, `ggen/package.toml`)**: Also frozen at version `0.1.0`.

When preparing the release, the publisher must ensure this discrepancy does not trigger unexpected compilation or resolution failures.

---

## 2. Step-by-Step Release Preparation

Follow these steps in sequence to guarantee publish readiness:

### Step 2.1: Revert Debug or Temporary Code Changes
Ensure the workspace is completely clean. Code modification and test changes must be isolated to prevent dirty builds.
```bash
git checkout -- src/ tests/
git status
```
*Verification: Git status should show no modified files under `src/` or `tests/`.*

### Step 2.2: Verify Code Styling
Formatting must strictly follow the standard Rust configuration:
```bash
cargo fmt --check
```
*Expected: Exit code 0 with no stdout/stderr output.*

### Step 2.3: Run Clippy Lints
Run the lints on all features, treating all warnings as hard errors:
```bash
cargo clippy --all-features -- -D warnings
```
*Expected: Clean compilation with no warnings or errors.*

### Step 2.4: Execute the Test Suites
Run the core tests, UI (trybuild) tests, and documentation tests:

```bash
# 1. Main Unit and Integration Tests
cargo test --all-features --tests

# 2. Trybuild Compile-time UI Tests
cargo test --test ui_tests -- --ignored

# 3. Documentation Examples
cargo test --doc --all-features
```

*Note: Since the trybuild tests and doc-tests depend on nightly features and strict compiler outputs, any output diffs or failures must be reported and categorized under `CLAUDE_TRACK` if they require source code adjustments.*

### Step 2.5: Verify the Package Manifest
List the files that will be packaged to ensure no junk files, tests, or agent metadata files are included:
```bash
cargo package --list
```
*Verification: Verify that only source files, LICENSE files, README, and necessary build scripts are listed. `.agents/` and build directories (`target/`) must be automatically excluded (as configured in the `exclude` block in `Cargo.toml`).*

### Step 2.6: Run a Dry-Run Publish
Test the actual packaging and upload process without actually publishing:
```bash
cargo publish --dry-run
```
*Verification: Cargo compiles the package and simulates publishing. It should complete successfully.*

---

## 3. Human Approval Requirement

**CRITICAL REQUIREMENT**: The final step of actually publishing to crates.io (`cargo publish`) **MUST NOT** be automated. It requires explicit human approval and verification from the orchestrator.
Do not execute the final publish command unless you have received clear, written human sign-off to proceed.

---

## Related Documentation

- Back to [README](../../README.md)
- [Publish Readiness Checklist](../reference/publish-checklist.md)
- [Verification Report](../reports/v26.6.4-verification-report.md)
