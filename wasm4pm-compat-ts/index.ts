// Public entry point for @wasm4pm/compat-ts.
//
// Re-exports the ggen-generated Zod schemas (and their inferred TypeScript
// types) for the wasm4pm-compat domain type law. The bindings file is
// author-time generated from `wasm4pm-compat/ggen` — see ./bindings/zod_schemas.ts.
export * from './bindings/zod_schemas.ts';
