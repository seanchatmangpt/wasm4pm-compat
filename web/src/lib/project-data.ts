// Server-only data access layer. All values derived from real project artifacts.
// No fixtures. No hardcoded lists. Every exported function reads the filesystem.

import { readdir, readFile } from 'fs/promises';
import { join } from 'path';

// The repo root is two levels up from web/src/lib
const REPO_ROOT = join(process.cwd(), '..');

export type SchemaEntry = {
  name: string;
  fields: string[];
};

export type ExampleEntry = {
  name: string;
  featureFlags: string[];
  docRef: string;
};

export type ModuleEntry = {
  name: string;
  pubCount: number;
};

/** Parse real Zod schema names and fields from the generated bindings file. */
export async function getSchemas(): Promise<SchemaEntry[]> {
  const filePath = join(REPO_ROOT, 'wasm4pm-compat-ts/bindings/zod_schemas.ts');
  const content = await readFile(filePath, 'utf-8');

  const schemas: SchemaEntry[] = [];
  const schemaBlocks = content.split(/(?=^export const \w+Schema)/m);

  for (const block of schemaBlocks) {
    const nameMatch = block.match(/^export const (\w+)Schema = z\.object\(/m);
    if (!nameMatch) continue;
    const name = nameMatch[1];
    // Extract quoted field names from z.object({...})
    const fields = [...block.matchAll(/"(\w+)": z\./g)].map(m => m[1]);
    schemas.push({ name, fields });
  }

  return schemas.sort((a, b) => a.name.localeCompare(b.name));
}

/** List real example files from examples/, skipping rough_ drafts and c8_ demos. */
export async function getExamples(): Promise<ExampleEntry[]> {
  const dir = join(REPO_ROOT, 'examples');
  const files = await readdir(dir);

  const results: ExampleEntry[] = [];
  for (const file of files) {
    if (!file.endsWith('.rs')) continue;
    if (file.startsWith('rough_') || file.startsWith('c8_')) continue;

    const name = file.replace(/\.rs$/, '');
    const content = await readFile(join(dir, file), 'utf-8');

    // Extract feature flag from 'cargo run --example name --features foo' comments or header
    const featureMatch = content.match(/Feature flag[^:]*:\s*`([^`]+)`|feature.*?`([^`]+)`/i);
    const featureFlags = featureMatch
      ? [(featureMatch[1] || featureMatch[2]).trim()]
      : [];

    // Extract doc ref from header comment
    const docRefMatch = content.match(/Doc reference:\s*(.+)/);
    const docRef = docRefMatch ? docRefMatch[1].trim() : `src/${name.replace(/_/g, '/')}.rs`;

    results.push({ name, featureFlags, docRef });
  }

  return results.sort((a, b) => a.name.localeCompare(b.name));
}

/** List real Rust source modules with their pub item counts. */
export async function getModules(): Promise<ModuleEntry[]> {
  const dir = join(REPO_ROOT, 'src');
  const files = await readdir(dir);

  const results: ModuleEntry[] = [];
  const skip = new Set(['lib', 'test_utils', 'prelude']);

  for (const file of files) {
    if (!file.endsWith('.rs')) continue;
    const name = file.replace(/\.rs$/, '');
    if (skip.has(name)) continue;

    const content = await readFile(join(dir, file), 'utf-8');
    const pubCount = (content.match(/^pub /gm) || []).length;
    if (pubCount === 0) continue; // Skip internal files with no pub surface

    results.push({ name, pubCount });
  }

  return results.sort((a, b) => b.pubCount - a.pubCount);
}

/** Count witnessed triples from DOC_COVERAGE_LOG. */
export async function getCoverageStatus(): Promise<{
  coveredModules: number;
  totalExamples: number;
  totalSchemas: number;
}> {
  const [examples, schemas] = await Promise.all([getExamples(), getSchemas()]);

  const log = await readFile(join(REPO_ROOT, 'DOC_COVERAGE_LOG.md'), 'utf-8');
  const coveredModules = (log.match(/✅/g) || []).length;

  return {
    coveredModules,
    totalExamples: examples.length,
    totalSchemas: schemas.length,
  };
}
