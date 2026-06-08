# Tsify Capability Map for wasm4pm-compat

**Date:** 2026-06-01  
**Purpose:** Detailed flow of tsify + wasm-bindgen + wasm-pack for automatic TypeScript generation  
**Scope:** How .d.ts files are generated, constraints, and per-module capability inventory

---

## Executive Summary

Tsify is a derive macro that automatically generates TypeScript interfaces from Rust structs and enums. When used with wasm-bindgen and wasm-pack, it:

1. **Derives type metadata** at compile time from Rust type definitions
2. **Generates .d.ts files** that define TypeScript interfaces matching your Rust types
3. **Eliminates manual type maintenance** — a single `#[derive(Tsify)]` produces both Rust serialization and TypeScript definitions
4. **Integrates with serde** — uses Serialize/Deserialize traits to understand type structure
5. **Outputs via wasm-pack** — automatically included in the npm package under `pkg/` directory

---

## 1. The Tsify Workflow: From Rust Source to TypeScript

```
Rust Source Code
    ↓
#[derive(Serialize, Deserialize, Tsify)]
struct MyType { ... }
    ↓
Tsify Macro Expansion
    ├─ Extracts type structure
    ├─ Reads serde attributes
    └─ Generates serialization glue
    ↓
wasm-pack build
    ├─ Calls rustc with wasm32 target
    ├─ Tsify emits type metadata
    ├─ wasm-bindgen processes #[wasm_bindgen] items
    └─ Generates pkg/index.d.ts
    ↓
pkg/ Directory
├─ index.d.ts           ← TypeScript interfaces (generated)
├─ index.js             ← JavaScript glue code
├─ wasm4pm_compat.wasm  ← Binary WebAssembly module
└─ package.json         ← NPM metadata
```

---

## 2. Tsify Attributes & Syntax

### Basic Derive

```rust
// Minimal tsify derive — generates TypeScript interface
#[derive(Serialize, Deserialize, Tsify)]
pub struct Event {
    pub id: String,
    pub timestamp: u64,
}

// Generated TypeScript:
// export interface Event {
//     id: string;
//     timestamp: number;
// }
```

### ABI Direction Hints (from_wasm_abi, into_wasm_abi)

```rust
// Data flows FROM JavaScript TO Rust (function parameters)
#[derive(Serialize, Deserialize, Tsify)]
#[tsify(from_wasm_abi)]
pub struct RawInput {
    pub payload: Vec<u8>,
}

// Data flows FROM Rust TO JavaScript (return values)
#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct AdmittedOutput {
    pub log: EventLog,
}

// Both directions (structs used as params and returns)
#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct EventLog {
    pub events: Vec<Event>,
}
```

### Namespace Attribute (for Enum Organization)

```rust
// Generate a TypeScript namespace for enum variants
#[derive(Serialize, Deserialize, Tsify)]
#[tsify(namespace)]
pub enum OutcomeType {
    Admitted(EventLog),
    Refused(String),
}

// Generated TypeScript (tagged union in namespace):
// export type OutcomeType =
//   | { Admitted: EventLog }
//   | { Refused: string };
```

### Rename & Skip Attributes

```rust
// Rename fields in TypeScript output
#[derive(Serialize, Deserialize, Tsify)]
pub struct EventData {
    #[serde(rename = "eventId")]
    pub id: String,
    
    #[serde(skip)]
    internal_metadata: HashMap<String, String>,
}

// Generated TypeScript:
// export interface EventData {
//     eventId: string;
//     // internal_metadata is omitted (skip)
// }
```

### Flatten (for Struct Composition)

```rust
#[derive(Serialize, Deserialize, Tsify)]
pub struct TimeRange {
    pub start: u64,
    pub end: u64,
}

#[derive(Serialize, Deserialize, Tsify)]
pub struct LogFilter {
    #[serde(flatten)]
    pub time_range: TimeRange,
    pub activity: String,
}

// Generated TypeScript:
// export interface LogFilter {
//     start: number;
//     end: number;
//     activity: string;
// }
```

---

## 3. Type Mapping: Rust → TypeScript

Tsify automatically converts Rust types to their TypeScript equivalents based on serde's understanding of the type:

