# How-To: Running the ALIVE Gate (Trybuild Verification)

This guide shows you how to run and manage the ALIVE Gate UI compilation test suite for `wasm4pm-compat` version `26.6.13`. 

---

## What is the ALIVE Gate?

The ALIVE Gate is the name of our compile-time verification test suite. It uses `trybuild` to compile mini-programs located under `tests/ui/` to prove that:
1. **Compile-Pass Fixtures**: Lawful code (e.g. valid place-to-transition arc relations) compiles cleanly.
2. **Compile-Fail Fixtures**: Unlawful code (e.g. place-to-place arc relations or double-spent tokens) fails compilation exactly at the expected type checking boundaries, generating predicted diagnostic errors.

These fixtures serve as the formal **type-law receipts** for the migrated.

---

## Step 1: Execute the ALIVE Gate

The gate currently covers **217 compile-fail** fixtures and **410 compile-pass** fixtures.

```bash
cargo make alive
```

This is equivalent to `cargo test --test ui_tests -- --ignored`. Use `cargo make alive` in the dev loop.

---

## Step 2: Interpret Failures

When running the ALIVE gate, failures generally indicate one of three issues:

### 1. Diagnostic Output Drift (Pattern Mismatches)
When the nightly compiler version updates, its diagnostic output layout can change slightly, causing trybuild to fail because the printed stderr doesn't match the checked-in `.stderr` file.
- **Fix**: Run the command with `TRYBUILD=overwrite` to automatically capture and write the current compiler output:
  ```bash
  TRYBUILD=overwrite cargo test --test ui_tests -- --ignored
  ```

### 2. Workspace Package Out of Sync (`no bin target named trybuild...`)
Trybuild creates temporary packages in the target directory. If these get cached improperly, cargo fails to map the compilation targets.
- **Fix**: Clear target cache and re-run:
  ```bash
  cargo clean
  cargo test --test ui_tests -- --ignored
  ```

### 3. Genuine Law Breach (Expected compilation path failed)
If a compile-pass fixture fails to compile, or a compile-fail fixture compiles successfully, it indicates a regression in the typestate implementation.
- **Fix**: Inspect the code changes in the library (such as `src/law.rs`, `src/state.rs`, or `src/evidence.rs`) to ensure traits and sealed bounds are correctly implemented.

---

## Step 3: Add a New Type-Law Receipt

When adding a new type-level boundary check, you must add corresponding pass/fail fixtures.

### Compile-fail fixture pattern

Use the **function-parameter pattern** — do not use `todo!()` to fabricate values. A typed parameter provides the value of the `pub(crate)`-constructed type; the type error fires at the call site:

```rust
// tests/ui/compile_fail/my_law_fail.rs
fn _test(ocel_ev: Evidence<String, Admitted, Ocel20>) {
    requires_xes_evidence(ocel_ev); // E0308 — wrong witness
}
```

For private-field non-forgeability fixtures, omit the private field entirely:

```rust
let _forged: WfNetConst<{ SoundnessState::Witnessed }> = WfNetConst {}; // E0063 + E0451
```

### Steps

1. Create `tests/ui/compile_pass/my_law_pass.rs` (lawful path compiles).
2. Create `tests/ui/compile_fail/my_law_fail.rs` (unlawful path using the function-parameter pattern).
3. Add both files to `tests/ui_tests.rs`.
4. Generate the `.stderr` snapshot:
   ```bash
   TRYBUILD=overwrite cargo test --test ui_tests -- --ignored
   ```
5. Confirm the snapshot matches on a clean run:
   ```bash
   cargo make alive
   ```
6. Commit both `.rs` and `.stderr` files.
