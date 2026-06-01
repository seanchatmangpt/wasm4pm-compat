# Feature Isolation Audit Specification

## Artifact
`audit-feature-isolation.sh.ggen`

Location: `/Users/sac/wasm4pm-compat/ggen/audits/audit-feature-isolation.sh.ggen`

## Purpose

Manufacture audit for feature isolation conformance. Verifies that Cargo features are properly isolated and do not leak dependencies or capabilities across feature boundaries.

## Audit Rules

### Rule 1: Default Feature Dependency Isolation
**Check:** Default feature (formats) does not enable forbidden crates.

- ✗ FAIL: `specta`, `tsify`, `wasm-bindgen`, `serde-wasm-bindgen` must NOT be in default feature deps
- ✓ PASS: Default feature has no forbidden crate dependencies

### Rule 2: Default Feature Code Isolation
**Check:** Always-on modules do not use wasm-bindgen or tsify bindings.

- ✗ FAIL: `src/*.rs` files must not import or use `wasm_bindgen` or `tsify`
- ✓ PASS: No wasm-bindgen or tsify usage in always-on modules

### Rule 3: TypeScript Feature Isolation
**Check:** `ts` feature is properly gated and isolated from WASM bindings unless paired.

- ✓ PASS: `ts` feature does not directly enable `wasm-bindgen`
- ✓ PASS: `ts` module is gated by `#[cfg(feature = "ts")]` in lib.rs
- ⚠ WARN: Submodules may not be explicitly gated (they inherit from parent)

### Rule 4: WASM Feature Engine Isolation
**Check:** `wasm` feature does not import engine-facing modules.

- ✗ FAIL: `src/wasm/*` must not import `engine_bridge`, `graduation`, `discovery`, `conformance_engine`, or `replay` modules
- ✓ PASS: WASM modules do not import engine-facing modules
- ✓ PASS: `wasm` module is gated by `#[cfg(feature = "wasm")]` in lib.rs

### Rule 5: WASM4PM Feature Gating
**Check:** `wasm4pm` feature is properly gated and engine types are isolated.

- ✓ PASS: `engine_bridge` module is gated by `#[cfg(feature = "wasm4pm")]` in lib.rs
- ✓ PASS: `engine_bridge.rs` declares the feature gate at the top
- ⚠ WARN: Always-on modules must not reference `GraduationReason` or `GraduationCandidate` without feature gates

### Cross-Feature Integrity Checks
**Check:** All feature-gated dependencies are declared as optional.

- ✓ PASS: `serde` is `optional = true`
- ✓ PASS: `specta` is `optional = true`
- ✓ PASS: `tsify` is `optional = true`
- ✓ PASS: `wasm-bindgen` is `optional = true`

### Feature Model Integrity
**Check:** Feature count and gating in lib.rs.

- ✓ PASS: Feature count >= 3 (formats, strict, wasm4pm, ts, wasm)
- ✓ PASS: Each feature has a corresponding `#[cfg(feature = "...")]` gate in lib.rs

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | All isolation rules PASS (violations = 0) |
| 1 | Feature isolation violation detected (violations > 0) |
| 2 | Audit setup error (Cargo.toml missing, invalid structure) |

## Usage

```bash
# Run audit on current directory (must contain Cargo.toml)
./ggen/audits/audit-feature-isolation.sh.ggen

# Run audit on specific repo root
./ggen/audits/audit-feature-isolation.sh.ggen /path/to/repo
```

## Output Format

Each rule section prints:
- ✓ PASS — isolation rule satisfied
- ✗ FAIL — isolation rule violated (violation count += 1)
- ⚠ WARN — isolation rule met but with warnings (warning count += 1)

## Dependencies

- `bash` >= 4.0
- `grep`, `sed`, `awk`, `find`
- `python3` (for feature extraction)

## Notes

- Warnings do not affect exit code (exit 0 if violations = 0)
- The audit is read-only and makes no modifications
- Feature gates are validated via regex patterns against source files
- Always-on modules are those in `src/*.rs` (not in `src/ts/`, `src/wasm/`, etc.)
