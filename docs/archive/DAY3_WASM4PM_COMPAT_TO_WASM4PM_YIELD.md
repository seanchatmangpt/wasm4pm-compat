# Day 3 Yield: wasm4pm-compat → wasm4pm

**Date:** 2026-06-03
**Verdict:** DAY3_WASM4PM_COMPAT_TO_WASM4PM_YIELD_READY

---

## What wasm4pm-compat provided

- **Witness-marker pack authority:** `ggen/ontology/wasm4pm-compat.ttl` — declares all `compat:WitnessMarker` instances that constitute the substrate witness law
- **Query:** `ggen/queries/extract-witnesses-full.rq` — 7 variables extracted per marker (uri, name, family, algorithm, group, description, aliases)
- **Template:** `ggen/templates/witness-marker.tera` — Tera template producing `witness_marker!()` call syntax
- **Manifest rule:** `witness-markers` declared in `ggen/ggen.toml` — binds TTL + Q + T into a named rule that `ggen sync` can target

---

## What ggen rendered

- **Output:** `src/witnesses.rs` — 48 `witness_marker!()` calls
- **Kind:** κ = RenderedSource (consumer instantiation of substrate witness law)
- **Rendered via:** `ggen sync --rule witness-markers`

---

## Why it is source after kind

- No DO NOT EDIT banner
- No `generated/` path
- Template comment states: "This IS the source. The ontology is the spec."
- `witness_marker!()` calls are indistinguishable from hand-written witness declarations
- Produced via the lawful chain: TTL + Q + T + Manifest → Source

---

## Why it is not second-class

- Lives at `src/witnesses.rs` (not `src/generated/`)
- No DO NOT EDIT caste marker
- Same kind as hand-written source: one happens to be compiled from ontology, one defines the trait
- Docstring in `witnesses.rs` explicitly states: "Neither is second-class."

---

## Use-site

- `pub mod witnesses;` declared in `src/lib.rs` (after `pub mod witness;`)
- Output is not ORPHAN: use-site is bound

---

## Why witnesses.rs lives in wasm4pm-compat (not wasm4pm)

`witnesses.rs` lives in `wasm4pm-compat/src/` rather than `wasm4pm/src/` because wasm4pm-compat is the canonical specification layer — it defines the `Witness` trait, `WitnessFamily` ontology URIs, and `AggregationView` contract that any consumer (including wasm4pm itself) must implement against, keeping the type authority in the compat crate and preventing wasm4pm from owning types it merely consumes.

Note: `wasm4pm/src/generated/witnesses.rs` exists as a generated consumer artifact that imports from wasm4pm-compat, which confirms the correct direction of the dependency.

---

## wasm4pm consumer boundary

- `wasm4pm/Cargo.toml`: `wasm4pm-compat = { path = "../../wasm4pm-compat" }`
- wasm4pm receives the witness types via Cargo, not a separate file
- The Day 3 transition is complete: contrib provides, ggen renders, wasm4pm consumes via dep

---

## What remains for downstream judgment

- **v2 pack-use receipt:** H(TTL, Q, T, manifest, output, UseSites, Tests, R_prev) — Day 4
- **Replay verification:** ϱ = 1 ⟺ μ_pack(TTL, Q, T) = output — Day 4
- **wasm4pm operational use** of witness types (use site in wasm4pm consumer) — Day 4

---

## Verdict

DAY3_WASM4PM_COMPAT_TO_WASM4PM_YIELD_READY
