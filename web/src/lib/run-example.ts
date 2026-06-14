/**
 * run-example.ts — executes a Rust example and returns its real stdout.
 *
 * Used by Route Handlers (not RSC directly, to avoid blocking the render).
 * Output is the true binary output — no mocking, no fixtures.
 */

import { execSync } from "child_process";
import { CRATE_ROOT } from "./project";

export interface ExampleResult {
  name: string;
  exitCode: number;
  stdout: string;
  stderr: string;
  durationMs: number;
}

/**
 * Runs `cargo run --example <name>` in the crate root and captures output.
 * Timeout: 60s (compilation may be required on cold cache).
 */
export function runExample(name: string): ExampleResult {
  const start = Date.now();
  try {
    const stdout = execSync(
      `cargo run --example ${name} 2>/tmp/ex_stderr_${name}`,
      {
        cwd: CRATE_ROOT,
        encoding: "utf-8",
        timeout: 60_000,
      }
    );
    const stderr = (() => {
      try {
        return execSync(`cat /tmp/ex_stderr_${name}`, { encoding: "utf-8" });
      } catch {
        return "";
      }
    })();
    return {
      name,
      exitCode: 0,
      stdout: stdout.trim(),
      stderr: stderr.trim(),
      durationMs: Date.now() - start,
    };
  } catch (e: unknown) {
    const err = e as { stdout?: string; stderr?: string; status?: number };
    return {
      name,
      exitCode: err.status ?? 1,
      stdout: (err.stdout ?? "").trim(),
      stderr: (err.stderr ?? "").trim(),
      durationMs: Date.now() - start,
    };
  }
}
