// Route Handler: returns the list of all example names as JSON.
// Edge-safe (reads are static at build time if needed, but we run on Node here).

import { readdir } from "fs/promises";
import { join } from "path";

const CRATE_ROOT = join(process.cwd(), "..");

export async function GET() {
  const dir = join(CRATE_ROOT, "examples");
  const files = (await readdir(dir)).filter((f) => f.endsWith(".rs"));
  const names = files.map((f) => f.replace(".rs", "")).sort();
  return Response.json({ examples: names, count: names.length });
}
