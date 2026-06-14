# wasm4pm-compat: Feature Audit Report

This report documents the audit of the feature flags model in the `wasm4pm-compat` crate. It confirms the active public features, identifies orphan files and directories in the filesystem, and highlights stale references to retired features in code, tests, and documentation.

---

## 1. Active Public Features Verification

An audit of [Cargo.toml](file:///Users/sac/wasm4pm-compat/Cargo.toml) confirms that the crate defines exactly three public Cargo features. 

### Cargo.toml Feature Declarations
```toml
[features]
default = ["formats"]

# formats: Import/export contracts, round-trip claims, and loss surfaces.
formats = []

# strict: Opt-in boundary judgment with stricter admission/refusal surfaces.
strict = []

# wasm4pm: Graduation bridge to engine-facing contracts.
wasm4pm = []
```

### Module Gate Compilation Checks
In [src/lib.rs](file:///Users/sac/wasm4pm-compat/src/lib.rs#L230-L241), module-level compilation is correctly mapped to these features:
* `#[cfg(feature = "wasm4pm")] pub mod engine_bridge;`
* `#[cfg(feature = "formats")] pub mod formats;`
* `#[cfg(feature = "strict")] pub mod strict;`

There are no references to `pub mod ts;` or `pub mod wasm;` in [src/lib.rs](file:///Users/sac/wasm4pm-compat/src/lib.rs). The only active public features compiled into the crate are exactly `formats`, `strict`, and `wasm4pm`.

---

## 2. Orphan/Stale Directories in the Filesystem

Although retired from the crate's compilation module tree, two full directories of code and bindings remain on the filesystem under the `src/` directory. Because they are not declared as submodules in `src/lib.rs`, they are ignored by the compiler.

Additionally, the external dependencies they require (such as `specta`, `tsify`, `wasm-bindgen`, and `serde-wasm-bindgen`) are not listed under `[dependencies]` in [Cargo.toml](file:///Users/sac/wasm4pm-compat/Cargo.toml), which prevents them from failing compilation when building the crate.

### Stale Directory 1: TypeScript Bindings Code (`src/ts/`)
This directory contains 14 files for generating and exporting TypeScript typestate bindings via `specta` and `tsify`:
* [src/ts/mod.rs](file:///Users/sac/wasm4pm-compat/src/ts/mod.rs)
* [src/ts/export.rs](file:///Users/sac/wasm4pm-compat/src/ts/export.rs) (Generates type projection declarations)
* [src/ts/brand.rs](file:///Users/sac/wasm4pm-compat/src/ts/brand.rs)
* [src/ts/law_projection.rs](file:///Users/sac/wasm4pm-compat/src/ts/law_projection.rs)
* Shape files matching ontology layouts: `bpmn_ts.rs`, `causality_ts.rs`, `declare_ts.rs`, `multiperspective_ts.rs`, `petri_ts.rs`, `powl_ts.rs`, `prediction_ts.rs`, `process_tree_ts.rs`, `streaming_ts.rs`, `workflow_ts.rs`.

### Stale Directory 2: WASM Boundary Code (`src/wasm/`)
This directory contains 4 files exposing boundary DTOs and memory helpers for JS crossings:
* [src/wasm/mod.rs](file:///Users/sac/wasm4pm-compat/src/wasm/mod.rs)
* [src/wasm/abi.rs](file:///Users/sac/wasm4pm-compat/src/wasm/abi.rs) (Linear memory pointer alignment validation)
* [src/wasm/bindings.rs](file:///Users/sac/wasm4pm-compat/src/wasm/bindings.rs) (WASM boundary endpoints using `wasm_bindgen`)
* [src/wasm/boundary.rs](file:///Users/sac/wasm4pm-compat/src/wasm/boundary.rs) (WASM boundary structs using `tsify`)

---

## 3. Leftover Feature Gates in Code and Tests

Stale references to the retired `ts` and `wasm` features are still present in active tests and ignored files:

### Code Leftovers
* **[src/ts/export.rs#L8](file:///Users/sac/wasm4pm-compat/src/ts/export.rs#L8)** and **[src/ts/export.rs#L18](file:///Users/sac/wasm4pm-compat/src/ts/export.rs#L18)**:
  ```rust
  #[cfg(feature = "wasm")]
  let wasm_proj = ...
  #[cfg(not(feature = "wasm"))]
  let wasm_proj = String::new();
  ```

### Integration Test Leftovers
* **[tests/graduation.rs#L47](file:///Users/sac/wasm4pm-compat/tests/graduation.rs#L47)**:
  ```rust
  #[cfg(feature = "ts")]
  mod ts_tests { ... }
  ```
* **[tests/graduation.rs#L74](file:///Users/sac/wasm4pm-compat/tests/graduation.rs#L74)**:
  ```rust
  #[cfg(all(feature = "wasm", target_arch = "wasm32"))]
  mod wasm_tests { ... }
  ```
  *Note: These tests are silently ignored during cargo test runs because `ts` and `wasm` features are not present in Cargo.toml.*

---

## 4. Stale References in Documentation and Audit Manifests

A search across the workspace reveals several documentation, markdown specifications, and templates referring to the retired `ts` or `wasm` features:

1. **[emitted/environment-hooks.md](file:///Users/sac/wasm4pm-compat/emitted/environment-hooks.md)**:
   * Lines 42-43: Describes `ts` and `wasm` features under the `Cargo.toml Feature Model` table.
   * Lines 59-63: Lists `ts` and `wasm` module declarations as conditional exports inside `src/lib.rs`.
2. **[emitted/plugin-registry-systems.md](file:///Users/sac/wasm4pm-compat/emitted/plugin-registry-systems.md)**:
   * Lines 53-57 and 286-287: Refers to `ts` and `wasm` modules gated by `#[cfg(feature = "ts")]` and `#[cfg(feature = "wasm")]` in `lib.rs`.
3. **[ggen/audits/AUDIT_SPEC.md](file:///Users/sac/wasm4pm-compat/ggen/audits/AUDIT_SPEC.md)**:
   * Rule 3 (Line 30): `ts module is gated by #[cfg(feature = "ts")] in lib.rs`.
   * Rule 4 (Line 38): `wasm module is gated by #[cfg(feature = "wasm")] in lib.rs`.
   * Feature Model Integrity (Lines 58-59): `Feature count >= 3 (formats, strict, wasm4pm, ts, wasm)`.
4. **[emitted/audit-machinery/partial-checkpoint-analysis.md](file:///Users/sac/wasm4pm-compat/emitted/audit-machinery/partial-checkpoint-analysis.md)**:
   * Line 84: Performs validation via `if grep -q '#\[cfg(feature = "ts")\]' "$REPO_ROOT/src/lib.rs"`.
5. **[emitted/hardening/dto-classification.md](file:///Users/sac/wasm4pm-compat/emitted/hardening/dto-classification.md)**:
   * Lines 57, 63, and 89: References tests/graduation.rs gated under `#[cfg(all(feature = "wasm", target_arch = "wasm32"))]`.
6. **[ggen/intel/SPECTA-INTELLIGENCE-INDEX.md](file:///Users/sac/wasm4pm-compat/ggen/intel/SPECTA-INTELLIGENCE-INDEX.md)**:
   * Line 235: `#[cfg_attr(feature = "ts", derive(Type, Serialize, Deserialize))]`.
7. **[ggen/manifests/wasm-boundary-template.manifest.md](file:///Users/sac/wasm4pm-compat/ggen/manifests/wasm-boundary-template.manifest.md)**:
   * Line 75: `Feature Gate: Only available when target_arch = "wasm32" AND feature = "wasm" AND feature = "wasm4pm"`.
   * Lines 236-238: Uses `cfg_attr(all(target_arch = "wasm32", feature = "wasm"), ...)` attributes.
8. **[ggen/templates/README-wasm-boundary.md](file:///Users/sac/wasm4pm-compat/ggen/templates/README-wasm-boundary.md)**:
   * Lines 159, 165, and 168: Refers to export conditions based on `feature = "wasm"`.
   * Lines 178-180: Uses `cfg_attr(all(target_arch = "wasm32", feature = "wasm"), ...)` templates.
9. **[ggen/templates/wasm-boundary.rs.tera](file:///Users/sac/wasm4pm-compat/ggen/templates/wasm-boundary.rs.tera)**:
   * Lines 71, 74, 87-89, 102-104, 118-120, 135-137, 162-164, 209-211, 229-231, 252-254, and 278: Emits conditional compiled blocks using `feature = "wasm"`.

---

## 5. Summary and Recommendations

### Verdict
The only **active and compiled** Cargo features in the codebase are exactly `formats`, `strict`, and `wasm4pm`. 

However, because the `ts` and `wasm` features were previously retired to adhere to the strict 3-feature limit, the filesystem contains orphan modules that are skipped during compilation. Furthermore, code comments, tests, and documentation files continue to reference these retired features.

### Recommended Next Steps
1. **Clean up orphan files**: Completely delete the unused `src/ts/` and `src/wasm/` directories if TypeScript and WASM boundary generation are no longer intended for this crate.
2. **Remove dead tests**: Clean up the `#[cfg(feature = "ts")]` and `#[cfg(feature = "wasm")]` blocks inside [tests/graduation.rs](file:///Users/sac/wasm4pm-compat/tests/graduation.rs).
3. **Update documentation and templates**: Standardize the documentation files (`emitted/environment-hooks.md`, `emitted/plugin-registry-systems.md`, and `ggen/` templates) to reflect the exact 3-feature model.
