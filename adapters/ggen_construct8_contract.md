# ggen ↔ CONSTRUCT8 Adapter Boundary Contract

**Status:** DRAFT | Boundary Definition | Adapter Specification
**Date:** 2026-06-01  
**Scope:** What ggen can emit, validate, own; what belongs to open-ontologies; interface contracts  
**Binding Authority:** CONSTRUCT8 doctrine (process-evidence manufacturing covenant)

---

## Executive Summary

This document defines the precise boundary between:
- **ggen**: Code generator driven by TTL ontologies + SPARQL queries, always outputting deterministic source artifacts with cryptographic receipts
- **open-ontologies**: Public ontology authority, holding admissible reference models (W3C, ISO, industry standards) and CONSTRUCT8 shape rules
- **CONSTRUCT8**: The covenant unifying the two — proof-first manufacturing pipeline from ontology → code → validation → receipt

**Core Contract:**
> ggen NEVER becomes the market physics engine, constraint solver, or domain business logic. ggen ALWAYS manufactures structure-preserving artifacts with evidence of lawful execution. open-ontologies NEVER hardcodes application logic; it defines shape, strategy, and proof rules only.

---

## What ggen CAN emit (artifact classes)

### Tier 1: Zero-Cost Abstractions (no semantics)

ggen is **always safe** to emit:

1. **Type stubs** — NewType wrappers, enum discriminants, struct fields (zero semantic meaning)
   - Example: `struct EventId(String);` from `c8:EventIdClass`
   - No validation logic, no business rules
   - Cost: exactly as much as writing by hand

2. **Trait implementations** — Automatic derives (Clone, Debug, Display) + manual trait stubs
   - Example: `impl Display for EventId { … }`
   - Purely ergonomic, no behavior
   - Cost: delegation only

3. **Registry tables** — Lookup maps, string arrays, const slices, enums
   - Example: `const VALID_STATES: &[&str] = &["Draft", "Live", …];`
   - Static, no mutation
   - Cost: memory + zero runtime

4. **Query builders** — Typed SPARQL AST nodes, parameter binding
   - Example: `struct SelectClause { vars: Vec<String> }`
   - No query execution, no data loading
   - Cost: struct allocation

5. **Documentation surfaces** — Generated rustdoc, markdown index, example code stubs
   - Example: `/// Construct an EventId from a string ID\npub fn new(id: &str) → Self { … }`
   - Teaching only, no inference
   - Cost: text generation

### Tier 2: Validated Structure (evidence required)

ggen can emit **only if SHACL validation receipt is proven**:

6. **Validation guards** — Input predicates extracted from SHACL shapes
   - Example: `if !name.is_ascii_lowercase() { return Err(…); }`
   - Derived from `sh:pattern`, `sh:minLength`, `sh:maxLength`
   - **Gate:** `onto:ValidationContract` must pass; receipt must reference source shape
   - Cost: predicate logic only

7. **Bounded delta codecs** — Encode/decode routines for delta snapshots
   - Example: `impl From<DeltaV1> for ProcessDelta { … }` (from c8-time submodule)
   - Derived from `c8:DeltaSchemaShape` in ontology
   - **Gate:** Round-trip SPARQL test must pass; transformation rule must be named in receipt
   - Cost: codec dispatch only, no interpretation

8. **Process model projections** — TS/JS type synonyms, structure projections to wasm4pm interfaces
   - Example: `type ProcessNet = WfNet<SoundnessPaper>;`
   - Derived from ontology `ts:LawProjection` nodes
   - **Gate:** ts-validation suite must compile; type law receipt required
   - Cost: zero at runtime (type erasure)

### Tier 3: Market-Physics-Aware (forbidden, reserved for wasm4pm)

ggen **MUST NOT** emit:

