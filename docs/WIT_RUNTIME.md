# WIT → WASM Component → TypeScript Runtime Path

Status: **PROVEN end-to-end (minimal slice).** A real `wasm4pm-compat` function was
called from JavaScript/TypeScript via a WebAssembly Component and returned real
values. This closes the "logic, not just Zod shapes, callable from TS" gap for one
function and documents the exact recipe to widen it.

## What exists today

1. **Zod shape surface** (already shipped): `wasm4pm-compat-ts/` is now a real
   package (`@wasm4pm/compat-ts`, has `package.json`) re-exporting 49 ggen-generated
   Zod schemas from `bindings/zod_schemas.ts`. This is *shape law only* — no logic.

2. **WIT interface** (this work): `ggen/wit/compat-all.wit` — a concrete,
   `wasm-tools`-valid WIT rendered from `ggen/templates/wasm4pm-compat.wit.tera`
   (FEATURES = formats+strict+wasm4pm, VERSION = 1.0.0). Declares interfaces
   `types`, `admission`, `loss`, `strict`, `graduation`, `witness-metadata` and the
   `compat-all` world.

3. **Runtime proof component** (this work): the sibling crate
   `../wasm4pm-compat-component/` exports ONE real function —
   `check-strict-boundary`, backed verbatim by
   `wasm4pm_compat::strict::ProcessBoundary::check()` — as a WASM component, jco-
   transpiled and called from `smoke-test.mjs`. Observed output:
   `checkStrictBoundary(fully-attested) -> true` and a missing-loss-policy boundary
   throws `["MissingLossPolicy"]`.

## Why the .wit.tera could not be rendered by `ggen sync`

- No ggen manifest wires `templates/wasm4pm-compat.wit.tera` to any rule (it is not
  in `ggen.toml`, `ggen-minimal.toml`, etc.).
- It consumes an **unbound `FEATURES` Tera variable** with a `.contains(...)` filter;
  no paired SPARQL query produces it. It was authored as a documentation artifact.
- Its bodies are also pseudo-WIT: they use inline anonymous records inside `variant`
  cases and reserved field names (`type`, `from`) that real WIT rejects.
- `ggen/wit/compat-all.wit` is therefore **hand-rendered** from the template with the
  WIT-syntax repairs noted in its header (named the anonymous records; `%`-escaped
  reserved identifiers). It validates: `wasm-tools component wit ggen/wit/compat-all.wit`.

## Architecture note: why a sidecar wrapper crate

`wasm4pm-compat` is `crate-type = ["rlib"]` and holds the invariants "exactly three
public Cargo features" + "no runtime dependencies". A component needs `cdylib` +
`wit-bindgen`. So — mirroring the `wasm4pm-compat-ts` specta sidecar pattern — the
component lives in a separate crate `../wasm4pm-compat-component/` that depends on
compat (`features = ["strict"]`) rather than polluting the core crate.

## Reproduce / widen the runtime path

Tooling (all installed cleanly on this machine, no drama):

```bash
rustup target add wasm32-wasip1
cargo install wit-bindgen-cli            # 0.58.0
npm i -g @bytecodealliance/jco           # 1.24.3 (brings wasm-tools too)
```

Build + componentize + transpile + run (from `../wasm4pm-compat-component/`):

```bash
# 1. core module (wit-bindgen generate! macro embeds the world from wit/world.wit)
cargo build --release --target wasm32-wasip1

# 2. lift core module to a Component (reactor WASI adapter bundled with jco)
ADAPTER=$(npm root -g)/@bytecodealliance/jco/lib/wasi_snapshot_preview1.reactor.wasm
wasm-tools component new \
  target/wasm32-wasip1/release/wasm4pm_compat_component.wasm \
  --adapt wasi_snapshot_preview1=$ADAPTER -o component.wasm
wasm-tools validate component.wasm

# 3. transpile to JS/TS bindings
jco transpile component.wasm -o dist

# 4. WASI shim for the reactor imports, then run the smoke test
npm i @bytecodealliance/preview2-shim
node smoke-test.mjs
# -> fully-attested exports-format -> true
# -> export w/o loss policy -> threw: true violations: ["MissingLossPolicy"]
# -> SMOKE TEST PASSED
```

## To extend to more functions

For each compat function you want callable from TS:
1. Add it to `wit/world.wit` (mirror the Rust signature; avoid reserved WIT
   identifiers — escape with `%` or rename).
2. Implement the `Guest` trait method in `src/lib.rs`, delegating to the real
   compat function (never reimplement logic).
3. Rebuild → `wasm-tools component new` → `jco transpile`.

Higher-leverage next targets (pure / structure-only, easy WIT shapes):
- `witness-metadata`: `get-witness-info` / `list-all-witnesses` (pure const lookups
  over the ~41-witness registry in `src/witness.rs`).
- `admission`: `admit-*-log` (returns `result<log, refusal-reason>`).

## Open items / honest limits

- Only the `strict` slice is proven at runtime. The `engine` world in the template
  (discovery/replay/conformance/receipts) describes the **wasm4pm engine** boundary
  that compat *imports* — those functions are NOT in the compat crate and were
  intentionally excluded from `compat-all.wit`'s exports.
- The component is a WASI **reactor** (pulls in `wasi:cli`/`wasi:filesystem` imports
  via the adapter) — heavier than a pure component. A `wasm32-unknown-unknown` +
  no-WASI build could shrink it, but wasip1 was the frictionless path here.
- Neither the component crate nor `dist/` is published; this is a proof + recipe.

## Surface drift ledger (2026-10-12)

- `ggen/wit/compat-all.wit`'s `strict` interface was refreshed to mirror the
  **current** `src/strict.rs` surface (V11 divergence closed): `boundary-kind` is
  now the full 9-variant enum (adds `claims-replay`,
  `claims-process-mining-support`, `claims-certification-coverage` — was
  `kind: string`), `process-boundary` carries all 11 attestation fields (adds
  `has-refusal-path`, `has-conformance-fields`, `has-receipt-shape`,
  `has-certification-mapping`, `exports-raw-evidence`, `hidden-pm-growth`;
  `has-round-trip` renamed `has-round-trip-fixture`), and `check-strict-boundary`
  returns `result<bool, list<string>>` — the full set of named law strings, not a
  single 4-variant `strict-violation`. File remains **hand-maintained** (same
  provenance as above); `wasm-tools component wit ggen/wit/compat-all.wit` exits 0
  after the refresh.
- The component implements world `wasm4pm:compat-proof` (in the sibling crate
  `../wasm4pm-compat-component/wit/world.wit`), exporting two interfaces:
  `strict` (check-strict-boundary) and `ocel-v2` (parse/validate/admit). Not the
  `compat-all` world — the component is the minimal runtime-proof slice.
- `ggen/wit/compat-ocel.wit` is the OCEL reference WIT for the `ocel-v2` surface.
- The old `ggen/templates/wasm4pm-compat.wit.tera` template remains unwired and
  unrenderable (unbound `FEATURES` Tera variable with a `.contains(...)` filter —
  the Tera defect noted above); `compat-all.wit` stays hand-rendered.
- Building the component from `../wasm4pm-compat-component/` requires
  `RUSTUP_TOOLCHAIN=nightly-2026-06-22` at build time (the compat crate is
  nightly-only; pin the toolchain env var for the wasm32-wasip1 build).
