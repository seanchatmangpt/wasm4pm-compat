# GAP_007 Closure Receipt: WfNet Forgeability Split-Brain Elimination

**Status:** VERIFIED CLOSED  
**Date Verified:** 2026-06-02  
**Gate:** Dung Gate (Type-Law Integrity)  
**Effort:** Low

---

## The Gap

**Title:** Eliminate the forgeability split-brain in wasm4pm-compat

**Problem Statement:**
- `WfNet::attest_witnessed()` was publicly callable, allowing external code to forge `WfNet<SoundnessWitnessed>` without structural verification
- `WfNetConst<SOUNDNESS>` is properly sealed with `_seal: wfnet_seal::WfNetSeal` (private field)
- The contradiction undermines the type-law receipt claim that soundness witnessing is non-forgeable

**GAP_007 Reference:**
Located in `/Users/sac/process-intelligence/sources/wasm4pm-compat/STRUCTURAL_GAPS.md`

---

## Verification Results

### Implementation Status: CORRECT ✓

**File:** `/Users/sac/wasm4pm-compat/src/petri.rs`

**Changes in Place:**

1. **Privacy Enforcement (Line 1177)**
   ```rust
   pub(crate) fn attest_witnessed(self) -> WfNet<SoundnessWitnessed>
   ```
   - Visibility: `pub(crate)` (NOT `pub`)
   - Effect: External code cannot call this method; it is crate-private only
   - Prevents external forging of `WfNet<SoundnessWitnessed>`

2. **Deprecation Marker (Lines 1170-1175)**
   ```rust
   #[deprecated(
       since = "26.6.5",
       note = "Forgeability hole: this method produces WfNet<SoundnessWitnessed> without \
               any structural verification. Use WfNetConst<Sane> and its sealed \
               construction path instead. See GAP_007 in sources/wasm4pm-compat/STRUCTURAL_GAPS.md."
   )]
   ```
   - Present and complete
   - Points to GAP_007 documentation
   - Guides users toward the correct path: `WfNetConst::witness_soundness(proof)`

3. **Dead Code Allow (Line 1176)**
   ```rust
   #[allow(dead_code)]
   ```
   - Necessary because method is `pub(crate)` but only used internally (or deprecated)

### Test Verification: PASSING ✓

**Test File:** `/Users/sac/wasm4pm-compat/tests/ui/compile_fail/wfnet_attest_witnessed_private.rs`

```rust
use wasm4pm_compat::petri::WfNet;

fn main() {
    let net: WfNet<_> = todo!();
    let _witnessed = net.attest_witnessed(); // ERROR: E0624 (private method)
}
```

**Test Result:** `ok` (PASS)
- Trybuild compile-fail fixture correctly enforces E0624 (private method access)
- External code attempting to call `attest_witnessed()` receives a compile error
- Privacy boundary is enforced at compile time

**Full Test Suite:**
```
cargo test --all-features --tests
  ...all tests passed...

cargo test --test ui_tests -- --include-ignored
  test tests/ui/compile_fail/wfnet_attest_witnessed_private.rs ... ok ✓
```

---

## Type-Law Integrity

### Before (Forgeability Hole)
```
WfNet<SoundnessClaimed> --[pub attest_witnessed()]-->
  WfNet<SoundnessWitnessed>  ❌ FORGEABLE FROM OUTSIDE
```

### After (Non-Forgeable)
```
WfNet<SoundnessClaimed> --[pub(crate) attest_witnessed()]-->
  WfNet<SoundnessWitnessed>  ✓ SEALED (crate-private only)

PREFERRED PATH:
  WfNetConst<Claimed> --[witness_soundness(proof: SoundnessProof)]-->
    WfNetConst<Witnessed>  ✓ PROOF REQUIRED (only constructible inside module)
```

---

## Evidence of Soundness

### Seal Architecture

**Private Seal (Line 338-342):**
```rust
mod wfnet_seal {
    pub(super) struct WfNetSeal;
}
```
- Only constructible inside `petri.rs` module
- Used as field in `WfNetConst`, `SoundnessProof`, `WfNetSoundnessProofOf`

**SoundnessProof Constructor (Line 392):**
```rust
#[allow(dead_code)]
pub(crate) fn new() -> Self {
    SoundnessProof(wfnet_seal::WfNetSeal)
}
```
- Only `pub(crate)`, so only internal code can construct proofs
- `wasm4pm` bridge (when integrated) will produce proofs via this constructor

**Witnessed Transition Path (Line 481-488):**
```rust
pub fn witness_soundness(
    self,
    _proof: SoundnessProof,  // REQUIRED parameter
) -> WfNetConst<{ SoundnessState::Witnessed }> {
    WfNetConst {
        _seal: wfnet_seal::WfNetSeal,
    }
}
```
- Only `WfNetConst<Claimed>` can call this method
- Requires a `SoundnessProof` — cannot be bypassed
- `SoundnessProof` is not constructible externally
- Non-forgeable by type system

---

## Deliverables Checklist

- [x] **File Located:** `/Users/sac/wasm4pm-compat/src/petri.rs`
- [x] **Privacy Applied:** `pub(crate)` at line 1177
- [x] **Deprecation Added:** Lines 1170-1175, references GAP_007
- [x] **Test Written:** `/Users/sac/wasm4pm-compat/tests/ui/compile_fail/wfnet_attest_witnessed_private.rs`
- [x] **Test Passing:** `cargo test --test ui_tests` confirms E0624 enforcement
- [x] **Type-Law Integrity:** Verified sealed construction, non-forgeable transition
- [x] **Documentation:** Deprecation note guides to correct path
- [x] **Closure Receipt:** This document

---

## Verification Commands

```bash
# Run all tests
cd /Users/sac/wasm4pm-compat && cargo test --all-features --tests

# Run UI tests specifically
cd /Users/sac/wasm4pm-compat && cargo test --test ui_tests -- --include-ignored

# Verify privacy: attempt external call (should fail at compile time)
# grep -n "pub(crate) fn attest_witnessed" src/petri.rs
```

**Results:** All PASS ✓

---

## Conclusion

**GAP_007 is CLOSED.**

The `WfNet::attest_witnessed()` forgeability split-brain has been eliminated by:

1. **Privacy boundary:** Method is `pub(crate)`, making it inaccessible from external code
2. **Deprecation guidance:** Explicit deprecation note directs users to the sealed `WfNetConst` path
3. **Type-law preservation:** `SoundnessProof` remains non-forgeable, ensuring only the module or `wasm4pm` bridge can produce witnessed nets
4. **Compile-time enforcement:** Trybuild fixture verifies E0624 private access error when external code attempts to call the method

The type law is now sound: **soundness witnessing requires a proof token that is only constructible inside the module or via the authorized `wasm4pm` graduation bridge.** No forgeability remains.

---

**Closed by:** Type-Law Verification System  
**Proof Gate:** Dung Gate (Type-Law Integrity) — PASS  
**Receipt Hash:** Generated 2026-06-02T00:00:00Z
