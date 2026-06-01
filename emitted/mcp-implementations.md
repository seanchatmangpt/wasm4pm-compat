# MCP Server Implementations — wasm4pm-compat Integration Map

**Date:** 2026-06-01  
**Authority:** wasm4pm-compat CROWN_ALIVE_004 + open-ontologies Cell8 attestation  
**Status:** Active — 19+ onto_* tools discovered, 2 server implementations mapped

---

## 1. Tool Definitions (OCEL Event Type Constants)

### Primary Tool Registry Location

**File:** `/Users/sac/open-ontologies/src/server.rs` (lines 100–175+)

All 19+ MCP tool names are defined as OCEL event type constants to prevent silent event-log drift (typos cause undetectable log gaps). Each constant appears ≥ 3 call sites.

### Registered Tools

| Tool Name | Constant | OCEL Event Type | Call Sites | Purpose |
|---|---|---|---|---|
| `onto_executive_projection` | `TOOL_EXECUTIVE_PROJECTION` | `"onto_executive_projection"` | 14 | LLM-driven process projection into executive-facing models |
| `onto_translate_candidate` | `TOOL_TRANSLATE_CANDIDATE` | `"onto_translate_candidate"` | 11 | Convert unstructured voice/text to typed `CandidateCtq` |
| `onto_manufacture_solution` | `TOOL_MANUFACTURE_SOLUTION` | `"onto_manufacture_solution"` | 4 | Manufacture a complete solution from a work order |
| `onto_admit_work_order` | `TOOL_ADMIT_WORK_ORDER` | `"onto_admit_work_order"` | 4 | Admission gate: validate work order structure + law |
| `onto_shacl` | `TOOL_SHACL` | `"onto_shacl"` | 5 | SHACL shape validation against ontology graph |
| `onto_validate` | `TOOL_VALIDATE` | `"onto_validate"` | 4 | Structural validation of OCEL/XES/BPMN inputs |
| `onto_admit_ctq` | `TOOL_ADMIT_CTQ` | `"onto_admit_ctq"` | 3 | Admission: map natural-language requirement to canonical CTQ |
| `onto_propose_requirement` | `TOOL_PROPOSE_REQUIREMENT` | `"onto_propose_requirement"` | 3 | Create a requirement without immediate admission |
| `onto_propose_work_order` | `TOOL_PROPOSE_WORK_ORDER` | `"onto_propose_work_order"` | 3 | Propose work order (deferred admission) |
| `onto_query` | `TOOL_QUERY` | `"onto_query"` | 3 | Execute SPARQL query over ontology graph |
| `onto_reason` | `TOOL_REASON` | `"onto_reason"` | 3 | Inference/reasoning over graph (forward chaining) |
| `onto_save` | `TOOL_SAVE` | `"onto_save"` | 3 | Persist ontology changes to store |
| `onto_version` | `TOOL_VERSION` | `"onto_version"` | 3 | Report version/build metadata |
| `onto_retention_pause` | `TOOL_RETENTION_PAUSE` | `"onto_retention_pause"` | 3 | Pause artifact/receipt retention (admin-only) |
| `onto_bootstrap_unlock` | `TOOL_BOOTSTRAP_UNLOCK` | `"onto_bootstrap_unlock"` | 3 | Admin unlock for bootstrap mode |
| `onto_groq_status` | `TOOL_GROQ_STATUS` | `"onto_groq_status"` | 3 | Report Groq LLM service status |
| `onto_gemini_status` | `TOOL_GEMINI_STATUS` | `"onto_gemini_status"` | 3 | Report Gemini LLM service status |
| `onto_old_ai_station` | `TOOL_OLD_AI_STATION` | `"onto_old_ai_station"` | 3 | Legacy AI Station compatibility bridge (deprecated) |
| `onto_load` | `TOOL_LOAD` | `"onto_load"` | 2 | Load ontology from file/repo (not network fetch) |
| `onto_lint` | `TOOL_LINT` | `"onto_lint"` | 2 | Syntax/structure linting for ontology sources |
| `onto_rollback` | `TOOL_ROLLBACK` | `"onto_rollback"` | 2 | Rollback graph to prior checkpoint |
| `onto_ingest` | `TOOL_INGEST` | `"onto_ingest"` | 2 | Ingest external RDF/Turtle data into graph |
| `onto_extend` | `TOOL_EXTEND` | `"onto_extend"` | 2 | Add/extend ontology with new classes/properties |
| `onto_receipts_revoke_batch` | `TOOL_RECEIPTS_REVOKE_BATCH` | `"onto_receipts_revoke_batch"` | 5 | Bulk-revoke receipts by batch ID (admin-only) |
| `onto_session_revoke_by_principal` | `TOOL_SESSION_REVOKE_BY_PRINCIPAL` | `"onto_session_revoke_by_principal"` | 4 | Revoke all sessions for a principal (admin-only) |
| `onto_retention_resume` | `TOOL_RETENTION_RESUME` | `"onto_retention_resume"` | 2 | Resume paused retention (admin-only) |

