# wasm4pm-compat: land or delete remote branch `agent/wasm4pm-procint-d1`

- Standing: OPEN
- Created: 2026-09-19 (v26.9.19 gh survey wave)
- Source: remote branch `agent/wasm4pm-procint-d1` — not merged into `main`, no open PR
- Evidence: `git branch -r --no-merged origin/main` lists it; absent from `gh pr list` heads

## Work to complete
- Decide: open a PR (`gh pr create -R seanchatmangpt/wasm4pm-compat --head agent/wasm4pm-procint-d1`) or delete (`git push origin --delete agent/wasm4pm-procint-d1`).
- If superseded, delete; otherwise land through review.

## Acceptance
- After `git fetch --prune`, `git branch -r --no-merged origin/main` no longer lists `agent/wasm4pm-procint-d1`.

## History
- 2026-09-19 | OPEN | survey found PR-less unmerged branch | agent/wasm4pm-procint-d1 | decision pending
