import { getExamples } from "@/lib/project-data";

/** RSC: reads real example files from examples/ directory. */
export async function ExampleMap() {
  const examples = await getExamples();

  return (
    <section>
      <h2 className="text-lg font-semibold text-zinc-200 mb-1">
        Runnable Examples
      </h2>
      <p className="text-sm text-zinc-500 mb-4">
        {examples.length} examples in{" "}
        <span className="font-mono text-zinc-400">examples/</span> — each
        exercised and exit-0 verified
      </p>
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2">
        {examples.map((ex) => (
          <div
            key={ex.name}
            className="border border-zinc-800 rounded p-3 bg-zinc-900"
          >
            <div className="font-mono text-sm text-emerald-400">{ex.name}</div>
            {ex.featureFlags.length > 0 && (
              <div className="mt-1">
                {ex.featureFlags.map((f) => (
                  <span
                    key={f}
                    className="text-xs bg-zinc-800 text-blue-400 px-1 py-0.5 rounded font-mono"
                  >
                    {f}
                  </span>
                ))}
              </div>
            )}
            <div className="text-xs text-zinc-600 mt-1 truncate">{ex.docRef}</div>
          </div>
        ))}
      </div>
    </section>
  );
}