**Total registered:** 19+ tools (extends to line 175+ in server.rs)

---

## 2. Server Boot Code

### Primary Server Implementation

**File:** `/Users/sac/open-ontologies/src/cmds/server.rs`

**Key Functions:**

| Function | Lines | Purpose |
|---|---|---|
| `load_cfg(config_path)` | 108–121 | Load or initialize config from `~/.open-ontologies/config.toml` |
| `build_cache_cfg(cfg, idle_ttl_secs, auto_refresh)` | 123–128 | Build cache configuration with optional overrides |
| `build_tool_filter_cfg(cfg, allow, deny)` | 130–137 | Parse tool allow/deny lists from CLI or config |
| `init_tracing_cfg(cfg)` | 139–150+ | Initialize structured logging (JSON or text format) |

### Server Init Command

**Location:** `/Users/sac/open-ontologies/src/cmds/server.rs` (lines 32–106)

Template defines default configuration structure:

```toml
[general]
data_dir = "~/.open-ontologies"
ontology_dirs = []

[http]
host = "127.0.0.1"
port = 8080
token = ""
cors_origins = []
rate_limit_rps = null

[logging]
level = "info"
file = ""

[cache]
idle_ttl_secs = 7200
auto_refresh = false

[monitor]
enabled = false
interval_secs = 30

[llm]
provider = "groq"
api_base = "https://api.groq.com/openai/v1"

[authority]
admin_principals = []
known_tenants = []

[retention]
artifacts = { days = 90 }
receipts = { days = 180 }

[verifier]
enabled = false
interval_secs = 60
```

---

## 3. Server Handler Architecture

### RMCPv2 Framework Integration

**File:** `/Users/sac/open-ontologies/src/server.rs` (lines 1–20)

```rust
use rmcp::{
    ServerHandler, RoleServer, 
    tool, tool_handler, tool_router,
    prompt, prompt_handler, prompt_router,
    handler::server::{tool::ToolRouter, router::prompt::PromptRouter, wrapper::Parameters},
    model::{
        ServerCapabilities, ServerInfo, Tool,
        PromptMessage, PromptMessageRole, GetPromptResult,
        GetPromptRequestParams, PaginatedRequestParams, ListPromptsResult,
    },
    service::RequestContext,
};
```

### Core Server Types

| Type | Module | Purpose |
|---|---|---|
| `OpenOntologiesServer` | `server.rs` | Main MCP server handler (extends `ServerHandler`) |
| `MaybeGatedServer` | `mcpp_gate.rs` | Optional MCPP (managed connection) gate wrapper |
| `ToolRouter` | `rmcp::handler::server::tool` | Routes tool invocations to handlers |
| `PromptRouter` | `rmcp::handler::server::router::prompt` | Routes prompt requests to handlers |

### Event Logging Integration

Every tool invocation emits OCEL events with constant guard keys:

| Constant | Value | Call Sites | Usage |
|---|---|---|---|
| `OCEL_EVENT_LLM_INVOKED` | `"llm_invoked"` | 8 | After every translate_candidate / manufacture_solution / executive_projection LLM call |
| `OCEL_EVENT_LLM_AUTHORITY_CLAIMED` | `"llm_authority_claimed"` | 4 | When LLM claims authority it cannot claim (forces `provisional=true`) |
| `OCEL_EVENT_LLM_CANDIDATE_TRANSLATED` | `"llm_candidate_translated"` | 2 | After LLM produces `CandidateCtq` |
| `OCEL_EVENT_ADMISSION_DENIED` | `"admission_denied"` | 2 | When admission gate rejects operation |
| `OCEL_EVENT_ADMISSION_BYPASS` | `"admission_bypass"` | 1 | When admission is bypassed by authorized operator |
| `OCEL_EVENT_REQUIREMENT_PROPOSED` | `"requirement_proposed"` | 1 | When natural-language requirement accepted |
| `OCEL_EVENT_WORK_ORDER_ADMITTED` | `"work_order_admitted"` | 1 | When work order fully validated and admitted |

