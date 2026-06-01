# Unified RDF Hook Ontology — Index & Quick Reference

**File:** `emitted/unified-rdf-hook-ontology.md`  
**Version:** 1.0  
**Date:** 2026-06-01  
**Authority:** wasm4pm-compat, process-mining-chicago-tdd, manufacturing-terminology

---

## Document Structure

The unified RDF hook ontology is organized into 7 major parts:

### **Part 1: Hook Class Hierarchy** (Lines 41–394)
Complete taxonomy of hook types from abstract markers to concrete implementations.

**Key Classes:**
- `AbstractHook` — root zero-cost type-level marker
- `StructuralHook` — type-system enforcement (compile-time)
- `SemanticHook` — event-log firing (SPARQL ASK, pm4py discovery)
- `ExecutableHook` — object-centric process transformation
- `AuthorityHook` — governance & witness enforcement
- `CompositionHook` — how hooks chain and branch
- `RefusalHook` — named law violation (terminal)
- `LossHook` — projection with explicit policy + report
- `ReceiptHook` — BLAKE3-chained proof gates
- `BlockingHook` — can prevent forward motion (e.g., Stop events)

**Specializations (Lines 173–394):**
- Session-level: `SessionStartHook`, `SessionStopHook`
- Tool-level: `PreToolUseHook`, `PostToolUseHook`
- User interaction: `UserPromptSubmitHook`
- Type-law: `TypeLawHook` (nightly features)
- Admission: `AdmissionHook` (Raw → Admitted)
- Graduation: `GraduationHook` (Admitted → wasm4pm)

---

### **Part 2: Core Hook Properties** (Lines 395–857)
Complete property schema for all hook characteristics.

**Properties by Category:**

| Category | Properties | Usage |
|----------|-----------|-------|
| **Identity & Metadata** | `hook:name`, `hook:title`, `hook:description`, `hook:docstring` | Label & document a hook |
| **Activation** | `hook:trigger`, `hook:precondition`, `hook:queryPattern` | Define firing conditions |
| **Effect** | `hook:effect`, `hook:inputState`, `hook:outputState` | Specify what hook does |
| **Authority & Governance** | `hook:authority`, `hook:witness`, `hook:refusalReason` | Control who can invoke |
| **Loss Accounting** | `hook:lossPolicy`, `hook:projectionName`, `hook:lossReport` | Track discarded data |
| **Receipt** | `hook:receipt`, `hook:receiptChain`, `hook:receiptMeasuredValue` | Proof chain metadata |

**Trigger Types (Lines 407–429):**
- `TriggerTypeSession` — session event (start, stop, prompt)
- `TriggerTypeEvent` — object-centric event (object state change)
- `TriggerTypeQuery` — SPARQL ASK returns true
- `TriggerTypeState` — typestate milestone (Raw→Parsed, etc.)

**Effect Types (Lines 442–473):**
- `EffectAdmit` — Raw → Admitted (via `Admit` impl)
- `EffectRefuse` — reject with named reason
- `EffectProject` — lossy transform with LossPolicy + LossReport
- `EffectReceipt` — emit BLAKE3-chained proof
- `EffectBlock` — prevent forward motion
- `EffectAdvance` — typestate transition
- `EffectReport` — emit diagnostic/loss/status

**Authority Hierarchy (Lines 476–533):**
- `AuthorityStandard` — published standards (OCEL 2.0, XES 1849)
- `AuthorityPaper` — academic papers (van der Aalst, Kourani, etc.)
- `AuthorityRustLaw` — Rust language + nightly features
- `AuthorityEngine` — wasm4pm execution engine
- `AuthorityUser` — session user (hooks in settings.json)

**Witness Property (Lines 536–585):**
- `hook:witnessKey` — machine-readable ID (e.g., "ocel-2.0")
- `hook:witnessTitle` — human-readable title
- `hook:witnessYear` — publication year
- `hook:witnessFamily` — Standard | Paper | ApiGrammar | RustLaw | InternalBridge

**Refusal Reasons (Lines 588–639):**
- `MissingObjectRelation` — OCEL E2O/O2O missing
- `FlatteningLoss` — OCEL → XES loses structure
- `DeadTransition` — Petri net reachability violated
- `UnsoundWfNet` — WF-net soundness law broken
- `InvalidPowlProjection` — POWL depth/arity violated
- `MissingWitness` — evidence carries no witness type
- `UnreplayableClaim` — receipt has no valid replay history

