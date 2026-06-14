import { getModules } from "@/lib/project-data";

// RSC: reads src/*.rs at render time and counts pub items.
// No hardcoded numbers — the table reflects the actual crate state.
export default async function ModulesPage() {
  const modules = await getModules();
  const totalPub = modules.reduce((s, m) => s + m.pubCount, 0);

  return (
    <div>
      <h1 className="text-2xl font-bold text-zinc-100 mb-1">Source Modules</h1>
      <p className="text-sm text-zinc-500 mb-6">
        {modules.length} modules in{" "}
        <span className="font-mono text-zinc-400">src/</span> with at least one
        public item — {totalPub} total public declarations. Derived from{" "}
        <span className="font-mono text-zinc-400">grep &apos;^pub &apos; src/*.rs</span>.
      </p>
      <div className="overflow-x-auto">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b border-zinc-800 text-left text-zinc-500">
              <th className="pb-2 font-medium">module</th>
              <th className="pb-2 font-medium text-right">pub items</th>
            </tr>
          </thead>
          <tbody>
            {modules.map((m, i) => (
              <tr
                key={m.name}
                className={`border-b border-zinc-900 ${i % 2 === 0 ? "bg-zinc-950" : "bg-zinc-900/30"}`}
              >
                <td className="py-1.5 pr-4 font-mono text-emerald-400">{m.name}</td>
                <td className="py-1.5 text-right font-mono text-zinc-300">{m.pubCount}</td>
              </tr>
            ))}
          </tbody>
          <tfoot>
            <tr className="border-t border-zinc-700">
              <td className="pt-2 text-zinc-500">total</td>
              <td className="pt-2 text-right font-mono font-bold text-zinc-200">{totalPub}</td>
            </tr>
          </tfoot>
        </table>
      </div>
    </div>
  );
}
