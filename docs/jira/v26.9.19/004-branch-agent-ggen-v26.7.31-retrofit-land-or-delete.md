# wasm4pm-compat: land or delete remote branch `agent/ggen-v26.7.31-retrofit`

- Standing: OPEN
- Created: 2026-09-19 (v26.9.19 gh survey wave)
- Source: remote branch `agent/ggen-v26.7.31-retrofit` — not merged into `main`, no open PR
- Evidence: `git branch -r --no-merged origin/main` lists it; absent from `gh pr list` heads

## Work to complete
- Decide: open a PR (`gh pr create -R seanchatmangpt/wasm4pm-compat --head agent/ggen-v26.7.31-retrofit`) or delete (`git push origin --delete agent/ggen-v26.7.31-retrofit`).
- If superseded, delete; otherwise land through review.

## Acceptance
- After `git fetch --prune`, `git branch -r --no-merged origin/main` no longer lists `agent/ggen-v26.7.31-retrofit`.

## History
- 2026-09-19 | OPEN | survey found PR-less unmerged branch | agent/ggen-v26.7.31-retrofit | decision pending
