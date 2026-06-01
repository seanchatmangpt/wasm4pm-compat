# WIT Template Usage Guide

**Template:** `ggen/templates/wasm4pm-compat.wit.ggen`  
**Engine:** Tera  
**Output:** 6 WIT files (feature-gated)

---

## Quick Start

### Render Template (Future)

```bash
# When ggen build pipeline is integrated:
cd /Users/sac/wasm4pm-compat
cargo make ggen-wit-render

# Or manual Tera invocation:
tera \
  --input ggen/templates/wasm4pm-compat.wit.ggen \
  --config ggen/ggen.toml \
  --data-source component.projection.yaml \
  --data-source wit-surface-ledger.yaml \
  --data-source graduation-surface-ledger.yaml \
  --output ggen/wit/
```

### Validate Generated WIT Files

```bash
# Validate syntax
wit-parser ggen/wit/compat.wit
wit-parser ggen/wit/compat-formats.wit
wit-parser ggen/wit/compat-strict.wit
wit-parser ggen/wit/compat-wasm4pm.wit
wit-parser ggen/wit/compat-all.wit
wit-parser ggen/wit/engine.wit

# Cross-check against Rust types
cargo test --all-features wit-roundtrip
```

---

## Output Files by Feature Combination

### 1. Base: No Feature Flags

**File:** `ggen/wit/compat.wit`

```wit
package wasm4pm:compat@1.0.0;

interface types { ... }      // Always present
interface admission { ... }   // Always present

world compat {
  export types;
  export admission;
}
```

**Interfaces:**
- `wasm4pm:compat/types` — Event logs, refusal reasons, boundaries
- `wasm4pm:compat/admission` — admit-event-log, admit-ocel-log, admit-xes-log

**Use case:** Structure-only validation; no graduation.

---

### 2. formats Feature

**File:** `ggen/wit/compat-formats.wit`

```wit
package wasm4pm:compat@1.0.0;

interface types { ... }
interface admission { ... }
interface loss { ... }        // NEW

world compat-formats {
  export types;
  export admission;
  export loss;
}
```

**Interfaces:**
- (types + admission from base)
- `wasm4pm:compat/loss` — project-ocel-to-xes, project-xes-to-dfg (with loss accounting)

**Use case:** Lossy transformations with loss policy and audit trail.

---

### 3. strict Feature

**File:** `ggen/wit/compat-strict.wit`

```wit
package wasm4pm:compat@1.0.0;

interface types { ... }
interface admission { ... }
interface strict { ... }      // NEW

world compat-strict {
  export types;
  export admission;
  export strict;
}
```

**Interfaces:**
- (types + admission from base)
- `wasm4pm:compat/strict` — check-strict-boundary (witness + loss policy + round-trip)

**Use case:** Boundary attestation; declare and enforce process boundaries.

---

### 4. wasm4pm Feature

**File:** `ggen/wit/compat-wasm4pm.wit`

```wit
package wasm4pm:compat@1.0.0;

interface types { ... }
interface admission { ... }
interface graduation { ... }           // NEW
interface witness-metadata { ... }     // NEW

world compat-wasm4pm {
  export types;
  export admission;
  export graduation;
  export witness-metadata;
}
```

**Interfaces:**
- (types + admission from base)
- `wasm4pm:compat/graduation` — graduate-to-wasm4pm (grounded semantics check)
- `wasm4pm:compat/witness-metadata` — get-witness-info, list-all-witnesses

**Imports (via engine world):**
- `wasm4pm:engine/discovery` — discover-dfg, discover-petri, discover-bpmn
- `wasm4pm:engine/replay` — replay-on-petri, align-on-petri
- `wasm4pm:engine/conformance` — check-conformance
- `wasm4pm:engine/ocpq` — query-object-lifecycle, query-object-relations
- `wasm4pm:engine/receipts` — generate-receipt, verify-receipt

**Use case:** Graduation to wasm4pm execution engine; linking to engine world.

---

### 5. All Features (formats + strict + wasm4pm)

**File:** `ggen/wit/compat-all.wit`

```wit
package wasm4pm:compat@1.0.0;

interface types { ... }
interface admission { ... }
interface loss { ... }
interface strict { ... }
interface graduation { ... }
interface witness-metadata { ... }

world compat-all {
  export types;
  export admission;
  export loss;
  export strict;
  export graduation;
  export witness-metadata;
}
```

**Use case:** Complete compat surface; full feature set.

---

### 6. Engine World (wasm4pm Feature)

**File:** `ggen/wit/engine.wit`

```wit
package wasm4pm:engine@1.0.0;

interface discovery { ... }
interface replay { ... }
interface conformance { ... }
interface ocpq { ... }
interface receipts { ... }

world engine {
  import discovery;
  import replay;
  import conformance;
  import ocpq;
  import receipts;
}
```

**Use case:** Engine world specification; imported by compat component via graduation.

---

## Template Structure Reference

### Key Sections

