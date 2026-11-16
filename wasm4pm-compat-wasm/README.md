# @wasm4pm/compat-wasm

jco-generated TypeScript wrappers over the wasm4pm `compat-ocel`
WebAssembly component: OCEL v2 parse, validate, and structural admission
from TypeScript/Node, version 26.10.12.

## What this IS

Typed structure and named refusals over the OCEL v2 surface —
`parseOcelJson`, `validateOcelJson`, `admitOcelJson` — compiled from Rust
to a WebAssembly component, then transpiled to JS + core Wasm by
`jco transpile` into `dist/` (shipped in the package).

## What this is NOT

No engine logic. This package carries process-evidence **structure and
named refusals only** — discovery, conformance checking, replay, and
alignment do not belong here; those graduate to `wasm4pm`. The one-way
lifecycle (`Raw → Parsed → Admitted → Refused`) is preserved across the
boundary: admission failures surface as specific named-law refusals,
never bare `InvalidInput`.

## Usage

```ts
import { ocelV2 } from '@wasm4pm/compat-wasm';

const normalized = ocelV2.parseOcelJson(ocelJson);
const violations = ocelV2.validateOcelJson(ocelJson);
// violations: { code, message }[] — empty means structurally valid
const admitted = ocelV2.admitOcelJson(ocelJson); // throws named law on refusal
```

The jco-generated binding lives at `dist/compat-ocel.js` (types at
`dist/compat-ocel.d.ts`), exposed as `@wasm4pm/compat-wasm/compat-ocel`
and re-exported from the package root. `OcelV2Surface` in `index.ts` is
derived from the generated types — one declaration, no drift.

## Layout

- `index.ts` — public re-exports + `OcelV2Surface` (derived)
- `dist/` — jco-generated wrapper: `compat-ocel.js`, `compat-ocel.d.ts`,
  core Wasm modules, `interfaces/`, and the component
  (`compat-ocel.wasm`)
- `test/` — `node:test` suite instantiating the real component

## Verification

```bash
npm install
npm test   # tsc --noEmit typecheck, then node --test against the real WASM
```

## Reference

Requirements and design:
[`../docs/jira/v26.10.12/000-ard-prd.md`](../docs/jira/v26.10.12/000-ard-prd.md).