---

## 4. MCP Tool Implementation Pattern

### Decorator-Based Handler Registration

**Framework:** `clap_noun_verb` (distributed slices via `linkme`)

**Location:** `/Users/sac/open-ontologies/src/cmds/server.rs` (verb registration)

Each tool is implemented as a `#[verb]` attributed function in `cmds/` submodules:

```rust
#[derive(Serialize)]
pub struct ServeOutput {
    pub status: String,
}

#[verb]
pub async fn serve(
    #[arg(short, long)] config: Option<String>,
    #[arg(long)] port: Option<u16>,
    #[arg(long)] workers: Option<usize>,
    #[arg(long, value_delimiter = ',')] tools_allow: Option<String>,
    #[arg(long, value_delimiter = ',')] tools_deny: Option<String>,
) -> NounVerbResult<ServeOutput> {
    // Server startup code
}
```

**Noun-Verb Convention:** All tools follow `onto <verb>` pattern:
- `onto serve` — start MCP server
- `onto translate_candidate` — invoke LLM translator
- `onto admit_work_order` — admission gate
- `onto query` — SPARQL query execution
- etc.

---

## 5. Tool Handler Integration Points

### Admission Gate (Type-Law Boundary)

**Gate Name:** `onto_admit_work_order` / `onto_admit_ctq`

**Wasm4pm-compat Connection:** 
- Checks that incoming work orders comply with wasm4pm-compat witness markers
- Validates evidence carriers carry correct `Evidence<T, Admitted, W>` type state
- Refusal carries named law from `Refusal<R, W>` (never generic error string)

**Evidence Flow:**
```
Raw input 
  → onto_translate_candidate (unstructured → CandidateCtq)
  → onto_admit_work_order (validates structure + law)
  → onto_propose_requirement (optional: deferred admission)
  → Evidence<T, Admitted, W> (only on success)
  → onto_manufacture_solution (LLM-driven manufacturing)
  → onto_executive_projection (executive model generation)
  → Receipt (BLAKE3 + optional Ed25519 seal)
```

### SPARQL Query Tool

**Tool Name:** `onto_query`

**Query Interface:**
- Input: SPARQL SELECT/CONSTRUCT/ASK query string
- Graph: RDF triples from `/Users/sac/open-ontologies/src/graph.rs` (`GraphStore`)
- Output: JSON results (SPARQL result format)

**Use Case:** Audit queries like:
```sparql
PREFIX compat: <https://wasm4pm-compat.rs/ontology#>
PREFIX paper: <https://wasm4pm-compat.rs/paper#>

SELECT ?witness ?witnessTitle ?paperKey
WHERE {
  ?witness a compat:WitnessMarker ;
           compat:witnessTitle ?witnessTitle ;
           compat:citePaper ?paperKey .
}
```

### Validation Tool

**Tool Name:** `onto_validate`

**Input Formats:** OCEL 2.0 JSON, XES 1.0, BPMN 2.0 XML, Petri Net (various)

**Process:** Structural validation against ontology-derived SHACL shapes

**Refusal:** Returns `onto:ValidationFailed` with named structural law (e.g., `MissingEventField`, `InvalidObjectLink`)

---

## 6. Server Dependencies & Runtime Config

### Primary Config File Location

`~/.open-ontologies/config.toml`

**Sections:**
- `[general]` — data directory, ontology search paths
- `[http]` — binding host/port, CORS, rate limiting, optional auth token
- `[logging]` — level, optional log file path
- `[cache]` — ontology compile cache TTL, auto-refresh
- `[monitor]` — background monitor loop interval
- `[llm]` — LLM provider (Groq, OpenAI) + API base
- `[authority]` — admin principals, known tenants
- `[retention]` — TTL windows for artifacts/receipts
- `[verifier]` — receipt verification worker config

### Key Runtime Dependencies

