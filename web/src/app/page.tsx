import { Suspense } from "react";
import {
  getProjectSummary,
  type ProjectSummary,
} from "@/lib/project";

// All data derived from real project artifacts at build time. No fixtures.

async function OverviewContent() {
  const s: ProjectSummary = getProjectSummary();

  return (
    <>
      {/* Header badges from Cargo.toml */}
      <header className="mb-8">
        <div className="flex flex-wrap gap-2 mb-3">
          <Badge color="orange">nightly-only</Badge>
          <Badge color="red">forbid(unsafe_code)</Badge>
          <Badge color="blue">structure-only</Badge>
          <Badge color="purple">v{s.crate.version}</Badge>
        </div>
        <h1 className="text-3xl font-bold text-zinc-100 tracking-tight">
          {s.crate.name}
        </h1>
        <p className="text-zinc-400 mt-2 max-w-2xl text-sm leading-relaxed">
          {s.crate.description ||
            "Process Intelligence Compatibility Core — typed, one-way evidence lifecycle for process mining."}
        </p>
        <p className="text-xs text-zinc-700 mt-2 font-mono">
          All numbers: live filesystem reads at build time — src/*.rs,
          examples/*.rs, tests/ui/, wasm4pm-compat-ts/bindings/, git log
        </p>
      </header>

      {/* Stats grid */}
      <section className="mb-10">
        <h2 className="text-sm font-semibold text-zinc-500 uppercase tracking-wider mb-4">
          At a glance
        </h2>
        <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-3">
          <StatCard
            value={s.moduleCount}
            label="Rust modules"
            sub="in src/"
            color="text-violet-400"
            href="/modules"
          />
          <StatCard
            value={s.totalPubItems}
            label="pub declarations"
            sub="grep ^pub"
            color="text-violet-300"
            href="/modules"
          />
          <StatCard
            value={s.exampleCount}
            label="examples"
            sub="in examples/"
            color="text-emerald-400"
            href="/examples"
          />
          <StatCard
            value={s.witnessCount}
            label="witness keys"
            sub="witness_corpus.rs"
            color="text-amber-400"
            href="/witnesses"
          />
          <StatCard
            value={s.alive.compileFail}
            label="compile-fail"
            sub="ALIVE fixtures"
            color="text-red-400"
          />
          <StatCard
            value={s.alive.compilePass}
            label="compile-pass"
            sub="ALIVE fixtures"
            color="text-green-400"
          />
        </div>
      </section>

      {/* Features from Cargo.toml */}
      {s.crate.features.length > 0 && (
        <section className="mb-10">
          <h2 className="text-sm font-semibold text-zinc-500 uppercase tracking-wider mb-3">
            Cargo features
          </h2>
          <div className="flex flex-wrap gap-2">
            {s.crate.features.map((f) => (
              <span
                key={f}
                className="font-mono text-xs bg-zinc-900 border border-zinc-700 text-zinc-300 px-2 py-1 rounded"
              >
                {f}
              </span>
            ))}
          </div>
          <p className="text-xs text-zinc-600 mt-2">
            Source:{" "}
            <span className="font-mono">
              Cargo.toml [features] — exactly 3 public features (formats,
              strict, wasm4pm)
            </span>
          </p>
        </section>
      )}

      {/* Witness families */}
      <section className="mb-10">
        <h2 className="text-sm font-semibold text-zinc-500 uppercase tracking-wider mb-3">
          Witness families (
          <a href="/witnesses" className="text-blue-400 hover:underline">
            {s.witnessCount} total
          </a>
          )
        </h2>
        <div className="flex flex-wrap gap-2">
          {Object.entries(s.witnessKeysByFamily)
            .sort((a, b) => b[1] - a[1])
            .map(([family, count]) => (
              <a
                key={family}
                href={`/api/witnesses?family=${family}`}
                className="font-mono text-xs bg-zinc-900 border border-zinc-800 text-zinc-400 hover:text-zinc-200 hover:border-zinc-600 px-2 py-1 rounded transition-colors"
              >
                {family}{" "}
                <span className="text-zinc-600">{count}</span>
              </a>
            ))}
        </div>
      </section>

      {/* ALIVE gate summary */}
      <section className="mb-10">
        <h2 className="text-sm font-semibold text-zinc-500 uppercase tracking-wider mb-3">
          ALIVE gate fixtures
        </h2>
        <div className="flex gap-4 text-sm">
          <div className="border border-zinc-800 rounded p-4 bg-zinc-900 flex-1">
            <div className="text-2xl font-bold font-mono text-red-400">
              {s.alive.compileFail}
            </div>
            <div className="text-zinc-400 mt-1">compile-fail</div>
            <div className="text-xs text-zinc-600">
              tests/ui/compile_fail/*.rs — must fail for named law
            </div>
          </div>
          <div className="border border-zinc-800 rounded p-4 bg-zinc-900 flex-1">
            <div className="text-2xl font-bold font-mono text-green-400">
              {s.alive.compilePass}
            </div>
            <div className="text-zinc-400 mt-1">compile-pass</div>
            <div className="text-xs text-zinc-600">
              tests/ui/compile_pass/*.rs — lawful path must compile
            </div>
          </div>
          <div className="border border-zinc-800 rounded p-4 bg-zinc-900 flex-1">
            <div className="text-2xl font-bold font-mono text-zinc-300">
              {s.alive.total}
            </div>
            <div className="text-zinc-400 mt-1">total</div>
            <div className="text-xs text-zinc-600">
              cargo make alive — run with trybuild
            </div>
          </div>
        </div>
      </section>

      {/* Recent commits — real git log */}
      <section className="mb-10">
        <h2 className="text-sm font-semibold text-zinc-500 uppercase tracking-wider mb-3">
          Recent commits
        </h2>
        <div className="space-y-1">
          {s.recentCommits.map((c) => (
            <div
              key={c.hash}
              className="flex gap-3 items-baseline text-sm border-b border-zinc-900 pb-1"
            >
              <span className="font-mono text-xs text-zinc-600 shrink-0 w-14">
                {c.hash}
              </span>
              <span className="text-zinc-300 flex-1 truncate">{c.subject}</span>
              <span className="text-xs text-zinc-600 shrink-0">{c.date}</span>
            </div>
          ))}
        </div>
      </section>

      {/* Evidence lifecycle invariant */}
      <section className="mb-8">
        <h2 className="text-sm font-semibold text-zinc-500 uppercase tracking-wider mb-3">
          The one-way door
        </h2>
        <pre className="font-mono text-xs bg-zinc-950 border border-zinc-800 rounded p-4 text-zinc-400">
          {`Raw ──parse──▶ Parsed ──admit──▶ Admitted ──▶ { Projected | Exportable | Receipted }
  │                                  ▲
  └────────────── refuse ────────────┴──▶ Refused  (terminal; carries a named law)`}
        </pre>
        <p className="text-xs text-zinc-600 mt-2">
          Source: CLAUDE.md §Architecture — The one-way door
        </p>
      </section>

      <footer className="border-t border-zinc-800 pt-6 text-xs text-zinc-700">
        <p>
          The product is CodeManufactory; RevOps is merely proof that
          CodeManufactory works.
        </p>
      </footer>
    </>
  );
}

