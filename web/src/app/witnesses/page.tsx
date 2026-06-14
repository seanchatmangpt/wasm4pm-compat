import { readFile } from "fs/promises";
import { join } from "path";

// RSC: parses witness_corpus.rs at render time.
// The 438 witnesses are the project's academic authority corpus.

const CRATE_ROOT = join(process.cwd(), "..");

interface WitnessEntry {
  key: string;
  family: string;
}

async function getWitnesses(): Promise<WitnessEntry[]> {
  const content = await readFile(
    join(CRATE_ROOT, "src", "witness_corpus.rs"),
    "utf-8"
  );
  const match = content.match(/ALL_WITNESS_KEYS:\s*&\[&str\]\s*=\s*&\[([\s\S]*?)\];/);
  if (!match) return [];
  return Array.from(match[1].matchAll(/"([^"]+)"/g)).map((m) => ({
    key: m[1],
    family: m[1].includes("/") ? m[1].split("/")[0] : "core",
  }));
}

export default async function WitnessesPage() {
  const witnesses = await getWitnesses();

  // Group by family
  const byFamily: Record<string, WitnessEntry[]> = {};
  for (const w of witnesses) {
    (byFamily[w.family] ??= []).push(w);
  }
  const families = Object.entries(byFamily).sort((a, b) => b[1].length - a[1].length);

  return (
    <div>
      <h1 className="text-2xl font-bold text-zinc-100 mb-1">Witness Corpus</h1>
      <p className="text-sm text-zinc-500 mb-2">
        {witnesses.length} witness keys in{" "}
        <span className="font-mono text-zinc-400">src/witness_corpus.rs</span>.
        Each names an academic paper, standard, or law that a type in this crate
        answers to. The compile-time uniqueness proof ensures no two witnesses
        share a key.
      </p>
      <p className="text-xs text-zinc-600 mb-6 font-mono">
        API: <a href="/api/witnesses" className="text-blue-500 hover:underline">/api/witnesses</a>{" "}
        · filter by family: <a href="/api/witnesses?family=pm" className="text-blue-500 hover:underline">/api/witnesses?family=pm</a>
      </p>

      <div className="space-y-6">
        {families.map(([family, items]) => (
          <section key={family}>
            <h2 className="text-sm font-semibold text-zinc-400 uppercase tracking-wider mb-2">
              {family}{" "}
              <span className="text-zinc-600 normal-case font-normal">
                ({items.length})
              </span>
            </h2>
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-1">
              {items.map((w) => (
                <div
                  key={w.key}
                  className="font-mono text-xs text-zinc-400 bg-zinc-900 border border-zinc-800 px-2 py-1 rounded truncate hover:text-zinc-200 hover:border-zinc-600 transition-colors"
                  title={w.key}
                >
                  {w.key.includes("/") ? w.key.split("/").slice(1).join("/") : w.key}
                </div>
              ))}
            </div>
          </section>
        ))}
      </div>
    </div>
  );
}