| Rust Type | TypeScript Type | Notes |
|-----------|-----------------|-------|
| `String` | `string` | Direct mapping |
| `&str` | `string` | Borrowed string; same TS type |
| `bool` | `boolean` | Direct mapping |
| `u32, u16, u8` | `number` | All map to JS number |
| `u64, i64, usize` | `number \| bigint` | May lose precision as number; serde-wasm-bindgen prefers bigint |
| `f32, f64` | `number` | Direct mapping |
| `Vec<T>` | `T[]` | Generic array |
| `Option<T>` | `T \| undefined` | or `T \| null` (configurable) |
| `Result<T, E>` | `{ ok: T } \| { err: E }` | Tagged union (serde default) |
| `HashMap<K, V>` | `Record<K, V>` | Object as key-value store |
| `[T; N]` | `[T, T, ..., T]` | Fixed-length tuple (const generics) |
| `struct Foo { ... }` | `interface Foo { ... }` | Recursively applies mapping |
| `enum Variant(T)` | Tagged union variant | E.g., `{ Variant: T }` |

---

## 4. Per-Module Capability Analysis for wasm4pm-compat

### 4.1 `eventlog` Module

**Current State:**
- `Event`, `Trace`, `EventLog` are concrete, serializable types
- Already supports `Serialize`/`Deserialize` via serde
- Includes builder methods (`EventBuilder`, fluent APIs)

**Tsify Capability:**
```rust
#[derive(Serialize, Deserialize, Tsify)]
pub struct Event {
    pub id: String,
    pub timestamp: u64,  // NS precision
    pub activity: String,
    pub resource: Option<String>,
}

#[derive(Serialize, Deserialize, Tsify)]
pub struct Trace {
    pub id: String,
    pub events: Vec<Event>,
}

#[derive(Serialize, Deserialize, Tsify)]
pub struct EventLog {
    pub traces: Vec<Trace>,
    pub attributes: HashMap<String, String>,
}
```

**Generated TypeScript:**
```typescript
export interface Event {
    id: string;
    timestamp: number;
    activity: string;
    resource?: string;
}

export interface Trace {
    id: string;
    events: Event[];
}

export interface EventLog {
    traces: Trace[];
    attributes: Record<string, string>;
}
```

**wasm-bindgen Export Pattern:**
```rust
#[wasm_bindgen]
pub fn new_event_log() -> EventLog {
    EventLog { traces: vec![], attributes: HashMap::new() }
}

#[wasm_bindgen]
pub fn validate_event_log(log: &EventLog) -> bool {
    // Structure-only validation
    !log.traces.is_empty()
}
```

**Constraint:** Builder methods (fluent API) don't automatically cross the boundary. Either expose the struct constructors directly or create separate builder functions in `#[wasm_bindgen]`.

---

### 4.2 `ocel` Module

**Current State:**
- `OcelLog`, `OcelEvent`, `ObjectObjectLink`, `EventObjectLink`, `ObjectChange` are serializable
- Rich structure with object-centric properties
- Validates structural invariants (no dangling links)

**Tsify Capability:**
```rust
#[derive(Serialize, Deserialize, Tsify)]
pub struct OcelEvent {
    pub id: String,
    pub activity: String,
    pub timestamp: u64,
    pub omap: Vec<String>,  // object IDs
}

#[derive(Serialize, Deserialize, Tsify)]
pub struct EventObjectLink {
    pub event_id: String,
    pub object_id: String,
    pub qualifier: Option<String>,
}

#[derive(Serialize, Deserialize, Tsify)]
pub struct OcelLog {
    pub events: Vec<OcelEvent>,
    pub event_object_links: Vec<EventObjectLink>,
    pub object_types: HashMap<String, String>,
}
```

**Generated TypeScript:**
```typescript
export interface OcelEvent {
    id: string;
    activity: string;
    timestamp: number;
    omap: string[];
}

export interface EventObjectLink {
    event_id: string;
    object_id: string;
    qualifier?: string;
}

export interface OcelLog {
    events: OcelEvent[];
    event_object_links: EventObjectLink[];
    object_types: Record<string, string>;
}
```

**wasm-bindgen Export Pattern:**
```rust
#[wasm_bindgen]
pub fn load_ocel_from_json(json_str: &str) -> Result<OcelLog, String> {
    serde_json::from_str(json_str).map_err(|e| e.to_string())
}

#[wasm_bindgen]
pub fn ocel_log_to_json(log: &OcelLog) -> Result<String, String> {
    serde_json::to_string(log).map_err(|e| e.to_string())
}
```

**Constraint:** Admission logic (the `Admit` trait) stays Rust-side. Only admit results (Admitted or Refused) cross to JavaScript.

---

### 4.3 `admission` Module

**Current State:**
- `Admission<T, W>` and `Refusal<R, W>` are generic over witness `W`
- Cannot expose directly as `#[wasm_bindgen]` (generic types forbidden)
- Refusal reasons must be named enums, never bare strings