9. ✗ **Collider algorithms** — State collision detection, event horizon computation, Planck cell quantization
10. ✗ **Game theory solvers** — Adversarial payoff matrices, strategy equilibrium search
11. ✗ **Conformance checkers** — Trace alignment, token play, process discovery
12. ✗ **Forecasting engines** — Outcome prediction, trend extrapolation, anomaly scoring
13. ✗ **Constraint optimization** — Planning, resource allocation, deadline management

**Why forbidden:** These require semantic interpretation of process dynamics. They live in wasm4pm's execution layer. ggen is structure-only; it cannot "know" what a process means.

---

## What ggen CAN validate (proof rules)

ggen's `validate()` method operates in two phases:

### Phase 1: Structural Consistency (all artifacts)

**Always executed; all gates A1-A7 must pass:**

- **A1 (Seed):** Artifact declared in manifest; file path matches registered output
- **A2 (Breed):** RDF/Turtle parses; no unprintable characters, valid Unicode
- **A3 (Validate):** SHACL shapes from `onto:ValidationContract` pass; cardinality, datatype, class constraints enforced
- **A4 (Reason):** OWL consistency check (if `onto:OwlConsistency` shape defined); no unsatisfiable classes
- **A5 (Prove):** Receipt chain linked; SHA-256 hashes chain to previous artifact or seed
- **A6 (Seal):** Cryptographic signature present; Ed25519 key ID matches manifest authority
- **A7 (Emit):** Output file is readable, non-empty, size within bounds

### Phase 2: Semantic Validation (ontology-driven)

**Executed only if artifact carries `ggen:validationRule` reference:**

- **TS Law Projection:** TypeScript exports validate type law using dts-gen + type-equivalence oracle
- **SPARQL Competency:** Named SPARQL ASK queries (from `onto:competency_question`) return expected boolean
- **Delta Codec Round-Trip:** `encode → decode → compare(original, decoded)` achieves < ε error bound (if defined in shape)
- **Process Model Soundness:** For `WfNetConst<SOUNDNESS>` types, extract Petri net from AST and run formal soundness oracle (pm4py bridge, if enabled)

**Critical:** ggen does NOT interpret the semantics. It only checks that the validation rule fired and recorded a receipt. The rule itself is owned by open-ontologies + domain experts.

---

## What ggen CANNOT and MUST NOT own

### Boundary Lines (firm)

1. **Domain modeling decisions**
   - ✗ "Should an Event have a timestamp OR a sequence number?"
   - ✓ Ontology decides via `c8:hasTimestampCardinality`, ggen emits the struct field
   
2. **Semantic constraint satisfaction**
   - ✗ "Does this delta sequence preserve causality?"
   - ✓ Ontology defines the invariant (e.g. `c8:CausalityInvariantShape`), wasm4pm enforces it, ggen emits the type
   
3. **Trade-off logic**
   - ✗ "Should we compress timestamps to save memory?"
   - ✓ Ontology marks a `c8:DeltaCodec` as `c8:isMemoryOptimized true`, ggen generates both paths, humans choose
   
4. **Algorithm selection**
   - ✗ "Which conformance algorithm (DTW, levenshtein, token play)?"
   - ✓ open-ontologies defines `onto:ConformanceAlgorithm` taxonomy; wasm4pm implements each; ggen emits adapter traits
   
5. **Execution semantics**
   - ✗ "How do we resolve simultaneous events at the same nanosecond?"
   - ✓ wasm4pm's causality engine decides; ggen provides type holes for strategy injection

### The No-Fake-.ggen-Artifacts Rule

**Problem:** Developers might commit `.ggen.toml` entries that are never validated by the manufacturing pipeline. This creates ghost contracts.

**Prevention (ggen responsibility):**
- Every entry in `.ggen.toml` must have a corresponding `ggen:ManufacturingContract` in the loaded ontology
- `cargo make sync` aborts with `MissingContractError` if an entry exists in TOML but no contract in TTL
- Every emitted artifact must record: (a) contract IRI, (b) validation gate results, (c) receipt signature
- Post-emit: `cargo make verify-receipts` must pass; runs for **all** .ggen artifacts, not just "important" ones

