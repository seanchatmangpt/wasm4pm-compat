# wasm4pm-compat: land or delete remote branch `agent/powl-v2-pydantic`

- Standing: OPEN
- Created: 2026-09-19 (v26.9.19 gh survey wave)
- Source: remote branch `agent/powl-v2-pydantic` — not merged into `main`, no open PR
- Evidence: `git branch -r --no-merged origin/main` lists it; absent from `gh pr list` heads

## Work to complete
- Decide: open a PR (`gh pr create -R seanchatmangpt/wasm4pm-compat --head agent/powl-v2-pydantic`) or delete (`git push origin --delete agent/powl-v2-pydantic`).
- If superseded, delete; otherwise land through review.

## Acceptance
- After `git fetch --prune`, `git branch -r --no-merged origin/main` no longer lists `agent/powl-v2-pydantic`.

## History
- 2026-09-19 | OPEN | survey found PR-less unmerged branch | agent/powl-v2-pydantic | decision pending