**Tsify Capability:**
```rust
// RUST-ONLY: Do not expose Evidence, Admission, Refusal directly

// Instead: Create concrete wrappers
#[derive(Serialize, Deserialize, Tsify)]
pub struct AdmittedOcelLog {
    pub log: OcelLog,
    pub witness_key: String,  // e.g., "ocel-2.0"
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(namespace)]
pub enum OcelAdmissionOutcome {
    Admitted(AdmittedOcelLog),
    Refused(OcelAdmissionRefusal),
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(namespace)]
pub enum OcelAdmissionRefusal {
    DanglingEventObjectLink { event_id: String, object_id: String },
    MissingFinalMarking { net_id: String },
    InvalidEventIdReference { reference: String },
}
```

**Generated TypeScript:**
```typescript
export interface AdmittedOcelLog {
    log: OcelLog;
    witness_key: string;
}

export type OcelAdmissionOutcome =
    | { Admitted: AdmittedOcelLog }
    | { Refused: OcelAdmissionRefusal };

export type OcelAdmissionRefusal =
    | { DanglingEventObjectLink: { event_id: string; object_id: string } }
    | { MissingFinalMarking: { net_id: string } }
    | { InvalidEventIdReference: { reference: string } };
```

**wasm-bindgen Export Pattern:**
```rust
#[wasm_bindgen]
pub fn admit_ocel_log_against_2_0(json: &str)
    -> Result<OcelAdmissionOutcome, String>
{
    let log = serde_json::from_str::<OcelLog>(json)?;
    match OcelAdmit::admit(&log) {
        Ok(admission) => Ok(OcelAdmissionOutcome::Admitted(AdmittedOcelLog {
            log: admission.value,
            witness_key: "ocel-2.0".to_string(),
        })),
        Err(refusal) => Ok(OcelAdmissionOutcome::Refused(refusal.reason)),
    }
}
```

**Constraint:** Do NOT expose `Admission<T, W>` or `Refusal<R, W>` directly. Create concrete outcome enums instead.

---

### 4.4 `loss` Module

**Current State:**
- `LossReport<From, To, Items>` is generic over shape types
- `ProjectionName` is a newtype wrapper around `&'static str`
- `LossPolicy` enum controls loss handling

**Tsify Capability:**
```rust
// Generic LossReport cannot cross boundary directly
// Create concrete serializable struct instead

#[derive(Serialize, Deserialize, Tsify)]
pub struct LossReportData {
    pub from_shape: String,       // e.g., "OcelLog"
    pub to_shape: String,         // e.g., "XesLog"
    pub items_lost: Vec<LostItem>,
    pub total_items: u32,
    pub items_retained: u32,
    pub loss_percentage: f32,     // 0.0 to 1.0
}

#[derive(Serialize, Deserialize, Tsify)]
pub struct LostItem {
    pub item_type: String,  // e.g., "EventObjectLink"
    pub count: u32,
    pub reason: String,     // e.g., "XES has no object-centric model"
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(namespace)]
pub enum LossPolicyType {
    RefuseLoss,
    AllowNamedProjection(String),
    AllowLossWithReport,
}
```

**Generated TypeScript:**
```typescript
export interface LossReportData {
    from_shape: string;
    to_shape: string;
    items_lost: LostItem[];
    total_items: number;
    items_retained: number;
    loss_percentage: number;
}

export interface LostItem {
    item_type: string;
    count: number;
    reason: string;
}

export type LossPolicyType =
    | "RefuseLoss"
    | { AllowNamedProjection: string }
    | "AllowLossWithReport";
```

**wasm-bindgen Export Pattern:**
```rust
#[wasm_bindgen]
pub fn project_ocel_to_xes(
    log: &OcelLog,
    policy: &str,  // "refuse_loss", "allow_named", "allow_with_report"
) -> Result<ProjectionResult, String>
{
    let loss_policy = match policy {
        "refuse_loss" => LossPolicy::RefuseLoss,
        "allow_with_report" => LossPolicy::AllowLossWithReport,
        _ => return Err("Unknown loss policy".to_string()),
    };

    // Projection logic stays Rust-side (structure-only)
    // Return serialized result
    Ok(ProjectionResult {
        projected_log: /* ... */,
        loss_report: /* ... */,
    })
}

#[derive(Serialize, Deserialize, Tsify)]
pub struct ProjectionResult {
    pub projected_log: String,  // Serialized JSON
    pub loss_report: LossReportData,
}
```

**Constraint:** Generic `Project` trait stays Rust-side. Only serialized projection results cross the boundary.

---

### 4.5 `conformance` Module

**Current State:**
- `Metric<KIND, NUM, DEN>` is const-generic for bounded type-level metrics
- `ConformanceVerdict` is a struct that CARRIES (not computes) metrics
- Structure-only; no actual token replay or alignment logic here