**Test case:**
```rust
#[test]
fn test_no_fake_ggen_artifacts() {
    let manifest = GgenManifest::load(".ggen.toml").unwrap();
    let ontology = OntologyStore::load("ontology/ggen-integration-law.ttl").unwrap();
    
    for (artifact_name, toml_entry) in manifest.artifacts() {
        let contract_iri = format!("ggen:{}", artifact_name);
        assert!(
            ontology.contains_subject(contract_iri),
            "TOML entry '{}' has no matching ggen:ManufacturingContract in ontology",
            artifact_name
        );
    }
}
```

---

## What open-ontologies OWNS (authority boundaries)

### Tier 1: Reference Models (public, shared)

open-ontologies is the **authoritative source** for:

1. **Process evidence structure** — event, object, lifecycle, link definitions
   - Modules: `eventlog`, `ocel`, `xes`, `bpmn`, `petri`, `powl`, `declare`, `ocpq`, `dfg`
   - Authority: W3C, IEEE 1849, van der Aalst papers
   - Surface: Turtle (.ttl) in `ontology/` directory
   - Consumption: ggen loads + validates via SHACL

2. **CONSTRUCT8 shape rules** — cell8-core.ttl, cell8-manufacturing.ttl, cell8-shapes.ttl
   - What is a "lawful process"?
   - What gates must fire in sequence?
   - What are valid state transitions?
   - Surface: SPARQL ASK/SELECT to prove conformance
   - Consumption: wasm4pm execution engine checks at runtime; ggen emits type holes

3. **TS/JS type projections** — cell8-profile.ttl → `ts/export.rs`
   - Which process-compat types project to TypeScript?
   - What interface do they implement?
   - What law surfaces are exported?
   - Surface: `ts:TypeProjection`, `ts:LawProjection` classes
   - Consumption: ggen generates .d.ts; ts-validation compiles them

4. **Public alignment** — public-alignment.ttl, crosswalk data
   - Which clinical codes (ICD-10, SNOMED) map to process terms?
   - Which industry standards (ISO 19448, BPMN 2.0.2) are admissible?
   - What shapes enforce clinical conformance?
   - Surface: `skos:exactMatch`, `rdfs:subClassOf` chains
   - Consumption: open-ontologies tools (alignment oracle); ggen emits type adapters

### Tier 2: Manufacturing Rules (internal, Cell8)

open-ontologies owns:

5. **Manufacturing gates** (A1-A7, A8-A13) — Gates sequence, failure modes
   - Defined in: ggen-integration-law.ttl under `:ManufacturingGates`
   - Who can emit what? When does validation halt?
   - When is a receipt valid proof of lawful execution?
   - Surface: onto:Gate, onto:GateSequence, onto:Action
   - Consumption: ggen runs gates; records results in receipt JSON

6. **Validation contracts** — SHACL shapes, constraint rules
   - All `.ttl` files in `ontology/` are validation authority
   - Every shape must have `sh:message` in English (human-readable failure reason)
   - Shapes are versioned; old versions archived in `ontology/profiles/`
   - Surface: SHACL shapes (.shacl-quads in ontology files)
   - Consumption: ggen runs SHACL validate; blocks on failure (A3 gate)

### Tier 3: Ecosystem Bridges (reserved for later)

open-ontologies **may later own**:

7. **SQL schema mapping** — Data ingestion into process evidence
8. **REST API stubs** — Service boundaries for process query
9. **Workflow orchestration DSL** — BPMN-compatible task definitions

But **currently reserved** — not yet specified.

---

## Public Ontologies (admissible sources)

open-ontologies can import from these **vetted, read-only** public ontology libraries:

