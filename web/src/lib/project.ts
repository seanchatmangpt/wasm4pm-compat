/**
 * project.ts — reads real project data from the filesystem and git.
 *
 * Every function here derives its return value from the actual source tree.
 * Nothing is hardcoded. This is the single data layer; UI components import
 * from here and never fabricate numbers.
 *
 * All functions are server-only (they use Node.js `fs` and `child_process`).
 * Import them only in Server Components or Route Handlers.
 */

import { execSync } from "child_process";
import { readdirSync, readFileSync, existsSync } from "fs";
import path from "path";

// Absolute path to the Rust crate root — one source of truth for all reads.
export const CRATE_ROOT = path.resolve(process.cwd(), "..");

// ── Module inventory ──────────────────────────────────────────────────────────

export interface RustModule {
  name: string;
  path: string;
  lineCount: number;
  pubItemCount: number; // grep count of `pub struct|enum|trait|fn|type`
}

export function getModules(): RustModule[] {
  const srcDir = path.join(CRATE_ROOT, "src");
  const files = readdirSync(srcDir).filter((f) => f.endsWith(".rs"));
  return files
    .map((file) => {
      const filePath = path.join(srcDir, file);
      const content = readFileSync(filePath, "utf-8");
      const lines = content.split("\n");
      const pubItems = lines.filter((l) =>
        /^pub (struct|enum|trait|fn|type|const)/.test(l)
      ).length;
      return {
        name: file.replace(".rs", ""),
        path: `src/${file}`,
        lineCount: lines.length,
        pubItemCount: pubItems,
      };
    })
    .sort((a, b) => a.name.localeCompare(b.name));
}

// ── Example inventory ─────────────────────────────────────────────────────────

export interface ExampleMeta {
  name: string;
  path: string;
  docTitle: string; // first `//!` line
  featureFlag: string | null; // `required-features` if any, else null
  lineCount: number;
}

export function getExamples(): ExampleMeta[] {
  const exDir = path.join(CRATE_ROOT, "examples");
  const files = readdirSync(exDir).filter((f) => f.endsWith(".rs"));
  return files
    .map((file) => {
      const filePath = path.join(exDir, file);
      const content = readFileSync(filePath, "utf-8");
      const lines = content.split("\n");
      // Extract first doc comment line
      const docLine = lines.find((l) => l.startsWith("//!"))?.replace(/^\/\/! ?/, "") ?? "";
      return {
        name: file.replace(".rs", ""),
        path: `examples/${file}`,
        docTitle: docLine.trim(),
        featureFlag: null, // resolve from Cargo.toml if needed
        lineCount: lines.length,
      };
    })
    .sort((a, b) => a.name.localeCompare(b.name));
}

// ── Witness corpus ────────────────────────────────────────────────────────────

export interface WitnessKey {
  key: string;
  family: string; // derived from prefix (e.g. "ai-llm", "pm", "wfnet")
}

export function getWitnessKeys(): WitnessKey[] {
  const corpusPath = path.join(CRATE_ROOT, "src", "witness_corpus.rs");
  const content = readFileSync(corpusPath, "utf-8");
  // Extract string literals from the ALL_WITNESS_KEYS array
  const match = content.match(/ALL_WITNESS_KEYS:\s*&\[&str\]\s*=\s*&\[([\s\S]*?)\];/);
  if (!match) return [];
  const keys = Array.from(match[1].matchAll(/"([^"]+)"/g)).map((m) => m[1]);
  return keys.map((key) => ({
    key,
    family: key.includes("/") ? key.split("/")[0] : "core",
  }));
}

// ── ALIVE gate fixtures ───────────────────────────────────────────────────────

export interface AliveCounts {
  compileFail: number;
  compilePass: number;
  total: number;
}

