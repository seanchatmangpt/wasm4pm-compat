/**
 * @wasm4pm/compat-wasm — jco-generated TypeScript wrappers over the
 * wasm4pm compat-ocel WebAssembly component.
 *
 * What this IS: typed structure + named refusals over the OCEL v2 surface
 * (parse / validate / structural admission), compiled from Rust to WASM
 * and transpiled to JS by `jco transpile` (see `dist/`).
 *
 * What this is NOT: no engine logic (discovery, conformance checking,
 * replay, alignment). Those graduate to wasm4pm.
 *
 * The generated module (`dist/compat-ocel.js`, types at
 * `dist/compat-ocel.d.ts`) exports two namespaces:
 *
 * - `ocelV2` — `parseOcelJson` / `validateOcelJson` / `admitOcelJson`
 *   over the real Rust `wasm4pm_compat::ocel` entry points.
 * - `strict` — `checkStrictBoundary` over
 *   `strict::ProcessBoundary::check`.
 *
 * `OcelV2Surface` is derived from the generated types (`typeof ocelV2`),
 * so there is exactly ONE declaration of the surface — the generated one.
 */

import { ocelV2 } from './dist/compat-ocel.js';

/** The stable OCEL v2 contract, derived from the jco-generated types. */
export type OcelV2Surface = typeof ocelV2;

export * from './dist/compat-ocel.js';