| Section | Lines | Purpose |
|---------|-------|---------|
| Header | 1–26 | Purpose, inputs, outputs, variables |
| Part 1: types | 29–408 | Shared type definitions (always emitted) |
| Part 2: admission | 410–467 | Admission interface (always emitted) |
| Part 3: loss | 469–535 | Loss interface (feature: formats) |
| Part 4: strict | 537–591 | Strict boundary interface (feature: strict) |
| Part 5: graduation | 593–648 | Graduation interface (feature: wasm4pm) |
| Part 6: witness-metadata | 650–675 | Witness registry interface (feature: wasm4pm) |
| Part 7: worlds | 677–748 | World definitions (6 variants) |
| Part 8: engine | 750–828 | Engine world interfaces (feature: wasm4pm) |

### Conditional Blocks

```tera
{%- if FEATURES.contains("formats") %}
  // Emitted only if formats feature enabled
{%- endif %}

{%- if FEATURES.contains("strict") %}
  // Emitted only if strict feature enabled
{%- endif %}

{%- if FEATURES.contains("wasm4pm") %}
  // Emitted only if wasm4pm feature enabled
{%- endif %}
```

---

## Integration with Build System

### ggen.toml (Future)

Add to `ggen.toml`:

```toml
[[generators]]
name = "wasm4pm-compat.wit"
template = "ggen/templates/wasm4pm-compat.wit.ggen"
input_ledgers = [
  "ggen/projections/component.projection.yaml",
  "ggen/intel/wit-surface-ledger.yaml",
  "ggen/intel/graduation-surface-ledger.yaml",
]
output = "ggen/wit/"
feature_flags = ["formats", "strict", "wasm4pm"]

[generators.tera_context]
PROJECT = "wasm4pm-compat"
VERSION = "1.0.0"
NAMESPACE = "wasm4pm:compat"
ENGINE_NAMESPACE = "wasm4pm:engine"
```

### build.rs Integration (Future)

```rust
use std::process::Command;

fn main() {
    // Render WIT templates
    Command::new("cargo")
        .args(&["make", "ggen-wit-render"])
        .output()
        .expect("Failed to render WIT templates");

    // Invoke wit-bindgen
    wit_bindgen::generate!({
        world: "wasm4pm:compat@1.0.0",
        path: "ggen/wit/compat-all.wit",
    });

    println!("cargo:rerun-if-changed=ggen/templates/wasm4pm-compat.wit.ggen");
    println!("cargo:rerun-if-changed=ggen/wit/");
}
```

### Cargo.toml Integration (Future)

```toml
[features]
default = ["formats"]
formats = []
strict = []
wasm4pm = ["formats"]  # wasm4pm implies formats

[build-dependencies]
wit-bindgen = "0.20"
```

---

## Witness Registry (Constant Time Lookup)

The template generates two functions that expose the witness registry:

### `get-witness-info(key: witness-id) → option<witness-info>`

Returns metadata for a single witness:

```wit
record witness-info {
  key: string,           // e.g., "ocel-2.0"
  family: string,        // "standard" | "paper" | "api-grammar" | "rust-law" | "internal-bridge"
  title: string,         // e.g., "OCEL 2.0"
  year: option<u16>,     // Publication year
}
```

**Known Witnesses:**
- **Standard:** ocel-2.0, xes-1849, bpmn-2.0, yawl-2.0, w3c-as-2.0
- **Paper:** wf-net-soundness-paper, powl-paper, inductive-miner, alpha-miner, declare-3
- **Internal:** wasm4pm-bridge (graduation boundary marker)
- ... (~35+ total)

### `list-all-witnesses() → list<witness-info>`

Returns all ~41 registered witnesses.

---

## Refusal Encoding Examples

Every `result<T, refusal-reason>` must return one of seven named variants:

### Example 1: DanglingEventObjectLink

```wasm
(call (import "admission:admit-ocel-log") (event-log-value))
// Returns:
// Err(dangling-event-object-link(
//   event-id: "evt-123",
//   object-id: "obj-456",
//   object-type: "customer"
// ))
```

Meaning: Event evt-123 references object obj-456 of type customer, but that object is not declared in the OCEL log.

### Example 2: InvalidLossPolicy

```wasm
(call (import "loss:project-ocel-to-xes")
  (admitted-ocel)
  (loss-policy: "allow-named-projection"))
// Returns:
// Err(invalid-loss-policy(
//   transformation: "ocel-to-xes",
//   policy-required: "allow-loss-with-report"
// ))
```

Meaning: OCEL → XES projection loses object-to-object links; policy "allow-named-projection" is insufficient. Requires "allow-loss-with-report".

### Example 3: CircularDependency

```wasm
(call (import "admission:admit-ocel-log") (malformed-ocel))
// Returns:
// Err(circular-dependency(
//   cycle: ["evt-1", "evt-2", "evt-3", "evt-1"]
// ))
```

Meaning: Events form a causal cycle. Temporal ordering is violated.

---

## Loss Report Audit Trail

When a projection succeeds with loss policy "allow-loss-with-report":

