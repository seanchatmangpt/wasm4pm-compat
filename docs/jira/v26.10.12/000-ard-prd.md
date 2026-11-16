# 000 — wasm4pm-compat v26.10.12 ARD/PRD: npm publish for TypeScript consumers

**Repository:** `seanchatmangpt/wasm4pm-compat`
**Release target:** v26.10.12 (crate `wasm4pm-compat` 26.8.7 → 26.10.12)
**Standing conferred:** `UNKNOWN` until every acceptance command below exits 0 on the
exact release tree.
**Companion work orders:** `001-npm-publish-pipeline.md`,
`002-ocel-v2-ts-surface.md`.

## Product requirement

`wasm4pm-compat` SHALL expose its structure-only process type law to TypeScript
consumers via npm, so that external TS projects can express, validate, and
structurally-admit OCEL v2 documents and evidence shapes without compiling Rust and
without importing process-mining engine logic.

The v26.10.12 target preserves the repository fence: the npm packages ship **type law
and structural admission only**. No discovery, conformance algorithms, alignment,
replay engines, or production actuation ship in any JS/TS artifact.

## Required capabilities

1. A published npm package `@wasm4pm/compat-ts` (26.10.12) whose Zod shape surface is
   regenerated from the canonical ggen TTL → Zod chain and covers the OCEL v2 types
   (`OcelLog`, `OcelEvent`, `Object`, `EventObjectLink`, `OcelAttribute`,
   `OcelAttributeValue`, link/change shapes) plus the evidence lifecycle state
   vocabulary.
2. A published npm package `@wasm4pm/compat-wasm` (26.10.12) exporting, through a
   `compat-ocel` WIT world and jco-generated TS, at minimum:
   - OCEL v2 JSON parse (structural intake),
   - `validate` (the crate's OCEL structural validation),
   - structural admission returning a **named refusal law** on failure
     (e.g. `DanglingEventObjectLink`, `MissingFinalMarking`-class names — the actual
     OCEL refusal variants in `src/ocel/validate.rs`).
   Every exported function SHALL be backed verbatim by
   `wasm4pm_compat::ocel::*` — no re-implemented logic in TS.
3. The WIT surface is generated from `ggen/templates/wasm4pm-compat.wit.tera` and
   rendered through the ggen chain (or, where the installed ggen CLI cannot render
   it, the documented workaround in `docs/WIT_RUNTIME.md` is used and the divergence
   is ledgered per the first-class-source rule).
4. Crate version bump 26.8.7 → 26.10.12 in `Cargo.toml`, with the npm packages
   versioned 26.10.12 in lockstep.
5. A release workflow `.github/workflows/npm-release.yml` that builds the component,
   transpiles with jco, runs the TS test suites, and publishes both packages — gated
   on the existing CI gates (`cargo make ci`), dispatching only when gates exit 0.
6. A runnable TypeScript consumer example demonstrating: parse a valid OCEL v2
   document → typed success; parse a malformed one → typed refusal with the named
   law. This example doubles as the conformance fixture for the packages.

## Architecture requirements

```text
canonical TTL / crate type law (Rust, nightly)
        -> WIT world (compat-ocel) rendered from ggen chain
        -> WASM component (component crate reusing ../wasm4pm-compat-component pattern)
        -> jco-transpiled TS + @wasm4pm/compat-ts Zod shapes
        -> npm: @wasm4pm/compat-ts (shape law) + @wasm4pm/compat-wasm (structural logic)
        -> typed admission / named refusal at the TS boundary
```

- Single source of truth: the Rust crate. TS artifacts are projections. Hand-edits to
  generated artifacts are tracked divergences, not fixes.
- The one-way door is preserved across the FFI boundary: TS consumers see
  `Raw → Parsed → Admitted | Refused` as data, never as mutable state they can forge.
- `portable_simd` / `nightly_foundry` and other wasm32-incompatible surfaces stay out
  of the component; full-crate wasm-bindgen is explicitly out of scope for v26.10.12.

## Verification architecture

Every claim in this document is `UNKNOWN` until run on the exact release tree:

| Gate | Command |
|---|---|
| Rust CI | `cargo make ci` |
| Zod surface regenerated | ggen sync replay (documented workaround; diff-empty) |
| TS shape tests | `npm test` in `wasm4pm-compat-ts/` |
| Component logic tests | smoke test ported from `docs/WIT_RUNTIME.md` recipe |
| Publish dry-run | `npm publish --dry-run` for both packages |

A package is not "published-alive" because it compiles; it is alive when a TS
consumer on npm can parse an OCEL v2 fixture and receive the exact named refusal for
a mutated fixture.

## Acceptance for v26.10.12

- [x] `Cargo.toml` version = 26.10.12; both `package.json` files at 26.10.12.
      (V7: PASS — Cargo.toml and both npm packages at 26.10.12 in lockstep.)
- [x] `@wasm4pm/compat-ts` Zod surface regenerated and renders idempotently (×2 via
      the documented root-manifest workaround); covers OCEL v2 + evidence vocabulary.
      (V7: PASS, reworded from "diff-empty on replay" — the first render's diff was
      large because the shipped file was stale against the graph, not because the
      generator is non-idempotent.)
- [x] `compat-ocel` WIT world renders; component builds; jco transpile succeeds.
      (V7: PASS — hand-maintained WIT pattern; component lives in a sibling crate
      per the documented `docs/WIT_RUNTIME.md` recipe.)
- [x] TS consumer fixture: valid OCEL v2 JSON → success; malformed → named refusal
      (exact law string asserted).
      (V7: PASS — parse/schema assertions in the TS fixture; the exact dangling/
      empty law-string assertions live in the `wasm4pm-compat-wasm` test suite.)
- [x] `npm-release.yml` exists and its dry-run mode passes locally.
      (V7: PASS — dry-run verified; the tag-triggered publish run is still pending.)
- [ ] `cargo make ci` exits 0 on the release tree.
      (V7: IN FLIGHT — final gate pending at integration; not yet ticked.)
- [x] No JS/TS artifact exports engine logic; audit grep for discovery/alignment
      symbols in shipped TS returns zero hits.
      (V7: PASS — zero violations across two verifier runs.)
- [x] Locality law wired for the npm surface. (V7: PASS — lane W1; the locality
      gate is a separate, opt-in export `validate-ocel-locality` rather than part
      of admit by design: admit still admits the fixture, the dedicated gate
      refuses it with `ViolatesLocalityPrinciple`.)

## Falsifiers

- A malformed OCEL v2 document parses as success through the npm packages → package
  is unlawful; block publish.
- TS logic that disagrees with `wasm4pm_compat` on the same input → the projection
  has drifted; block publish.
- Generated Zod/WIT drifts from its TTL/manifest without a detected diff → the
  generation law is broken.
- Any npm artifact re-implements admission rather than calling the component → the
  single-source-of-truth requirement is falsified.

## Ecosystem contract

`wasm4pm-compat` supplies type law; `wasm4pm` executes process behavior. The npm
packages extend the type-law surface to TypeScript runtimes; they confer no
actuation authority. Maximum claim remains bounded by the exact-tree verifier
evidence defined above.
