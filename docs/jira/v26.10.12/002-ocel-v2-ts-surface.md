# 002 — OCEL v2 TypeScript surface (Zod + WIT/component)

- Standing: OPEN
- Created: 2026-10-12
- Source: v26.10.12 ARD/PRD (`000-ard-prd.md`), capabilities 1–3 and 6.
- Blocks: `001-npm-publish-pipeline.md`.

## Work to complete

1. Extend the ggen TTL → Zod chain so `wasm4pm-compat-ts` covers the OCEL v2 types
   (`OcelLog`, `OcelEvent`, `Object`, `EventObjectLink`, `OcelAttribute`,
   `OcelAttributeValue`) and the evidence lifecycle state vocabulary. No
   hand-written schemas; regenerating via the documented `ggen sync run` workaround
   in CLAUDE.md must be diff-empty.
2. Define the `compat-ocel` WIT world (OCEL v2 parse, validate, structural admission
   returning named refusal laws) rendered from
   `ggen/templates/wasm4pm-compat.wit.tera` (or documented workaround; ledger
   divergence).
3. Build the component (sibling-crate pattern of `../wasm4pm-compat-component`),
   backed verbatim by `wasm4pm_compat::ocel::*`; jco transpile.
4. TS consumer fixture test: valid OCEL v2 JSON → success; mutated/malformed →
   typed refusal with the exact named law asserted.
5. Bump `Cargo.toml` and both `package.json` files to 26.10.12.

## Acceptance

- `npm test` in `wasm4pm-compat-ts/` exits 0 including the new OCEL fixtures.
- Component smoke test parses a real OCEL v2 doc and returns the named refusal for
  the mutated one (assert the exact law string).
- ggen replay diff-empty; `cargo make ci` exits 0.

## Falsifier

- Malformed OCEL parses as success, or the refusal loses its named law across the
  FFI boundary → surface is unlawful; block release.
- Locality wiring: `validate_gianola_2026_locality` (src/ocel.rs:855) is defined
  but never invoked — `LinkedOcel::admit` (src/ocel.rs:1000-1014) calls only
  `OcelLog::validate` (src/ocel.rs:803-813), which fires
  `EmptyEventObjectLinks`/`DanglingEventObjectLink` and nothing else, and
  `admit_ocel_v2_json` (src/ocel.rs:1181-1194) is parse → `ocel_to_log` → admit.
  `MissingReferenceObject`, `MultipleReferenceObjects`, and
  `ViolatesLocalityPrinciple` are therefore unreachable from the TS surface, and
  the `locality` entry in `test/ocel-v2-fixture.test.ts` `LAW_STRINGS` names a law
  the wasm boundary cannot emit. Wire the locality check into the admit path (or
  demote it in `LAW_STRINGS` with an explicit "not yet enforced" marker) and
  re-point `ocel_v2_mutation_locality.json` at a live `admitOcelJson` assertion;
  until then the fixture asserts a refusal that no code path can produce.
- Verified amendment: the fixture itself is a faithful Gianola (2026) Example 4
  violation — e1 links `t1`(team)+`p2`(employee), e2 links `t2`(team)+`p2`, so
  `p2`'s parent moves `t1→t2` and the check (if wired) refuses with
  `ViolatesLocalityPrinciple`. Both reference objects being "team"-type is the
  violation's precondition, not its absence. The gap is non-wiring, not a
  mis-built fixture.

## History

- 2026-10-12 | OPEN | work order created from ARD/PRD | surface not yet widened
- 2026-10-12 | PARTIAL_ALIVE | W-B/W-B2 | ggen `zod-schemas` rule enabled and rendered
  (idempotent ×2); `OcelLogSchema`/`EvidenceStateSchema` (7 states) ship in
  `wasm4pm-compat-ts/bindings/zod_schemas.ts`; OcelLog-region `z.any()` = 0
  (time → z.string() RFC3339, attribute values → z.union per serde untagged enum).
  `npm test` 6/6 incl. all OCEL fixtures (RFC3339-converted).