function Badge({
  children,
  color,
}: {
  children: React.ReactNode;
  color: "orange" | "red" | "blue" | "purple";
}) {
  const styles = {
    orange:
      "bg-orange-900/40 text-orange-400 border-orange-800",
    red: "bg-red-900/40 text-red-400 border-red-800",
    blue: "bg-blue-900/40 text-blue-400 border-blue-800",
    purple: "bg-violet-900/40 text-violet-400 border-violet-800",
  };
  return (
    <span
      className={`font-mono text-xs border px-2 py-0.5 rounded ${styles[color]}`}
    >
      {children}
    </span>
  );
}

function StatCard({
  value,
  label,
  sub,
  color,
  href,
}: {
  value: number;
  label: string;
  sub: string;
  color: string;
  href?: string;
}) {
  const inner = (
    <div className="border border-zinc-800 rounded-lg p-4 bg-zinc-900 hover:border-zinc-700 transition-colors h-full">
      <div className={`text-2xl font-bold font-mono ${color}`}>{value}</div>
      <div className="text-xs text-zinc-300 font-medium mt-1">{label}</div>
      <div className="text-xs text-zinc-600">{sub}</div>
    </div>
  );
  return href ? (
    <a href={href} className="block">
      {inner}
    </a>
  ) : (
    inner
  );
}

export default function Home() {
  return (
    <Suspense
      fallback={
        <div className="animate-pulse space-y-4">
          <div className="h-8 w-80 bg-zinc-800 rounded" />
          <div className="grid grid-cols-6 gap-3">
            {Array.from({ length: 6 }).map((_, i) => (
              <div key={i} className="h-20 bg-zinc-800 rounded-lg" />
            ))}
          </div>
        </div>
      }
    >
      <OverviewContent />
    </Suspense>
  );
}