**Tsify Capability:**
```rust
// Const generics don't serialize; create concrete verdict struct instead

#[derive(Serialize, Deserialize, Tsify)]
pub struct ConformanceVerdictData {
    pub fitness: f32,           // 0.0 to 1.0
    pub precision: f32,         // 0.0 to 1.0
    pub f1: f32,                // 0.0 to 1.0
    pub generalization: Option<f32>,
    pub simplicity: Option<f32>,
    pub model_id: String,
    pub log_id: String,
}

#[derive(Serialize, Deserialize, Tsify)]
pub struct DeviationData {
    pub step_index: u32,
    pub sync_move_type: String,  // "sync", "log_only", "model_only"
    pub activity: String,
}
```

**Generated TypeScript:**
```typescript
export interface ConformanceVerdictData {
    fitness: number;
    precision: number;
    f1: number;
    generalization?: number;
    simplicity?: number;
    model_id: string;
    log_id: string;
}

export interface DeviationData {
    step_index: number;
    sync_move_type: string;
    activity: string;
}
```

**wasm-bindgen Export Pattern:**
```rust
#[wasm_bindgen]
pub fn load_conformance_verdict(json: &str)
    -> Result<ConformanceVerdictData, String>
{
    serde_json::from_str(json).map_err(|e| e.to_string())
}

#[wasm_bindgen]
pub fn verdict_summary(verdict: &ConformanceVerdictData) -> String {
    format!(
        "Fitness: {:.2}, Precision: {:.2}, F1: {:.2}",
        verdict.fitness, verdict.precision, verdict.f1
    )
}
```

**Constraint:** Verdicts are structure-only. No token replay, alignment computation, or actual conformance checking happens here. Graduate those to wasm4pm.

---

### 4.6 `petri` Module

**Current State:**
- `PetriNet`, `Place`, `Transition`, `Arc` are serializable structures
- `WfNetConst<SOUNDNESS>` encodes soundness as a const-generic witness
- Complex type-level invariants (arc cardinality, marking soundness, etc.)

**Tsify Capability:**
```rust
// Regular types are serializable; const generics are not

#[derive(Serialize, Deserialize, Tsify)]
pub struct Place {
    pub id: String,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Tsify)]
pub struct Transition {
    pub id: String,
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize, Tsify)]
pub struct Arc {
    pub source: String,
    pub target: String,
    pub weight: u32,
}

#[derive(Serialize, Deserialize, Tsify)]
pub struct PetriNet {
    pub id: String,
    pub places: Vec<Place>,
    pub transitions: Vec<Transition>,
    pub arcs: Vec<Arc>,
    pub initial_marking: HashMap<String, u32>,
    pub final_markings: Vec<HashMap<String, u32>>,
}

// Soundness claim is metadata, not a generic type parameter
#[derive(Serialize, Deserialize, Tsify)]
pub struct SoundnessClaim {
    pub petri_net_id: String,
    pub is_sound: bool,  // Boolean claim; proof stays Rust-side
    pub reason: Option<String>,  // e.g., "deadlock at place P1"
}
```

**Generated TypeScript:**
```typescript
export interface Place {
    id: string;
    name?: string;
}

export interface Transition {
    id: string;
    label?: string;
}

export interface Arc {
    source: string;
    target: string;
    weight: number;
}

export interface PetriNet {
    id: string;
    places: Place[];
    transitions: Transition[];
    arcs: Arc[];
    initial_marking: Record<string, number>;
    final_markings: Record<string, number>[];
}

export interface SoundnessClaim {
    petri_net_id: string;
    is_sound: boolean;
    reason?: string;
}
```

**wasm-bindgen Export Pattern:**
```rust
#[wasm_bindgen]
pub fn load_petri_net(json: &str) -> Result<PetriNet, String> {
    serde_json::from_str(json).map_err(|e| e.to_string())
}

#[wasm_bindgen]
pub fn petri_net_structure_valid(net: &PetriNet) -> bool {
    // Structure-only validation: no duplicate IDs, arcs point to real places/transitions
    net.places.iter().map(|p| &p.id).collect::<std::collections::HashSet<_>>().len()
        == net.places.len()
}
```

**Constraint:** Const-generic soundness witnesses don't cross. Encode soundness as a concrete boolean or enum in the claim structure instead.

---

### 4.7 `witness` Module

**Current State:**
- Witness markers are empty enums (Ocel20, Xes1849, etc.)
- Zero-sized at runtime; compile-time only
- Carry metadata constants (KEY, TITLE, YEAR, FAMILY)

