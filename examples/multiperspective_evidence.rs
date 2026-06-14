//! Multi-perspective process evidence shapes — Mannhardt et al. (2016).
//!
//! Demonstrates the `multiperspective` module:
//!
//! - [`ProcessPerspective`] — four classic perspectives with `Display`
//! - [`ControlFlowPerspective`] / [`DataPerspective`] / [`ResourcePerspective`] /
//!   [`TimePerspective`] — zero-cost perspective marker types
//! - [`MultiPerspectiveEvidence<T, Perspectives>`] — evidence tagged with a
//!   perspective combination at the type level
//! - [`PerspectiveCombination<A, B>`] — compose two perspectives into one type
//! - [`ParityComparer`] — epsilon-close float comparison utility
//!
//! **The four-perspective framework (van der Aalst):** control-flow (what and
//! in what order), data (attributes and guards), resource (who performs what),
//! time (when, how long). Each perspective is a distinct zero-sized type;
//! `MultiPerspectiveEvidence` threads perspective coverage through the type
//! system so a function that expects time-annotated evidence cannot accidentally
//! receive control-flow-only evidence.
//!
//! **Failure witness:** `Display` strings, `inner` field values, and `assert_epsilon_close`
//! are all asserted — renames or removals break this example.
//!
//! Doc reference: `src/multiperspective.rs`, `docs/API_TOUR.md`

use core::marker::PhantomData;
use wasm4pm_compat::multiperspective::{
    ControlFlowPerspective, DataPerspective, MultiPerspectiveEvidence, ParityComparer,
    PerspectiveCombination, ProcessPerspective, ResourcePerspective, TimePerspective,
};

fn main() {
    println!("=== Multi-perspective process evidence shapes ===\n");

    // ── Part 1: ProcessPerspective — four perspectives with Display ───────────
    println!("Part 1: ProcessPerspective — four classic perspectives");

    let perspectives = [
        (ProcessPerspective::ControlFlow, "control-flow"),
        (ProcessPerspective::Data,        "data"),
        (ProcessPerspective::Resource,    "resource"),
        (ProcessPerspective::Time,        "time"),
    ];
    for (p, expected) in &perspectives {
        assert_eq!(format!("{p}"), *expected);
        println!("  ✓ {expected}");
    }

    // ── Part 2: Perspective marker types — zero-cost, distinct ───────────────
    println!("\nPart 2: Perspective marker types");

    // Each perspective is a distinct zero-sized type. In PhantomData position,
    // they prevent substituting one perspective for another.
    struct EvidenceSlot<P>(PhantomData<P>);

    let _cf: EvidenceSlot<ControlFlowPerspective> = EvidenceSlot(PhantomData);
    let _dp: EvidenceSlot<DataPerspective>        = EvidenceSlot(PhantomData);
    let _rp: EvidenceSlot<ResourcePerspective>    = EvidenceSlot(PhantomData);
    let _tp: EvidenceSlot<TimePerspective>        = EvidenceSlot(PhantomData);
    // Each is a different type — the compiler would reject substitution.
    println!("  ✓ ControlFlowPerspective, DataPerspective, ResourcePerspective, TimePerspective");
    println!("    are distinct zero-sized types");

    // ── Part 3: MultiPerspectiveEvidence<T, P> — single perspective ──────────
    println!("\nPart 3: MultiPerspectiveEvidence — single perspective");

    let cf_evidence: MultiPerspectiveEvidence<u32, ControlFlowPerspective> =
        MultiPerspectiveEvidence::new(42);
    assert_eq!(cf_evidence.inner, 42);
    println!("  ✓ MultiPerspectiveEvidence::<u32, ControlFlowPerspective>::new(42).inner = 42");

    let resource_evidence: MultiPerspectiveEvidence<&str, ResourcePerspective> =
        MultiPerspectiveEvidence::new("org:Alice");
    assert_eq!(resource_evidence.inner, "org:Alice");
    println!("  ✓ MultiPerspectiveEvidence::<&str, ResourcePerspective>.inner = \"org:Alice\"");

    // ── Part 4: PerspectiveCombination<A, B> — combining two perspectives ────
    println!("\nPart 4: PerspectiveCombination — composing perspectives");

    // Control-flow + Data combined into one perspective type.
    type FlowAndData = PerspectiveCombination<ControlFlowPerspective, DataPerspective>;
    let combined_evidence: MultiPerspectiveEvidence<u64, FlowAndData> =
        MultiPerspectiveEvidence::new(100_u64);
    assert_eq!(combined_evidence.inner, 100_u64);
    println!("  ✓ PerspectiveCombination<ControlFlow, Data>: inner = 100");

    // Three perspectives: nest two PerspectiveCombinations.
    type FlowDataResource = PerspectiveCombination<
        FlowAndData,
        ResourcePerspective,
    >;
    let triple: MultiPerspectiveEvidence<&str, FlowDataResource> =
        MultiPerspectiveEvidence::new("three-perspective evidence");
    assert_eq!(triple.inner, "three-perspective evidence");
    println!("  ✓ PerspectiveCombination<FlowAndData, Resource>: nested combination");

    // All four perspectives.
    type AllFour = PerspectiveCombination<FlowDataResource, TimePerspective>;
    let full: MultiPerspectiveEvidence<bool, AllFour> =
        MultiPerspectiveEvidence::new(true);
    assert!(full.inner);
    println!("  ✓ All four perspectives combined: inner = true");

    // ── Part 5: ParityComparer — epsilon-close float comparison ───────────────
    println!("\nPart 5: ParityComparer::assert_epsilon_close");

    // This utility is used in the multi-perspective conformance framework
    // to compare perspective weights that might differ by floating-point noise.
    ParityComparer::assert_epsilon_close(0.9_f64, 0.9_f64);
    println!("  ✓ assert_epsilon_close(0.9, 0.9) — passes (exact match)");

    ParityComparer::assert_epsilon_close(0.9_f64, 0.9 + 1e-7);
    println!("  ✓ assert_epsilon_close(0.9, 0.9+1e-7) — passes (within epsilon)");

    // Values closer than 1e-6 don't trigger the violation.
    ParityComparer::assert_epsilon_close(1.0_f64, 1.0 - 5e-7);
    println!("  ✓ assert_epsilon_close(1.0, 1.0-5e-7) — passes (within epsilon)");

    // ── Part 6: Structure-only contract ──────────────────────────────────────
    println!("\nPart 6: Structure-only contract");
    println!("  ✓ No alignment method or conformance checker on MultiPerspectiveEvidence");
    println!("  ✓ Graduate to wasm4pm for: multi-perspective alignment, cost weighting,");
    println!("    balanced conformance checking (Mannhardt et al. 2016)");

    println!("\n=== All assertions passed — multiperspective module surface is witnessed ===");
    println!("  Covered: ProcessPerspective (4 kinds + Display), ControlFlowPerspective,");
    println!("           DataPerspective, ResourcePerspective, TimePerspective,");
    println!("           MultiPerspectiveEvidence (single + combined perspectives),");
    println!("           PerspectiveCombination (2, 3, and 4-way nesting), ParityComparer.");
}
