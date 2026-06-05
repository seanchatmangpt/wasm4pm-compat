# How-To: Running the ALIVE Gate (Trybuild Verification)

This guide shows you how to run and manage the ALIVE Gate UI compilation test suite for `wasm4pm-compat` version `26.6.5`. 

---

## What is the ALIVE Gate?

The ALIVE Gate is the name of our compile-time verification test suite. It uses `trybuild` to compile mini-programs located under `tests/ui/` to prove that:
1. **Compile-Pass Fixtures**: Lawful code (e.g. valid place-to-transition arc relations) compiles cleanly.
2. **Compile-Fail Fixtures**: Unlawful code (e.g. place-to-place arc relations or double-spent tokens) fails compilation exactly at the expected type checking boundaries, generating predicted diagnostic errors.

These fixtures serve as the formal **type-law receipts** for the compatibility layer.

---

## Step 1: Execute the ALIVE Gate

To run the UI test suite, pass the target name `ui_tests` and use the `--ignored` flag:

```bash
cargo test --test ui_tests -- --ignored
```

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

When adding a new type-level boundary check, you must add corresponding pass/fail fixtures:

1. Create a passing test under `tests/ui/compile_pass/my_law_pass.rs`.
2. Create a failing test under `tests/ui/compile_fail/my_law_fail.rs`.
3. Add the files to `tests/ui_tests.rs`.
4. Run:
   ```bash
   TRYBUILD=overwrite cargo test --test ui_tests -- --ignored
   ```
   This will generate the required `.stderr` check files for `my_law_fail.rs`.
5. Verify and commit the generated `.stderr` files as part of the source repository.
