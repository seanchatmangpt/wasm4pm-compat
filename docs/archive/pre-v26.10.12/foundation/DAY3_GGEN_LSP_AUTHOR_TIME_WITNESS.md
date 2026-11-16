# ggen LSP — Author-Time Witness of Provision Legality

> *Adam names the animal before the steward acts.*

ggen LSP occupies the **author-time witness** position in the provision chain. It does not yield. It does not judge downstream consequence. It witnesses whether the provision relation is lawful **before** the steward invokes ggen.

---

## Position in the Chain

```
wasm4pm-compat (seed / substrate / pack authority)
       ↓
ggen-lsp WITNESSES provision relation before yield
       ↓
ggen YIELDS source after kind
       ↓
wasm4pm (consumer — receives via Cargo dep)
       ↓
process evidence (court material)
       ↓
receipt (proof)
       ↓
replay (witness of reproduction)
```

**ggen-lsp ≠ Court.  ggen-lsp = AuthorTimeWitness.**

The court is downstream: ProcessIntelligence + Receipt + Replay.
ggen-lsp names the relation before the steward acts.

---

## Role Map

| Layer | Role |
|---|---|
| `wasm4pm-compat` | seed / substrate / pack authority |
| `ggen` | provision instrument |
| **`ggen-lsp`** | **author-time witness of provision legality** |
| `wasm4pm` | consumer surface receiving first-fruit (via Cargo dep) |
| `wasm4pm` process evidence | court material |
| receipt | proof |
| replay | witness of reproduction |

---

## What ggen LSP Watches

```
ggen.toml
queries/*.rq
templates/*.tera
ontology/*.ttl
declared output_file
declared output_dir
consumer root
expected use-site
```

---

## The Seven Diagnostics (GGEN-YIELD-001 through GGEN-YIELD-006)

Each diagnostic maps to a σ violation from the Kind Ledger (`DAY3_KIND_LEDGER.md`).

| Diagnostic | σ violation | Description |
|---|---|---|
| `GGEN-YIELD-001` | `LAYER_VIOLATION` | output target is pack root, not consumer root. Expected: `wasm4pm/src/witnesses.rs`. Found: `wasm4pm-compat/src/witnesses.rs`. |
| `GGEN-YIELD-002` | `SECOND_CLASS` | rendered source would be second-class. Path contains `generated/` or template emits `DO NOT EDIT` banner. |
| `GGEN-YIELD-003` | `ORPHAN` | rendered source has no use-site. No `pub mod witnesses;` in consumer root. An output without a use-site is `ORPHAN ∈ σ`. |
| `GGEN-YIELD-004` | `COMPETING_AUTHORITY` | competing manifest authority controls the same output_file. Two rules writing the same path = undefined behavior. |
| `GGEN-YIELD-005` | `REMOTE_FETCH_PROHIBITED` | remote ontology fetch enters replay path. `allow_remote_fetch = true` for an ontology pack means replay is non-reproducible. `Replayable(Pack) ⇒ RemoteFetch = false`. |
| `GGEN-YIELD-006` | `SECOND_CLASS` | template emits `DO NOT EDIT` banner. The template comment should say "This IS the source", not "this file is auto-generated." |
| `GGEN-YIELD-007` | `UNKNOWN` | pack inputs incomplete. TTL, query, template, or manifest rule is missing or unresolvable. |

---

## The Error ggen LSP Would Have Caught

The first workflow run targeted `wasm4pm-compat/src/witnesses.rs` and called it the consumer transition. ggen LSP diagnostic `GGEN-YIELD-001` would have fired before yield:

```
GGEN-YIELD-001: output target is pack root, not consumer root
  Manifest output_dir = ".." (from ggen/) = wasm4pm-compat/
  output_file = "src/witnesses.rs"
  Effective target: wasm4pm-compat/src/witnesses.rs
  Expected consumer root: wasm4pm/src/witnesses.rs (for the transition)
  
  Fix: Add consumer-side ggen manifest in wasm4pm/ referencing compat pack,
       OR confirm that wasm4pm-compat self-yield IS the correct first-fruit target
       and the consumer receives types via Cargo dep, not a rendered file.
```

This is the exact category error: yield into the seed bank and call it consumer transition.

