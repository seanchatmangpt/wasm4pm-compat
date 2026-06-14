import { getSchemas, type SchemaEntry } from "@/lib/project-data";

function SchemaCard({ schema }: { schema: SchemaEntry }) {
  return (
    <div className="border border-zinc-800 rounded p-3 bg-zinc-900 hover:border-zinc-600 transition-colors">
      <div className="font-mono text-sm font-semibold text-amber-400">
        {schema.name}
      </div>
      <div className="text-xs text-zinc-500 mt-1">
        {schema.fields.length} field{schema.fields.length !== 1 ? "s" : ""}
      </div>
      {schema.fields.length > 0 && (
        <div className="mt-2 flex flex-wrap gap-1">
          {schema.fields.slice(0, 4).map((f) => (
            <span
              key={f}
              className="text-xs bg-zinc-800 text-zinc-400 px-1 py-0.5 rounded font-mono"
            >
              {f}
            </span>
          ))}
          {schema.fields.length > 4 && (
            <span className="text-xs text-zinc-600">
              +{schema.fields.length - 4}
            </span>
          )}
        </div>
      )}
    </div>
  );
}

/** RSC: reads real Zod schema bindings from wasm4pm-compat-ts/bindings/zod_schemas.ts */
export async function SchemaCatalog() {
  const schemas = await getSchemas();

  return (
    <section>
      <h2 className="text-lg font-semibold text-zinc-200 mb-1">
        TypeScript Schemas
      </h2>
      <p className="text-sm text-zinc-500 mb-4">
        {schemas.length} Zod schemas generated from{" "}
        <span className="font-mono text-zinc-400">
          wasm4pm-compat-ts/bindings/zod_schemas.ts
        </span>
      </p>
      <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-2">
        {schemas.map((s) => (
          <SchemaCard key={s.name} schema={s} />
        ))}
      </div>
    </section>
  );
}