**Loss Policy (Lines 642–677):**
- `RefuseLoss` — reject if lossy
- `AllowNamedProjection` — allow under named `ProjectionName`
- `AllowLossWithReport` — allow + must emit `LossReport`

**Receipt Properties (Lines 680–724):**
- `receiptChain` — BLAKE3 hash pointer
- `receiptWitness` — authority this receipt answers to
- `receiptProofGate` — gate name (e.g., "TokenReplayFitness")
- `receiptMeasuredValue` — quality dimension (fitness, precision, etc.)
- `receiptTimestamp` — ISO 8601 proof time
- `receiptObjectId` — which case/artifact this receipt applies to
- `receiptReplayPointer` — OCEL pointer for verification

---

### **Part 3: Hook Activation Grammar** (Lines 858–1055)
SPARQL query patterns that define when hooks fire.

**6 Query Patterns:**

1. **Typestate Transition (Structural)** — Evidence reaches `Admitted` state
   - Precondition: `ASK WHERE { ?ev evidence:state state:Admitted . }`
   - Effect: `EffectAdvance`

2. **Object-Centric Event Log (Semantic)** — Object reaches final state in OCEL
   - Precondition: `ASK WHERE { ?obj ocel:state ocel:Final . }`
   - Effect: `EffectReceipt`

3. **Loss Accounting (Projection)** — Lossy projection without LossPolicy
   - Precondition: `ASK WHERE { ?proj formats:isLossy true . FILTER NOT EXISTS { ?proj loss:policy ?pol . } }`
   - Effect: `EffectRefuse` (FlatteningLoss)

4. **Session Loop Control (Blocking)** — Stop event + loop not complete
   - Precondition: `ASK WHERE { <urn:loop:current> loop:iterationNumber ?iter ; loop:maxIterations ?max . FILTER (?iter < ?max) . }`
   - Effect: `EffectBlock`

5. **Timestamp Monotonicity (Conformance)** — Event timestamps violate ordering
   - Precondition: `ASK WHERE { ?e1 event:timestamp ?ts1 ; event:nextEvent ?e2 . ?e2 event:timestamp ?ts2 . FILTER (?ts1 > ?ts2) . }`
   - Effect: `EffectRefuse` (TemporalConformanceViolation)

6. **Witness Matching (Authority)** — Hook witness matches caller authority
   - Precondition: `ASK WHERE { ?hook hook:witness witness:Ocel20 ; hook:authority ?auth . FILTER (?auth = witness:Ocel20Authority) . }`
   - Effect: Hook authorized to execute

**Hook Composition via SPARQL (Lines 1032–1055):**
Find hook execution order:
```sparql
SELECT ?hook1 ?hook2 WHERE {
    ?hook1 hook:effect hook:EffectAdmit ; hook:outputState ?state1 .
    ?hook2 hook:effect hook:EffectProject ; hook:inputState ?state1 .
}
```

---

### **Part 4: Hook Composition** (Lines 1056–1238)
Typestate lifecycle and hook dependency graphs.

**Typestate Lifecycle (Lines 1061–1106):**
```
Raw → Parsed → Admitted → { Projected, Exportable, Receipted }
                  ↓
               Refused (terminal)
```

**State Definitions:**
- `Raw` — untrusted external evidence
- `Parsed` — structure validated
- `Admitted` — witness assigned, boundary laws checked
- `Projected` — transformed via named projection
- `Exportable` — ready for format crossing
- `Receipted` — proof chain sealed, object locked
- `Refused` — terminal, named law violation

**State Transitions (Lines 1109–1139):**
Each transition is one-way:
- `Raw → Parsed` via `parse()`
- `Parsed → Admitted` via `Admit::admit()` (only authorized path)
- `Admitted → Projected` via `Project` (requires `LossPolicy`)
- `Projected → Exportable` via `into_exportable()`
- `Exportable → Receipted` via `emit_receipt()` (proof gate)
- `Admitted → Receipted` via direct receipt (no projection)

**Hook Dependency Graph (Lines 1143–1186):**
- `AdmissionHook` blocks `ProjectionHook` (requires Admitted input)
- `ProjectionHook` requires `LossPolicyHook` (blocks without policy)
- `ReceiptHook` requires `ProjectionHook` + `ConformanceCheckHook` + `TokenReplayHook`
- `RefusedHook` blocks all forward motion (terminal)

