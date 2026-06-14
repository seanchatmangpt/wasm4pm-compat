"use server";

// Server action: run a pre-compiled example binary and return its output.
// This is the only way the UI triggers a binary execution — no client-side
// process spawning, no fixtures. The output is real or the action fails.

import { execSync } from "child_process";
import { existsSync } from "fs";
import { join } from "path";

const CRATE_ROOT = join(process.cwd(), "..");

export interface RunResult {
  name: string;
  stdout: string;
  exitCode: number;
  durationMs: number;
  ranVia: "binary" | "unavailable";
  error?: string;
}

export async function runExampleAction(name: string): Promise<RunResult> {
  // Validate: only alphanumeric + underscore — no shell injection path
  if (!/^[a-z0-9_]+$/.test(name)) {
    return {
      name,
      stdout: "",
      exitCode: 1,
      durationMs: 0,
      ranVia: "unavailable",
      error: "invalid example name",
    };
  }

  const binaryPath = join(CRATE_ROOT, "target", "debug", "examples", name);

  if (!existsSync(binaryPath)) {
    return {
      name,
      stdout: "",
      exitCode: 1,
      durationMs: 0,
      ranVia: "unavailable",
      error: `binary not found: target/debug/examples/${name} — run \`cargo build --examples\` first`,
    };
  }

  const start = Date.now();
  try {
    const stdout = execSync(binaryPath, {
      encoding: "utf-8",
      timeout: 30_000,
    });
    return {
      name,
      stdout: stdout.trim(),
      exitCode: 0,
      durationMs: Date.now() - start,
      ranVia: "binary",
    };
  } catch (e: unknown) {
    const err = e as { stdout?: string; stderr?: string; status?: number };
    return {
      name,
      stdout: ((err.stdout ?? "") + "\n" + (err.stderr ?? "")).trim(),
      exitCode: err.status ?? 1,
      durationMs: Date.now() - start,
      ranVia: "binary",
      error: "non-zero exit",
    };
  }
}
