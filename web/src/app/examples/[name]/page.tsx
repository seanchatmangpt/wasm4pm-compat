import { readdir, readFile } from "fs/promises";
import { execSync } from "child_process";
import { join } from "path";
import { notFound } from "next/navigation";

// RSC + static generation: pre-renders every example at build time by actually
// running `cargo run --example <name>`. Output is real binary stdout/stderr.
// No mocking. No pre-captured fixtures. The HTML contains what the binary produced.

const CRATE_ROOT = join(process.cwd(), "..");

export async function generateStaticParams() {
  const dir = join(CRATE_ROOT, "examples");
  const files = await readdir(dir);
  return files
    .filter((f) => f.endsWith(".rs"))
    .map((f) => ({ name: f.replace(".rs", "") }));
}

interface RunResult {
  stdout: string;
  exitCode: number;
  durationMs: number;
}

function runExample(name: string, featureFlag: string | null): RunResult {
  const start = Date.now();
  const cmd = featureFlag
    ? `cargo run --example ${name} --features ${featureFlag}`
    : `cargo run --example ${name}`;
  try {
    const raw = execSync(`${cmd} 2>&1`, {
      cwd: CRATE_ROOT,
      encoding: "utf-8",
      timeout: 180_000,
    });
    const lines = raw.split("\n");
    const runIdx = lines.reduce(
      (last: number, line: string, i: number) => (line.trimStart().startsWith("Running `") ? i : last),
      -1
    );
    const output = (runIdx >= 0 ? lines.slice(runIdx + 1) : lines)
      .join("\n")
      .trim();
    return { stdout: output, exitCode: 0, durationMs: Date.now() - start };
  } catch (e: unknown) {
    const err = e as { stdout?: string; stderr?: string; status?: number };
    const combined = ((err.stdout ?? "") + "\n" + (err.stderr ?? "")).trim();
    return {
      stdout: combined,
      exitCode: err.status ?? 1,
      durationMs: Date.now() - start,
    };
  }
}

export default async function ExamplePage({
  params,
}: {
  params: Promise<{ name: string }>;
}) {
  const { name } = await params;

  const filePath = join(CRATE_ROOT, "examples", `${name}.rs`);
  let source: string;
  try {
    source = await readFile(filePath, "utf-8");
  } catch {
    notFound();
  }

  const lines = source.split("\n");
  const docLines = lines
    .filter((l: string) => l.startsWith("//!"))
    .map((l: string) => l.replace(/^\/\/! ?/, ""));
  const docTitle = docLines[0]?.trim() ?? name;
  const docBody = docLines.slice(1).filter(Boolean).join("\n").trim();

  const featureMatch = source.match(
    /`(formats|strict|wasm4pm)`.*feature|feature.*`(formats|strict|wasm4pm)`/i
  );
  const featureFlag = featureMatch ? featureMatch[1] || featureMatch[2] : null;

  const result = runExample(name, featureFlag);

  const exitColor = result.exitCode === 0 ? "text-emerald-400" : "text-red-400";
  const exitBadge =
    result.exitCode === 0
      ? "bg-emerald-950 border-emerald-800 text-emerald-400"
      : "bg-red-950 border-red-800 text-red-400";

  return (
    <div className="max-w-4xl">
      <nav className="text-xs text-zinc-600 mb-4 font-mono">
        <a href="/examples" className="hover:text-zinc-400">examples</a>
        <span className="mx-1">/</span>
        <span className="text-zinc-400">{name}</span>
      </nav>

      <h1 className="text-2xl font-bold font-mono text-emerald-400 mb-1">{name}</h1>
      {docTitle && docTitle !== name && (
        <p className="text-zinc-300 text-sm mb-1">{docTitle}</p>
      )}
      <div className="flex flex-wrap gap-3 items-center text-xs text-zinc-600 mb-6">
        <span>{lines.length} lines</span>
        {featureFlag && (
          <span className="font-mono text-blue-400 bg-blue-950/50 border border-blue-900 px-1.5 py-0.5 rounded">
            --features {featureFlag}
          </span>
        )}
        <span className="font-mono text-zinc-500">cargo run --example {name}</span>
      </div>

      <section className="mb-8">
        <div className="flex items-center gap-3 mb-2">
          <h2 className="text-sm font-semibold text-zinc-400 uppercase tracking-wider">
            Real binary output
          </h2>
          <span className={`text-xs font-mono px-2 py-0.5 rounded border ${exitBadge}`}>
            EXIT {result.exitCode}
          </span>
          <span className="text-xs text-zinc-600">{result.durationMs}ms</span>
        </div>
        <pre className={`text-xs font-mono bg-zinc-950 border border-zinc-800 rounded p-4 overflow-x-auto whitespace-pre-wrap ${exitColor}`}>
          {result.stdout || "(no stdout)"}
        </pre>
      </section>

      {docBody && (
        <section className="mb-8">
          <h2 className="text-sm font-semibold text-zinc-400 uppercase tracking-wider mb-2">
            Documentation
          </h2>
          <div className="text-sm text-zinc-400 bg-zinc-900 border border-zinc-800 rounded p-4 whitespace-pre-wrap font-mono">
            {docBody}
          </div>
        </section>
      )}

      <section>
        <h2 className="text-sm font-semibold text-zinc-400 uppercase tracking-wider mb-2">
          Source ({lines.length} lines)
        </h2>
        <pre className="text-xs font-mono bg-zinc-950 border border-zinc-800 rounded p-4 overflow-x-auto text-zinc-400 max-h-96 overflow-y-auto whitespace-pre">
          {source}
        </pre>
      </section>
    </div>
  );
}
