# zod_integration

**Unshipped demo directory.** Not part of the published package — examples and
integration sketches only.

## Schema source of truth

Canonical schemas live at `../bindings/zod_schemas.ts` (ggen-generated from
`zod-types.ttl`). This directory does **not** maintain its own copy: the local
`zod_schemas.ts` is a re-export shim (`export * from '../bindings/zod_schemas'`)
plus the two aggregate schemas the generator does not emit (`EventLogSchema`,
`PetriNetSchema`), composed from canonical parts.

## Wire format

`time` fields are RFC3339 strings (e.g. `2024-06-09T00:00:00Z`), matching the
canonical schema. The hand copy's `z.union([z.number(), z.string()])` variant is
retired — do not reintroduce epoch-millis times in fixtures here.

## Run

The `npm` scripts in `package.json` assume ts-node with the directory's own
deps; they are convenience sketches. Typecheck with:

```bash
npx tsc -p tsconfig.json --noEmit
```

(module resolution walks up to the root `node_modules` for `zod`/`typescript`.)
