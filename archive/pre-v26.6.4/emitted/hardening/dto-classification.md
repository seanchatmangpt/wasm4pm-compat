# DTO Marking Violations — Classification Report

**Audit Date:** 2026-06-01  
**Violations Found:** 2  
**Classification Summary:** 1 FALSE_POSITIVE, 1 CONTEXT_ANNOTATION_REQUIRED

---

## Violation 1: `src/wasm/bindings.rs:29`

### Pattern
```rust
// Line 29 in src/wasm/bindings.rs
pub fn get_state_tags() -> Result<JsValue, JsValue> {
    let tags = vec![
        WasmStateTag {
            name: "Raw".into(),
            is_terminal: false,
        },
        // ... (state tag constructors for Raw, Parsed, Admitted, Refused, Projected, Exportable, Receipted)
    ];
    serde_wasm_bindgen::to_value(&tags).map_err(|e| JsValue::from_str(&e.to_string()))
}
```

### Context
- **File:** `src/wasm/bindings.rs` (WASM boundary layer)
- **Module:** `src/wasm/` — exposed via `#[wasm_bindgen]` for FFI to JavaScript
- **What it does:** Exposes the canonical list of typestate lifecycle tags (`Raw`, `Parsed`, `Admitted`, `Refused`, `Projected`, `Exportable`, `Receipted`) as a serialized WASM ABI value.
- **Type being exposed:** `WasmStateTag` (defined in `src/wasm/boundary.rs`) with fields:
  - `name: String` — the state label
  - `is_terminal: bool` — whether the state is terminal
- **Serialization:** Via `serde_wasm_bindgen::to_value()` → `JsValue`

### Analysis

The audit detected a match on the substring `state_tag` inside the function name `get_state_tags()`. This is a **false positive** for the following reasons:

1. **No DTO flattening occurs.** The function constructs a `Vec<WasmStateTag>` with full, typed struct instances. No fields are flattened to raw primitives.
2. **The match target was the function name.** The audit pattern `state_tag` appears only in the identifier `get_state_tags`, not in actual data structure flattening code.
3. **Type law is preserved across the boundary.** `WasmStateTag` is a standalone struct (not a DTO composed from flattened payload_json/state_tag fields). Each state carries its metadata as named fields.
4. **This is a catalog/metadata function.** It is not involved in evidence serialization, admission, or receipt paths. It exposes read-only enumeration metadata to JavaScript.
5. **The function is necessary for WASM FFI.** Omitting it would prevent JavaScript from discovering the valid states in the typestate machine.

### Verdict

**FALSE_POSITIVE** — The detected string `state_tag` is part of the function identifier (`get_state_tags`), not a flattening violation. The function correctly exposes a well-typed state catalog across the WASM boundary without loss of structure or fidelity.

**Recommended Action:** Update the audit script's pattern regex to exclude `get_state_tags` from the forbidden set (e.g., via a negative lookahead for function identifiers ending in `s`, or by distinguishing between field access patterns (`state_tag.` or `.state_tag`) and identifiers).

---

## Violation 2: `tests/graduation.rs:85`

### Pattern
```rust
// Line 85 in tests/graduation.rs (inside cfg(all(feature = "wasm", target_arch = "wasm32")))
let tags_val = get_state_tags().unwrap();
```

### Context
- **File:** `tests/graduation.rs` — feature-gated test module
- **Test:** `test_wasm_boundary_functions()` under `#[cfg(all(feature = "wasm", target_arch = "wasm32"))]`
- **What it does:** Calls the WASM boundary function `get_state_tags()` and deserializes the result into `Vec<WasmStateTag>`.
- **Full test context:**
  ```rust
  #[test]
  fn test_wasm_boundary_functions() {
      let catalog_val = get_witness_catalog().unwrap();
      let catalog: Vec<WasmWitness> = from_value(catalog_val).unwrap();
      assert!(catalog.iter().any(|w| w.key == "ocel20"));
      assert!(catalog.iter().any(|w| w.key == "xes1849"));
  
      let tags_val = get_state_tags().unwrap();  // ← Line 85 (violation)
      let tags: Vec<WasmStateTag> = from_value(tags_val).unwrap();
      assert!(tags.iter().any(|t| t.name == "Admitted"));
  
      // Precondition validations
      let res: WasmAdmissionResult = ...
      // Graduation Candidate creation on the boundary
      let candidate: WasmGraduationCandidate = ...
  }
  ```

### Analysis

This is a **test fixture call** to the WASM boundary function that was flagged as suspicious. However, unlike a true DTO flattening violation:

1. **Test code context:** This line resides in feature-gated test code (`#[cfg(all(feature = "wasm", target_arch = "wasm32"))]`), which is explicitly allowed as `test_fixture_allowed`.
2. **No structural loss occurs.** The function call and deserialization preserve the full typed structure of `WasmStateTag`.
3. **Legitimate test purpose:** The test validates that the WASM boundary correctly exposes the state catalog and that the round-trip serialization/deserialization is sound.
4. **The violation is indirect:** The audit matched on the substring `state_tag` in the function call `get_state_tags()`, not on any actual flattening pattern.

### Verdict

**CONTEXT_ANNOTATION_REQUIRED** — While this is technically a test fixture (which is allowed), the audit rule detects it as a blocking violation because the annotation mechanism requires explicit marking. The call itself is sound and necessary.

**Recommended Action:** Add a context annotation on line 84 (immediately before the call) to mark it as an allowed test fixture:

```rust
// CONTEXT: test_fixture_allowed
let tags_val = get_state_tags().unwrap();
```

Alternatively, if the audit script is updated to exclude function identifiers (per Violation 1), this line would not trigger at all.

---

## Root Cause Analysis

Both violations stem from the same root issue: **The audit pattern `state_tag` is too broad.** It matches:
- The function identifier `get_state_tags` (a catalog accessor, not a flattening violation)
- Calls to that function (which inherently mention the name)

The pattern should be refined to detect actual **field access patterns**, such as:
- `obj.state_tag` (direct field access)
- `payload_json` (flattened JSON blob)
- `to_json_string()` (serialization without loss context)

---

## Summary Table

| Violation | Location | Pattern | Type | Severity | Remediation |
|-----------|----------|---------|------|----------|-------------|
| 1 | `src/wasm/bindings.rs:29` | `get_state_tags()` function definition | FALSE_POSITIVE | None | Refine audit pattern regex |
| 2 | `tests/graduation.rs:85` | `get_state_tags()` function call in test | CONTEXT_ANNOTATION_REQUIRED | Low | Add `// CONTEXT: test_fixture_allowed` on line 84, OR refine audit pattern |

---

## Recommendations

### Immediate Actions
1. **For Violation 2:** Add annotation on line 84 of `tests/graduation.rs`:
   ```rust
   // CONTEXT: test_fixture_allowed
   let tags_val = get_state_tags().unwrap();
   ```

2. **For Both:** Update `ggen/audits/audit-no-dto-flattening.sh` to refine the `state_tag` pattern:
   - Current: `state_tag` (matches anywhere)
   - Proposed: `\.state_tag|->state_tag` (field access patterns only)
   - Or: Exclude patterns like `get_state_tags` from the forbidden set

### Long-term Hardening
- **Clarify DTO patterns:** Document field-access vs. identifier distinction in the audit script.
- **Increase specificity:** Forbid patterns only when they represent direct data structure flattening, not function/method names.
- **Test the audit:** Add fixtures to `tests/ui/` demonstrating both allowed and forbidden DTO patterns.