| Standard | Authority | Use Case | Admissible |
|---|---|---|---|
| **W3C PROV-O** | W3C | Provenance chain (receipts) | ✓ Read-only import |
| **W3C RDF/OWL** | W3C | Core semantic web | ✓ Always loaded |
| **IEEE 1849 XES** | IEEE | Event log taxonomy | ✓ Via `xes_vocab.ttl` |
| **OCEL 2.0** | van der Aalst et al. | Object-centric logs | ✓ Via `ocel_vocab.ttl` |
| **BPMN 2.0.2** | OMG | Process modeling | ✓ Via `bpmn_vocab.ttl` |
| **ISO 9001 Quality** | ISO | Quality gates/metrics | ✓ Via `iso9001_vocab.ttl` |
| **SKOS** | W3C | Thesaurus/alignment | ✓ For crosswalks only |
| **Dublin Core (dct)** | DCMI | Metadata (creator, license) | ✓ Always loaded |

**Not admissible:**
- ✗ Vendor-specific ontologies (Salesforce, SAP, Workday)
- ✗ Proprietary process models not peer-reviewed
- ✗ Graph databases' internal schema (Neo4j, ArangoDB schema IRIs)

**Reason:** Admissible sources allow open-ontologies to remain neutral arbiter. Vendor lock-in kills portability.

---

## Surface Format Allocation (TTL, RQ, TERA, TOML, TS)

Where each artifact type lives and who owns it:

| Format | Artifact Type | Owner | Read/Write Authority | Consumer |
|---|---|---|---|---|
| **.ttl** (Turtle) | Ontology definitions, validation shapes, manufacturing contracts | open-ontologies | open-ontologies (append-only) | ggen (load, validate, query) |
| **.rq** (SPARQL) | Competency queries, extraction rules, projection definitions | open-ontologies (templates) + ggen (generated) | open-ontologies writes templates; ggen executes | ggen (execute) → wasm4pm (for conformance oracles) |
| **.rs.tera** (Tera template) | Source code generation templates | ggen (author) | ggen (edit source) | ggen (render with SPARQL results) |
| **.rs** (Rust source) | Generated code artifacts | ggen (manufactured) | No human edit (regenerate from .ttl) | Rust compiler (type-check, compile) |
| **.d.ts** (TypeScript def) | Type projections for TS/JS consumers | ggen (manufactured from `ts:LawProjection`) | No human edit | tsc, deno (type-check) |
| **.toml** (.ggen.toml) | Manufacturing manifest, rule registry | Human (in wasm4pm-compat) | Human edit; ggen validates → onto | ggen (load, validate against ontology) |
| **.json** (Receipt) | Manufacturing proof: contract IRI, gate results, signatures | ggen (emitted) | No edit (immutable) | ggen verify, humans (audit), CI (gate checks) |
| **.md** (Markdown) | Type-law documentation, interface specs | ggen (generated) + humans (documentation) | Humans (edit separately) | Developers (read) + CI (lint) |

---

## Bounded Delta ↔ Source Law Artifact

A critical adaptive surface for c8-time submodule:

### Concept

**DeltaV1 encoding:** Compress a sequence of `ProcessChange` into a compact binary format.
**Source law artifact:** The SPARQL rule + SHACL shape that defines what deltas are **legal**.

### Contract

