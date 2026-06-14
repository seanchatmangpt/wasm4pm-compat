import { Suspense } from "react";
import { CoverageStats } from "./_components/CoverageStats";
import { SchemaCatalog } from "./_components/SchemaCatalog";
import { ExampleMap } from "./_components/ExampleMap";
import { ModuleList } from "./_components/ModuleList";

// Each section has its own Suspense boundary so they stream independently.
// Slow I/O in one section (e.g. large schema file parse) does not block others.

function SectionSkeleton({ rows = 6 }: { rows?: number }) {
  return (
    <div className="animate-pulse space-y-2">
      {Array.from({ length: rows }).map((_, i) => (
        <div key={i} className="h-8 bg-zinc-800 rounded" style={{ width: `${60 + (i % 3) * 15}%` }} />
      ))}
    </div>
  );
}

function StatsSkeleton() {
  return (
    <div className="animate-pulse grid grid-cols-3 gap-4">
      {[0, 1, 2].map((i) => (
        <div key={i} className="h-20 bg-zinc-800 rounded-lg" />
      ))}
    </div>
  );
}

export default function Home() {
  return (
    <main className="max-w-6xl mx-auto w-full px-6 py-12 space-y-12">

      {/* Header — static, renders instantly */}
      <header>
        <div className="flex items-center gap-3 mb-2">
          <span className="font-mono text-xs bg-orange-900/40 text-orange-400 border border-orange-800 px-2 py-0.5 rounded">
            nightly-only
          </span>
          <span className="font-mono text-xs bg-red-900/40 text-red-400 border border-red-800 px-2 py-0.5 rounded">
            forbid(unsafe_code)
          </span>
          <span className="font-mono text-xs bg-blue-900/40 text-blue-400 border border-blue-800 px-2 py-0.5 rounded">
            structure-only
          </span>
        </div>
        <h1 className="text-3xl font-bold text-zinc-100 tracking-tight">
          wasm4pm-compat
        </h1>
        <p className="text-zinc-400 mt-2 max-w-2xl">
          Process Intelligence Compatibility Core — v26.6.14. Typed, one-way
          evidence lifecycle for process mining. Every type here is structure;
          no engine logic, no conformance checking, no replay. Graduate to{" "}
          <span className="font-mono text-zinc-300">wasm4pm</span> for execution.
        </p>
        <p className="text-xs text-zinc-600 mt-2 font-mono">
          All numbers below are derived at build time from the actual project
          artifacts — no fixtures.
        </p>
      </header>

      {/* Coverage stats — streams independently */}
      <section>
        <h2 className="text-lg font-semibold text-zinc-200 mb-4">
          Coverage at a Glance
        </h2>
        <Suspense fallback={<StatsSkeleton />}>
          <CoverageStats />
        </Suspense>
      </section>

      {/* Schema catalog — streams independently (parses 891-line file) */}
      <Suspense
        fallback={
          <div>
            <div className="h-6 w-48 bg-zinc-800 rounded mb-4 animate-pulse" />
            <SectionSkeleton rows={8} />
          </div>
        }
      >
        <SchemaCatalog />
      </Suspense>

      {/* Example map — streams independently */}
      <Suspense
        fallback={
          <div>
            <div className="h-6 w-48 bg-zinc-800 rounded mb-4 animate-pulse" />
            <SectionSkeleton rows={6} />
          </div>
        }
      >
        <ExampleMap />
      </Suspense>

      {/* Module list — streams independently */}
      <Suspense
        fallback={
          <div>
            <div className="h-6 w-48 bg-zinc-800 rounded mb-4 animate-pulse" />
            <SectionSkeleton rows={10} />
          </div>
        }
      >
        <ModuleList />
      </Suspense>

      {/* Footer — static */}
      <footer className="border-t border-zinc-800 pt-6 text-xs text-zinc-600">
        <p>
          The product is CodeManufactory; RevOps is merely proof that
          CodeManufactory works.
        </p>
        <p className="mt-1">
          Data source: filesystem reads at render time —{" "}
          <span className="font-mono">src/*.rs</span>,{" "}
          <span className="font-mono">examples/*.rs</span>,{" "}
          <span className="font-mono">wasm4pm-compat-ts/bindings/zod_schemas.ts</span>,{" "}
          <span className="font-mono">DOC_COVERAGE_LOG.md</span>
        </p>
      </footer>
    </main>
  );
}