```wit
record project-ocel-to-xes-result {
  xes-log: xes-log,
  report: loss-report,
}

record loss-report {
  from-format: "ocel",
  to-format: "xes",
  items-lost: [
    loss-item { item-type: "object-to-object-link", ... },
    loss-item { item-type: "object-change", ... },
    loss-item { item-type: "attribute", source-id: "color", ... },
  ],
  summary: "Lossy projection from OCEL to XES: 3 items lost (O2O links, object changes, non-string attributes)",
}
```

Every lossy transformation is documented at the boundary.

---

## Graduation Readiness Check

When graduating to wasm4pm engine:

```wit
record graduation-candidate {
  kind: "discovery",         // Type of engine operation
  is-grounded: true,         // Ready for engine?
  reason: null,              // Why ungrounded (if false)
}
```

If `is-grounded == false`:

```wit
record graduation-candidate {
  kind: "conformance",
  is-grounded: false,
  reason: "missing-loss-policy",  // Specific blocking reason
}
```

Reasons include:
- "missing-loss-policy" — Lossy projection without accounting
- "missing-refusal-path" — Refusal surface not fully explored
- "hidden-process-mining-growth" — Model grew after admission
- "missing-temporal-ordering" — Events lack timestamp context
- "missing-object-life" — Object lifecycle incomplete

---

## Conformance Metrics (Engine Output)

After engine processes evidence:

```wit
record metric {
  kind: "fitness",
  numerator: 95,
  denominator: 100,
  notes: "95% of trace events match model transitions",
}
```

All metrics bounded in [0, 1] via Between01 typestate:
- **fitness** — Trace replayability
- **precision** — Model allows no unobserved behavior
- **generalization** — Model captures all observed sequences
- **simplicity** — Structural parsimony (minimized nodes/edges)

---

## Roadmap: Phases 1–4

### Phase 1: Template Validation (Week 1–2)
- [x] Manufacture wasm4pm-compat.wit.ggen template (828 lines)
- [ ] Validate Tera syntax
- [ ] Dry-run rendering with mock context
- [ ] Generate all 6 WIT files locally

### Phase 2: WIT Syntax Validation (Week 2–3)
- [ ] Run wit-parser on all 6 generated .wit files
- [ ] Verify interface imports/exports are consistent
- [ ] Verify no circular dependencies
- [ ] Validate record/variant/function signatures

### Phase 3: Cross-Check with Rust Types (Week 3–4)
- [ ] Compare wit event-log record with src/eventlog.rs::EventLog
- [ ] Compare wit ocel-log record with src/ocel.rs::OcelLog
- [ ] Verify refusal-reason variants match admission.rs enums
- [ ] Verify witness-info matches Witness trait metadata

### Phase 4: Type-Law Receipt Tests (Week 4–5)
- [ ] Write compile-fail WIT fixtures (witness mismatch)
- [ ] Write compile-pass WIT fixtures (valid admission)
- [ ] Add WIT validation to CI pipeline
- [ ] Verify ALIVE gate covers WIT-level law

---

## Troubleshooting

### Template Not Rendering

**Symptom:** `cargo make ggen-wit-render` fails

**Diagnosis:**
- Check Tera syntax in template (verify conditionals are paired)
- Verify ggen.toml points to correct template file
- Ensure input ledgers exist

**Fix:**
```bash
cd /Users/sac/wasm4pm-compat
cargo make ggen-lint  # Validate template
cat ggen/ggen.toml    # Check config
```

### Generated WIT Parse Errors

**Symptom:** `wit-parser ggen/wit/compat.wit` fails

**Diagnosis:**
- Check interface names (must be `interface name@version { ... }`)
- Verify function signatures (must be `func-name: func(args) -> result<ok, err>;`)
- Check variant payload syntax (must be `variant-name(record { field: type })`)

**Fix:**
```bash
wit-parser --explain ggen/wit/compat.wit
# Shows which line/column failed parsing
```

### Feature Flag Mismatch

**Symptom:** Interface defined but not exported by world

**Diagnosis:**
- Template conditional `{%- if FEATURES.contains("X") %}` mismatch
- Tera context not passing FEATURES variable

**Fix:**
- Ensure ggen.toml sets `feature_flags = ["formats", "strict", "wasm4pm"]`
- Verify ggen.toml provides `FEATURES` context variable to Tera

---

## References

- **WIT Reference:** https://component-model.bytecodealliance.org/design/wit.html
- **wit-parser:** https://docs.rs/wit-parser
- **wit-bindgen:** https://github.com/bytecodealliance/wit-bindgen
- **Component Model MVP:** https://github.com/WebAssembly/component-model/blob/main/design/mvp/Explainer.md

---

## Summary

This template is the single source of truth for the wasm4pm-compat WIT surface. It:

1. **Generates 6 WIT files** from one source (feature-gated)
2. **Documents type-law boundaries** (admission → graduation → engine)
3. **Encodes named laws** (7 refusal variants, 41 witness markers)
4. **Enforces loss accounting** (every projection requires policy + report)
5. **Gates graduation** (grounded semantics check before engine)
6. **Provides witness metadata** (const registry for authority lookup)

The template is ready for integration into the ggen build pipeline and wit-bindgen toolchain.
