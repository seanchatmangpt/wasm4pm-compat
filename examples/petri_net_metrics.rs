//! Example: PetriNet structural metrics — shape-only, no log-based scoring
//!
//! Demonstrates the structural metric surface of `wasm4pm_compat::models::PetriNet`:
//! - `is_structural_workflow_net()` — degree-counting verdict on the net's own shape
//! - `structural_unsoundness_score()` — bitset-algebra defect count (0.0 = sound structure)
//! - `mdl_score()` and `mdl_score_with_ontology()` — structural description length
//! - `explain()` — self-derived summary; every clause comes from `self`
//! - `canonical_hash()` — deterministic structural fingerprint
//! - `incidence_matrix()` — compiled flat incidence matrix with `FlatIncidenceMatrix::get()`
//! - `validate()` — named `PetriNetRefusal` laws
//!
//! **Failure witness:** each metric is asserted against a known structural value
//! derived from the net's own shape. `explain()` is checked to contain the
//! node counts and to differ between two structurally different nets. If
//! `is_structural_workflow_net()` returns the wrong value, or `explain()` is
//! decoupled from `self`, the assertions fail and this example exits non-zero.
//!
//! Structure only — no token replay, no fitness measurement, no log needed.
//! Graduate to `wasm4pm` for: replay-based fitness, trace alignment, precision.
//!
//! Run: `cargo run --example petri_net_metrics`
//! Doc reference: `src/models.rs`, `docs/API_TOUR.md`

use wasm4pm_compat::models::{Arc, PetriNet, PetriNetRefusal, Place, Transition};
use wasm4pm_compat::petri::Marking;

/// Build a minimal sound WF-net: i → t → o (one source, one sink, connected).
fn minimal_wf_net() -> PetriNet {
    PetriNet::new(
        [Place::new("i"), Place::new("o")],
        [Transition::new("t", "do")],
        [
            Arc::place_to_transition("i", "t"),
            Arc::transition_to_place("t", "o"),
        ],
        Marking::new([("i".to_string(), 1)]),
    )
}

/// Build a non-WF net: two source places (structural defect).
fn multi_source_net() -> PetriNet {
    PetriNet::new(
        [Place::new("p1"), Place::new("p2"), Place::new("sink")],
        [Transition::new("t1", "A"), Transition::new("t2", "B")],
        [
            Arc::place_to_transition("p1", "t1"),
            Arc::place_to_transition("p2", "t2"),
            Arc::transition_to_place("t1", "sink"),
            Arc::transition_to_place("t2", "sink"),
        ],
        Marking::new([("p1".to_string(), 1), ("p2".to_string(), 1)]),
    )
}

