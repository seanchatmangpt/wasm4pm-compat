# wasm4pm-compat: land or delete remote branch `codex/ggen-elixir-ash-types`

- Standing: OPEN
- Created: 2026-09-19 (v26.9.19 gh survey wave)
- Source: remote branch `codex/ggen-elixir-ash-types` — not merged into `main`, no open PR
- Evidence: `git branch -r --no-merged origin/main` lists it; absent from `gh pr list` heads

## Work to complete
- Decide: open a PR (`gh pr create -R seanchatmangpt/wasm4pm-compat --head codex/ggen-elixir-ash-types`) or delete (`git push origin --delete codex/ggen-elixir-ash-types`).
- If superseded, delete; otherwise land through review.

## Acceptance
- After `git fetch --prune`, `git branch -r --no-merged origin/main` no longer lists `codex/ggen-elixir-ash-types`.

## History
- 2026-09-19 | OPEN | survey found PR-less unmerged branch | codex/ggen-elixir-ash-types | decision pending
