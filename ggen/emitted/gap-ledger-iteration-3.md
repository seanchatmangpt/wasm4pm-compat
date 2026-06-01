# Gap Ledger Iteration 3 — Critical & HIGH Gap Classification

**Generated:** 2026-06-01  
**Authority:** Sean Chatman (xpointsh@gmail.com)  
**Source:** ggen/emitted/gap-ledger.yaml + artifact analysis  
**Total Gaps Classified:** 6 (3 CRITICAL, 3 HIGH)

---

## Executive Summary

| Gap ID | Name | Severity | Status | Closure Condition | Blockers |
|:------:|------|:--------:|:------:|---|---|
| GAP_001 | wasm4pm-compat Integration Bridge | HIGH | IN_PROGRESS | Type bridge + witness preservation + refusal law alignment | Integration tests pending |
| GAP_COMPONENT | Component Model Gap | CRITICAL | IN_PROGRESS | WIT interfaces generate for all feature gates; witness encoding valid | wit-bindgen conformance testing |
| GAP_LOSS | Loss Accounting Rules Gap | HIGH | MANUFACTURED | Loss policies auto-detected; LossReport on all lossy projections | Trait impl coverage validation |
| GAP_PROCESS_TREE | Process Tree Type Laws Gap | HIGH | MANUFACTURED | Arity/POWL soundness/projection legality at compile-time | Fixture audit completeness |
| GAP_TS | TypeScript Projection Gap | CRITICAL | IN_PROGRESS | .d.ts surfaces via specta; zero-cost phantom encoding | specta codegen validation |
| GAP_WASM | WASM ABI Boundary Gap | CRITICAL | IN_PROGRESS | Memory-safe, type-law-respecting; wasm-bindgen generated | wasm-pack audit + prohibited list validation |

---

## Per-Gap Classification

### GAP_001: wasm4pm-compat Integration Bridge

**Severity:** HIGH  
**Current Status:** IN_PROGRESS  
**Classification Rationale:** Ledger marks `MANUFACTURED`, but closure condition requires integration tests that prove OCEL → discovery → PetriNet → conformance → receipt. Manufacturing templates exist; audit gate requires runtime validation.

**Closure Condition:**
- compat → wasm4pm type bridge implemented
- Witnesses preserved through boundary
- Refusal laws aligned (no law loss at boundary crossing)

**Audit Gate:**
```
Integration tests: OCEL → discovery → PetriNet → conformance → receipt
```

**Blockers:**
1. **Integration test suite not yet written** — requires executable proof that process mining round-trip preserves law
2. **Witness enum serialization** — must verify `Witness` types survive WASM boundary without loss
3. **Refusal law mapping** — wasm4pm refusal types must bijectively map to compat refusal types

**Evidence Files:**
- `ggen/intel/dependency-boundary-map.yaml` — boundary contract
- `ggen/intel/graduation-surface-ledger.yaml` — graduation criteria
- `ggen/rules/graduation-law.yaml` — law preservation rules
- `ggen/templates/wasm4pm-lifecycle.tera` — lifecycle template
- `ggen/templates/wasm4pm-replay.tera` — replay template

**Next Action:** Write integration test harness that exercises OCEL → process mining → wasm4pm graduation → conformance check. Require per-artifact witness chain receipt.

---

### GAP_COMPONENT: Component Model Gap

**Severity:** CRITICAL  
**Current Status:** IN_PROGRESS  
**Classification Rationale:** Ledger marks `MANUFACTURED`, but closure condition is WIT interface generation for all feature gates. Templating exists; codegen execution and wit-bindgen validation pending.

**Closure Condition:**
- Component Model WIT interfaces generate correctly for all feature gates (formats, strict, wasm4pm, combinations)
- Witness encoding in WIT surface valid (no type mismatch with Rust)
- WIT syntax passes wit-validator

**Audit Gate:**
```
1. WIT syntax valid
2. wit-bindgen generates trait Guest
3. Component Model conformance pass (round-trip: Rust → WIT → TypeScript → back to Rust)
```

**Blockers:**
1. **WIT codegen not executed** — templates exist but no evidence of generated `.wit` files
2. **wit-bindgen integration missing** — no proof that TypeScript guest traits can be generated
3. **Feature gate coverage unclear** — are combinations (formats + strict, formats + wasm4pm) tested?

**Evidence Files:**
- `ggen/intel/COMPONENT-MODEL-RESEARCH-SYNTHESIS.md` — model design
- `ggen/intel/wit-surface-ledger.yaml` — WIT surface spec
- `ggen/rules/component-boundary-law.yaml` — boundary law
- `ggen/templates/wasm4pm-compat.wit.ggen` — WIT template
- `ggen/templates/witness-marker.tera` — witness encoding template