**Tsify Capability:**
```rust
// Witness markers CANNOT be serialized (zero-sized)
// Instead: Expose their metadata as a concrete struct

#[derive(Serialize, Deserialize, Tsify)]
pub struct WitnessMetadata {
    pub key: String,              // e.g., "ocel-2.0"
    pub title: String,            // e.g., "OCEL 2.0"
    pub year: Option<u16>,        // e.g., Some(2023)
    pub family: String,           // e.g., "Standard"
}

#[derive(Serialize, Deserialize, Tsify)]
pub struct WitnessRegistry {
    pub witnesses: Vec<WitnessMetadata>,
}
```

**Generated TypeScript:**
```typescript
export interface WitnessMetadata {
    key: string;
    title: string;
    year?: number;
    family: string;
}

export interface WitnessRegistry {
    witnesses: WitnessMetadata[];
}
```

**wasm-bindgen Export Pattern:**
```rust
#[wasm_bindgen]
pub fn get_all_witnesses() -> WitnessRegistry {
    WitnessRegistry {
        witnesses: vec![
            WitnessMetadata {
                key: "ocel-2.0".into(),
                title: "OCEL 2.0".into(),
                year: Some(2023),
                family: "Standard".into(),
            },
            // ... others
        ],
    }
}

#[wasm_bindgen]
pub fn get_witness_by_key(key: &str) -> Option<WitnessMetadata> {
    match key {
        "ocel-2.0" => Some(WitnessMetadata { /* ... */ }),
        // ... others
        _ => None,
    }
}
```

**Constraint:** Witness marker enums themselves never cross. Extract and expose their metadata only.

---

### 4.8 `state` Module

**Current State:**
- State tokens (Raw, Parsed, Admitted, Refused, etc.) are empty enums
- Pure typestate markers; zero-sized
- Used only in `PhantomData<State>` within `Evidence<T, State, W>`

**Tsify Capability:**
```rust
// State tokens CANNOT be serialized (zero-sized, phantom)
// Instead: Encode state information in response/outcome enums

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(namespace)]
pub enum EvidenceOutcome {
    Raw(RawPayload),
    Parsed(ParsedPayload),
    Admitted(AdmittedPayload),
    Refused(RefusedPayload),
    Exported(ExportedPayload),
}

#[derive(Serialize, Deserialize, Tsify)]
pub struct RawPayload {
    pub data: Vec<u8>,
    pub source_format: String,
}

#[derive(Serialize, Deserialize, Tsify)]
pub struct ParsedPayload {
    pub data: String,  // Parsed, serializable form
    pub format: String,
}

#[derive(Serialize, Deserialize, Tsify)]
pub struct AdmittedPayload {
    pub data: String,
    pub witness_key: String,
}

#[derive(Serialize, Deserialize, Tsify)]
pub struct RefusedPayload {
    pub reason: String,
    pub law_violated: String,
}
```

**Generated TypeScript:**
```typescript
export type EvidenceOutcome =
    | { Raw: RawPayload }
    | { Parsed: ParsedPayload }
    | { Admitted: AdmittedPayload }
    | { Refused: RefusedPayload }
    | { Exported: ExportedPayload };

export interface RawPayload { /* ... */ }
export interface ParsedPayload { /* ... */ }
export interface AdmittedPayload { /* ... */ }
export interface RefusedPayload { /* ... */ }
```

**wasm-bindgen Export Pattern:**
```rust
#[wasm_bindgen]
pub fn parse_raw_evidence(
    raw_bytes: &[u8],
    format_hint: &str,
) -> Result<EvidenceOutcome, String>
{
    match format_hint {
        "json-ocel" => {
            match serde_json::from_slice::<OcelLog>(raw_bytes) {
                Ok(parsed) => Ok(EvidenceOutcome::Parsed(ParsedPayload {
                    data: serde_json::to_string(&parsed)?,
                    format: "ocel".into(),
                })),
                Err(e) => Ok(EvidenceOutcome::Refused(RefusedPayload {
                    reason: e.to_string(),
                    law_violated: "ParseError".into(),
                })),
            }
        }
        _ => Err("Unknown format hint".into()),
    }
}
```

**Constraint:** State tokens never cross as type parameters. Encode state in outcome enum variants instead.

---

### 4.9 `engine_bridge` Module

**Current State:**
- `GraduationCandidate` is a concrete struct with String fields
- Already serializable; no generic parameters
- Designed to be passed to the engine

**Tsify Capability:**
```rust
// Already ABI-safe; minimal changes needed

#[derive(Serialize, Deserialize, Tsify)]
pub struct GraduationCandidate {
    pub reason: String,         // e.g., "needs_discovery"
    pub subject: String,        // e.g., "p2p OCEL log"
    pub evidence_ref: String,   // e.g., "blake3:abc123"
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(namespace)]
pub enum GraduationReasonTag {
    NeedsDiscovery,
    NeedsConformanceExecution,
    NeedsReplay,
    NeedsReceipts,
    NeedsBenchmarkGate,
    NeedsObjectCentricQueryExecution,
    RebuildingProcessMiningLocally,
}
```

