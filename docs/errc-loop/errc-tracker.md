# ERRC Tracker — wasm4pm-compat as reference RDF/ggen implementation

Scope: `wasm4pm-compat` becoming the reference RDF/ggen implementation for
process-mining type shapes. Sourced from the current /loop directive
(job `4021d1dd`, hourly) and this session's own work-in-progress.

## Cycle 5 — 2026-08-24 — closed out

Ran via `wf_3117cb19-5d6`, post-charter-execution (commit `120e473` had
just landed and been pushed). 0 verified, 4 parked — but one parked item
("3 of 15 clusters non-generatable") was mis-scoped as "not safe to
verify": it was actually a concrete, bounded gap once identified (which 3
clusters), not a policy decision. Identified and fixed same session:
**`log`, `ocel`, `powl`** were the 3 clusters never run through the
generation pipeline (bpmn + the 11 authored clusters were done; these 3
were missed). Generated, tested (5 real tests), committed `b5202e9`,
pushed. **All 15/15 ontology clusters now have real Rust projections**
(except `stochastic_petri`/`yawl`, correctly non-generatable — zero
classes).

Remaining genuinely parked (standing-configuration questions, not
bounded fixes — surfaced to the user, not resolved unilaterally):

- **Makefile.toml still calls non-existent ggen CLI flags** on 8+ tasks
  (`--manifest`/`--rule`). Confirmed real, parked since cycle 2 — the
  fix shape (single sync-all task vs. removing per-rule tasks vs.
  waiting on upstream ggen) is a scope choice.
- **Cron job `4021d1dd` may be stale** — it re-derives "awaiting charter
  decision" framing from cycles 1-3 even though the charter is resolved
  and executed. Not stopped/redirected unilaterally (user's own standing
  job); flagged for the user to decide whether to update its prompt.
- **`docs/errc-loop/pending-decisions.md` doesn't exist** — the tracker's
  own inline parked-items sections have been serving that role instead.
  Flagged as a documentation-structure preference, not changed.

## Charter decision — 2026-08-24

User delegated the DMEDI charter decision ("figure it out"). Resolved:
**prove the generation pipeline first** (bpmn/log/ocel/powl → real
Rust/Elixir/Python/WIT), not the remaining 11 clusters — same Gall's-Law
reasoning already applied to POWL's ontology sourcing this session (a
smaller working system beats a wider unused catalog). Charter written to
`docs/superpowers/specs/2026-08-24-reference-rdf-ggen-charter.md`.

This unblocks 2 of the 3 items parked since cycle 1:
- The 11-cluster item stays parked *deliberately* now (explicit non-goal
  per the charter), not just unresolved.
- The pipeline-wiring item is now the active work — cycle 4 should start
  building `ggen/queries/extract-type-shapes.rq` and the `bpmn` Rust
  template, per the charter's success criteria, not re-park it.

## Cycle 3 — 2026-08-24 — closed out, nothing to auto-execute

Ran via `wf_6a2d00be-83c`. 0 verified, 3 parked (same 3 as cycles 1–2,
unchanged). Confirms this cycle's own prediction from cycle 2's close-out:
the only open items left are the 3 that need a human decision. The
workflow's own recommendation: resolve the DMEDI charter question first,
since the other two parked items (11 clusters, pipeline wiring) both
depend on it to avoid rework — brought to the user directly rather than
looping again on the same unresolved backlog.

**Cron job `4021d1dd` left running** — but further hourly firings will
keep reporting this same "nothing to auto-execute" result until the user
answers. Not stopping the job unilaterally (the user set it up), but
flagging here so cycle 4+ doesn't re-derive this from scratch.

## Cycle 2 — 2026-08-24 — closed out

Ran via `Workflow({scriptPath: errc-cycle, ...})` (`wf_b1a81b70-29b`),
fired by the hourly cron (job `4021d1dd`). 2 verified, 3 parked (same 3
as cycle 1 — unchanged, still awaiting user decision).

- **4 pre-existing stale-stderr fixtures** — done, commit `9baa36e`.
  Confirmed pure nightly-rustc diagnostic-format drift (extra context
  lines, same error code/message/anchor line on every one) before
  regenerating via `TRYBUILD=overwrite`, then re-verified clean with a
  plain run: `test result: ok. 2 passed; 0 failed` — **the repo-wide
  `cargo make alive` gate is fully green for the first time this
  session.**
- **ggen CLI rough edges, expanded** — done, same commit. Added that
  per-rule selection is confirmed unsupported by the installed CLI
  entirely (not just a flag-name mismatch), and that `cwd = "ggen"` (not
  `--manifest`) is the actual mechanism making today's tasks find their
  manifest at all. Makefile.toml itself still not rewritten — that
  redesign remains a parked decision.
- **3 parked items** — unchanged from cycle 1 (remaining ~11 ontology
  clusters, wiring ontology into a generation pipeline, DMEDI charter for
  "reference RDF/ggen implementation"). Still awaiting user scoping;
  re-parked rather than re-derived.

**Open for cycle 3:** nothing new and bounded — the only remaining open
items are the 3 parked ones, which need a human decision, not another
automated verify pass. If cycle 3 finds nothing new to categorize beyond
those 3, it should say so plainly rather than manufacture busywork.

