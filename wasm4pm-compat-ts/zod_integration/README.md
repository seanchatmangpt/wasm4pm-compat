# wasm4pm-compat Zod Integration Example

This directory contains a full example of how to use **auto-generated Zod schemas** to validate process mining evidence in TypeScript.

## Architecture

1.  **Source of Truth**: The `ggen/ontology/` directory defines the structural laws of process evidence (Rust types).
2.  **Generation**: The `ggen` tool queries the ontology and renders Zod schemas into `bindings/zod_schemas.ts`.
3.  **Validation**: Frontend applications use these Zod schemas to ensure data received from APIs or WASM boundaries conforms to the `wasm4pm-compat` standard.

## Generated Schemas

The following types are exported from `bindings/zod_schemas.ts`:
- `EventLog`, `Trace`, `Event`
- `OcelLog`, `OcelEvent`, `OcelObject`
- `PetriNet`, `Place`, `Transition`, `Arc`

## Setup

```bash
# From the project root, ensure schemas are generated
# (This uses the ggen minimal manifest for Zod)
ggen sync --manifest ggen_zod.toml

# Navigate to the example
cd examples/zod_integration

# Install dependencies
npm install

# Run the demo
npm start
```

## Why Zod?

While TypeScript provides compile-time safety, **Zod provides runtime safety**. 
In process mining, when data crosses boundaries (e.g., from a Python backend to a Web frontend), we must ensure that the structural invariants (like having at least one object reference in an OCEL event) are maintained. 

By generating Zod schemas directly from the same ontology that drives the Rust core, we achieve **End-to-End Type Law Enforcement**.
