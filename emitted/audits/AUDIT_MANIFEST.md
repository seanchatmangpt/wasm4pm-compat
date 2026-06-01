# DTO Flattening Boundary Audit Manifest

## Deliverable

- **Template**: `/Users/sac/wasm4pm-compat/ggen/templates/audit-no-dto-flattening.sh.tera`
- **Generated Script**: `/Users/sac/wasm4pm-compat/emitted/audits/audit-no-dto-flattening.sh`
- **Documentation**: `/Users/sac/wasm4pm-compat/ggen/templates/README.audit-no-dto-flattening.md`

## What It Does

### Forbidden Patterns (Blocking, Exit 1)

Must be wrapped in an allowed context:

1. **Type references**: `EvidenceDto`, `AdmissionDto`, `RefusalDto`, `ReceiptDto`
2. **Payload field access**: `payload_json`, `state_tag`
3. **Lossy serialization**: `to_json_string()`, `receipt_json`

These patterns indicate potential type-law boundary violations where:
- DTO structures are flattened into raw JSON
- Type safety is compromised at the WASM boundary
- Witness context (loss accounting) is missing

### Allowed Contexts (Warnings, Exit 0)

Violations in these contexts are permitted but logged as warnings:

1. **`compat_core_violation`** — Core wasm4pm bridge code
   - Struct mapping, FFI shims, graduation bridges
   - Must be isolated to `src/wasm/` or `src/graduation/`

2. **`wasm_boundary_allowed_with_loss_report`** — WASM FFI with explicit witness
   - Requires `LossReport<From, To, Items>` in scope
   - Serialization must document loss before export

3. **`engine_projection_allowed`** — Process mining engine projections
   - Discovery, conformance checking, replay algorithms
   - Isolated to `src/engine/` (future module)
   - Allowed to inspect internal structure for analysis

4. **`test_fixture_allowed`** — Test fixtures and trybuild fail cases
   - Unit tests, integration tests, examples
   - Trybuild `compile_fail/` fixtures

## Behavior

### Strict Shell Mode

```bash
set -euo pipefail
```

- Fails on undefined variables
- Exits on pipe errors
- Fails on any command error

### Scan Phases

1. **Phase 1** — `src/` core modules
2. **Phase 2** — `tests/` fixtures
3. **Phase 3** — `examples/` runnable examples
4. **Phase 4** — Generated TypeScript/WASM bindings

### Output

- **Console**: Colored PASS/FAIL with violation details
- **JSON Report**: Timestamped audit result (`emitted/audits/audit-no-dto-flattening-YYYYMMDD-HHMMSS.json`)

### Exit Codes

| Code | Meaning |
|------|---------|
| **0** | PASS — No blocking violations |
| **1** | FAIL — Blocking violations found |
| **3** | Configuration error |

## Usage Examples

### Run with defaults (no custom patterns/contexts)

```bash
bash emitted/audits/audit-no-dto-flattening.sh
```

### Audit only `payload_json` and `state_tag`

```bash
FORBIDDEN_PATTERNS="payload_json,state_tag" \
bash emitted/audits/audit-no-dto-flattening.sh
```

### Allow specific context

```bash
FORBIDDEN_PATTERNS="EvidenceDto,AdmissionDto,RefusalDto,ReceiptDto" \
ALLOWED_CONTEXTS="test_fixture_allowed" \
bash emitted/audits/audit-no-dto-flattening.sh
```

### Custom scan directories

```bash
SRC_DIR=/path/to/src \
TESTS_DIR=/path/to/tests \
EXAMPLES_DIR=/path/to/examples \
bash emitted/audits/audit-no-dto-flattening.sh
```

## Real-World Test Results

Ran against wasm4pm-compat codebase on 2026-06-01:

```
Forbidden patterns: EvidenceDto,AdmissionDto,RefusalDto,ReceiptDto,payload_json,state_tag,to_json_string,receipt_json
Allowed contexts:   compat_core_violation,wasm_boundary_allowed_with_loss_report,engine_projection_allowed,test_fixture_allowed

[Phase 1] Scanning src/ (core modules)
  Found 46 .rs file(s)
  ✗ src/wasm/bindings.rs:29 BLOCKING: pub fn get_state_tags() -> Result<JsValue, JsValue> {

[Phase 2] Scanning tests/ (test fixtures)
  Found 642 test file(s)
  ✗ tests/graduation.rs:85 BLOCKING: let tags_val = get_state_tags().unwrap();

Results:
  Blocking violations:  2
  Allowed violations:   0
  Status: FAIL (exit 1)
```

Both violations are in the wasm FFI layer and require annotation.

## Annotation Syntax

### Inline (single violation)

```rust
// CONTEXT: wasm_boundary_allowed_with_loss_report
let json = to_json_string(&evidence)?;
```

### Block (multiple lines)

```rust
/* ALLOW: compat_core_violation */
fn bridge_mapping() {
    let state = evidence.state_tag();
    let payload = evidence.payload_json();
}
```

Scanner looks up to 5 lines backward for annotations.

## Template Features

- **Tera rendering**: Supports `allowed_contexts` loop for context list generation
- **Environment variables**: All paths and patterns overridable
- **Deterministic output**: JSON report timestamped for audit trail
- **Extensible patterns**: Comma-separated forbidden patterns and contexts

## Related

- Type-law surfaces: `/Users/sac/wasm4pm-compat/src/loss.rs`
- Refusal/reason types: `/Users/sac/wasm4pm-compat/src/admission.rs`
- WASM boundary: `/Users/sac/wasm4pm-compat/src/wasm/`
- Type-law receipts: `/Users/sac/wasm4pm-compat/tests/ui/`

---

**Generated**: 2026-06-01
**Template**: ggen/templates/audit-no-dto-flattening.sh.tera (11 KB)
**Script**: emitted/audits/audit-no-dto-flattening.sh (11 KB, executable)