**Generated TypeScript:**
```typescript
export interface GraduationCandidate {
    reason: string;
    subject: string;
    evidence_ref: string;
}

export type GraduationReasonTag =
    | "NeedsDiscovery"
    | "NeedsConformanceExecution"
    | "NeedsReplay"
    // ... rest
```

**wasm-bindgen Export Pattern:**
```rust
#[wasm_bindgen]
pub fn graduation_case_for_log(log: &EventLog) -> GraduationCandidate {
    GraduationCandidate::new(
        GraduationReason::NeedsDiscovery,
        "event log requiring discovery",
        "log_hash_placeholder",
    )
}

#[wasm_bindgen]
pub fn is_hard_graduation_signal(reason: &str) -> bool {
    matches!(
        reason,
        "needs_discovery"
            | "needs_conformance_execution"
            | "needs_replay"
            | "needs_object_centric_query_execution"
            | "rebuilding_process_mining_locally"
    )
}
```

**Constraint:** None; this module is already boundary-friendly.

---

## 5. The .d.ts Generation Process

### 5.1 How wasm-pack Generates .d.ts

```bash
wasm-pack build --target web --out-dir pkg
```

**Steps:**

1. **Invoke Cargo + rustc** with `--crate-type cdylib` and `wasm32-unknown-unknown` target
2. **Tsify attribute expansion** — derive macro reads type structure and serde attributes
3. **wasm-bindgen codegen** — processes `#[wasm_bindgen]` items and generates glue
4. **Tsify output** — emits type definitions to a temporary location
5. **wasm-pack processing** — collects all type definitions and merges into `pkg/index.d.ts`
6. **Output artifacts**:
   ```
   pkg/
   ├── index.d.ts           ← All exported types (generated)
   ├── index.js             ← JavaScript glue
   ├── package.json         ← NPM metadata
   └── wasm4pm_compat.wasm  ← Binary
   ```

### 5.2 Example Generated .d.ts for wasm4pm-compat

```typescript
// Generated by wasm-pack + tsify

export interface Event {
    id: string;
    timestamp: number;
    activity: string;
    resource?: string;
}

export interface Trace {
    id: string;
    events: Event[];
}

export interface EventLog {
    traces: Trace[];
    attributes: Record<string, string>;
}

export interface OcelLog {
    events: OcelEvent[];
    event_object_links: EventObjectLink[];
    object_types: Record<string, string>;
}

export type AdmissionOutcome =
    | { Admitted: AdmittedEventLog }
    | { Refused: AdmissionRefusal };

export function new_event_log(): EventLog;
export function load_ocel_from_json(json: string): Result<OcelLog, string>;
export function admit_event_log(log: EventLog): AdmissionOutcome;

// ... rest of exports
```

### 5.3 Customizing .d.ts Output

**Tsify respects serde attributes:**

```rust
#[derive(Serialize, Deserialize, Tsify)]
pub struct MyType {
    #[serde(rename = "myFieldName")]
    pub my_field: String,

    #[serde(skip)]
    pub internal_metadata: Vec<String>,

    #[serde(flatten)]
    pub nested: NestedType,
}
```

**Results in:**
```typescript
export interface MyType {
    myFieldName: string;  // renamed
    // internal_metadata skipped
    // nested fields flattened into MyType
}
```

---

## 6. Serialization Choice: serde_json vs serde-wasm-bindgen

### 6.1 serde_json (JSON-based)

**Pros:**
- Human-readable; easy to debug
- Smaller dependency footprint
- Works everywhere (including non-WASM)
- Wide ecosystem support

**Cons:**
- Larger serialized output (~2-3x binary size vs serde-wasm-bindgen)
- Number precision loss (u64 → f64, loses precision > 2^53)
- No native Map/Set support; encodes as `{ "key": "value" }`
- Slower than binary serialization

**Use when:**
- Debugging and human inspection are priorities
- JSON APIs need to be called
- Code size is not critical
- All numbers fit safely in f64 range

**Example:**
```rust
#[wasm_bindgen]
pub fn load_log_from_json(json_str: &str) -> Result<EventLog, String> {
    serde_json::from_str(json_str).map_err(|e| e.to_string())
}
```

### 6.2 serde-wasm-bindgen (Binary)

**Pros:**
- Smaller code size (no JSON parser overhead)
- Faster serialization/deserialization
- Preserves number types (u64 → BigInt in JS)
- Supports Map, Set, array buffers natively
- Direct JsValue conversion (no parsing step)

**Cons:**
- Binary format; not human-readable
- Smaller ecosystem than serde_json
- Debugging requires binary inspection
- Not suitable for external JSON APIs