**Composition Patterns (Lines 1189–1238):**
- **OR:** `Admit | Refuse` (Result pattern)
- **OR with side-effect:** `Project + LossReport` (lossy transform must report)
- **Conditional:** `Graduate | Stay` (opt-in wasm4pm)

---

### **Part 5: Hook Authority Model** (Lines 1239–1355)
Governance hierarchy and permission enforcement.

**Authority Hierarchy (Lines 1244–1313):**

| Level | Authority | Can Define | Can Modify | Examples |
|-------|-----------|-----------|-----------|----------|
| 0 | RustCompiler | TypeLawHook, AdmissionHook | — | nightly features |
| 1 | StandardsBody | AdmissionHook, FormatCrossingHook | FormatCrossingHook | OCEL, XES, BPMN |
| 2 | PaperAuthority | ConformanceHook, SoundnessHook | ConformanceHook | van der Aalst, Kourani |
| 3 | Engine | GraduationHook, TokenReplayHook, DiscoveryHook | TokenReplayHook | wasm4pm |
| 4 | SessionUser | SessionHook, ToolHook | SessionHook | settings.json hooks |

**Authority Lattice (Lines 1310–1313):**
```
RustCompiler > StandardsBody > PaperAuthority > Engine > SessionUser
```
Higher level cannot be overridden by lower level.

**Witness Authority Mapping (Lines 1317–1355):**
- `Ocel20` → `StandardsBody` (2023)
- `Xes1849` → `StandardsBody` (2016)
- `WfNetSoundnessWitness` → `PaperAuthority` (van der Aalst 1998)
- `PowlPaperWitness` → `PaperAuthority` (Kourani 2023)
- `RustLawWitness` → `RustCompiler` (nightly)
- `EngineWitness` → `Engine` (wasm4pm)
- `UserWitness` → `SessionUser` (settings.json)

**Permission Check via SPARQL (Lines 1358–1382):**
Verify caller can invoke hook:
```sparql
SELECT ?hook ?allowed WHERE {
    ?hook hook:witness ?w ; hook:authority ?auth .
    ?witness rdfs:subClassOf ?w .
    ?callerAuth auth:level ?callerLevel .
    ?auth auth:level ?requiredLevel .
    BIND (IF (?callerLevel >= ?requiredLevel, "yes", "no") AS ?allowed) .
}
```

---

### **Part 6: Integration Example — Ralph Loop + RDF Stop Hook** (Lines 1356–1465)
Real-world hook composition with blocking semantics.

**Hook Chain: Session Stop Event (Lines 1361–1392):**
1. User requests exit
2. Global RDF Stop Hook fires (`rdf-stop-hook.sh`)
   - Read `~/.claude/rdf-loop/state.ttl` (RDF Turtle)
   - Query: `loop:iterationNumber < loop:maxIterations?`
   - If false → approve exit
   - If true → extract last output, update state, check completion pattern
   - If pattern matched → approve exit
   - If not → block exit, return status message
3. Plugin Ralph Loop Stop Hook fires
4. Plugin Hookify Stop Hook fires
5. Merge decisions: any block → exit blocked

**RDF State File Example (Lines 1395–1425):**
```turtle
<urn:loop:current>
    a prov:Activity, schema:Action ;
    dcterms:identifier "session-uuid-12345" ;
    loop:iterationNumber 3 ;
    loop:maxIterations 5 ;
    loop:completionPattern "ALIVE_001_sealed" ;
    prov:value "last_assistant_output_text" ;
    prov:startedAtTime "2026-06-01T14:23:45Z" .
```

**Hook Preconditions in SPARQL (Lines 1428–1465):**
- Query 1: Is iteration count reached? (blocks until max)
- Query 2: Is completion pattern matched? (blocks until found)

**Hook Handler (Bash Script) (Lines 1468–1510):**
Real implementation of `~/.claude/rdf-loop/rdf-stop-hook.sh`:
1. Check iteration count via rdfquery
2. Extract last output from transcript JSONL
3. Update state.ttl with new iteration + output
4. Check completion pattern
5. Return `{"decision": "approve" | "block"}`

**Hook Composition (Lines 1513–1547):**
Multiple hooks on same event:
```turtle
hook:StopEventHooks
    hook:hooks [ rdf:first hook:GlobalRdfStopHook ; rdf:rest ( hook:PluginRalphLoopStopHook hook:PluginHookifyStopHook ) ] ;
    hook:mergeStrategy "any_block_blocks_all" .
```

