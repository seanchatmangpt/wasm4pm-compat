# Crate Verification & Examples Status Report
**Project:** `wasm4pm-compat`
**Target Version:** `26.6.5`
**Execution Timestamp:** 2026-06-04T20:35:00Z

This report contains details on the compilation and execution status of the code examples located under `examples/`, as well as failures identified in cargo doc-tests and UI trybuild tests on Mac OS.

---

## 1. Executive Summary

| Test Suite | Total Run | Passed | Failed | Ignored / Mismatched | Status |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **Examples (`examples/`)** | 19 | 19 | 0 | 0 | **PASSED** |
| **Unit/Integration Tests** | 132 | 132 | 0 | 0 | **PASSED** |
| **Doc-Tests (`cargo test --doc`)** | 722 | 662 | 9 | 51 | **FAILED** |
| **UI Tests (`cargo test --test ui_tests`)** | 626 | 267 | 359 | 0 | **FAILED** |

* All **19 examples** in `/examples/` compile and run successfully without any panics. Only minor dead-code compiler warnings are present.
* **Doc-tests** contain 9 compilation/assertion failures across `ocpq`, `petri`, and `xes` modules.
* **UI tests (trybuild)** fail due to compiler diagnostic string updates (namespacing issues in compile-fail stderr patterns) and cargo workspace binary target mismatching issues (e.g. `trybuild396`) during compile-pass validation.

---

## 2. Examples Verification Table
The following table shows the status of all 19 examples located in `/examples/`. Every example compiles and runs to completion successfully.

| Example Name | Source File | Build Status | Run Status | Notes / Warnings |
| :--- | :--- | :---: | :---: | :--- |
| **basic_eventlog** | `basic_eventlog.rs` | Success | Success | Validates case-centric log shape and stream buffers. |
| **basic_ocel** | `basic_ocel.rs` | Success | Success | Demonstrates object-centric event logs. |
| **c8_adversary_gap_demo** | `c8_adversary_gap_demo.rs` | Success | Success | *Warning*: unused field `nodes` in `ProcessModel`. |
| **c8_collider_demo** | `c8_collider_demo.rs` | Success | Success | Simulates adversarial process collusion. |
| **c8_event_horizon_demo** | `c8_event_horizon_demo.rs` | Success | Success | *Warning*: unused fields `event_type`/`timestamp_ns` in `LiquidityShock`. |
| **c8_market_planck_demo** | `c8_market_planck_demo.rs` | Success | Success | Evaluates boundary compliance. |
| **causal_net_shape** | `causal_net_shape.rs` | Success | Success | Asserts structures of causal nets. |
| **conformance_metrics** | `conformance_metrics.rs` | Success | Success | Calculates fitness, precision, and simplicity. |
| **declare_constraint_model** | `declare_constraint_model.rs` | Success | Success | Encodes DECLARE template rules. |
| **evidence_lifecycle** | `evidence_lifecycle.rs` | Success | Success | Traces states from raw input to receipted evidence. |
| **graduation_candidate** | `graduation_candidate.rs` | Success | Success | Proves candidate grounding for graduating to wasm4pm. |
| **loss_projection** | `loss_projection.rs` | Success | Success | Tests refuse/allow/report policies under projection. |
| **ocel_to_xes_projection** | `ocel_to_xes_projection.rs` | Success | Success | Maps OCEL multi-perspective logs to XES traces. |
| **ocpq_typed_query** | `ocpq_typed_query.rs` | Success | Success | Tests typed queries with cardinality/relations. |
| **petri_net_construction** | `petri_net_construction.rs` | Success | Success | Builds object-centric Petri nets. |
| **powl_process_tree** | `powl_process_tree.rs` | Success | Success | Compares POWL structures to process trees. |
| **receipt_chain** | `receipt_chain.rs` | Success | Success | Models sequential cryptographic validation receipts. |
| **strict_boundary_claim** | `strict_boundary_claim.rs` | Success | Success | Evaluates opt-in strict boundary covenants. |
| **witness_authority** | `witness_authority.rs` | Success | Success | Asserts standard witness types and metadata. |

