# audit-no-dto-flattening Usage Guide

## Quick Start

```bash
# Run audit
bash emitted/audits/audit-no-dto-flattening.sh

# Expected output (PASS):
# ✓ PASS: No DTO flattening violations detected
# Exit code: 0

# Expected output (FAIL):
# ✗ FAIL: 2 blocking DTO flattening violation(s)
# Exit code: 1
```

## Understanding Violations

The audit detects two classes of violations:

### 1. Blocking Violations (Exit Code 1)

Lines that contain forbidden patterns **without** an allowed context annotation.

**Example — bare `to_json_string()` call (BLOCKING):**

```rust
// src/my_module.rs:42
fn export_evidence(e: &Evidence) {
    let json = to_json_string(e);  // ← BLOCKING: Missing context annotation
    send_to_wasm(json);
}
```

**Audit output:**
```
✗ src/my_module.rs:42 BLOCKING: let json = to_json_string(e);
Exit code: 1
```

**Fix:**

Add an annotation on the preceding line:

```rust
fn export_evidence(e: &Evidence) {
    // CONTEXT: wasm_boundary_allowed_with_loss_report
    let json = to_json_string(e);  // ← Now allowed, logged as warning
    send_to_wasm(json);
}
```

### 2. Allowed-Context Violations (Exit Code 0, Warning)

Lines containing forbidden patterns that **do** have an allowed context annotation.

**Audit output (warning only):**
```
⚠ src/my_module.rs:42 [wasm_boundary_allowed_with_loss_report] let json = to_json_string(e);
```

These still exit with 0 (PASS) because the context is explicitly declared.

## Annotation Patterns

### Inline Annotation (single violation)

```rust
// CONTEXT: wasm_boundary_allowed_with_loss_report
let payload = to_json_string(&evidence)?;
```

Scanner looks backward up to 5 lines. These all work:

```rust
// CONTEXT: test_fixture_allowed
let state = evidence.state_tag();

// 1 line forward — still found
let state = evidence.state_tag();

// CONTEXT: engine_projection_allowed
// Multiple lines...
// Still within 5 lines backward
let state = evidence.state_tag();
```

### Block Annotation (multiple violations)

```rust
/* ALLOW: compat_core_violation */
fn wasm_bridge_struct_mapping() {
    let evidence_dto = EvidenceDto {
        payload_json: serde_json::to_string(&e).ok(),
        state_tag: e.state().tag(),
    };
    serialize_to_js(&evidence_dto)
}
```

This allows all violations within the block.

## Context Classifications

Choose the context that matches your use case:

### `compat_core_violation`

**When to use:** Core WASM ↔ Rust FFI bridge code.

**Examples:**
- Struct mapping in `src/wasm/bindings.rs`
- JS object serialization
- Type conversion in graduation bridges

**Marks that:** This code is necessary for the bridge but violates pure type law.

```rust
// CONTEXT: compat_core_violation
let dto = EvidenceDto::from_evidence(&evidence);
```

### `wasm_boundary_allowed_with_loss_report`

**When to use:** Serialization with explicit loss accounting.

**Requirements:**
- Must have `LossReport<From, To, Items>` in scope
- Loss must be documented before serialization

**Examples:**
- OCEL → XES projection with loss report
- Evidence export with witness context

```rust
// CONTEXT: wasm_boundary_allowed_with_loss_report
let loss = LossReport::new(Ocel20, Xes1849, vec![...]);
let json = to_json_string(&evidence)?;
emit_receipt(loss);
```

### `engine_projection_allowed`

**When to use:** Process mining engine analysis code.

**Examples:**
- Discovery algorithms inspecting event structure
- Conformance checking examining object lifecycles
- Replay engines accessing state tags for alignment

```rust
// CONTEXT: engine_projection_allowed
let state_tag = receipt.state_tag();  // Allowed for engine analysis
let alignment = compute_alignment(evidence, state_tag);
```

### `test_fixture_allowed`

**When to use:** Test and example code.

**Examples:**
- Unit test assertions
- Integration test setup
- Trybuild `compile_fail/` fixtures
- Examples showing "what not to do"

```rust
#[test]
fn test_dto_serialization() {
    // CONTEXT: test_fixture_allowed
    let payload = to_json_string(&evidence);
    assert!(payload.contains("state_tag"));
}
```

## Real-World Example: Fix a Blocking Violation

### Starting State

```bash
$ bash emitted/audits/audit-no-dto-flattening.sh
```

```
[Phase 1] Scanning src/ (core modules)
  Found 46 .rs file(s)
  ✗ src/wasm/bindings.rs:29 BLOCKING: pub fn get_state_tags() -> Result<JsValue, JsValue> {
```

### Step 1: Examine the violation

```bash
sed -n '25,35p' src/wasm/bindings.rs
```

