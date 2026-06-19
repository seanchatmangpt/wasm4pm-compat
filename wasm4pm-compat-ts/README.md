# @wasm4pm/compat-ts

TypeScript / [Zod](https://zod.dev) binding surface for the **wasm4pm-compat**
domain type law. These schemas are author-time generated from the compat
ontology via `ggen` (`wasm4pm-compat/ggen`, query `extract-zod-schemas.rq` +
template `zod-schemas.ts.tera`) — see `./bindings/zod_schemas.ts`.

49 exported `*Schema` consts (plus matching `z.infer<>` types), including
`ConformanceResultSchema`, `ConformanceVerdictSchema`, `BpmnProcessSchema`,
`PetriNetSchema`, `ArcSchema`, `CompatDiagnosticSchema`, and the rest of the
compat domain.

Schemas are zod **v3 and v4 compatible** (`z.record` is emitted with explicit
key + value types).

## Install

```bash
pnpm add @wasm4pm/compat-ts zod
```

## Usage

```ts
import { ConformanceResultSchema, type ConformanceResult } from '@wasm4pm/compat-ts';

const result: ConformanceResult = ConformanceResultSchema.parse({
  deviating_traces: 3,
  fitness: 0.97,
  fitting_traces: 97,
  precision: 0.88,
  total_traces: 100,
});
```

## Scripts

```bash
pnpm run typecheck   # tsc --noEmit --strict --skipLibCheck
pnpm test            # typecheck + node --test the parse round-trip
```

## Re-generating from the ontology

These bindings are generated. To re-render, run `ggen sync` over the compat
ggen manifest (or use the self-contained pack `packs/wasm4pm-compat-ts` in the
wasm4pm repo). Do not hand-edit `bindings/zod_schemas.ts` — sync reverts it.
