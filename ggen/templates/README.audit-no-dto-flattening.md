# audit-no-dto-flattening.sh.tera

## Purpose

Template-generated audit script that enforces strict zero-cost type-law surfaces by detecting forbidden DTO flattening patterns. Prevents lossy serialization bypasses and type-law violations at the WASM boundary.

## Forbidden Patterns (Blocking)

Detected patterns that **must** be wrapped in allowed context:

- **Type references**: `EvidenceDto`, `AdmissionDto`, `RefusalDto`, `ReceiptDto`
- **Payload flattening**: `payload_json`, `state_tag` field accesses
- **Lossy serialization**: `to_json_string()`, `receipt_json` without witness context

## Allowed Contexts (Requires Annotation)

Violations inside these contexts are marked as warnings, not failures:

- **`compat_core_violation`** — Core bridge code (wasm4pm graduation, struct mapping)
- **`wasm_boundary_allowed_with_loss_report`** — WASM FFI with explicit `LossReport` witness
- **`engine_projection_allowed`** — Process mining engine projections (discovery, conformance)
- **`test_fixture_allowed`** — Test fixtures and trybuild fail cases

## Usage

### Basic Run (Default Forbidden Patterns + Allowed Contexts)

```bash
bash emitted/audits/audit-no-dto-flattening.sh
```

Exit code: `0` (PASS), `1` (FAIL with blocking violations), `3` (config error)

### Custom Forbidden Patterns

```bash
FORBIDDEN_PATTERNS="payload_json,state_tag" bash emitted/audits/audit-no-dto-flattening.sh
```

Comma-separated, no spaces.

### Custom Allowed Contexts

```bash
ALLOWED_CONTEXTS="test_fixture_allowed,engine_projection_allowed" bash emitted/audits/audit-no-dto-flattening.sh
```

### Custom Scan Directories

```bash
SRC_DIR=/path/to/src \
TESTS_DIR=/path/to/tests \
EXAMPLES_DIR=/path/to/examples \
bash emitted/audits/audit-no-dto-flattening.sh
```

## Annotation Syntax

### Inline Annotation (single line)

```rust
// CONTEXT: wasm_boundary_allowed_with_loss_report
let payload = to_json_string(&evidence)?;
```

### Block Annotation

```rust
/* ALLOW: compat_core_violation */
fn bridge_struct_mapping() {
    let state_tag = evidence.state_tag(); // allowed here
}
```

The audit scanner looks up to 5 lines backward for annotations.

## Output

### Console Output

- **Green (✓)** — No violations detected
- **Red (✗)** — Blocking violations found (exit 1)
- **Yellow (⚠)** — Allowed-context violations found (exit 0, but logged)

### JSON Audit Report

Generated in `emitted/audits/audit-no-dto-flattening-YYYYMMDD-HHMMSS.json`:

```json
{
  "audit_name": "audit-no-dto-flattening",
  "timestamp": "2026-06-01T12:04:55Z",
  "forbidden_patterns": "EvidenceDto,AdmissionDto,RefusalDto,ReceiptDto,payload_json,state_tag,to_json_string,receipt_json",
  "allowed_contexts": "compat_core_violation,wasm_boundary_allowed_with_loss_report,engine_projection_allowed,test_fixture_allowed",
  "results": {
    "blocking_violations": 2,
    "allowed_violations": 0,
    "status": "FAIL"
  }
}
```

## Exit Codes

| Code | Meaning |
|------|---------|
| **0** | PASS — No blocking violations |
| **1** | FAIL — Blocking violations detected (action required) |
| **2** | Reserved for future use |
| **3** | Configuration error (missing directories, invalid input) |

## Template Variables (Tera)

The `.tera` template supports rendering of allowed contexts for display:

```tera
{%- for context in allowed_contexts %}
     - // CONTEXT: {{ context }}
{%- endfor %}
```

This renders all allowed contexts in the action items section.

## Integration

### CI/CD

Add to your CI script to block merges with DTO flattening violations:

```bash
bash emitted/audits/audit-no-dto-flattening.sh || {
  echo "DTO boundary audit failed"
  exit 1
}
```

### Pre-commit Hook

```bash
#!/bin/bash
bash emitted/audits/audit-no-dto-flattening.sh || exit 1
```

### Development Workflow

Run periodically during feature development:

```bash
watch -n 10 'bash emitted/audits/audit-no-dto-flattening.sh'
```

## Related

- `/Users/sac/wasm4pm-compat/CLAUDE.md` — Type-law receipt gates
- `src/loss.rs` — Loss accounting and `LossReport` witness
- `src/admission.rs` — Refusal and reason types
- `tests/ui/` — Trybuild receipts (type-law validation)