```rust
    #[wasm_bindgen]
    pub fn get_state_tags() -> Result<JsValue, JsValue> {
        let tags = STATE_TAGS.with(|s| {
            s.borrow().iter().map(|e| e.state_tag()).collect::<Vec<_>>()
        });
        Ok(serde_wasm_bindgen::to_value(&tags)?)
    }
```

### Step 2: Decide the context

This is WASM FFI bridge code that needs to export state information. The appropriate context is `compat_core_violation` because:
- It's necessary for the JS ↔ Rust bridge
- It exposes internal state structure
- It violates pure type law but is unavoidable for interop

### Step 3: Add annotation

```rust
    #[wasm_bindgen]
    // CONTEXT: compat_core_violation
    pub fn get_state_tags() -> Result<JsValue, JsValue> {
        let tags = STATE_TAGS.with(|s| {
            s.borrow().iter().map(|e| e.state_tag()).collect::<Vec<_>>()
        });
        Ok(serde_wasm_bindgen::to_value(&tags)?)
    }
```

### Step 4: Re-run audit

```bash
bash emitted/audits/audit-no-dto-flattening.sh
```

```
[Phase 1] Scanning src/ (core modules)
  Found 46 .rs file(s)
  ⚠ src/wasm/bindings.rs:29 [compat_core_violation] pub fn get_state_tags() -> Result<JsValue, JsValue> {

Results:
  Blocking violations:  0
  Allowed violations:   1
  Status: PASS (exit 0)
```

The violation is now a **warning** (yellow ⚠), and the script exits with 0 (PASS).

## Integration with CI

### GitHub Actions

```yaml
name: Type-Law Audit

on: [pull_request, push]

jobs:
  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Run DTO Flattening Audit
        run: bash emitted/audits/audit-no-dto-flattening.sh
```

### Pre-commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit

bash emitted/audits/audit-no-dto-flattening.sh
if [ $? -ne 0 ]; then
  echo "Type-law audit failed. Add annotations or fix violations."
  exit 1
fi
```

### Development Loop

```bash
# Watch for violations during development
watch -n 10 'bash emitted/audits/audit-no-dto-flattening.sh'
```

## Environment Variables

Override default behavior:

```bash
# Scan only these patterns
FORBIDDEN_PATTERNS="payload_json,state_tag" \
bash emitted/audits/audit-no-dto-flattening.sh

# Allow only test fixtures
ALLOWED_CONTEXTS="test_fixture_allowed" \
bash emitted/audits/audit-no-dto-flattening.sh

# Custom directory paths
SRC_DIR=/path/to/src \
TESTS_DIR=/path/to/tests \
bash emitted/audits/audit-no-dto-flattening.sh

# All together
FORBIDDEN_PATTERNS="EvidenceDto" \
ALLOWED_CONTEXTS="engine_projection_allowed" \
SRC_DIR=/custom/src \
bash emitted/audits/audit-no-dto-flattening.sh
```

## Audit Report

Every run produces a JSON report:

```bash
cat emitted/audits/audit-no-dto-flattening-20260601-120805.json
```

```json
{
  "audit_name": "audit-no-dto-flattening",
  "timestamp": "2026-06-01T19:08:05Z",
  "forbidden_patterns": "EvidenceDto,AdmissionDto,RefusalDto,ReceiptDto,payload_json,state_tag,to_json_string,receipt_json",
  "allowed_contexts": "compat_core_violation,wasm_boundary_allowed_with_loss_report,engine_projection_allowed,test_fixture_allowed",
  "results": {
    "blocking_violations": 0,
    "allowed_violations": 5,
    "status": "PASS"
  }
}
```

Parse for dashboards:

```bash
# Count violations by type
jq '.results' emitted/audits/audit-no-dto-flattening-*.json

# Get all audit timestamps
jq -r '.timestamp' emitted/audits/audit-no-dto-flattening-*.json | sort
```

## Troubleshooting

### "Configuration error: SRC_DIR not found"

```bash
# Check paths
echo "SRC_DIR=$SRC_DIR"
ls -d "$SRC_DIR"

# Run from repo root
cd /Users/sac/wasm4pm-compat
bash emitted/audits/audit-no-dto-flattening.sh
```

### Script hangs on large codebases

```bash
# Check progress in another terminal
lsof | grep audit-no-dto-flattening

# Kill and try with smaller scope
TESTS_DIR=/dev/null bash emitted/audits/audit-no-dto-flattening.sh
```

### Annotation not detected

Ensure comment is on the **preceding line**:

```rust
// ✓ CORRECT: Comment directly above
// CONTEXT: test_fixture_allowed
let x = payload_json;

// ✗ WRONG: Comment on same line
let x = payload_json; // CONTEXT: test_fixture_allowed

// ✗ WRONG: Comment too far back
// CONTEXT: test_fixture_allowed
// Some other code
let x = payload_json;
```

---

**Last Updated**: 2026-06-01
**Audit Script Version**: emitted/audits/audit-no-dto-flattening.sh (11 KB)
