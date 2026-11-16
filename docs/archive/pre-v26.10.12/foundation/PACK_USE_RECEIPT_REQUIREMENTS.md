# Pack-Use Receipt Requirements

*Derived from `DAY3_FOUNDATION_LAW.md`. Defines what constitutes a valid pack-use receipt.*

---

## Why the Current Receipt Is Not Valid

The current ggen receipt (`wasm4pm/.ggen/receipts/latest.json`) records:

```json
{
  "input_hash": "ggen.toml:ead9d4dca7a0433...",
  "output_hash": "./wasm4pm/src/generated/witnesses.rs:15bd460a...",
  "signature": "..."
}
```

This receipt proves **only** that a specific `ggen.toml` produced a specific output file. It does not prove:

- Which ontology TTL files were the source data
- Which SPARQL query was used to extract projections
- Which Tera template was used to render the output
- Whether the output is operationally imported anywhere
- Whether the output can be replayed from the pack contract

A receipt that proves only `manifest → output` is **not a pack-use receipt**. It is a manifest-to-artifact hash. It proves that ggen ran. It does not prove that the substrate's kind was carried forward into an operational consumer surface.

---

## Minimum Valid Pack-Use Receipt

A pack-use receipt MUST hash the following inputs, in this order:

### 1. Ontology Source Files

Every TTL file that participated in the generation:

```json
"ontology_hashes": {
  "wasm4pm-compat.ttl": "<blake3>",
  "papers.ttl": "<blake3>",
  "domain-process-forms.ttl": "<blake3>",
  "domain-evidence-structure.ttl": "<blake3>",
  "domain-graduation-boundaries.ttl": "<blake3>"
}
```

If the ontology changes and is re-hashed, the receipt is invalidated and a new ggen sync is required.

### 2. SPARQL Query Files

Every `.rq` file that extracted projections for this rule:

```json
"query_hashes": {
  "extract-witnesses-full.rq": "<blake3>"
}
```

### 3. Tera Template Files

Every `.tera` file that rendered the extracted projections:

```json
"template_hashes": {
  "witness-marker.tera": "<blake3>"
}
```

### 4. Pack Manifest

The ggen.toml that declared the rule:

```json
"manifest_hash": {
  "ggen.toml": "<blake3>"
}
```

### 5. Rendered Output Files

Every file produced by the rule:

```json
"output_hashes": {
  "src/witnesses.rs": "<blake3>"
}
```

### 6. Consumer Use-Sites

The module paths and file locations where the rendered output is operationally referenced. This must be recorded at receipt time (not inferred later):

```json
"use_sites": [
  {
    "kind": "mod_declaration",
    "file": "src/lib.rs",
    "line": 14,
    "declaration": "pub mod witnesses;"
  },
  {
    "kind": "use_statement",
    "file": "src/discovery.rs",
    "line": 3,
    "declaration": "use crate::witnesses::PowlPaper;"
  }
]
```

If `use_sites` is empty, the receipt MUST be refused. An orphaned output is not a valid pack-use receipt.

### 7. Test Compilation Coverage

At least one test that compiles and uses the rendered output:

```json
"test_coverage": [
  {
    "test_file": "tests/witness_compilation.rs",
    "test_name": "powl_paper_witness_compiles",
    "uses_output": "src/witnesses.rs"
  }
]
```

### 8. Previous Receipt Hash

```json
"previous_receipt_hash": "<blake3 of the prior receipt JSON>"
```

---

## Receipt Schema (Normative)

```json
{
  "receipt_version": "2",
  "rule_name": "<ggen rule name, e.g. 'witness-markers'>",
  "pack_name": "<pack name, e.g. 'wasm4pm-compat'>",
  "consumer_name": "<consumer name, e.g. 'wasm4pm'>",
  "timestamp_utc": "<ISO-8601>",
  "ontology_hashes": { "<filename>": "<blake3>", ... },
  "query_hashes": { "<filename>": "<blake3>", ... },
  "template_hashes": { "<filename>": "<blake3>", ... },
  "manifest_hash": { "<filename>": "<blake3>" },
  "output_hashes": { "<output_path>": "<blake3>", ... },
  "use_sites": [
    { "kind": "<mod_declaration|use_statement|function_call|test_body>",
      "file": "<relative path>", "line": <int>, "declaration": "<text>" }
  ],
  "test_coverage": [
    { "test_file": "<path>", "test_name": "<name>", "uses_output": "<path>" }
  ],
  "previous_receipt_hash": "<blake3 or null>",
  "signature": "<ed25519 hex>"
}
```

