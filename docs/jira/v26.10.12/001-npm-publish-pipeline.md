# 001 — npm publish pipeline for v26.10.12

- Standing: OPEN
- Created: 2026-10-12
- Source: v26.10.12 ARD/PRD (`000-ard-prd.md`), capability 5.
- Depends on: `002-ocel-v2-ts-surface.md` (packages must have content before
  publishing).

## Work to complete

1. Create `.github/workflows/npm-release.yml`:
   - trigger: `workflow_dispatch` + tag push `v*`;
   - steps: `cargo make ci` → build the OCEL wasm component (reuse the recipe in
     `docs/WIT_RUNTIME.md` / the `../wasm4pm-compat-component` crate pattern) →
     jco transpile → `npm ci && npm test` in `wasm4pm-compat-ts/` →
     `npm publish --dry-run` gate → `npm publish` for `@wasm4pm/compat-ts` and
     `@wasm4pm/compat-wasm`;
   - npm token from repo secrets; publish only when every prior step exits 0.
2. Local dry-run proof before first real publish.
3. Record the first published version + npm URLs in History below.

## Acceptance

- Workflow file exists; local `npm publish --dry-run` for both packages exits 0.
- A run of the workflow on the release tag publishes both packages at 26.10.12.

## Falsifier

- The workflow publishes while `cargo make ci` or the TS tests fail → publish gate is
  vacuous; fix before first release.

## History

- 2026-10-12 | OPEN | work order created from ARD/PRD | pipeline not yet started
- 2026-10-12 | PARTIAL_ALIVE | W-G + P13 + W-E2 | `.github/workflows/npm-release.yml`
  written, linted (YAML parse OK), corrected (package dir names, vendored-component
  fail-fast gate, `--name compat-ocel`, wasm copy step, stray secret removed);
  local `npm publish --dry-run` exit 0 for both packages (@wasm4pm/compat-ts 4 files,
  @wasm4pm/compat-wasm 22 files incl. compat-ocel.wasm). NOT-YET: a real workflow run
  on a `v*` tag (requires vendored `component/` + NPM_TOKEN secret + push).
- 2026-10-12 | PARTIAL_ALIVE | component vendoring | the component crate lives at the
  sibling checkout `../wasm4pm-compat-component` (built, lifted, validated, jco
  transpiled — smoke 6/6 pass). CI requires `rsync -a --exclude target
  ../wasm4pm-compat-component/ component/` before tagging (workflow fail-fasts with
  these instructions).