```turtle
@prefix c8: <https://c8.io/onto/> .
@prefix ggen: <https://ggen.io/onto/ggen/> .

:DeltaCodecContract
  a ggen:Contract ;
  rdfs:label "Delta Codec Manufacturing Contract" ;
  ggen:loads [
    ggen:graphUri c8:delta-core ;
    ggen:format "turtle" ;
  ] ;
  ggen:executes [
    rdfs:label "Extract Delta Schema from SPARQL" ;
    ggen:queryFile "sparql/extract-delta-schema.rq" ;
    ggen:againstGraph c8:delta-core ;
    ggen:produces c8:DeltaSchemaFacts ;
  ] ;
  ggen:renders [
    ggen:template "templates/delta-codec.rs.tera" ;
    ggen:inputFacts c8:DeltaSchemaFacts ;
    ggen:outputFile "src/c8/delta_codec.rs" ;
  ] ;
  ggen:validates [
    rdfs:label "Delta Codec Round-Trip Validation" ;
    onto:validationRule c8:DeltaRoundTripShape ;
    onto:testCorpus "fixtures/delta_roundtrip_*.json" ;
    ggen:epsilon 0.0 ;  # Perfect fidelity required
  ] ;
  ggen:emits [
    ggen:path "src/c8/delta_codec.rs" ;
    ggen:registersAs onto:delta-codec-artifact ;
  ] ;
  ggen:produces [
    ggen:receiptPath "receipts/delta-codec-receipt.json" ;
  ] .

:DeltaRoundTripShape
  a sh:NodeShape ;
  sh:targetClass c8:DeltaCodec ;
  sh:property [
    sh:path c8:encodeDecodeInverse ;
    sh:maxCount 0 ;  # If this property exists and is non-null, round-trip failed
    sh:message "Delta codec failed round-trip: encode(v) → decode(v') where v ≠ v'" ;
  ] .
```

### Workflow

1. **Ontology authority** (open-ontologies team) defines:
   - `c8:DeltaField` — which attributes can delta-encode?
   - `c8:DeltaEncoding` — which algorithm (zigzag, varint, etc.)?
   - `c8:DeltaRoundTripShape` — test cases that validate encode/decode fidelity

2. **ggen** manufactures:
   - Extracts delta fields via SPARQL
   - Renders codec.rs with match arms for each field
   - Runs round-trip validation (fixture corpus)
   - Emits receipt with validation gate results

3. **wasm4pm** consumes:
   - Imports compiled `DeltaCodec` trait impl
   - Uses at runtime to compress process traces
   - **Never interprets** the encoding logic (structure-only)

### Receipt Structure

```json
{
  "contract": "urn:ggen:delta-codec-contract",
  "gates": {
    "A1_seed": { "passed": true, "artifact": "src/c8/delta_codec.rs" },
    "A2_breed": { "passed": true, "parse_errors": 0 },
    "A3_validate": { 
      "passed": true,
      "shacl_shapes": ["c8:DeltaRoundTripShape"],
      "test_corpus_size": 47,
      "failures": 0,
      "epsilon_bound": 0.0
    },
    "A5_prove": { "passed": true, "chain_to": "urn:receipt:delta-core-ontology" },
    "A6_seal": { "passed": true, "algorithm": "ed25519", "key_id": "ggen-v1" },
    "A7_emit": { "passed": true, "bytes": 12847 }
  },
  "lineage": {
    "ontology_sources": ["ontology/c8-time.ttl"],
    "sparql_queries": ["sparql/extract-delta-schema.rq"],
    "templates": ["templates/delta-codec.rs.tera"],
    "validation_rule_iris": ["https://c8.io/onto/DeltaRoundTripShape"]
  },
  "signature": {
    "alg": "ed25519",
    "sig": "3a9f...",
    "key_id": "ggen-v1"
  }
}
```

---

## How ggen Avoids Fake .ggen Surfaces

### Problem Statement

A developer could write a `.ggen.toml` entry like:

```toml
[[artifacts]]
name = "my_perfect_struct"
output = "src/my_struct.rs"
# But there's NO ggen:ManufacturingContract in the ontology backing it
```

This creates a **ghost artifact** — claimed to be manufactured, but never validated.

### Solution: Contract Traceability

**In ontology (open-ontologies responsibility):**

```turtle
:MyPerfectStructContract
  a ggen:Contract ;
  rdfs:label "My Perfect Struct Manufacturing" ;
  ggen:tomlEntryName "my_perfect_struct" ;  # MUST match .ggen.toml [[artifacts]].name
  ggen:loads [ … ] ;
  ggen:executes [ … ] ;
  ggen:renders [ … ] ;
  ggen:produces [ … ] ;
  ggen:validates [ … ] .
```