**Next Action:** Execute WIT codegen for all 7 feature configurations (none, formats, strict, wasm4pm, formats+strict, formats+wasm4pm, all-features). Validate each with wit-validator. Run wit-bindgen on each. Prove round-trip type preservation.

---

### GAP_LOSS: Loss Accounting Rules Gap

**Severity:** HIGH  
**Current Status:** MANUFACTURED  
**Classification Rationale:** Ledger marks `MANUFACTURED`. Core infrastructure (LossPolicy, LossReport, ProjectionName) implemented in crate. Closure condition is auto-detection + audit gate is format conversion trait impl coverage.

**Closure Condition:**
- Loss policies auto-detected from projection targets
- LossReport emitted on all lossy projections
- No silent structure loss (all loss traced and named)

**Audit Gate:**
```
All format conversions carry named policies; loss accounting trace complete
```

**Blockers:**
1. **Trait impl coverage** — are all lossy format pairs (OCEL→XES, OCEL→BPMN, etc.) implemented with LossPolicy?
2. **Auto-detection completeness** — does projection selection correctly identify all lossy paths?
3. **Loss naming** — does every LossReport name the specific structure elements that were lost?

**Evidence Files:**
- `ggen/intel/CARGO-FEATURE-AUDIT.md` — feature audit
- `ggen/intel/optional-dependency-law.yaml` — dependency rules
- `ggen/templates/module-docs.tera` — doc generation
- `ggen/audits/audit-feature-isolation.sh.ggen` — isolation audit

**Next Action:** Generate comprehensive loss matrix (from×to format pairs). For each pair: verify LossPolicy trait impl exists, verify LossReport is emitted, verify loss is named (not generic). Coverage: 100% of reachable format pairs.

---

### GAP_PROCESS_TREE: Process Tree Type Laws Gap

**Severity:** HIGH  
**Current Status:** MANUFACTURED  
**Classification Rationale:** Ledger marks `MANUFACTURED`. Compile-fail and compile-pass fixture templates exist. Closure condition is enforcement of arity, POWL soundness, projection legality at compile-time via type system.

**Closure Condition:**
- Process tree arity constraints enforced at compile-time
- POWL soundness laws verified (e.g., no unreachable nodes)
- Projection legality enforced (e.g., only onto valid subtrees)

**Audit Gate:**
```
Compile-fail fixtures prove:
  - Wrong arity rejected
  - Invalid projections rejected
  - Unsound POWL rejected
```

**Blockers:**
1. **Fixture completeness** — are all arity violations (0-arity, negative, overflow) covered?
2. **POWL law coverage** — are soundness violations (unreachable nodes, dead marks, live locks) tested?
3. **Projection legality** — are invalid projection targets (non-subtrees, skipped layers) rejected?

**Evidence Files:**
- `ggen/intel/RUST-PUBLIC-API-INTELLIGENCE-INDEX.md` — API surface
- `ggen/intel/non-projectable-type-ledger.yaml` — invalid projections
- `ggen/intel/projectable-type-ledger.yaml` — valid projections
- `ggen/templates/compile-fail-fixture.tera` — failure template
- `ggen/templates/compile-pass-fixture.tera` — success template

**Next Action:** Audit all compile-fail fixtures in `tests/ui/compile_fail/` against the projectable/non-projectable ledgers. Verify each fixture fails for the **intended named law** (e.g., `ArityViolation`, `UnreachableNode`), not for typos or missing imports. Measure coverage: what % of laws have ≥1 compile-fail receipt?

---

### GAP_TS: TypeScript Projection Gap

**Severity:** CRITICAL  
**Current Status:** IN_PROGRESS  
**Classification Rationale:** Ledger marks `MANUFACTURED`, but closure condition is `.d.ts surfaces generated from Rust types via specta + zero-cost phantom encoding`. Specta integration not yet proven.

**Closure Condition:**
- TypeScript `.d.ts` surfaces generated from Rust types via specta
- Zero-cost phantom encoding preserved (PhantomData invisible to .d.ts)
- Round-trip: Rust type → specta → `.d.ts` → TypeScript → back to Rust via serde

**Audit Gate:**
```
1. audit-no-dto-flattening.sh passes
2. specta codegen produces valid .d.ts
3. TypeScript types match Rust source of truth (no flattening, no field reordering)
```

**Blockers:**
1. **specta integration missing** — no evidence of `.d.ts` generation
2. **Zero-cost phantom validation** — how is it proven that PhantomData doesn't leak into .d.ts?
3. **DTO flattening audit** — does audit-no-dto-flattening.sh exist and pass?

