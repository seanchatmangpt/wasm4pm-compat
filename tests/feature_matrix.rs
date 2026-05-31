//! Feature-matrix smoke test.
//!
//! This file is intentionally **feature-agnostic**: it uses only always-on items
//! (the `interop` module) so it compiles and passes under every feature
//! combination — `--no-default-features`, `--features formats`, `--features strict`,
//! `--features wasm4pm`, and `--all-features`. Its job is to prove the crate links
//! and the always-on adoption grammar is present in every build.

use wasm4pm_compat::interop::{
    check_filter_shape, ArtifactGrounding, ConformanceTriple, FilterShape, InteropRefusal,
    Pm4pyShape, SummaryShape,
};

#[test]
fn always_on_interop_is_present_in_every_build() {
    // Shape tags are stable.
    assert_eq!(Pm4pyShape::EventLog.tag(), "event_log");
    assert!(Pm4pyShape::ObjectCentricLog.is_object_centric());
    assert!(!Pm4pyShape::EventLog.is_object_centric());

    // A grounded artifact admits on the flat path; an OCEL one refuses by name.
    let flat = ArtifactGrounding::<()>::new(Pm4pyShape::EventLog, "ev:1");
    assert!(flat.admit_flat().is_ok());

    let ocel = ArtifactGrounding::<()>::new(Pm4pyShape::ObjectCentricLog, "oc:1");
    assert_eq!(
        ocel.admit_flat(),
        Err(InteropRefusal::FlatClaimOverObjectCentric)
    );

    // Ungrounded artifacts refuse with their own named law.
    let ungrounded = ArtifactGrounding::<()>::new(Pm4pyShape::PetriNet, "");
    assert_eq!(
        ungrounded.admit_flat(),
        Err(InteropRefusal::UngroundedArtifact)
    );
}

#[test]
fn conformance_triple_grounding_is_structural() {
    let t = ConformanceTriple::fitness_and_precision();
    assert_eq!(t.claimed_count(), 2);
    assert!(t.is_grounded());

    let empty = ConformanceTriple {
        claims_fitness: false,
        claims_precision: false,
        claims_generalization: false,
    };
    assert!(!empty.is_grounded());
}

#[test]
fn filter_shape_dimension_mismatch_is_named() {
    // Object-type filtering over a flat log is a named refusal.
    assert_eq!(
        check_filter_shape(Pm4pyShape::EventLog, FilterShape::ObjectType),
        Err(InteropRefusal::DimensionShapeMismatch)
    );
    // Activity filtering is always structurally admissible.
    assert!(check_filter_shape(Pm4pyShape::EventLog, FilterShape::Activity).is_ok());
    // Object-type filtering over an OCEL is fine.
    assert!(check_filter_shape(Pm4pyShape::ObjectCentricLog, FilterShape::ObjectType).is_ok());
}

#[test]
fn summary_shapes_are_distinct() {
    // A trivial structural sanity check that the summary vocabulary is usable.
    let shapes = [
        SummaryShape::Counts,
        SummaryShape::TraceVariants,
        SummaryShape::ActivityDistribution,
        SummaryShape::TimingProfile,
        SummaryShape::ObjectTypeDistribution,
    ];
    assert_eq!(shapes.len(), 5);
    assert_ne!(SummaryShape::Counts, SummaryShape::TraceVariants);
}