---

### **Part 7: Complete Knowledge Base Summary** (Lines 1548–1672)
Comprehensive registries and quick-reference tables.

**7.1 All Hook Classes (Lines 1551–1590):**
Complete tree from `AbstractHook` to 30+ specializations:
```
AbstractHook
├── StructuralHook (TypeLawHook, AdmissionHook, WitnessMatchHook)
├── SemanticHook (ObjectLifecycleHook, TimestampMonotonicityHook, ConformanceCheckHook, TokenReplayHook)
├── ExecutableHook (LossHook→ProjectionHook, ReceiptHook→*Receipt, GraduationHook, BlockingHook→*, ToolHook→*)
├── AuthorityHook (StandardWitness, PaperWitness, RustLawWitness, EngineWitness, UserWitness)
├── CompositionHook (ChainHook, OrHook, ConditionalHook, ParallelHook)
└── RefusalHook (8 refusal reason variants)
```

**7.2 All Hook Properties (Lines 1593–1642):**
Complete reference of 40+ properties organized by category:
- Identity & Metadata (4 props)
- Activation (4 props)
- Effect (3 props)
- Authority & Governance (3 props)
- Loss Accounting (4 props)
- Receipt (7 props)
- Composition (5 props)
- Execution (6 props)

**7.3 All Witness Markers (Lines 1645–1672):**
Registry of 20+ witness types:

| Family | Witness | Year | Authority |
|--------|---------|------|-----------|
| **Standard** | `Ocel20`, `Xes1849`, `Bpmn2`, `Pnml` | 2023, 2016, 2011, 2004 | StandardsBody |
| **Paper** | `WfNetSoundnessPaper`, `PowlPaper`, `ObjectCentricPetriNetPaper`, `OcpqPaper`, `DeclareFamily`, `InductiveMinerPaper` | 1998–2024 | PaperAuthority |
| **API Grammar** | `Pm4pyApiGrammar`, `PmaxConsumerGrammar` | — | StandardsBody |
| **Rust Law** | `RustLawWitness`, `ForbidUnsafeCodeLaw`, `TypestateAdmissionLaw` | 2015 | RustCompiler |
| **Engine** | `EngineWitness`, `TokenReplayWitness`, `DiscoveryWitness` | — | Engine |

**7.4 All Refusal Reasons (Lines 1675–1715):**
Registry of 8 named refusal types:
- `MissingObjectRelation` (OCEL law)
- `FlatteningLoss` (format covenant)
- `DeadTransition` (Petri net)
- `UnsoundWfNet` (WF-net soundness)
- `InvalidPowlProjection` (POWL law)
- `MissingWitness` (governance)
- `UnreplayableClaim` (receipt)
- `TemporalConformanceViolation` (ordering)

---

## Quick Lookup Tables

### Hook Effect → Type Mapping

| Effect | Input State | Output State | Example Class |
|--------|-------------|--------------|---------------|
| `EffectAdmit` | Raw | Admitted | AdmissionHook |
| `EffectRefuse` | Raw/Any | Refused | RefusedHook |
| `EffectProject` | Admitted | Projected | ProjectionHook |
| `EffectReceipt` | Projected/Admitted | Receipted | ReceiptHook |
| `EffectBlock` | Any | Any | SessionStopHook |
| `EffectAdvance` | State X | State X+1 | TypestateAdvanceHook |
| `EffectReport` | Any | Any | DiagnosticHook |

### Witness Family → Authority Mapping

| Family | Authority | Examples | Mutable? |
|--------|-----------|----------|----------|
| Standard | StandardsBody | OCEL 2.0, XES 1849 | No (by standards process) |
| Paper | PaperAuthority | WF-net soundness, POWL | No (papers are eternal) |
| ApiGrammar | StandardsBody | pm4py, pmax | No (API is stable) |
| RustLaw | RustCompiler | generic_const_exprs, forbid(unsafe_code) | No (Rust decides) |
| InternalBridge | Engine | wasm4pm graduation | Yes (engine evolves) |

### SPARQL Pattern → Hook Effect

