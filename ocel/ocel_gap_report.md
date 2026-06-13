# OCEL Gap Report

**Status:** Open gaps — not complete.

## Known Gaps (as of 2026-06-13)

- `ocel/anti_llm_cheat_lsp_ocel.json` — all 7 events have `attributes: []` (empty); no typed evidence carried
- `ocel/anti_llm_cheat_lsp_ocel.json` — object carries placeholder digest `"temp_val"` instead of computed BLAKE3 hash
- `ocel/anti_llm_cheat_lsp_ocel.receipt.json` — missing required CROWN fields: `output_hash`, `run_id`, `replay_pointer`
- No OCEL log bound to the main wasm4pm-compat manufacture pipeline

## Not a status claim

This report enumerates known gaps. It does not assert completeness.
Any "No gaps found" or "All systems functional" language is forbidden
per AGENTS.md Rule 8 — bounded status vocabulary only.
