"use client";

// Client component: calls the runExampleAction server action on button click.
// The server action runs the pre-compiled binary server-side and streams the
// result back. No client-side process spawning. Real output or a real error.

import { useActionState } from "react";
import { runExampleAction, type RunResult } from "@/app/actions/run-example";

const initialState: RunResult | null = null;

export function RunButton({ name }: { name: string }) {
  const [result, formAction, isPending] = useActionState(
    (_prev: RunResult | null, _formData: FormData) =>
      runExampleAction(name),
    initialState
  );

  return (
    <div className="mt-6">
      <form action={formAction}>
        <button
          type="submit"
          disabled={isPending}
          className="font-mono text-xs px-3 py-1.5 rounded border border-emerald-800 bg-emerald-950/40 text-emerald-400 hover:bg-emerald-950/80 disabled:opacity-50 disabled:cursor-wait transition-colors"
        >
          {isPending ? "running…" : "↺ re-run via server action"}
        </button>
      </form>

      {result && !isPending && (
        <div className="mt-3">
          <div className="flex items-center gap-3 mb-1 text-xs text-zinc-500">
            <span
              className={
                result.exitCode === 0 ? "text-emerald-400" : "text-red-400"
              }
            >
              EXIT {result.exitCode}
            </span>
            <span>{result.durationMs}ms via server action ({result.ranVia})</span>
            {result.error && (
              <span className="text-red-400">{result.error}</span>
            )}
          </div>
          <pre className="text-xs font-mono bg-zinc-950 border border-zinc-800 rounded p-3 overflow-x-auto whitespace-pre-wrap text-zinc-300 max-h-64 overflow-y-auto">
            {result.stdout || "(no stdout)"}
          </pre>
        </div>
      )}
    </div>
  );
}