**Evidence Files:**
- `ggen/intel/SPECTA-INTELLIGENCE-INDEX.md` — specta research
- `ggen/intel/specta-capability-map.md` — specta capabilities
- `ggen/intel/specta-ts-projection-candidates.yaml` — types to project
- `ggen/rules/ts-projection-law.yaml` — projection law
- `ggen/templates/ts-projection.rs.ggen` — projection template
- `ggen/audits/audit-no-dto-flattening.sh.ggen` — flattening audit

**Next Action:** Integrate specta into the build. Run codegen for all public types in the crate. Validate `.d.ts` output against audit criteria: no flattening, no field reordering, phantom types invisible. Measure: 100% of public types must have a passing specta projection.

---

### GAP_WASM: WASM ABI Boundary Gap

**Severity:** CRITICAL  
**Current Status:** IN_PROGRESS  
**Classification Rationale:** Ledger marks `MANUFACTURED`, but closure condition is memory-safe + type-law-respecting boundary. wasm-bindgen integration + prohibited list validation not yet proven.

**Closure Condition:**
- WASM boundary is memory-safe (no unsafe code at crossing; UB prevented by type system)
- Type law is respected (no law loss at boundary; witnesses preserved or explicitly rejected)
- wasm-bindgen bindings generated and validated

**Audit Gate:**
```
1. wasm-pack build succeeds
2. WASM boundary validates against prohibited list (no tool types, no OTel spans in ABI)
3. Memory safety proven (no dangling pointers, no use-after-free)
```

**Blockers:**
1. **wasm-pack build not executed** — no evidence of successful WASM compilation
2. **Prohibited list validation missing** — does audit-no-tools-in-compat.sh exist and pass?
3. **Memory safety audit** — how is it proven that the boundary is safe (not just type-safe)?

**Evidence Files:**
- `ggen/intel/WASM-ABI-INTELLIGENCE.md` — ABI research
- `ggen/intel/wasm-abi-map.yaml` — ABI surface map
- `ggen/intel/wasm-boundary-prohibited.yaml` — prohibited types
- `ggen/rules/wasm-boundary-law.yaml` — boundary law
- `ggen/templates/wasm-boundary.rs.ggen` — boundary template
- `ggen/audits/audit-no-tools-in-compat.sh.ggen` — prohibited audit
- `ggen/templates/wasm4pm-conformance.tera` — conformance template

**Next Action:** Build WASM target via wasm-pack. Execute audit-no-tools-in-compat.sh. Validate that all types crossing the boundary are either simple scalars or explicitly-approved composite types. Prove memory safety via inspection + automated checks (miri, valgrind, or similar). Target: 100% boundary audit pass.

---

## Dependency Graph

```
GAP_LOSS (HIGH) ✓
    ↓
GAP_PROCESS_TREE (HIGH) ✓
    ↓
GAP_COMPONENT (CRITICAL) → blocks → GAP_TS, GAP_WASM
    ↓
GAP_TS (CRITICAL)
GAP_WASM (CRITICAL)
    ↓
GAP_001 (HIGH) ← requires all above
```

**Critical Path:**
1. Resolve GAP_LOSS (already MANUFACTURED; validate coverage)
2. Resolve GAP_PROCESS_TREE (already MANUFACTURED; audit fixtures)
3. Resolve GAP_COMPONENT (templates exist; run codegen + validation)
4. Resolve GAP_TS & GAP_WASM in parallel (both template-driven; integrate specta + wasm-pack)
5. Resolve GAP_001 (integration tests; witness chain proof)

---

## Closure Checklist

- [ ] **GAP_LOSS:** Loss matrix 100% covered; all format pairs have LossPolicy + LossReport
- [ ] **GAP_PROCESS_TREE:** Compile-fail fixtures cover ≥90% of named laws; each fixture fails for intended law
- [ ] **GAP_COMPONENT:** WIT codegen runs for all 7 feature configs; wit-validator passes; wit-bindgen produces valid traits
- [ ] **GAP_TS:** specta integrated; `.d.ts` generated for all public types; audit-no-dto-flattening.sh passes
- [ ] **GAP_WASM:** wasm-pack builds successfully; audit-no-tools-in-compat.sh passes; boundary memory-safe
- [ ] **GAP_001:** Integration tests prove OCEL → discovery → PetriNet → conformance → receipt; witnesses preserved end-to-end

---

## Status Legend

- **MANUFACTURED:** Templates, audits, rules exist; codegen/validation may be pending
- **IN_PROGRESS:** Active integration work; some blockers identified
- **CLOSED:** All blockers resolved; closure checklist 100% complete

**Last Updated:** 2026-06-01