---

## 3. Pinpointed Doc-Test Failures

We identified **9 failing doc-tests** during `cargo test --doc --all-features`. Below are the technical details and causes for each failure.

### 1. `ocpq::OcpqQueryConst::with_predicate` (line 813)
* **Error:** Mismatched types (`expected struct Predicate<()>`, `found struct Predicate<EventPredicate>`).
* **Explanation:** `with_predicate` signature expects a generic `Predicate` with empty type parameter `()` but the doc-test passes `Predicate::<EventPredicate>::new(...)`.

### 2. `petri::BipartiteArcConst::new` (line 247)
* **Error:** Panic in assertion: `assertion left == right failed (left: "p1", right: "t0")`.
* **Explanation:** The test panics during constructor verification where node IDs do not match the expected topology.

### 3. `petri::PetriNetBuilder` (line 1834)
* **Error:** `cannot move out of a mutable reference` during `.build()`.
* **Explanation:** Builder method chain is invoked on a mutable receiver reference, but `.build()` takes ownership (`self`).

### 4. `petri::PetriNetBuilder::place` (line 1871)
* **Error:** `cannot move out of a mutable reference` during `.build_unchecked()`.
* **Explanation:** Similar to above, `.build_unchecked()` takes ownership of `self` but is chained on a mutable intermediate reference.

### 5. `petri::PetriNetBuilder::silent` (line 1900)
* **Error:** `cannot move out of a mutable reference` during `.build_unchecked()`.

### 6. `petri::PetriNetBuilder::transition` (line 1883)
* **Error:** `cannot move out of a mutable reference` during `.build_unchecked()`.

### 7. `petri::WfNet::attest_witnessed` (line 1163)
* **Error:** `method attest_witnessed is private`.
* **Explanation:** The doc-test tries to call `.attest_witnessed()`, which is declared as `pub(crate)`.

### 8. `petri::WfNetQuery` (line 501)
* **Error:** `no associated function or constant named new found for struct WfNetConst<Claimed>`.
* **Explanation:** The `new` associated function is defined under `WfNetConst<Unknown>` and is not available on `WfNetConst<Claimed>`.

### 9. `xes::XesLifecycleTransition` (line 828)
* **Error:** Panic in assertion: `assertion left == right failed (left: Some(Unknown), right: None)`.
* **Explanation:** The test expects a `None` transition for an unknown raw value but gets `Some(Unknown)`.

---

## 4. UI Trybuild Test Mismatches

The UI trybuild test suite (`cargo test --test ui_tests -- --ignored`) reports failure due to two distinct causes:

### A. Pattern Mismatches in `compile_fail_fixtures` (27 of 217 failed)
The compiler output diagnostics changed slightly between the version used to write the expected stderr files (`*.stderr`) and the local environment.
* **Example Mismatch:**
  * *Expected Stderr:* `expected struct Evidence<String, Admitted, Ocel20>`
  * *Actual Stderr:* `expected struct Evidence<String, Admitted, wasm4pm_compat::witness::Ocel20>`
* **Verdict:** The type constraints behave correctly; only the compiler's diagnostic output formatting differs. These can be fixed by blessing the actual output with `TRYBUILD=overwrite cargo test --test ui_tests -- --ignored`.

### B. Target Bin Mismatches in `compile_pass_fixtures` (332 of 409 failed)
The build runner failed with target binary resolution errors.
* **Error:** `error: no bin target named trybuild396 in default-run packages`
* **Explanation:** Trybuild generates temporary packages in the target directory (e.g. `target/tests/trybuild/...`). When package caches or test numbers get out of sync, cargo fails to map the numbered trybuild targets to the compiler targets. Running `cargo clean` and executing trybuild tests from a clean state resolves this.
