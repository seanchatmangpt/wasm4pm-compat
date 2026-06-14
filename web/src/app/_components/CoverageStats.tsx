import { getCoverageStatus } from "@/lib/project-data";

/** RSC: derives coverage numbers from real DOC_COVERAGE_LOG.md and filesystem. */
export async function CoverageStats() {
  const { coveredModules, totalExamples, totalSchemas } =
    await getCoverageStatus();

  const stats = [
    {
      label: "Witnessed modules",
      value: coveredModules,
      sub: "EXIT 0 verified",
      color: "text-emerald-400",
    },
    {
      label: "Runnable examples",
      value: totalExamples,
      sub: "in examples/",
      color: "text-amber-400",
    },
    {
      label: "Zod schemas",
      value: totalSchemas,
      sub: "TypeScript bindings",
      color: "text-blue-400",
    },
  ];

  return (
    <div className="grid grid-cols-3 gap-4">
      {stats.map((s) => (
        <div
          key={s.label}
          className="border border-zinc-800 rounded-lg p-4 bg-zinc-900"
        >
          <div className={`text-3xl font-bold font-mono ${s.color}`}>
            {s.value}
          </div>
          <div className="text-sm text-zinc-300 font-medium mt-1">{s.label}</div>
          <div className="text-xs text-zinc-600">{s.sub}</div>
        </div>
      ))}
    </div>
  );
}
