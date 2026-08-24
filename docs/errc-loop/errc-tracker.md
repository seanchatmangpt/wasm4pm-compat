# ERRC Tracker — wasm4pm-compat as reference RDF/ggen implementation

Scope: `wasm4pm-compat` becoming the reference RDF/ggen implementation for
process-mining type shapes. Sourced from the current /loop directive
(job `4021d1dd`, hourly) and this session's own work-in-progress.

## Cycle 7 — 2026-08-24 — closed out, all parked (no unilateral execution)

Ran via `errc-cycle` skill (`wf_fcb420b8-086`), scope: "real, unresolved
items only." 6 categorized, 0 verified, 6 parked — every item this cycle
turned out to require a user decision (architecture choice, design
direction, or scope confirmation), not a bounded auto-fixable action:

- **ELIMINATE** — cron job `4021d1dd`'s prompt framing may be stale
  (already flagged cycles 3/5/6). Park reason: unilaterally
  redirecting/stopping the user's own standing job was explicitly declined
  three times already; needs the user's own call.
- **REDUCE** — the 9 confirmed-but-not-actioned findings in
  `docs/errc-loop/adversarial-review-2026-08-24.md`. Park reason: each
  needs its own severity/ambiguity read before a fix vs. won't-fix call;
  bundling would hide per-item judgment.
- **RAISE** ×2 — typed id newtypes for `sh:class` references (design
  direction, blast radius across all 15 clusters); struct-name-collision
  naming convention across clusters (no live bug yet — premature to
  invent a convention with nothing to verify it against).
- **CREATE** ×2 — the cross-repo `~/wasm4pm` output blocker from the
  Makefile.toml/ggen.toml fix above (needs a ggen-upgrade-vs-second-
  manifest architecture decision, plus confirming the sibling repo's
  breed files are actually stale before touching another repo);
  formalizing the one-off 20-agent adversarial review into a repeatable
  check (cadence/cost question, no pipeline change since the last pass to
  verify against).

**Recommendation:** nothing here is ready for unilateral execution. All 6
items are logged for the user to pick a direction on, not re-parked
indefinitely by default — each has a concrete proposed action above once
a decision is made.

## Makefile.toml / ggen.toml path-scheme rewrite — 2026-08-24 — RESOLVED

Was parked as "`Makefile.toml` still calls dead `--manifest`/`--rule` ggen
flags" in Cycle 6 below. Consolidating the 8 broken `[tasks.ggen-*]` into
`ggen-sync`/`ggen-sync-dry` surfaced two deeper, real, pre-existing bugs in
`ggen/ggen.toml` itself — both now fixed:

1. **`[inference]` rule `alive-gate`'s CONSTRUCT query lacked `ORDER BY`**
   (`error[E0011]`) — contradicted `CLAUDE.md`'s stale claim this was
   "fixed 2026-06-03... replaced with a file reference"; it was still an
   inline, broken query. Fixed: added `ORDER BY ?s ?p ?o` before `LIMIT 1`.
2. **`output_dir = "/Users/sac/wasm4pm-compat"` (absolute) is now rejected**
   by the installed CLI (`[FM-WRITE-002] to: path must be relative`), and
   every relative `source`/`query`/`template` path in the manifest was
   written assuming cwd=`ggen/` — both incompatible with the installed
   CLI's actual resolution rule (`ggen.toml` resolves paths relative to
   itself, and it only resolves from cwd, no `--manifest` flag). Fixed:
   rewrote every relative ontology/query/template path in `ggen/ggen.toml`
   to be repo-root-relative (`ggen/ontology/...`, `ggen/queries/...`,
   `ggen/templates/...`), set `output_dir = "."`, and changed
   `Makefile.toml`'s `ggen-sync`/`ggen-sync-dry` tasks to stage a temporary
   `ggen.toml` copy at repo root (the proven `cp ... && ggen sync run ...
   && rm ...` workaround from `~/.claude/skills/run-ggen/SKILL.md`) instead
   of `cd`-ing into `ggen/`.

