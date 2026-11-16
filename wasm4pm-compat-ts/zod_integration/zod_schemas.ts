// Pure re-export shim. The canonical ggen-generated schema surface lives at
// ../bindings/zod_schemas.ts — this directory maintains no local definitions.
//
// History: EventLogSchema/PetriNetSchema were locally composed here because the
// generator emitted per-type schemas only. As of the 2026-10-12 render the
// canonical file exports both directly (EventLogSchema at bindings line ~39755,
// PetriNetSchema at ~41231), so local composition would shadow the canonical
// definitions. Kept as a shim only for the demo imports in this directory.

export * from '../bindings/zod_schemas';