**From imports in `cmds/server.rs`:**

| Module | Purpose |
|---|---|
| `open_ontologies::config::{Config, expand_tilde}` | Config loading + path expansion |
| `open_ontologies::graph::GraphStore` | RDF triple store (underlying Oxigraph) |
| `open_ontologies::mcpp_gate::MaybeGatedServer` | Optional managed connection wrapper |
| `open_ontologies::server::OpenOntologiesServer` | Main MCP server handler |
| `open_ontologies::state::StateDb` | Persistent state store (SQLite) |
| `open_ontologies::toolfilter::ToolFilter` | Allow/deny list filtering |

### Storage Backend

**Files:**
- `/Users/sac/.open-ontologies/open-ontologies.db` (Oxigraph RDF store, 674 MB as of 2026-06-01)
- `/Users/sac/.open-ontologies/config.toml` (user configuration)
- `/Users/sac/.open-ontologies/cache/` (compiled ontology cache)

---

## 7. Wasm4pm-Compat Integration Points

### Type-Law Admission Bridge

**Tool:** `onto_admit_work_order`

**Maps to:** wasm4pm-compat `src/admission.rs` (Admit::admit path)

**What flows:** `Evidence<T, Raw, W>` → `Evidence<T, Admitted, W>`

**Witness markers checked:**
- `Ocel20` — standard compliance
- `WfNetSoundnessPaper` — Petri net law enforcement
- `VanDerAalst2016` — process discovery authority
- Paper-derived `compat:WitnessMarker` instances from `ggen/ontology/papers.ttl`

### Loss Accounting Bridge

**Tool:** `onto_validate` (with `LossPolicy` parameter)

**Maps to:** wasm4pm-compat `src/loss.rs` (Project::project path)

**What flows:**
```
Input (Ocel20) 
  → onto_validate(policy=AllowNamedProjection, name="ocel_to_xes_lossy")
  → LossReport<Ocel20, Xes1849, Items>
  → Evidence<T, Exportable, Xes1849>
```

### Conformance Feedback Loop

**Tools:** `onto_query` (discover violations) + `onto_propose_requirement` (suggest fixes)

**Map to:** wasm4pm-compat `src/conformance.rs` (Metric bounds, fitness/precision/generalization)

**Violation Report:** RDF triples with namespace `audit:*` (from `ggen/ontology/audit-machinery.ttl`)

---

## 8. Audit Machinery & Receipt Chain

### Receipt Emission Points

**Tool:** Any onto_* tool (14+ call sites across codebase)

**Function:** `emit_tool_ocel(tool_name, event_type, attributes)` 

**Attributes:**
- `latency_ms` — measured LLM call latency
- `provisional` — whether result is backed by full replay (Stream-2 vs Stream-3)
- `refinements` — number of swarm refinement cycles
- `receipt_hash` — BLAKE3 chain commitment

**OCEL Event Structure:**
```json
{
  "event_type": "onto_executive_projection",
  "timestamp": "2026-06-01T15:43:00Z",
  "attributes": {
    "latency_ms": 1250,
    "provisional": false,
    "refinements": 3,
    "receipt_hash": "blake3:abc123..."
  }
}
```

### Receipts Storage

**Location:** `/Users/sac/open-ontologies/src/receipts.rs`

**Types:**
- Autoreceipts (BLAKE3 only, automatic)
- Sealed receipts (+ Ed25519 signature, optional, admin-gated)

**Export Format:** JSON (keyed by session ID + timestamp)

---

## 9. Admin Gating & Defects

### Admin-Only Tools

| Tool | Reason | Call Sites |
|---|---|---|
| `onto_retention_pause` | Pause artifact retention | 3 |
| `onto_bootstrap_unlock` | Bootstrap mode access | 3 |
| `onto_receipts_revoke_batch` | Revoke receipt batches | 5 |
| `onto_session_revoke_by_principal` | Revoke principal sessions | 4 |
| `onto_retention_resume` | Resume paused retention | 2 |

**Gating Mechanism:** Check `OPEN_ONTOLOGIES_ADMIN_PRINCIPALS` environment variable

**Defect Constant:** `DEFECT_REASON_NOT_ADMIN` (consolidated from 6 literal sites to prevent silent drift)

### Defect Reporting

**Constant:** `ADMISSION_VERDICT_DENIED` (consolidated from 10 literal sites)