- 2026-10-12 | PARTIAL_ALIVE | W-D/W-E2 | component (sibling crate) exports
  parse/validate/admit verbatim-backed by the crate; jco output landed in
  `wasm4pm-compat-wasm/dist`; Node tests instantiate the real WASM and assert exact
  law strings (Dangling/Empty + DANGLING_E2O diagnostic), 6/6 pass. NOT-YET: locality
  law wiring (see V17 falsifier entry above).
- 2026-10-12 | ALIVE | V1-FIX | loss accounting: `ocel_to_log_with_policy` +
  `OcelToLogProjection` (impl Project) + itemized `OcelToLogLoss`; `ocel_to_log`
  deprecated wrapper. cargo check/lib/doc ocel all exit 0.
- 2026-10-12 | OPEN | V17 verification — LOCALITY GAP | `validate_gianola_2026_locality`
  (src/ocel.rs:855-905) is never called on the admission path:
  `LinkedOcel::admit` (src/ocel.rs:1000-1014) delegates solely to
  `OcelLog::validate` (src/ocel.rs:803-813), so only `EmptyEventObjectLinks` and
  `DanglingEventObjectLink` can fire; `admit_ocel_v2_json` (src/ocel.rs:1181-1194)
  never reaches the locality law. Note: `ocel_v2_mutation_locality.json` DOES
  encode a true violation per the algorithm (p2's team parent moves t1→t2 across
  e1/e2, all refs "team"-type — the Example 4 shape); it is the enforcement wiring
  that is missing, not the fixture. Acceptance item 4 ("mutated/malformed → typed
  refusal with the exact named law asserted") is unmeetable for the locality law
  until wired.
- 2026-10-12 | ALIVE | lane W1 — LOCALITY WIRED (supersedes the NOT-YET status of
  the LOCALITY GAP entry above; the wiring note remains accurate that admit does
  not call the locality check — that separation is by design) | crate fn
  `validate_ocel_locality_json` landed in src/ocel.rs (doctest passing); the wasm
  component exports `validate-ocel-locality`; the component smoke test asserts
  the `ViolatesLocalityPrinciple` refusal on the mutation fixture. The locality
  gate is deliberately separate/opt-in from admit: `admitOcelJson` still admits
  the locality-violating fixture (structural laws only), while the dedicated
  `validateOcelLocality` export refuses it with the exact named law. npm tests
  8/8.
- 2026-10-12 | OPEN | V17 verification — TIME-SHAPE GAP | three surfaces disagree
  on OCEL event time. Crate serde: `OCELEvent.time: DateTime<FixedOffset>`
  (src/ocel.rs:68-77) — RFC3339 string only (see admit docstring example,
  src/ocel.rs:1172). TS fixtures: epoch-millis numbers
  (`test/fixtures/ocel/*.json`, `zod_integration/sample_ocel.json`). Zod:
  generated `bindings/zod_schemas.ts:37149` emits `"time": z.any()` (schema is
  generated from the unrelated `OcelEvent { timestamp_ns: u64 }` struct,
  src/ocel.rs:607-613); hand-written `zod_integration/zod_schemas.ts:385,457`
  emits `z.union([z.number(), z.string()])`. DECISION: standardize on RFC3339
  strings end-to-end — it is the only shape the crate admits, OCEL 2.0/XES
  convention, and unambiguous across FFI. Fixtures' epoch millis must be
  converted; after regen the zod schema must assert `z.string()` (a union or
  `z.any()` would silently admit values the wasm parse refuses as
  `ocel-v2:parse:invalid-json`). Until converted, every current fixture fails
  `admit_ocel_v2_json` at Parse — meaning the structural-pass claims in
  `test/ocel-v2-fixture.test.ts` never touch the wasm boundary at all.
