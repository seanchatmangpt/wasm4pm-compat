import { readdir, readFile } from "fs/promises";
import { join } from "path";

// RSC: reads examples/*.rs directly — every row is a real file.
// Includes ALL examples including c8_* (which are real project capabilities).

const CRATE_ROOT = join(process.cwd(), "..");

interface Example {
  name: string;
  docTitle: string;
  lineCount: number;
  featureFlag: string | null;
}

async function getAllExamples(): Promise<Example[]> {
  const dir = join(CRATE_ROOT, "examples");
  const files = (await readdir(dir)).filter((f) => f.endsWith(".rs"));
  const results = await Promise.all(
    files.map(async (file) => {
      const content = await readFile(join(dir, file), "utf-8");
      const lines = content.split("\n");
      const docTitle =
        lines
          .find((l) => l.startsWith("//!"))
          ?.replace(/^\/\/! ?/, "")
          .trim() ?? "";
      // Look for feature hints in doc comments
      const featureMatch = content.match(/`(formats|strict|wasm4pm)`.*feature|feature.*`(formats|strict|wasm4pm)`/i);
      const featureFlag = featureMatch
        ? (featureMatch[1] || featureMatch[2])
        : null;
      return {
        name: file.replace(".rs", ""),
        docTitle,
        lineCount: lines.length,
        featureFlag,
      };
    })
  );
  return results.sort((a, b) => a.name.localeCompare(b.name));
}

export default async function ExamplesPage() {
  const examples = await getAllExamples();

  const c8 = examples.filter((e) => e.name.startsWith("c8_"));
  const cross = examples.filter((e) => e.name.includes("pipeline") || e.name.includes("composition"));
  const core = examples.filter(
    (e) => !e.name.startsWith("c8_") && !e.name.includes("pipeline") && !e.name.includes("composition")
  );

  return (
    <div>
      <h1 className="text-2xl font-bold text-zinc-100 mb-1">Runnable Examples</h1>
      <p className="text-sm text-zinc-500 mb-6">
        {examples.length} examples in{" "}
        <span className="font-mono text-zinc-400">examples/</span>.
        Each exercises real API surface. Links open the live output page.
      </p>

      <ExampleGroup title="Per-capability examples" examples={core} />
      <ExampleGroup title="Cross-product composition" examples={cross} />
      <ExampleGroup title="C8 domain demonstrations" examples={c8} />
    </div>
  );
}

function ExampleGroup({ title, examples }: { title: string; examples: Example[] }) {
  if (examples.length === 0) return null;
  return (
    <section className="mb-8">
      <h2 className="text-base font-semibold text-zinc-300 mb-3">
        {title}{" "}
        <span className="text-zinc-600 font-normal text-sm">({examples.length})</span>
      </h2>
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2">
        {examples.map((ex) => (
          <a
            key={ex.name}
            href={`/examples/${ex.name}`}
            className="block border border-zinc-800 rounded p-3 bg-zinc-900 hover:border-zinc-600 hover:bg-zinc-800 transition-colors group"
          >
            <div className="font-mono text-sm text-emerald-400 group-hover:text-emerald-300">
              {ex.name}
            </div>
            {ex.featureFlag && (
              <span className="text-xs bg-zinc-800 text-blue-400 px-1 py-0.5 rounded font-mono mt-1 inline-block">
                --features {ex.featureFlag}
              </span>
            )}
            {ex.docTitle && (
              <div className="text-xs text-zinc-500 mt-1 line-clamp-2">{ex.docTitle}</div>
            )}
            <div className="text-xs text-zinc-700 mt-1">{ex.lineCount} lines</div>
          </a>
        ))}
      </div>
    </section>
  );
}
