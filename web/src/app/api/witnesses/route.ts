// Route Handler: returns the full witness corpus as JSON.
// Data source: src/witness_corpus.rs — parsed at request time.

import { readFile } from "fs/promises";
import { join } from "path";

const CRATE_ROOT = join(process.cwd(), "..");

export async function GET(request: Request) {
  const url = new URL(request.url);
  const family = url.searchParams.get("family");

  const corpusPath = join(CRATE_ROOT, "src", "witness_corpus.rs");
  const content = await readFile(corpusPath, "utf-8");

  const match = content.match(/ALL_WITNESS_KEYS:\s*&\[&str\]\s*=\s*&\[([\s\S]*?)\];/);
  if (!match) return Response.json({ error: "corpus not found" }, { status: 500 });

  const keys = Array.from(match[1].matchAll(/"([^"]+)"/g)).map((m) => m[1]);

  const witnesses = keys.map((key) => ({
    key,
    family: key.includes("/") ? key.split("/")[0] : "core",
  }));

  const filtered = family
    ? witnesses.filter((w) => w.family === family)
    : witnesses;

  return Response.json({
    total: keys.length,
    returned: filtered.length,
    witnesses: filtered,
  });
}