## Cycle 1 — 2026-08-24 — closed out

Ran via `Workflow({name: "errc-cycle", ...})` (`wf_f4f81eaa-301`). 5
verified, 3 parked. Executed the same session, in order:

- **Item 1 (ALIVE gate crash)** — premise did not hold. Both new fixtures
  pass; the reported "crash" was trybuild's own summary-assertion panic
  over 4 pre-existing, unrelated stale-stderr fixtures
  (`loss_report_items_type_mismatch`, `loss_without_report_on_allow_path`,
  `petri_bipartite_arc_noncopy_weight`, `refusal_without_named_law`,
  `refuse_loss_path_emitting_report`) — confirmed by two independent full
  `cargo test --test ui_tests -- --ignored` runs. **Not fixed this
  cycle** — real, separate, pre-existing item, not bundled in.
- **Item 2 (commit status_driven work)** — done, commit `b630f88`, after
  confirming the two relevant fixtures pass in both runs.
- **Item 3 (remaining ~11 ontology clusters)** — parked, needs user
  scoping (large multi-cluster research/authoring decision).
- **Item 4 (wire ontology into ggen pipeline)** — parked, needs
  scoping/sequencing decision.
- **Item 5 (ggen CLI rough edges)** — documented in `CLAUDE.md`, commit
  `31fc9ca`. Not fixed (Makefile.toml rewrite deferred — per-rule
  selection appears unsupported by installed ggen v26.8.18, needs its own
  redesign decision).
- **Item 6 (bindings/elixir uncommitted work)** — done, commit `e22fdae`
  (`.gitignore` + `plain_types.ex`/test committed). Also caught and fixed
  the generator source itself (`ggen/ash-types.toml`,
  `ggen/templates/plain-types.ex.tera`) left uncommitted from earlier in
  this session — commit `812a62f`, plus `python/.gitignore` for the same
  build-artifact-noise pattern.
- **Item 7 (design-doc addendum)** — done, commit `f49bae9`.
- **Item 8 (DMEDI charter for "reference RDF/ggen implementation")** —
  parked, sets overall project direction, needs the user's decision.

**Open for cycle 2** (next hourly firing, job `4021d1dd`): the 4
pre-existing stale-stderr fixtures (item 1's fallout), plus the 3 parked
items above once the user has weighed in.

## Cycle 1 — 2026-08-24 (open items, as first categorized)

1. Diagnose and fix the `cargo make alive` `compile_fail_fixtures` crash.
   Likely `tests/ui/compile_fail/status_driven_probability_over_one.rs` or
   `tests/ui/compile_fail/stochastic_transition_kind_conflated.rs` (or their
   `.stderr`), causing trybuild's rustc subprocess to panic rather than
   fail cleanly with a stderr mismatch. Not yet diagnosed — investigation
   was interrupted mid-flight.
2. Commit the verified-passing pre-existing uncommitted work
   (`src/status_driven.rs`, `c8-receipts/src/lib.rs`'s real
   `c8_market`/`c8_time` wiring, `docs/PAPER_COVERAGE_LEDGER.md`
   corrections) once the ALIVE gate above is clean. `cargo make test`,
   `cargo make check`, and `c8-receipts`'s own `cargo test` (28/28) already
   pass; only the ALIVE gate blocks this.
3. Continue the ontology restart: resolve public-ontology-or-author for the
   remaining ~11 clusters (`process_tree`, `petri_net`, `transition_system`,
   `oc_causal_net`, `ocpn`, `dfg`, `heuristics_net`, `org`, `trie`,
   `genetic_matrix`, `random_variables`) per the rule established this
   session (reuse a real public RDF/OWL ontology where one exists; author
   one at standards-submission quality, grounded in a real reference
   implementation, where none does).
4. Wire the completed ontology clusters (`bpmn`/`log`/`ocel`/`powl`,
   already committed under `ggen/ontology/type-shapes/`) into an actual
   `ggen` generation pipeline (Rust/Elixir/Python/WIT) per
   `docs/superpowers/specs/2026-08-24-ontology-first-restart-design.md`'s
   strangler-fig rollout. Currently only shapes exist — nothing generates
   from them yet.
5. The installed `ggen` CLI has confirmed rough edges this session: no
   `--manifest`/`--rule` flags despite `Makefile.toml` assuming they exist,
   rejects `output_dir = ".."` as path traversal when run from `ggen/`,
   requires staging a temporary root-level `ggen.toml` as a workaround
   (used successfully to generate `plain_types.ex` earlier this session).
6. `bindings/elixir/` has uncommitted `plain_types.ex` + `ash_types.ex`
   real-field work from earlier in this session, alongside untracked
   `_build/`, `deps/`, `mix.lock` (build artifacts, not source).
7. `docs/superpowers/specs/2026-08-24-ontology-first-restart-design.md`
   needs an addendum recording the reuse-public-ontology-or-author rule
   (currently only stated in the session's plan file,
   `~/.claude/plans/launch-5-explore-agents-jazzy-crystal.md`).
8. "Reference RDF/ggen implementation" as a stated goal is broad — may
   need its own Define/Measure charter (DMEDI discipline) before more
   Create-bucket work is justified rather than open-ended scope growth.
