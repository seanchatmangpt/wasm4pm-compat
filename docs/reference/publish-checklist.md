# Reference: Publish Readiness Checklist

This document provides a checklist of all verification steps and metadata audits required to approve a new release of `wasm4pm-compat` to crates.io.

---

## 1. Metadata and Crate Registry Audits

| Check Category | Description / Verification Method | Status | Notes / Known Constraints |
|---|---|---|---|
| **Crate Version** | Verify root version matches target release version (`26.6.13`). | ✅ | Workspace subcrates and configs are properly aligned to `26.6.13`. |
| **Repository URL** | Cargo.toml `repository` field must point to `https://github.com/seanchatmangpt/wasm4pm-compat`. |  | Matches actual repository URL. |
| **Readme Metadata** | Cargo.toml `readme` must point to `README.md`. |  | Configured properly in root manifest. |
| **License Files** | Verify presence of `LICENSE-MIT` and `LICENSE-APACHE` in the package root. |  | Both files exist in root directory. |
| **Feature Model** | Crate must have exactly **three** features: `formats`, `strict`, `wasm4pm`. |  | migrated features `ts` and `wasm` are retired. |

---

## 2. Compilation and Code Quality Checks

| Check Category | Command / Verification Method | Status | Notes |
|---|---|---|---|
| **Styling & Fmt** | `cargo fmt --check` | PASS | Adheres to rustfmt rules. |
| **Static Analysis** | `cargo clippy --all-features -- -D warnings` | PASS | Clean compilation under warnings-as-errors. |
| **Library Build** | `cargo build --all-features` | PASS | Compiles successfully. |

---

## 3. Test Suite and Boundary Verification

| Check Category | Command / Verification Method | Status | Notes |
|---|---|---|---|
| **Unit & Integration** | `cargo test --all-features --tests` | PASS | All 132 logical boundary tests passed. |
| **Doctests** | `cargo test --doc --all-features` | PASS | Standard inline doc examples compile and pass (exactly zero warnings). |
| **Trybuild (UI)** | `cargo test --test ui_tests -- --ignored` | PASS | All 409 UI tests compiled and passed. |

---

## 4. Packaging and Dry-Run Verification

| Check Category | Command / Verification Method | Status | Notes |
|---|---|---|---|
| **File Exclusions** | `cargo package --list` | PASS | Tracked .DS_Store untracked and LSP build directories ignored. |
| **Dry-Run Publish** | `cargo publish --dry-run` | PASS | Passes cleanly with dry-run upload aborted. |

---

## 5. Deployment Sign-off

| Step | Responsible Role | Status | Requirement |
|---|---|---|---|
| **Actual Publish** | Human / Orchestrator | 🛑 | **DO NOT AUTOMATE**. A human operator must manually run the final `cargo publish` command after verifying this checklist. |

---

## Related Documentation

- Back to [README](../../README.md)
- [Preparing for a Crates.io Release](../how-to/prepare-crates-io-publish.md)
- [Verification Report](../reports/v26.6.13-verification-report.md)
- [Feature Model](feature-model.md)
