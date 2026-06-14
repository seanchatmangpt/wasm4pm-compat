# How-To: Verifying the Crate and Resolving Test Failures

This guide provides step-by-step instructions for executing the full validation suite of `wasm4pm-compat` v26.6.13, and details how to resolve common trybuild mismatches.

**Always use `cargo make`.** Direct `cargo` invocations are reserved for running a single test by name. See `Makefile.toml` for the full recipe list.

---

## 1. Formatting and Compilation

```bash
# Check code formatting
cargo make fmt

# Type-check all features
cargo make check-all
```

---

## 2. Unit and Integration Tests

```bash
cargo make test-all
```

Expected: 33 integration tests pass, 0 failures.

---

## 3. Doc-Tests (explicit opt-in — slow)

```bash
cargo make doc-test
```

Doc-tests are disabled in the default test run because each doctest that touches `generic_const_exprs` or `adt_const_params` types spawns a separate nightly `rustc` invocation; 200+ invocations make the loop take 4+ minutes. Run this explicitly before a release.

If you see failures in `ocpq`, `petri`, or `xes`:

- **`petri::PetriNetBuilder` / `silent` / `transition` failures**: Chain builder methods by value; do not call them on a mutable reference receiver.
- **`petri::WfNet::attest_witnessed` private errors**: This method is `pub(crate)`. Use the public `WfNet` query interfaces instead.

---

## 4. Type-Law Receipt Gate (ALIVE)

The ALIVE gate runs 217 compile-fail fixtures and 410 compile-pass fixtures via trybuild:

```bash
cargo make alive
```

Compile-fail fixtures use the **function-parameter pattern** — no `todo!()`. A typed parameter provides the value; the type error fires at the call site inside `_test`:

```rust
fn _test(xes_ev: Evidence<String, Admitted, Xes1849>) {
    requires_ocel_evidence(xes_ev); // E0308 — proven at compile time
}
```

### Resolving snapshot mismatches

If the compiler's printed diagnostic type names drift from the `.stderr` snapshots (e.g. after a nightly update):

```bash
# Regenerate all snapshots from current compiler output
TRYBUILD=overwrite cargo test --test ui_tests -- --ignored

# Confirm all snapshots match on the second run
cargo make alive
```

### Resolving target resolution errors

If cargo cannot map trybuild workspace package binaries:

```bash
# Last resort — clean the build cache and re-run
cargo clean
cargo make alive
```

---

## 5. Full CI Pipeline

```bash
cargo make ci
```

Runs check-all, test-all, clippy, fmt, and alive in sequence. Use this before pushing.

---

## 6. Anti-Cheat Gate

```bash
just anti-cheat-gate
```

Scans `src/` law modules, `tests/`, and `wasm4pm-compat-lsp/src/` for fabricated evidence patterns. Per-repo suppressions for domain vocabulary live in `anti-llm.toml`.
