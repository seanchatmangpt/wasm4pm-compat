import { Suspense } from "react";
import { readFile } from "fs/promises";
import { join } from "path";
import { getCoverageStatus, getModules, getExamples } from "@/lib/project-data";

// Partial Prerendering: this page opts in via cacheComponents (formerly ppr).
// The static shell (title, description) is rendered at build time.
// CoverageBody is wrapped in Suspense — it streams in at request time
// because it reads DOC_COVERAGE_LOG.md and emitted/gap-ledger.yaml dynamically.
// See: next.config.ts experimental.cacheComponents = true

// RSC: derives every number from real project artifacts.
// DOC_COVERAGE_LOG.md is the authoritative source for bijective coverage status.
// emitted/gap-ledger.yaml is the authoritative source for gap classification.

const CRATE_ROOT = join(process.cwd(), "..");

interface GapEntry {
  id: string;
  name: string;
  classification: string;
  severity?: string;
}

async function getGapLedger(): Promise<GapEntry[]> {
  try {
    const content = await readFile(
      join(CRATE_ROOT, "emitted", "gap-ledger.yaml"),
      "utf-8"
    );
    // Parse gap entries: each starts with `  - id: GAP_\d+`
    const gaps: GapEntry[] = [];
    const blocks = content.split(/(?=  - id: GAP_)/);
    for (const block of blocks) {
      const idMatch = block.match(/id:\s*(\S+)/);
      const nameMatch = block.match(/name:\s*"([^"]+)"/);
      const classMatch = block.match(/classification:\s*(\S+)/);
      const sevMatch = block.match(/severity:\s*(\S+)/);
      if (!idMatch || !nameMatch || !classMatch) continue;
      gaps.push({
        id: idMatch[1],
        name: nameMatch[1],
        classification: classMatch[1],
        severity: sevMatch?.[1],
      });
    }
    return gaps;
  } catch {
    return [];
  }
}