**New finding surfaced mid-fix, also now handled (not silently dropped):**
the installed CLI additionally rejects **any** `output_file` outside the
manifest's own project root — both absolute (`/Users/sac/wasm4pm/...`) and
relative-traversal (`../wasm4pm/...`) forms fail
`[FM-WRITE-002] ... contains a traversal component; it must stay inside the
project root`. This affects the 5 rules that write into the sibling
`~/wasm4pm` repo (`breed-ids`, `breed-registration`, `registry-json`,
`paper-pointers-test`, `universal-anticheat`). **Cross-repo generation via
a single ggen.toml is not supported by this CLI version at all** — this is
a structural CLI capability regression from whatever version this manifest
was originally written against, not a path-syntax bug fixable by more
rewriting. Disabled those 5 rules in `ggen/ggen.toml` with a dated comment
explaining why and how to re-enable (future ggen version restoring
cross-project-root output, or a second manifest rooted at `~/wasm4pm`
itself). **Logged here as open, not silently dropped** — if the sibling
`~/wasm4pm` repo's breed files need regenerating, that now requires either
a ggen version upgrade or a dedicated manifest run from `~/wasm4pm`.

Verified: `cargo make ggen-sync-dry` and `cargo make ggen-sync` both
succeed (all 13 remaining active rules render); `cargo make check-all`
passes clean afterward; `git diff` on regenerated files (`src/witnesses.rs`,
`audits/*.sh`) shows only cosmetic whitespace/newline churn, no semantic
regression (confirmed via `cargo fmt` reverting `witnesses.rs` to byte-
identical).

## Cycle 6 — 2026-08-24 — closed out

Ran via `wf_aa0da59c-71a`. 2 verified (both executed, commit `7b704ee`),
5 parked:

- **Done:** owl:Ontology headers added to the 5 files missing them
  (dfg/genetic_matrix/random_variables/transition_system/trie), confirmed
  header-only (diffed pre/post generated Rust, zero field-shape change).
  Done: log.ttl's EA/OA gap disclosure comment added.
- **Parked (real, not bounded):** typed id newtypes for `sh:class`
  references (design decision), struct-name-collision naming convention
  (no live bug, premature), Makefile.toml ggen CLI flags (3-way scope
  choice, unchanged since cycle 2), cron job `4021d1dd` staleness
  (user's own standing job, not touched unilaterally).

**Remaining backlog is now entirely user-decision items** — no more
bounded/mechanical work is queued. Cycle 7+ should keep checking rather
than assume this is final (a new cluster, a new user request, or new
review can always add real work), but should say plainly if nothing
changed rather than re-park the same 4 items verbatim every hour.

## Between cycles 5 and 6 — adversarial panel review (user-requested, not an ERRC cycle)

User requested an adversarial multi-expert review ("post-AGI Dr. Wil van der
Aalst et al., not one person") of the completed ontology/pipeline work.
20-agent ultracode workflow (`wf_7a7a2507-ec2`): 5 independent expert
lenses, every finding independently re-verified by a separate skeptical
reader. 14 confirmed findings (0 blocking, 10 significant, 3 minor, 1
cosmetic). Fixed the 5 systemic-root-cause ones same session (commit
`146fe52`): two real template bugs (sh:class ignored maxCount, sh:in
ignored minCount -- both silently discarding real SHACL cardinality
constraints) plus 2 ontology-modeling inconsistencies (bpmn.ttl missing
datatypes on 4 properties, powl.ttl's circular sh:class self-reference).
All 15 clusters regenerated, 20/20 tests updated and passing. 9 further
confirmed-but-not-actioned findings logged in
`docs/errc-loop/adversarial-review-2026-08-24.md` with stated reasons,
not silently dropped.

This is exactly the kind of gap an ERRC cycle's own verify-phase should
also be able to catch going forward -- cycle 6+ should treat "run an
adversarial review pass on the generation pipeline" as a candidate
REDUCE/CREATE item if the pipeline changes further, not just this one
user-requested pass.

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