**In ggen code (ggen responsibility):**

```rust
pub fn validate_manifest_contracts(
    manifest: &GgenManifest,
    ontology: &OntologyStore,
) -> Result<(), ManifestError> {
    for artifact in manifest.artifacts() {
        let contract_iri = artifact_to_contract_iri(&artifact.name);
        if !ontology.contains_subject(&contract_iri) {
            return Err(ManifestError::MissingContract {
                artifact: artifact.name.clone(),
                expected_contract: contract_iri,
            });
        }
    }
    Ok(())
}
```

**In CI (wasm4pm-compat responsibility):**

```bash
# Pre-sync: verify all .ggen.toml entries have backing contracts
cargo make validate-manifest

# Sync: manufacture all artifacts
cargo make sync

# Post-sync: verify all receipts are present and valid
cargo make verify-receipts

# If any artifact has no receipt or receipt is invalid → CI blocks → human investigation
```

**Test case (wasm4pm-compat):**

```rust
#[test]
fn test_all_toml_entries_have_backing_contracts() {
    let manifest = GgenManifest::load("ggen.toml").unwrap();
    let ontology = OntologyStore::load_all("ontology/").unwrap();
    
    // This test must pass before any artifact is generated
    ggen::validate_manifest_contracts(&manifest, &ontology).unwrap();
}

#[test]
fn test_all_receipts_are_present_after_sync() {
    // After `cargo make sync` completes, verify:
    // For each artifact in manifest, receipts/<artifact>.json must exist
    let manifest = GgenManifest::load("ggen.toml").unwrap();
    for artifact in manifest.artifacts() {
        let receipt_path = PathBuf::from("receipts")
            .join(format!("{}.json", artifact.name));
        assert!(receipt_path.exists(),
            "Missing receipt for artifact '{}' at {:?}",
            artifact.name, receipt_path
        );
        // Verify receipt has valid signature
        let receipt = Receipt::load(&receipt_path).unwrap();
        assert!(receipt.verify_signature().unwrap());
    }
}
```

---

## Fixture Example: TS Type Projection (Optional)

If the boundary is **obvious** and repo is **clean**, add a minimal test fixture:

```rust
#[test]
#[ignore] // Explicit: TS export optional, requires ts feature
fn test_ts_law_projection_contract() {
    // Demonstrates the contract boundary:
    // 1. Ontology defines ts:TypeProjection
    // 2. ggen extracts and emits .d.ts
    // 3. ts-validation compiles .d.ts
    // 4. Receipt proves round-trip

    use wasm4pm_compat::ts::LawProjection;
    use wasm4pm_compat::petri::WfNetConst;
    use wasm4pm_compat::receipt::Receipt;

    // Step 1: Load ontology
    let ontology_path = "ontology/cell8-profile.ttl";
    assert!(std::path::Path::new(ontology_path).exists(),
        "Ontology not found: {}", ontology_path);

    // Step 2: Simulate ggen manufacturing
    // (In real scenario, ggen renders .d.ts from ontology)
    let ts_export = r#"
export type ProcessNet = WfNet<SoundnessPaper>;
export interface Admitted<T, W> {
    readonly value: T;
    readonly witness: W;
}
    "#;

    // Step 3: Verify receipt exists
    let receipt_path = "receipts/ts-law-projection-receipt.json";
    assert!(std::path::Path::new(receipt_path).exists(),
        "Receipt not found: {}", receipt_path);

    // Step 4: Load and verify receipt
    let receipt = Receipt::load_from_file(receipt_path)
        .expect("Failed to load receipt");
    
    assert_eq!(receipt.contract_iri(), "urn:ggen:ts-law-projection-contract");
    assert!(receipt.gate("A7_emit").unwrap().passed, "Emit gate failed");
}
```

---

## Summary: Who Does What