**Defect Class Field:** `defect.kind` (e.g., `"FalsePass"`, `"StructureViolation"`, `"AuthorityMissing"`)

**Defect Reason Field:** `defect.reason` (e.g., `"not_admin"`, `"invalid_evidence_state"`, `"missing_witness_marker"`)

---

## 10. MCP Server Features Supported

### RMCPv2 Capability Set

| Capability | Status | Implemented |
|---|---|---|
| **Tools** | ✓ Required | Yes — 19+ onto_* tools registered |
| **Prompts** | Partial | Yes — prompt router available, usage TBD |
| **Resources** | Planned | Partial — GraphStore acts as implicit resource |
| **Sampling** | No | Not implemented |
| **Connection Upgrade** | Yes | MCPP gating available (`MaybeGatedServer`) |

### Tool Registration Schema

Each onto_* tool exposes:
- **Name:** e.g., `onto_executive_projection`
- **Description:** Human-readable purpose
- **Input JSON Schema:** Arguments + types
- **Output:** JSON result or error struct with `defect` field

---

## 11. Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│ MCP Client (Claude, Studio, etc.)                           │
└────────────────────┬────────────────────────────────────────┘
                     │ RMCPv2 Protocol
                     ▼
┌─────────────────────────────────────────────────────────────┐
│ MaybeGatedServer (Optional MCPP wrapper)                    │
│   - Connection management                                   │
│   - Session state tracking                                  │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│ OpenOntologiesServer (extends ServerHandler)                │
│   - Tool registration (ToolRouter)                          │
│   - Prompt handling (PromptRouter)                          │
│   - Event logging (emit_tool_ocel)                          │
└────────────────────┬────────────────────────────────────────┘
                     │
        ┌────────────┼────────────┐
        ▼            ▼            ▼
   ┌─────────┐ ┌──────────┐ ┌───────────┐
   │ Tool    │ │ Admin    │ │ LLM       │
   │ Handlers│ │ Gate     │ │ Backends  │
   │(onto_*) │ │(OAPP)    │ │(Groq/OAI) │
   └────┬────┘ └─────┬────┘ └─────┬─────┘
        │            │            │
        └────────────┼────────────┘
                     │
                     ▼
        ┌────────────────────────┐
        │ GraphStore (Oxigraph)  │
        │ - RDF triple store     │
        │ - SPARQL engine        │
        │ - Reasoner (optional)  │
        └────────────┬───────────┘
                     │
                     ▼
        ┌────────────────────────┐
        │ StateDb (SQLite)       │
        │ - Sessions             │
        │ - Artifacts            │
        │ - Receipts             │
        │ - Audit trail          │
        └────────────────────────┘
```

---

## 12. Tauri Studio Integration

**File:** `/Users/sac/open-ontologies/studio/src-tauri/src/mcp.rs`

Experimental Tauri desktop app with MCP server backend. Mirrors main server.rs but runs embedded in Electron-like context.

---

## Summary

**wasm4pm-compat integration with open-ontologies MCP servers:**

1. **Tool Registry:** 19+ onto_* tools defined as OCEL event constants (prevents silent drift)
2. **Server Boot:** Noun-verb CLI framework via `clap_noun_verb` + config template in `cmds/server.rs`
3. **Admission Gate:** `onto_admit_work_order` bridges to wasm4pm-compat `Admit::admit()` path
4. **Evidence Flow:** Raw → Parsed → Admitted → Projected | Exportable | Receipted (typed state machine)
5. **Loss Accounting:** `onto_validate` bridges to wasm4pm-compat `Project::project()` path with LossReport
6. **Receipt Chain:** BLAKE3 + optional Ed25519 seal, stored in Oxigraph + SQLite
7. **Audit Trail:** OCEL events in graph store, queryable via `onto_query` (SPARQL)
8. **Admin Gating:** 5 admin-only tools, checked via `OPEN_ONTOLOGIES_ADMIN_PRINCIPALS` env var
9. **RMCPv2 Protocol:** Full tool router + partial prompt router, no sampling/resources yet
10. **Config:** `~/.open-ontologies/config.toml` template with LLM, cache, retention, authority sections

**Critical:** All tool names and event types are consolidated into constants to make event-log drift immediately detectable (typos cause visible audit failures, not silent gaps).