async function getCoverageLog(): Promise<{
  bijectiveComplete: boolean;
  closedCount: number;
  iterationCount: number;
  lastIteration: string;
}> {
  const content = await readFile(
    join(CRATE_ROOT, "DOC_COVERAGE_LOG.md"),
    "utf-8"
  );
  const bijectiveComplete = content.includes("BIJECTIVE COVERAGE STATUS: COMPLETE");
  const closedCount = (content.match(/✅/g) || []).length;
  const iterations = content.match(/^## Iteration \d+/gm) || [];
  const lastIteration = iterations[iterations.length - 1] ?? "—";
  return {
    bijectiveComplete,
    closedCount,
    iterationCount: iterations.length,
    lastIteration: lastIteration.replace("## ", ""),
  };
}

async function CoverageBody() {
  const [coverage, modules, examples, gaps, log] = await Promise.all([
    getCoverageStatus(),
    getModules(),
    getExamples(),
    getGapLedger(),
    getCoverageLog(),
  ]);

  const classColor: Record<string, string> = {
    CLOSED: "text-emerald-400 border-emerald-900 bg-emerald-950/30",
    SEALED: "text-violet-400 border-violet-900 bg-violet-950/30",
    AUTHORIZED: "text-blue-400 border-blue-900 bg-blue-950/30",
    PARTIAL: "text-amber-400 border-amber-900 bg-amber-950/30",
    UNRESOLVED: "text-red-400 border-red-900 bg-red-950/30",
  };

  return (
    <>
      {/* Bijective status header */}
      <div
        className={`border rounded-lg p-5 mb-8 ${
          log.bijectiveComplete
            ? "border-emerald-800 bg-emerald-950/20"
            : "border-amber-800 bg-amber-950/20"
        }`}
      >
        <div className="flex items-center gap-3 mb-2">
          <span
            className={`text-2xl font-bold font-mono ${
              log.bijectiveComplete ? "text-emerald-400" : "text-amber-400"
            }`}
          >
            {log.bijectiveComplete ? "BIJECTIVE COVERAGE: COMPLETE ✓" : "BIJECTIVE COVERAGE: OPEN"}
          </span>
        </div>
        <p className="text-sm text-zinc-400">
          Source: <span className="font-mono text-zinc-300">DOC_COVERAGE_LOG.md</span>
          {" "}· {log.iterationCount} iterations · last: {log.lastIteration}
        </p>
        <div className="grid grid-cols-3 gap-4 mt-4">
          <Stat value={log.closedCount} label="modules closed" color="text-emerald-400" />
          <Stat value={coverage.totalExamples} label="running examples" color="text-amber-400" />
          <Stat value={modules.reduce((s, m) => s + m.pubCount, 0)} label="total pub items" color="text-violet-400" />
        </div>
      </div>

      {/* Module × example coverage table */}
      <section className="mb-10">
        <h2 className="text-sm font-semibold text-zinc-500 uppercase tracking-wider mb-3">
          Module coverage ({modules.length} modules, {coverage.totalExamples} examples)
        </h2>
        <div className="overflow-x-auto">
          <table className="w-full text-xs font-mono">
            <thead>
              <tr className="border-b border-zinc-800 text-left text-zinc-600">
                <th className="pb-2 font-medium">module</th>
                <th className="pb-2 font-medium text-right">pub</th>
                <th className="pb-2 pl-4">example</th>
              </tr>
            </thead>
            <tbody>
              {modules.map((m) => {
                const ex = examples.find(
                  (e) =>
                    e.name.includes(m.name.replace(/_/g, "")) ||
                    e.name === m.name ||
                    e.name.startsWith(m.name)
                );
                return (
                  <tr key={m.name} className="border-b border-zinc-900">
                    <td className="py-1 pr-4 text-emerald-400">{m.name}</td>
                    <td className="py-1 text-right text-zinc-500">{m.pubCount}</td>
                    <td className="py-1 pl-4">
                      {ex ? (
                        <a
                          href={`/examples/${ex.name}`}
                          className="text-blue-400 hover:underline"
                        >
                          {ex.name}
                        </a>
                      ) : (
                        <span className="text-zinc-700">—</span>
                      )}
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      </section>

      {/* Gap ledger */}
      {gaps.length > 0 && (
        <section className="mb-8">
          <h2 className="text-sm font-semibold text-zinc-500 uppercase tracking-wider mb-3">
            Gap ledger — <span className="font-mono text-zinc-400">emitted/gap-ledger.yaml</span>
          </h2>
          <div className="space-y-2">
            {gaps.map((g) => (
              <div
                key={g.id}
                className={`border rounded px-3 py-2 flex items-baseline gap-3 text-sm ${
                  classColor[g.classification] ?? "text-zinc-400 border-zinc-800"
                }`}
              >
                <span className="font-mono font-bold shrink-0">{g.id}</span>
                <span className="flex-1 text-zinc-300 text-xs">{g.name}</span>
                <span className="font-mono text-xs shrink-0">{g.classification}</span>
                {g.severity && (
                  <span className="text-xs text-zinc-600 shrink-0">{g.severity}</span>
                )}
              </div>
            ))}
          </div>
          <p className="text-xs text-zinc-700 mt-2">
            Source: <span className="font-mono">emitted/gap-ledger.yaml</span> — parsed at render time
          </p>
        </section>
      )}
    </>
  );
}

function Stat({
  value,
  label,
  color,
}: {
  value: number;
  label: string;
  color: string;
}) {
  return (
    <div>
      <div className={`text-2xl font-bold font-mono ${color}`}>{value}</div>
      <div className="text-xs text-zinc-500">{label}</div>
    </div>
  );
}

export default function CoveragePage() {
  return (
    <div className="max-w-4xl">
      <h1 className="text-2xl font-bold text-zinc-100 mb-1">Coverage Map</h1>
      <p className="text-sm text-zinc-500 mb-6">
        Bijective doc↔example coverage status derived from{" "}
        <span className="font-mono text-zinc-400">DOC_COVERAGE_LOG.md</span>{" "}
        and gap classification from{" "}
        <span className="font-mono text-zinc-400">emitted/gap-ledger.yaml</span>.
        Every number is read from the real artifacts — no fixtures.
      </p>
      <Suspense
        fallback={
          <div className="animate-pulse space-y-4">
            <div className="h-32 bg-zinc-900 rounded-lg" />
            <div className="h-64 bg-zinc-900 rounded-lg" />
          </div>
        }
      >
        <CoverageBody />
      </Suspense>
    </div>
  );
}