| Pattern Type | Trigger | Precondition | Effect |
|--------------|---------|--------------|--------|
| Typestate | State change | `?ev evidence:state state:Admitted` | `EffectAdvance` |
| OCEL | Event emitted | `?obj ocel:state ocel:Final` | `EffectReceipt` |
| Format | Projection requested | `?proj formats:isLossy true . FILTER NOT EXISTS { ?proj loss:policy . }` | `EffectRefuse` |
| Session | Stop event | `loop:iterationNumber < loop:maxIterations` | `EffectBlock` |
| Temporal | Timestamp order | `?ts1 > ?ts2` (consecutive events) | `EffectRefuse` |
| Authority | Witness match | `?hook hook:witness ?w . ?caller auth:level >= auth:level(?w)` | Execute |

### Refusal Reason → Law Mapping

| Reason | Domain | Standard/Paper | Law |
|--------|--------|-----------------|-----|
| MissingObjectRelation | OCEL | Ghahfarokhi 2021 | E2O/O2O mandatory |
| FlatteningLoss | Format | Xes1849 | Lossy needs policy + report |
| DeadTransition | Petri | Murata 1989 | Bipartite reachability |
| UnsoundWfNet | Petri | van der Aalst 1998 | Source/sink connectivity |
| InvalidPowlProjection | POWL | Kourani 2023 | Depth/arity bounds |
| MissingWitness | Governance | Rust law | Evidence must carry W |
| UnreplayableClaim | Receipt | — | Replay path in log |
| TemporalConformanceViolation | Events | — | Timestamp monotonicity |

---

## File Organization

```
unified-rdf-hook-ontology.md (1525 lines)
├── Part 1: Hook Class Hierarchy (354 lines)
│   ├── 1.1 Core taxonomy (354 lines of Turtle + definitions)
│   └── 1.2 Specializations (221 lines)
├── Part 2: Core Hook Properties (463 lines)
│   ├── 2.1 Property schema (463 lines of Turtle + definitions)
│   └── Properties: 40+ across 8 categories
├── Part 3: Hook Activation Grammar (198 lines)
│   ├── 3.1 SPARQL patterns (198 lines)
│   └── 6 concrete query examples + real state file
├── Part 4: Hook Composition (183 lines)
│   ├── 4.1 Typestate lifecycle
│   ├── 4.2 Dependency graph
│   └── 4.3 Composition patterns (OR, conditional)
├── Part 5: Hook Authority Model (117 lines)
│   ├── 5.1 Hierarchy (5 levels: Rust > Standards > Papers > Engine > User)
│   ├── 5.2 Witness authority mapping
│   └── 5.3 Permission check via SPARQL
├── Part 6: Integration Example — Ralph Loop (110 lines)
│   ├── 6.1 Hook chain (Stop event)
│   ├── 6.2 RDF state file (real Turtle)
│   ├── 6.3 SPARQL preconditions
│   ├── 6.4 Bash hook handler
│   └── 6.5 Hook composition (multiple hooks)
└── Part 7: Knowledge Base Summary (125 lines)
    ├── 7.1 All hook classes (40+)
    ├── 7.2 All properties (40+)
    ├── 7.3 All witnesses (20+)
    └── 7.4 All refusal reasons (8+)
```

---

## Authority & Grounding

**Synthesized from:**
1. `/Users/sac/wasm4pm-compat/CLAUDE.md` — nightly-first type-law crate
2. `/Users/sac/wasm4pm-compat/settings-hooks-exhaustive.md` — complete hook registry
3. `/Users/sac/wasm4pm-compat/WASM4PM-COMPAT-PRD-ARD.md` — architecture & authority
4. `/Users/sac/.claude/rules/process-mining-chicago-tdd.md` — van der Aalst constitution
5. `/Users/sac/.claude/rules/manufacturing-terminology.md` — CodeManufactory doctrine
6. `/Users/sac/wasm4pm-compat/src/witness.rs` — witness marker implementation
7. `/Users/sac/wasm4pm-compat/src/admission.rs` — admission/refusal semantics

**Doctrine:**
> The product is CodeManufactory; RevOps is merely proof that CodeManufactory works.

A hook is not code. A hook is a **named structural transformation** that carries a witness, fires when preconditions are met, performs an effect, may emit loss reports and receipts, and respects authority hierarchy. Rust nightly is the court.

---

**Generated:** 2026-06-01  
**Format:** Markdown (quick reference) + Turtle RDF (formal ontology)  
**Page Count:** 2 pages (this index) + 55 KB (main document)
