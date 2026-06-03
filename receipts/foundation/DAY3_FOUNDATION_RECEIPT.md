# Day 3 Foundation Receipt

**Date:** 2026-06-03
**Branch:** `fix/debt-markers-and-gap-close`
**Recipient:** Sean Chatman
**Receipt Version:** FOUNDATION (human-readable, not v2 cryptographic)

---

## What Was Found

### The Macro Failure

Eight hours of prior work treated the substrate/pack/consumer problem as a **repair problem** — moving types into compat, fixing ggen output paths, patching templates. That was the wrong frame.

The actual problem: **the system had never declared what kind of thing each layer is.** Without that declaration, every patch was theory-less. Without that declaration, a generated witness is indistinguishable from an orphan. Without that declaration, a type claiming paper authority is indistinguishable from a label sticker.

The four compounding failures found:

| Failure | Evidence |
|---|---|
| `PowlArena` hand-carved in consumer without substrate registration | `wasm4pm/src/powl_arena.rs`, no ontology entry, no receipt |
| `src/generated/` folder with DO-NOT-EDIT banner | `wasm4pm/wasm4pm/src/generated/witnesses.rs` — 1 of 37 witnesses, orphaned |
| ggen receipt hashes only `ggen.toml`, not TTL/query/template | `wasm4pm/.ggen/receipts/latest.json` input_hash = manifest only |
| Substrate receipt has empty input/output hashes | `wasm4pm-compat/.ggen/receipts/latest.json` — signed empty envelope |

### The Root Architecture State

```
SUBSTRATE (wasm4pm-compat/src/)     ████████████░░░░░░░░  ~60% correct
PACK (wasm4pm-compat/ggen/)         █████░░░░░░░░░░░░░░░  ~25% operational
CONSUMER (wasm4pm/)                 ██░░░░░░░░░░░░░░░░░░  ~10% compliant
RECEIPTS                            █░░░░░░░░░░░░░░░░░░░  ~5% valid
```

### POWL Specifically

The POWL paper (arXiv:2505.07052) is the canonical example of the architectural failure:

- `PowlArena` is `CONSUMER_HAND_CARVED` — pm4py mirror, valid design rationale (wasm-bindgen compatibility), but never declared, never receipted, never registered as a consumer surface
- Two types named `PowlNode` exist simultaneously — substrate generic struct and consumer enum — classified as `DUPLICATE_AUTHORITY`
- `DecisionGraphNode` and `ChoiceGraph` both represent the same paper concept (Definition 1) in two different layers — dual-representation defect

---

## What Was NOT Changed

**No code was modified.** Specifically:

- No Rust types were moved
- No `ggen.toml` files were edited
- No templates were changed
- No `generated/` folders were deleted
- No migrations were continued
- No cargo builds were run
- No receipt chains were rewritten

The prior session's changes (POWL repointing to compat, `DfgMiner`/`PetriNetBuilder` additions, `ChoiceGraph::new()` constructor) are on-disk as committed. This receipt does not roll them back or validate them — they are categorized in the POWL audit but not adjudicated.

---

## Why No Patch Was Applied

A patch applied without a declared boundary is an assertion without authority. Every patch from the prior session was an assertion: "this belongs here" or "this should do that." None of them were derived from a stated first principle.

Day 3 work cannot be patch work because:

1. A patch to the wrong layer is worse than no patch — it creates false confidence
2. The receipting system cannot validate a patch without the chain requirements declared
3. The POWL naming collision (`PowlNode` duplicate) cannot be resolved by moving code — it requires declaring which name is canonical first

The Nehemiah precedent applies: he surveyed the wall for three days before announcing a plan. The survey itself was the essential work. The survey is this document set.

---

## Boundary Laws Now Declared

The following foundation documents are committed in this session:

| Document | Path | Declares |
|---|---|---|
| Foundation Law | `docs/foundation/DAY3_FOUNDATION_LAW.md` | Substrate, pack, consumer, witness, use-site, receipt, replay; second-class prohibition; no-distinction law |
| Boundary Declaration | `docs/foundation/CONTRIB_PACK_CONSUMER_BOUNDARY.md` | Boundary table with specific example mappings; anti-patterns; wall survey |
| Receipt Requirements | `docs/foundation/PACK_USE_RECEIPT_REQUIREMENTS.md` | v2 receipt schema; refusal conditions; current state vs requirements |
| POWL Audit | `docs/foundation/POWL_V2_FOUNDATION_AUDIT.md` | Full matrix of 10 POWL objects; classification for each; priority order for foundation resolution |
| This receipt | `receipts/foundation/DAY3_FOUNDATION_RECEIPT.md` | Summary; verdict |

---

## What the Next Admissible Implementation Slice Is

The foundation is now declared. The first implementation work must prove the chain end-to-end before anything else is generalized. The slice must:

1. **Resolve the `PowlNode` naming collision** — rename the consumer enum to `PowlArenaNode`. This is the CRITICAL priority from the POWL audit. It is a purely mechanical rename in `wasm4pm/src/powl_arena.rs` and all its callers. It does not change any types — only removes the `DUPLICATE_AUTHORITY` defect.

2. **Declare `compat:PowlArena` in the ontology** — register the consumer surface as `compat:PowlArena` with `compat:graduatesToWasm4pm true` and a note on its pm4py mirror rationale. This is an ontology-only change (TTL edit).

3. **Prove one complete pack-use receipt** — pick the simplest rule (e.g. `witness-markers`), run it from the compat ggen, land output in `wasm4pm/src/witnesses.rs` (no `generated/` subfolder), `mod`-declare it in `lib.rs`, and produce a v2 receipt that hashes TTL + query + template + output + use-site. This proves the chain works before anything is generalized.

4. **Delete `wasm4pm/src/generated/witnesses.rs`** — after the above receipt is valid, the orphan is deleted. The `generated/` folder disappears with it.

This four-step slice, completed and receipted, proves the foundation law is operational, not just declared.

---

## Verdict

**`DAY3_FOUNDATION_PARTIAL`**

The foundation law is declared. The boundary is mapped. The defects are classified. The receipt requirements are specified.

The foundation is not `DAY3_FOUNDATION_READY` because the first implementation slice (above) has not been completed and receipted. A declared boundary law that has not been proven on one concrete slice remains aspirational.

The foundation is not `DAY3_FOUNDATION_BLOCKED` because there are no unresolvable contradictions. Every defect has a clear classification and a priority ordering. The architecture is coherent — the failures are implementation position errors, not design errors.

**Day 4 begins when the `PowlArenaNode` rename and the first v2 pack-use receipt are committed.**

---

## Chain Verification

This receipt is not cryptographically signed (foundation receipts are human-readable). Its integrity is verified by:

```bash
# Verify the five documents exist and are non-empty
ls -la /Users/sac/wasm4pm-compat/docs/foundation/
ls -la /Users/sac/wasm4pm-compat/receipts/foundation/

# Verify no Rust was modified in this session
git diff HEAD --name-only -- '*.rs' | grep -v foundation

# Verify no ggen.toml was modified in this session
git diff HEAD --name-only -- '*.toml' | grep -v foundation

# Verify cargo still compiles (nothing was broken)
cargo check -p wasm4pm-compat
```

Expected result: five `.md` files created, zero `.rs` or `.toml` files modified, `cargo check` passes.

---

*This receipt is the boundary stone. No implementation may claim Day 3 completion without it. No patch may claim substrate authority without the foundation law declared here as its controlling document.*
