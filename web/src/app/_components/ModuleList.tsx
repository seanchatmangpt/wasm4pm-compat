import { getModules } from "@/lib/project-data";

/** RSC: reads real Rust module pub-item counts from src/. */
export async function ModuleList() {
  const modules = await getModules();

  const maxCount = Math.max(...modules.map((m) => m.pubCount));

  return (
    <section>
      <h2 className="text-lg font-semibold text-zinc-200 mb-1">
        Rust Modules
      </h2>
      <p className="text-sm text-zinc-500 mb-4">
        {modules.length} modules in{" "}
        <span className="font-mono text-zinc-400">src/</span> with public API —
        bar width = pub item count
      </p>
      <div className="space-y-1">
        {modules.map((m) => (
          <div key={m.name} className="flex items-center gap-2">
            <span className="font-mono text-xs text-zinc-400 w-40 shrink-0 truncate">
              {m.name}
            </span>
            <div className="flex-1 h-4 bg-zinc-900 rounded overflow-hidden">
              <div
                className="h-full bg-violet-700 rounded"
                style={{ width: `${(m.pubCount / maxCount) * 100}%` }}
              />
            </div>
            <span className="text-xs text-zinc-600 w-6 text-right">
              {m.pubCount}
            </span>
          </div>
        ))}
      </div>
    </section>
  );
}