fn main() {
    println!("=== petri_net_metrics ===");
    println!("Structural metrics: no replay, no log, no conformance scoring.\n");

    let wf = minimal_wf_net();
    let multi = multi_source_net();

    // ── 1. is_structural_workflow_net ────────────────────────────────────────
    println!("--- is_structural_workflow_net ---");
    assert!(
        wf.is_structural_workflow_net(),
        "minimal WF-net should be structural WF-net"
    );
    println!("  minimal WF-net → true  ✓");
    assert!(
        !multi.is_structural_workflow_net(),
        "multi-source net is not a WF-net"
    );
    println!("  multi-source net → false  ✓");
    // Empty net is not a WF-net
    assert!(!PetriNet::default().is_structural_workflow_net());
    println!("  empty net → false  ✓");

    // ── 2. structural_unsoundness_score ──────────────────────────────────────
    println!("\n--- structural_unsoundness_score ---");
    let wf_score = wf.structural_unsoundness_score();
    assert_eq!(wf_score, 0.0, "WF-net score should be 0.0, got {wf_score}");
    println!("  minimal WF-net score = {wf_score:.1}  ✓");
    let multi_score = multi.structural_unsoundness_score();
    assert!(
        multi_score > 0.0,
        "multi-source net score should be > 0, got {multi_score}"
    );
    println!("  multi-source net score = {multi_score:.1} (> 0 = defects present)  ✓");
    // Empty net returns 10.0 (sentinel)
    let empty_score = PetriNet::default().structural_unsoundness_score();
    assert_eq!(empty_score, 10.0);
    println!("  empty net score = {empty_score:.1} (sentinel)  ✓");

    // ── 3. mdl_score / mdl_score_with_ontology ───────────────────────────────
    println!("\n--- mdl_score ---");
    // minimal WF-net: 1 transition, 2 arcs, vocab=1, log2(1)=0 => 1.0 + 2*0 = 1.0
    let mdl = wf.mdl_score();
    assert_eq!(mdl, 1.0, "mdl_score mismatch: got {mdl}");
    println!("  minimal WF-net mdl_score() = {mdl:.1}  ✓");
    // with ontology size 4: 1 + 2*log2(4) = 1 + 2*2 = 5.0
    let mdl4 = wf.mdl_score_with_ontology(Some(4));
    assert_eq!(mdl4, 5.0, "mdl_score_with_ontology(4) mismatch: got {mdl4}");
    println!("  mdl_score_with_ontology(4) = {mdl4:.1}  ✓");
    // empty net returns 0.0 (no transitions)
    assert_eq!(PetriNet::default().mdl_score(), 0.0);
    println!("  empty net mdl_score() = 0.0  ✓");

    // ── 4. explain — self-derived, not aspirational ──────────────────────────
    println!("\n--- explain ---");
    let summary = wf.explain();
    assert!(
        summary.contains("2 places"),
        "explain missing place count: {summary}"
    );
    assert!(
        summary.contains("1 transitions"),
        "explain missing transition count: {summary}"
    );
    assert!(
        summary.contains("2 arcs"),
        "explain missing arc count: {summary}"
    );
    assert!(
        summary.contains("true"),
        "explain should report WF-net=true: {summary}"
    );
    assert!(
        summary.contains("0.0"),
        "explain should report unsoundness=0.0: {summary}"
    );
    println!("  explain() = \"{summary}\"  ✓");
    // Different nets produce different summaries
    let multi_summary = multi.explain();
    assert_ne!(
        summary, multi_summary,
        "structurally distinct nets must differ in explain()"
    );
    println!("  explain() differs for distinct structures  ✓");

    // ── 5. canonical_hash — deterministic fingerprint ────────────────────────
    println!("\n--- canonical_hash ---");
    let h1 = minimal_wf_net().canonical_hash();
    let h2 = minimal_wf_net().canonical_hash();
    assert_eq!(h1, h2, "same structure must hash identically");
    println!("  same structure → same hash ({h1:#x})  ✓");
    let h3 = multi.canonical_hash();
    assert_ne!(h1, h3, "distinct structures must hash differently");
    println!("  distinct structures → different hashes  ✓");

    // ── 6. incidence_matrix ──────────────────────────────────────────────────
    println!("\n--- incidence_matrix ---");
    // Build with two-place, one-transition net for predictable incidence
    let net2 = PetriNet::new(
        [Place::new("p1"), Place::new("p2")],
        [Transition::new("t1", "A")],
        [
            Arc::place_to_transition("p1", "t1"),
            Arc::transition_to_place("t1", "p2"),
        ],
        Marking::new([("p1".to_string(), 1)]),
    );
    let matrix = net2.incidence_matrix();
    assert_eq!(matrix.places_count, 2);
    assert_eq!(matrix.transitions_count, 1);
    assert_eq!(matrix.get(0, 0), -1, "p1 consumed by t1: should be -1");
    assert_eq!(matrix.get(1, 0), 1, "p2 produced by t1: should be +1");
    println!(
        "  matrix[p1,t1]={} (consumed)  matrix[p2,t1]={} (produced)  ✓",
        matrix.get(0, 0),
        matrix.get(1, 0)
    );

    // ── 7. validate — PetriNetRefusal ────────────────────────────────────────
    println!("\n--- PetriNetRefusal ---");
    let empty_net = PetriNet::default();
    let refusal = empty_net.validate().unwrap_err();
    assert_eq!(refusal, PetriNetRefusal::EmptyNet);
    println!("  empty net → PetriNetRefusal::EmptyNet  ✓");

    println!("\n=== All assertions passed — models::PetriNet structural metrics witnessed ===");
    println!("  Covered: is_structural_workflow_net, structural_unsoundness_score,");
    println!("           mdl_score, mdl_score_with_ontology, explain (self-derived),");
    println!("           canonical_hash, incidence_matrix, PetriNetRefusal::EmptyNet.");
    println!("  Witness: all metric values asserted against known structural inputs;");
    println!("           explain() checked for node counts, not static strings.");
    println!("  Structure only — no log, no replay, no fitness measurement.");
    println!("  Graduate to wasm4pm for: token replay, trace alignment, precision scoring.");
}