export function getAliveCounts(): AliveCounts {
  const failDir = path.join(CRATE_ROOT, "tests", "ui", "compile_fail");
  const passDir = path.join(CRATE_ROOT, "tests", "ui", "compile_pass");
  const compileFail = existsSync(failDir)
    ? readdirSync(failDir).filter((f) => f.endsWith(".rs")).length
    : 0;
  const compilePass = existsSync(passDir)
    ? readdirSync(passDir).filter((f) => f.endsWith(".rs")).length
    : 0;
  return { compileFail, compilePass, total: compileFail + compilePass };
}

// ── Zod schema inventory ──────────────────────────────────────────────────────

export interface ZodSchema {
  name: string; // e.g. "ArcSchema"
  typeName: string; // e.g. "Arc"
}

export function getZodSchemas(): ZodSchema[] {
  const zodPath = path.join(
    CRATE_ROOT,
    "wasm4pm-compat-ts",
    "bindings",
    "zod_schemas.ts"
  );
  if (!existsSync(zodPath)) return [];
  const content = readFileSync(zodPath, "utf-8");
  return Array.from(content.matchAll(/^export const (\w+Schema) = /gm)).map(
    (m) => ({
      name: m[1],
      typeName: m[1].replace("Schema", ""),
    })
  );
}

// ── Git log ───────────────────────────────────────────────────────────────────

export interface GitCommit {
  hash: string;
  subject: string;
  date: string;
  author: string;
}

export function getRecentCommits(n = 10): GitCommit[] {
  try {
    const raw = execSync(
      `git -C "${CRATE_ROOT}" log --pretty=format:"%H|%s|%ad|%an" --date=short -${n}`,
      { encoding: "utf-8" }
    );
    return raw
      .trim()
      .split("\n")
      .map((line) => {
        const [hash, subject, date, author] = line.split("|");
        return { hash: hash.slice(0, 7), subject, date, author };
      });
  } catch {
    return [];
  }
}

// ── Cargo.toml metadata ───────────────────────────────────────────────────────

export interface CrateMetadata {
  name: string;
  version: string;
  description: string;
  edition: string;
  features: string[];
}

export function getCrateMetadata(): CrateMetadata {
  const cargoPath = path.join(CRATE_ROOT, "Cargo.toml");
  const content = readFileSync(cargoPath, "utf-8");
  const get = (key: string) =>
    content.match(new RegExp(`^${key}\\s*=\\s*"([^"]+)"`, "m"))?.[1] ?? "";
  // Extract feature names from [features] section
  const featuresSection = content.match(/\[features\]([\s\S]*?)(?=\n\[)/)?.[1] ?? "";
  const features = Array.from(featuresSection.matchAll(/^(\w+)\s*=/gm)).map(
    (m) => m[1]
  );
  return {
    name: get("name"),
    version: get("version"),
    description: get("description"),
    edition: get("edition"),
    features,
  };
}

// ── Summary (all in one call for the overview page) ──────────────────────────

export interface ProjectSummary {
  crate: CrateMetadata;
  moduleCount: number;
  totalPubItems: number;
  exampleCount: number;
  witnessCount: number;
  witnessKeysByFamily: Record<string, number>;
  alive: AliveCounts;
  zodSchemaCount: number;
  recentCommits: GitCommit[];
}

export function getProjectSummary(): ProjectSummary {
  const modules = getModules();
  const witnesses = getWitnessKeys();
  const byFamily: Record<string, number> = {};
  for (const w of witnesses) {
    byFamily[w.family] = (byFamily[w.family] ?? 0) + 1;
  }
  return {
    crate: getCrateMetadata(),
    moduleCount: modules.length,
    totalPubItems: modules.reduce((s, m) => s + m.pubItemCount, 0),
    exampleCount: getExamples().length,
    witnessCount: witnesses.length,
    witnessKeysByFamily: byFamily,
    alive: getAliveCounts(),
    zodSchemaCount: getZodSchemas().length,
    recentCommits: getRecentCommits(8),
  };
}