| Task | Owner | Authority | Proof |
|---|---|---|---|
| Define what process models are lawful | open-ontologies | Cell8 doctrine | SHACL shapes, papers in docs/ |
| Define which types ggen can emit | open-ontologies (via ggen-integration-law.ttl) + ggen (implementation) | Manufacturing contracts | ggen:Contract classes in ontology |
| Manufacture type stubs, validators, adapters | ggen | Code generation pipeline | Receipt JSON (SHA-256, Ed25519) |
| Validate manufactured artifacts structurally | ggen | SHACL shapes from ontology | A1-A7 gates in receipt |
| Interpret manufactured types semantically | wasm4pm execution engine | Process dynamics logic | Runtime checks, conformance traces |
| Prevent fake .ggen surfaces | CI (wasm4pm-compat) | Manifest traceability | test_all_toml_entries_have_backing_contracts |
| Resolve which compressed delta format to use | Human (domain expert) | Trade-off decision | Documented in ontology, noted in receipt |
| Define what "lawful delta encoding" means | open-ontologies | SHACL DeltaRoundTripShape | Test corpus in fixtures/ + shape in ontology |

---

## Questions This Contract Answers

### Q: Can ggen emit a conformance checker?
**A:** No. Conformance checking interprets process dynamics (which traces are "correct"). That's a semantic question answered by wasm4pm, not structure. ggen can emit **types** for conformance results (e.g., `struct ConformanceScore { … }`) and validators for input shape, but not the algorithm.

### Q: Can ggen emit a TypeScript interface for wasm4pm?
**A:** Yes, **if** open-ontologies defines the type projection in `ts:LawProjection`. ggen extracts via SPARQL, renders via Tera, validates via ts-validation suite. Receipt proves the law was followed.

### Q: Can ggen emit a Petri net soundness checker?
**A:** No. Soundness checking requires **semantic reasoning** about reachability, liveness, boundedness. ggen can emit **types** that carry soundness witnesses (e.g., `WfNetConst<SOUND>` with non-forgeable proof), but not the decision procedure.

### Q: Can ggen emit SQL triggers or stored procedures?
**A:** No. SQL is **executable logic** with side effects. ggen is structure-only, zero-cost. Reserved for future `sql` feature behind explicit opt-in + strong warnings.

### Q: Can ggen emit a Tera template?
**A:** No. Tera templates are ggen's **input language**, not output. ggen renders **from** Tera templates **to** source code. Emitting a template would be circular.

### Q: What if two projects have conflicting ontologies?
**A:** That's a **governance problem**, not a ggen problem. open-ontologies enforces alignment through:
- Central `public-alignment.ttl` for crosswalks
- SKOS equivalence mappings
- Git history for version drift detection
- `onto_diff` tool for human review

ggen is neutral — it manufactures whatever ontology you load.

### Q: Who decides if a receipt is valid proof?
**A:** The **validation contract** in ontology decides. ggen just runs the contract. If `onto:onFailure` says "halt," ggen halts. If it says "warn," ggen warns. The contract is the law.

---

## Next Steps (Future Work)

1. **Publish first real ggen-construct8 contract** — Add `urn:ggen:wasm4pm-stub-contract` to `open-ontologies/ontology/ggen-integration-law.ttl`
2. **Fixture test** — Add `tests/ts_law_projection_contract.rs` once TS export is stable
3. **Manifest validation gate** — Add `cargo make validate-manifest` to wasm4pm-compat CI
4. **Receipt audit trail** — Implement `cargo make verify-receipts --audit-log` for compliance
5. **Alignment oracle** — Extend `onto_align` to detect contract drift across versions

---

**Authority:** CONSTRUCT8 doctrine (process-evidence manufacturing)  
**Binding:** wasm4pm-compat, ggen, open-ontologies  
**Enforcement:** CI gates (manifest validation, receipt verification, shape conformance)  
**Review Date:** 2026-09-01 (quarterly audit)