---

## Refusal Conditions

A receipt engine MUST refuse to issue a receipt if any of the following conditions hold:

### REFUSE_ORPHAN
The rendered output has no use-sites recorded. An orphan output proves nothing and its receipt is fraudulent.

```
REFUSE_ORPHAN: output 'src/witnesses.rs' has no recorded use-sites.
Pack-use receipt refused. Run ggen sync after adding mod declaration.
```

### REFUSE_INCOMPLETE_HASHES
The receipt omits any of: ontology hashes, query hashes, template hashes.

```
REFUSE_INCOMPLETE_HASHES: ontology_hashes is empty.
A manifest-only receipt does not prove the manufacturing chain.
```

### REFUSE_COMPETING_AUTHORITY
Two or more ggen.toml files declare rules with overlapping output paths for the same consumer.

```
REFUSE_COMPETING_AUTHORITY: both 'ggen.toml' (root) and 'ggen/ggen.toml' (nested)
declare output '../wasm4pm-compat/src/witnesses.rs' / 'wasm4pm/src/generated/witnesses.rs'.
Competing authorities cannot both be receipted. Consolidate to one manifest.
```

### REFUSE_SECOND_CLASS_OUTPUT
The output path contains a `generated/` path component.

```
REFUSE_SECOND_CLASS_OUTPUT: output path 'src/generated/witnesses.rs' contains 'generated/'.
Rendered source is source. Relocate output to 'src/witnesses.rs'.
```

### REFUSE_UNREGISTERED_WITNESS
A witness is used in the output but the type it tags has no ontology declaration. (Enforced during the pack chain execution, not at receipt issuance — but the receipt records the violation.)

```
REFUSE_UNREGISTERED_WITNESS: witness 'PowlPaper' is asserted on 'PowlArena'
but 'PowlArena' has no ontology declaration.
Asserted witness is invalid past bootstrapping phase.
```

---

## Replay Contract

A receipt is a promise that the rendered output can be reproduced. **Replay** is the verification:

```bash
# Replay contract: run the pack chain from the receipted inputs,
# verify that the output hash matches the receipted output hash.
ggen replay --receipt receipts/pack-use/witness-markers-<timestamp>.json
```

Replay MUST:
1. Re-fetch (from cache) the exact ontology TTL files at the receipted hashes
2. Re-run the receipted SPARQL query against those TTL files
3. Re-render the receipted template from the query results
4. Compute BLAKE3 of the new output
5. Assert bit-identity with `output_hashes` in the receipt

If replay diverges, the receipt is broken. This is an Andon pull.

---

## Current State vs Requirements

| Requirement | Current Receipt (`latest.json`) | Gap |
|---|---|---|
| Ontology hashes | ❌ Missing | `ontology_hashes` field absent |
| Query hashes | ❌ Missing | `query_hashes` field absent |
| Template hashes | ❌ Missing | `template_hashes` field absent |
| Manifest hash | ✅ Present (as `input_hash`) | Exists but conflated with ontology |
| Output hashes | ✅ Present (as `output_hash`) | Exists; path includes `generated/` (REFUSE_SECOND_CLASS_OUTPUT) |
| Use-sites | ❌ Missing | `use_sites` field absent; output is orphaned |
| Test coverage | ❌ Missing | `test_coverage` field absent |
| Previous receipt hash | ✅ Present | Chain exists |
| Signature | ✅ Present | Ed25519 present |

**Current receipt validity: REFUSED** on: REFUSE_ORPHAN + REFUSE_INCOMPLETE_HASHES + REFUSE_SECOND_CLASS_OUTPUT

The substrate receipt (wasm4pm-compat) has empty `input_hashes` and `output_hashes`. This means it is **not a receipt at all** — it is a signed empty envelope.

---

## Receipt Versioning

| Version | Introduced | Minimum Required Fields |
|---|---|---|
| v1 (current) | ~2026-05 | `input_hash` (manifest only), `output_hash`, `signature` |
| v2 (this document) | 2026-06-03 | All fields above; refusal conditions enforced |

No existing receipt is v2-compliant. v2 compliance is a Day 4 implementation target, not a Day 3 enforcement. This document declares the requirement; enforcement is the next admissible work cell.

---

*This document is the controlling specification for pack-use receipts. The ggen receipt engine must be updated to implement these requirements before any pack-use can be claimed as proven.*