**Use when:**
- Performance and code size matter
- Large numeric IDs (u64, i64) need to be preserved
- Map/Set data structures are used
- Binary protocols are acceptable

**Example:**
```rust
#[wasm_bindgen]
pub fn load_log_from_js(value: JsValue) -> Result<EventLog, String> {
    serde_wasm_bindgen::from_value(value)
        .map_err(|e| e.to_string())
}
```

### 6.3 Recommendation for wasm4pm-compat

**Use hybrid approach:**

1. **Primary path (serde-wasm-bindgen):** Direct JsValue ↔ Rust conversion for performance
2. **Secondary path (serde_json):** JSON import/export for interop and debugging
3. **Feature gate:**
   ```rust
   #[cfg(feature = "json-support")]
   #[wasm_bindgen]
   pub fn load_ocel_from_json(json: &str) -> Result<OcelLog, String> {
       serde_json::from_str(json).map_err(|e| e.to_string())
   }

   #[cfg(all(feature = "formats", target_arch = "wasm32"))]
   #[wasm_bindgen]
   pub fn load_ocel_from_js(value: JsValue) -> Result<OcelLog, String> {
       serde_wasm_bindgen::from_value(value)
           .map_err(|e| e.to_string())
   }
   ```

---

## 7. Feature Gating for WASM Compilation

### 7.1 Conditional Dependencies in Cargo.toml

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"], optional = true }
tsify = { version = "0.4.5", features = ["js"], optional = true }
serde-wasm-bindgen = { version = "0.6", optional = true }

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"

[features]
default = []

# Feature for TypeScript/WASM support
ts = ["dep:tsify", "dep:serde", "serde/derive"]
wasm = ["dep:wasm-bindgen", "dep:serde-wasm-bindgen", "ts"]

# JSON support (optional)
json = ["serde_json"]
```

### 7.2 Using #[cfg] in Source Code

```rust
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
use wasm_bindgen::prelude::*;

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
#[wasm_bindgen]
pub fn admit_event_log(json: &str) -> Result<EventLog, String> {
    // Implementation
}

// Rust-only (no WASM export)
#[cfg(not(target_arch = "wasm32"))]
impl EventLog {
    pub fn from_file(path: &str) -> std::io::Result<Self> {
        // File I/O (not available in WASM)
    }
}
```

### 7.3 Building for Different Targets

```bash
# Build native Rust library
cargo build

# Build WASM with TypeScript support
cargo build --target wasm32-unknown-unknown --features wasm
wasm-pack build --target web --features wasm --out-dir pkg

# Build WASM for Node.js
wasm-pack build --target nodejs --features wasm --out-dir pkg-node

# Minimal WASM (no serde, no tsify)
wasm-pack build --target web --no-default-features
```

---

## 8. Testing the TypeScript Bindings

### 8.1 Verification Script

```bash
# After wasm-pack build
npm install
# Check that pkg/index.d.ts exists and is valid
npx tsc --noEmit  # Typecheck against generated .d.ts
```

### 8.2 Example TypeScript Consumer

```typescript
import init, { 
    new_event_log, 
    admit_event_log,
    EventLog,
    AdmissionOutcome 
} from './pkg/wasm4pm_compat.js';

async function main() {
    await init();

    // Create and manipulate
    const log: EventLog = new_event_log();

    // TypeScript knows the shape; IDE autocomplete works
    const outcome: AdmissionOutcome = admit_event_log(log);

    if ('Admitted' in outcome) {
        console.log('Admitted:', outcome.Admitted);
    } else if ('Refused' in outcome) {
        console.log('Refused:', outcome.Refused);
    }
}

main().catch(console.error);
```

---

## 9. Known Limitations & Workarounds

### 9.1 Generic Type Parameters on #[wasm_bindgen]

**Problem:**
```rust
#[wasm_bindgen]
pub struct Evidence<T, State, W> {  // ERROR: generics not allowed
    pub value: T,
}
```

**Error:**
```
error: #[wasm_bindgen] does not support generic types
```

**Workaround:**
Create concrete wrappers instead of exposing the generic type:
```rust
#[wasm_bindgen]
pub struct AdmittedEventLog {
    pub value: EventLog,
}