---

## What ggen LSP Is Not

```
ggen-lsp ≠ ggen (the provision instrument)
ggen-lsp ≠ ProcessIntelligence (the court)
ggen-lsp ≠ Receipt (the proof)
ggen-lsp ≠ Replay (the witness of reproduction)
ggen-lsp ≠ the judge of downstream goodness
```

ggen-lsp witnesses admissibility **before** the steward acts. The goodness of the fruit is judged downstream after yield, use-site, receipt, and replay.

---

## Wiring Target

ggen LSP should be wired to observe the manifest/output relation, not just file syntax:

```
ggen-lsp watches:
  ggen.toml (output_dir, output_file, rule names)
  queries/*.rq (SPARQL validity, variable alignment with template)
  templates/*.tera (DO NOT EDIT detection, generated/ path detection)
  ontology/*.ttl (WitnessMarker instances, remote fetch detection)
  consumer root (expected path for consumer-side output)
  use-site declarations (does the consumer lib.rs declare the module?)
```

The LSP emits diagnostics in the editor before any steward invokes `ggen sync`.

---

## Relationship to cargo-cicd-lsp

`cargo-cicd-lsp` is already identified as author-time witness for Cargo/Rust relation-state drift (`DAY3_PROJECT_ATLAS.md`). ggen LSP is the same role applied to the provision chain:

```
cargo-cicd-lsp: "What Rust/Cargo relation-state drift is observed before false closure?"
ggen-lsp:       "What provision relation error is visible before false yield?"
```

They are parallel author-time witnesses at different layers of the chain.

---

## Current Status (Found)

**ggen-lsp exists at `/Users/sac/ggen/crates/ggen-lsp/`** — a real crate, not invented.

Three-plane architecture (per `/Users/sac/ggen/docs/how-to/integrate-ggen-lsp-with-claude-code.md`):
- **LSP plane**: `ggen-lsp` binary (Content-Length-framed JSON-RPC over stdio); wires via Claude Code plugin marketplace
- **MCP plane**: `ggen-lsp-mcp` (newline-framed JSON-RPC); wires via `.mcp.json`
- **A2A plane**: `ggen-lsp-a2a` (leaf crate bridging to A2A hosts)

Marketplace declaration: `/Users/sac/ggen/.claude-plugin/marketplace.json` — already written; covers `.ttl`, `.nt`, `.nq`, `.rq`, `.sparql`, `.tera`, `.toml`.

**Current blocking fact:** `ggen-lsp` binary is NOT on PATH. Neither LSP nor MCP plane is wired to `wasm4pm-compat`.

Day 4 wiring (two commands):
```bash
cargo install --path /Users/sac/ggen/crates/ggen-lsp   # puts ggen-lsp on PATH
# in Claude Code:
/plugin marketplace add /Users/sac/ggen                  # wires LSP plugin
```

MCP plane requires adding to `wasm4pm-compat/.mcp.json`:
```json
{
  "mcpServers": {
    "ggen-lsp-mcp": { "command": "cargo", "args": ["run", "-q", "-p", "ggen-lsp-mcp"] }
  }
}
```

For Day 3, this document records the **author-time witness position and the wiring path** so Day 4 can execute it in one step.

---

## Day 3 Verdict on ggen LSP

**Named and positioned. Not yet wired.**

The role is assigned: `ggen-lsp = AuthorTimeWitness(ggenProvision)`. The seven diagnostics are specified. The relationship to cargo-cicd-lsp is named. The exact error it would have caught (`GGEN-YIELD-001`) is documented.

Wiring it is Day 4 cultivation.

---

## Verdict

**`DAY3_GGEN_LSP_WITNESS_POSITIONED`**

ggen LSP is positioned as author-time witness of provision legality. It witnesses the provision relation before yield — pack authority, rule, output target, consumer root, first-class source status, use-site expectation. The seven GGEN-YIELD diagnostics are specified. Wiring the binary is Day 4.

*See `DAY3_PROJECT_ATLAS.md` for ggen-lsp's place in the full constellation map.*
*See `DAY3_BRANCH_DISCLOSURE_DISCIPLINE.md` for BranchDiscipline governing author-time witness actions.*
