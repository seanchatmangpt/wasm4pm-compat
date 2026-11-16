# v26.10.12 lane leases — coordinator-owned fan-out

Two-wave same-checkout fan-out. Wave 1 = read-only Explore lanes (edit plans only).
Wave 2 lanes get disjoint file ownership assigned by the coordinator after conflict
resolution. Agents never run git state commands; coordinator owns all transitions.

## Wave 1 (read-only) — LANDED

| Lane | Key deliverable |
|---|---|
| L1 | OCEL Rust export surface + gap list (src/ocel.rs, src/ocel/*) |
| L2 | ggen Zod chain edit plan (ggen/, wasm4pm-compat-ts/bindings) |
| L3 | compat-ocel WIT world plan (frozen seam text for W-C/W-D/W-E) |
| L4 | Component crate recipe + OCEL edit plan (../wasm4pm-compat-component) |
| L5 | @wasm4pm/compat-wasm package scaffolding plan |
| L6 | OCEL v2 fixtures + TS test plan |
| L7 | npm-release.yml content plan |
| L8 | Version bump inventory 26.10.12 (Cargo.toml/lock, package.jsons, mix.exs) |
| L9 | Fence audit procedure (no engine logic in TS) |
| L10 | ggen render dry-run proof (/tmp only) |

## Wave 2 — implementation lanes — LANDED

| Lane | Key deliverable |
|---|---|
| W-A | Cargo.toml/lock + all package.json version bumps (sole writer) |
| W-B | ggen TTL/query/template Zod OCEL v2 extension + regenerated bindings |
| W-B2 | Zod schema type strengthening (bindings regenerated with tighter types) |
| W-C | ggen/wit/compat-ocel.wit + template wiring |
| W-D | ../wasm4pm-compat-component OCEL exports |
| W-D2 | Component export follow-up fixes |
| W-E | compat-wasm/ new package dir |
| W-E2 | real jco binding landing (compiled component bindings into compat-wasm/) |
| W-F | wasm4pm-compat-ts tests + fixtures |
| W-G | .github/workflows/npm-release.yml |
| W-H2 | protocol test repair (wasm4pm-compat-ts protocol tests) |

## Publish lanes — LANDED (all, incl. P18)

| Lane | Key deliverable |
|---|---|
| P1 | cargo metadata audit (publish-required fields complete) |
| P2 | package file-set audit (files list matches shipped surface) |
| P3 | publish dry-run (`cargo publish --dry-run` / `npm pack --dry-run`) |
| P4 | rustdoc build audit (docs build clean, all-features) |
| P5 | license/readme inclusion audit (LICENSE-MIT + README in package) |
| P6 | examples compile/run audit |
| P7 | npm pack audit pass 1 (tarball contents) |
| P8 | npm pack audit pass 2 (post-fix re-check) |
| P9 | package metadata polish (description/keywords/repo links) |
| P10 | metadata polish follow-up |
| P11 | version sweep 26.10.12 across Cargo/package.json/mix.exs |
| P12 | fence audit (no engine logic leaked past compat boundary) |
| P13 | workflow lint (npm-release.yml actionlint/validity) |
| P14 | changelog audit (26.10.12 entry complete) |
| P15 | crates.io docs readiness (docs.rs build surface) |
| P16 | tarball consumer smoke (install packed tarball in scratch project) |
| P17 | elixir binding sanity (mix compile/test in bindings/elixir) |
| P18 | ALIVE gate (trybuild type-law receipts) — LANDED |

## Verifier lanes — LANDED

| Lane | Key deliverable |
|---|---|
| V1 | OCEL additions audit (src/ocel.rs + src/ocel/* surface check) |
| V3 | bindings inventory (TS/Elixir/component binding completeness) |
| V4 | wire-shape verification (JSON-string WIT payload shapes) |
| V5 | workflow reconciliation (npm-release.yml vs pipeline stages) |
| V9 | protocol hazard review (TS protocol tests) |
| V10 | ggen manifest audit (ggen.toml rules/templates) |
| V11 | component fidelity (wasm4pm-compat-component exports) |
| V12 | Rust test census (full test inventory, compile-abort vs failure) |
| V13 | TS test census (wasm4pm-compat-ts) |
| V14 | fence re-audit (post-fix boundary check) |
| V15 | link sweep (docs cross-references) |
| V16 | build census (feature-matrix build inventory) |
| V17 | gap drafts (deferred/known-gap documentation) |
| V1-FIX | V1 defect fix lane |
| V15-FIX | V15 defect fix lane |

## Wave 3 — LANDED

| Lane | Scope |
|---|---|
| W1–W19 | implementation/dispatch lanes; W19 = lane-build-root cleanup + this ledger — all LANDED |

## Definition-of-done lanes — LANDED

| Lane | Deliverable (verified on disk) |
|---|---|
| D1–D16 | DoD gates; receipts verified: `docs/jira/v26.10.12/003-receipt.md`, `docs/jira/v26.10.12/004-integration-manifest.md`, `scripts/regen-compat-wasm.sh`, audit status clean |

## Wave 4 — IN FLIGHT

| Lane | Scope |
|---|---|
| E1–E16 | next-wave implementation lanes in flight |

## Seam decisions log

- **JSON-string WIT seam**: WIT world carries OCEL v2 payloads as JSON strings across
  the component boundary (W-C/W-D/W-E shared seam); parse/validation lives on the TS
  side via the generated Zod chain, keeping the WIT surface version-stable.
- **RFC3339 time standardization**: all timestamps crossing WIT/TS boundaries are
  RFC3339 strings, not raw integer ns; ns precision retained in Rust, converted at
  the projection edge.
- **Prelude re-export fix**: generated bindings re-exported through the crate/package
  prelude so consumers import from one path; fixed shadowing of provided types.
- **Loss-accounting Project routing**: OCEL→TS projections routed through `Project`
  with `LossPolicy` + `LossReport` (no silent structure loss at the boundary).
- **LICENSE-MIT into npm files**: LICENSE-MIT added to npm package `files` so the
  published tarball ships the license text.

## Lane build-root cleanup law (enforced at integration)

Lane build roots are leases, not assets. Coordinator deletes every lane build root
BEFORE the final integration commit. Regenerable via `mix deps.get` / `cargo build`.

| Root | Size (before) | Status |
|---|---|---|
| bindings/elixir/_build | 81M | **DELETED — coordinator executed (98M → 84K)** |
| bindings/elixir/deps | 17M | **DELETED — coordinator executed (98M → 84K)** |

Both are gitignored and unreferenced by any lane file. No `_build-lane*` or
`MIX_BUILD_ROOT` artifacts exist in the repo. `/tmp` rigs (t1–t5 scratch dirs noted
during scan) are out of repo and out of scope. Cargo `target/` dirs are not lane
build roots (canonical checkout build caches, not lane leases).