#[wasm_bindgen]
pub fn admit_log(raw: &EventLog) -> Result<AdmittedEventLog, String> {
    // Implementation
}
```

### 9.2 Const Generics Don't Serialize

**Problem:**
```rust
pub struct Metric<const KIND: QualityMetricKind, const NUM: u64, const DEN: u64> { }
// Const generics: no runtime representation; cannot serialize
```

**Workaround:**
Create concrete metric structs:
```rust
#[derive(Serialize, Deserialize, Tsify)]
pub struct MetricValue {
    pub kind: String,        // "Fitness", "Precision", etc.
    pub numerator: u64,
    pub denominator: u64,
    pub value: f32,          // Pre-computed for convenience
}
```

### 9.3 PhantomData Doesn't Cross the Boundary

**Problem:**
```rust
pub struct Evidence<T, State: EvidenceState, W> {
    pub state: PhantomData<State>,      // Zero-sized; can't serialize
    pub witness: PhantomData<W>,        // Zero-sized; can't serialize
}
```

**Workaround:**
Encode state and witness as concrete fields:
```rust
#[derive(Serialize, Deserialize, Tsify)]
pub struct EvidenceSnapshot {
    pub value: String,              // Serialized T
    pub state: String,              // e.g., "Admitted"
    pub witness_key: String,        // e.g., "ocel-2.0"
}
```

### 9.4 Lifetimes Are Compile-Time Only

**Problem:**
```rust
pub fn process_log<'a>(log: &'a EventLog) -> &'a str {
    // Lifetime doesn't cross ABI; JS has no concept of borrow
}
```

**Workaround:**
Return owned types:
```rust
#[wasm_bindgen]
pub fn process_log(log: &EventLog) -> String {
    // Return owned String instead of &str
    "result".into()
}
```

---

## 10. Quick Reference: Per-Module Tsify Checklist

| Module | Types Exportable | Do NOT Export | Strategy |
|--------|------------------|---------------|----------|
| `eventlog` | Event, Trace, EventLog | Builder methods (fluent API) | Derive Tsify; expose constructors as `#[wasm_bindgen]` functions |
| `ocel` | OcelLog, OcelEvent, Links | Generic link types with lifetimes | Create concrete newtype wrappers |
| `admission` | Admission outcomes (concrete) | Admission<T,W>, Refusal<R,W> generics | Create AdmittedOutcome, RefusedOutcome enums |
| `loss` | LossReport (concrete), LossPolicy | Generic LossReport<From,To,Items> | Serialize generic results to concrete structs |
| `conformance` | ConformanceVerdictData | Metric<KIND,NUM,DEN> const-generics | Create concrete verdict structs |
| `petri` | PetriNet, Place, Transition, Arc | WfNetConst<SOUNDNESS> const-generic | Encode soundness as boolean/enum field |
| `witness` | WitnessMetadata (extracted) | Witness marker enums (empty, zero-sized) | Extract metadata into concrete struct |
| `state` | EvidenceOutcome enum | State token types (Raw, Admitted, etc.) | Encode state in outcome enum variants |
| `engine_bridge` | GraduationCandidate | None | Already ABI-safe; derive Tsify directly |

---

## 11. Conclusion

Tsify makes it easy to export Rust types to TypeScript, but **wasm4pm-compat's sophisticated type-law surface (generics, const-generics, PhantomData witnesses, empty typestate tokens) cannot cross the ABI boundary directly**. 

The solution is to:

1. **Keep the type-law layer Rust-only** — Evidence<T, State, W>, Witness markers, State tokens
2. **Create concrete, serializable migrateds** — AdmittedEventLog, OcelAdmissionOutcome, WitnessMetadata
3. **Expose only boundary-safe types** — Concrete structs, string enums, simple metrics
4. **Encode state/witness as runtime values** — Not phantom types, but visible enum variants or string fields
5. **Let TypeScript reconstruct the model** — The .d.ts will show all exported types and their relationships

This preserves the elegance of wasm4pm-compat's type law while making it accessible to JavaScript consumers.

---

## Sources

- [tsify - crates.io](https://crates.io/crates/tsify)
- [Supported Types - The wasm-bindgen Guide](https://rustwasm.github.io/docs/wasm-bindgen/reference/types.html)
- [Rust Type conversions - The wasm-bindgen Guide](https://rustwasm.github.io/docs/wasm-bindgen/contributing/design/rust-type-conversions.html)
- [Arbitrary Data with Serde - The wasm-bindgen Guide](https://rustwasm.github.io/docs/wasm-bindgen/reference/arbitrary-data-with-serde.html)
- [Share Rust Types With TypeScript for WebAssembly in 30 Seconds](https://dawchihliou.github.io/articles/share-rust-types-with-typescript-for-webassembly-in-30-seconds)
- [serde-wasm-bindgen - Rust docs](https://docs.rs/serde-wasm-bindgen/latest/serde_wasm_bindgen/)
- [wasm-pack build command](https://rustwasm.github.io/docs/wasm-pack/commands/build.html)
- [How to Add WebAssembly Support to a General-Purpose Crate](https://rustwasm.github.io/book/reference/add-wasm-support-to-crate.html)
