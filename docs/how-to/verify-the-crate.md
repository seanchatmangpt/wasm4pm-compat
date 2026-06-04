# How-To: Verifying the Crate and Resolving Test Failures

This guide provides step-by-step instructions for executing the full validation suite of `wasm4pm-compat` version `26.6.4` on Mac OS, and details how to resolve common doc-test and UI trybuild mismatches.

---

## 1. Run Standard Formatting and Compilation Checks

To ensure styling and basic compilations are valid, execute the following commands in the root of the repository:

```bash
# Check code formatting compliance
cargo fmt --check

# Validate compilation of all features
cargo check --all-features
```

---

## 2. Run the Main Unit and Integration Test Suite

Execute the primary test suite which validates the type boundaries, witness lattices, and evidence containers:

```bash
cargo test --all-features --tests
```

*Expected output: All 132 tests pass successfully with 0 failures.*

---

## 3. Run and Resolve Doc-Test Failures

If you run the doc-test suite:

```bash
cargo test --doc --all-features
```

You might encounter failures in modules like `ocpq`, `petri`, or `xes` due to strict compile-time signature changes. To resolve specific failures:

### A. Resolve `petri::PetriNetBuilder` or `silent`/`transition` failures
- Ensure you call builder methods by chaining them immediately or passing ownership rather than trying to call them on a mutable reference receiver.
- Example: Chain `.place(...)` and `.build()` directly on the builder object by value.

### B. Resolve `petri::WfNet::attest_witnessed` private errors
- Do not call `.attest_witnessed()` directly in external code or doc examples, as it is marked `pub(crate)` to maintain soundness. Wrap verification in the public `WfNet` query interfaces instead.

---

## 4. Execute and Fix UI (Trybuild) Test Failures

UI tests ensure that invalid code fails to compile exactly as expected. These are located in `tests/ui/compile_fail/`.

Run the UI test suite:

```bash
cargo test --test ui_tests -- --ignored
```

If you see failures, they usually fall into two categories:

### Case A: Compiler Diagnostic Output Mismatches
If the compiler's printed diagnostic type names differ slightly from the expected `.stderr` files (for example, printing `wasm4pm_compat::witness::Ocel20` instead of `Ocel20`), you can automatically overwrite the stale expected outputs with the current compiler's output:

```bash
# Force trybuild to update expected stderr outputs
TRYBUILD=overwrite cargo test --test ui_tests -- --ignored
```

### Case B: Target Binary Resolution Errors (`no bin target named trybuild...`)
If you see errors where cargo cannot map temporary trybuild workspace package binaries, the package cache is out of sync. Clean the workspace and re-run:

```bash
# Clean the build directory to clear the trybuild target cache
cargo clean

# Re-run UI tests
cargo test --test ui_tests -- --ignored
```

---

## Conclusion

Following these steps ensures that the codebase remains fully compliant with the quality definitions required for release version `26.6.4`.
