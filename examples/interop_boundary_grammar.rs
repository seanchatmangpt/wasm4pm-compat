//! Interop boundary grammar — structure-only adoption vocabulary.
//!
//! Demonstrates the `interop` module: the smallest vocabulary a host needs to
//! describe what it is handing across the PM4Py / PMAx boundary and what it is
//! *claiming* about that handoff.
//!
//! - [`Pm4pyShape`] — 7 artifact-kind tags (EventLog, OCEL, PetriNet, …)
//! - [`FilterShape`] — 5 filter-dimension descriptors
//! - [`SummaryShape`] — 5 summary-family descriptors
//! - [`ConformanceTriple`] — 3-dimension conformance claim (no measured values)
//! - [`ArtifactGrounding<W>`] — evidence-reference binding with named refusals
//! - [`InteropRefusal`] — 5 named boundary law violations
//! - [`check_filter_shape`] — runtime shape-compatibility check
//! - [`OcelToXesProjection`] + [`XesToOcedProjection`] — named projection descriptors
//! - [`FilterShapeConst<IS_OC>`] + [`assert_filter_oc_compatible`] — compile-time shape law
//! - [`GraduationCandidate`] — sealed boundary marker
//!
//! **This crate does NOT compute.** It names shapes, claims, and refusals.
//! Graduate to `wasm4pm` for execution.
//!
//! Doc reference: `src/interop.rs`

use wasm4pm_compat::interop::{
    assert_filter_oc_compatible, check_filter_shape, graduation_seal, ArtifactGrounding,
    ConformanceTriple, FilterShape, FilterShapeConst, GraduationCandidate, InteropRefusal,
    OcedShape, OcelShape, OcelToXesProjection, Pm4pyShape, SummaryShape, XesShape,
    XesToOcedProjection,
};
use wasm4pm_compat::loss::LossPolicy;

fn main() {
    println!("=== Interop boundary grammar (structure only) ===\n");

    // ── Part 1: Pm4pyShape — 7 artifact-kind tags ─────────────────────────────
    println!("Part 1: Pm4pyShape tags and is_object_centric");

    let shapes = [
        (Pm4pyShape::EventLog, "event_log", false),
        (Pm4pyShape::ObjectCentricLog, "ocel", true),
        (Pm4pyShape::PetriNet, "petri_net", false),
        (Pm4pyShape::ProcessTree, "process_tree", false),
        (Pm4pyShape::Bpmn, "bpmn", false),
        (Pm4pyShape::DirectlyFollowsGraph, "dfg", false),
        (Pm4pyShape::Declare, "declare", false),
    ];
    for (shape, expected_tag, expected_oc) in &shapes {
        assert_eq!(shape.tag(), *expected_tag, "tag mismatch for {:?}", shape);
        assert_eq!(
            shape.is_object_centric(),
            *expected_oc,
            "oc mismatch for {:?}",
            shape
        );
        println!(
            "  ✓ {:?}: tag={}, is_object_centric={}",
            shape, expected_tag, expected_oc
        );
    }

    // ── Part 2: FilterShape and SummaryShape ───────────────────────────────────
    println!("\nPart 2: FilterShape and SummaryShape variants");

    let filters = [
        FilterShape::Activity,
        FilterShape::Timeframe,
        FilterShape::Variant,
        FilterShape::Attribute,
        FilterShape::ObjectType,
    ];
    for f in &filters {
        println!("  ✓ FilterShape::{:?}", f);
    }

    let summaries = [
        SummaryShape::Counts,
        SummaryShape::TraceVariants,
        SummaryShape::ActivityDistribution,
        SummaryShape::TimingProfile,
        SummaryShape::ObjectTypeDistribution,
    ];
    for s in &summaries {
        println!("  ✓ SummaryShape::{:?}", s);
    }

    // ── Part 3: ConformanceTriple — 3-dimension claim ─────────────────────────
    println!("\nPart 3: ConformanceTriple");

    let t = ConformanceTriple::fitness_and_precision();
    assert!(t.claims_fitness);
    assert!(t.claims_precision);
    assert!(!t.claims_generalization);
    assert_eq!(t.claimed_count(), 2);
    assert!(t.is_grounded());
    println!("  ✓ fitness_and_precision: claimed_count=2, is_grounded=true");

    let full = ConformanceTriple {
        claims_fitness: true,
        claims_precision: true,
        claims_generalization: true,
    };
    assert_eq!(full.claimed_count(), 3);
    println!("  ✓ all three: claimed_count=3");

    let empty = ConformanceTriple {
        claims_fitness: false,
        claims_precision: false,
        claims_generalization: false,
    };
    assert!(!empty.is_grounded());
    println!("  ✓ empty triple: is_grounded=false (vacuous claim)");

    // ── Part 4: ArtifactGrounding — evidence binding with named refusals ───────
    println!("\nPart 4: ArtifactGrounding refusals");

    // Lawful grounding
    let g = ArtifactGrounding::<()>::new(Pm4pyShape::EventLog, "blake3:abc123");
    assert!(g.is_grounded());
    assert!(g.admit_flat().is_ok());
    println!("  ✓ EventLog grounded + admit_flat() ok");

    // UngroundedArtifact — empty evidence ref
    let bad = ArtifactGrounding::<()>::new(Pm4pyShape::PetriNet, "");
    assert!(!bad.is_grounded());
    assert_eq!(bad.admit_flat(), Err(InteropRefusal::UngroundedArtifact));
    println!("  ✓ empty ref → UngroundedArtifact");

    // FlatClaimOverObjectCentric — OCEL admitted as flat
    let ocel_g = ArtifactGrounding::<()>::new(Pm4pyShape::ObjectCentricLog, "ocel:fixture-1");
    assert_eq!(
        ocel_g.admit_flat(),
        Err(InteropRefusal::FlatClaimOverObjectCentric)
    );
    println!("  ✓ ObjectCentricLog via admit_flat() → FlatClaimOverObjectCentric");

    // ── Part 5: InteropRefusal — 5 named laws ─────────────────────────────────
    println!("\nPart 5: InteropRefusal named law strings");

    let refusals = [
        (InteropRefusal::UngroundedArtifact, "UngroundedArtifact"),
        (
            InteropRefusal::FlatClaimOverObjectCentric,
            "FlatClaimOverObjectCentric",
        ),
        (
            InteropRefusal::VacuousConformanceClaim,
            "VacuousConformanceClaim",
        ),
        (
            InteropRefusal::DimensionShapeMismatch,
            "DimensionShapeMismatch",
        ),
        (
            InteropRefusal::UnadmittedRawInterpretation,
            "UnadmittedRawInterpretation",
        ),
    ];
    for (r, expected_law) in &refusals {
        assert_eq!(r.law(), *expected_law, "law() mismatch");
        let displayed = format!("{r}");
        assert!(
            displayed.contains(expected_law),
            "Display must contain law name"
        );
        println!("  ✓ {}", expected_law);
    }

    // ── Part 6: check_filter_shape — runtime shape-compatibility ──────────────
    println!("\nPart 6: check_filter_shape runtime gate");

    assert!(check_filter_shape(Pm4pyShape::EventLog, FilterShape::Activity).is_ok());
    assert!(check_filter_shape(Pm4pyShape::ObjectCentricLog, FilterShape::ObjectType).is_ok());
    let mismatch = check_filter_shape(Pm4pyShape::EventLog, FilterShape::ObjectType);
    assert_eq!(mismatch, Err(InteropRefusal::DimensionShapeMismatch));
    println!("  ✓ EventLog+Activity → ok");
    println!("  ✓ ObjectCentricLog+ObjectType → ok");
    println!("  ✓ EventLog+ObjectType → DimensionShapeMismatch");

    // ── Part 7: OcelToXesProjection + XesToOcedProjection ─────────────────────
    println!("\nPart 7: Projection descriptors");

    let ocel_to_xes = OcelToXesProjection::new("order");
    assert_eq!(ocel_to_xes.case_type(), "order");
    assert_eq!(
        ocel_to_xes.projection_name().as_str(),
        "ocel-flatten-to-xes:by-case-type"
    );
    let report = ocel_to_xes.clone().project(LossPolicy::AllowLossWithReport);
    assert!(report.is_ok());
    println!("  ✓ OcelToXesProjection: case_type=order, projection_name correct, project() ok");

    // Shape markers are zero-sized uninhabited enums
    assert_eq!(core::mem::size_of::<OcelShape>(), 0);
    assert_eq!(core::mem::size_of::<XesShape>(), 0);
    assert_eq!(core::mem::size_of::<OcedShape>(), 0);
    println!("  ✓ OcelShape/XesShape/OcedShape are zero-sized uninhabited markers");

    let xes_to_oced = XesToOcedProjection::new("order");
    assert_eq!(xes_to_oced.introduced_object_type(), "order");
    assert_eq!(
        xes_to_oced.projection_name().as_str(),
        "xes-lift-to-oced:by-case-type"
    );
    // RefuseLoss must refuse (lifting always has structural loss)
    let refused = xes_to_oced.clone().project(LossPolicy::RefuseLoss);
    assert!(refused.is_err());
    let allowed = xes_to_oced.project(LossPolicy::AllowLossWithReport);
    assert!(allowed.is_ok());
    println!("  ✓ XesToOcedProjection: RefuseLoss→err, AllowLossWithReport→ok");

    // ── Part 8: FilterShapeConst compile-time gate ────────────────────────────
    println!("\nPart 8: FilterShapeConst compile-time object-centric law");

    // Only FilterShapeConst<true> passes the RequiresObjectCentric bound.
    assert_filter_oc_compatible(&FilterShapeConst::<true>);
    // FilterShapeConst<false> would be a compile error:
    //   assert_filter_oc_compatible(&FilterShapeConst::<false>); // DimensionShapeMismatch law
    println!("  ✓ FilterShapeConst<true> passes assert_filter_oc_compatible");
    println!("  ✓ FilterShapeConst<false> would be compile error (DimensionShapeMismatch)");

    // ── Part 9: GraduationCandidate sealed marker ─────────────────────────────
    println!("\nPart 9: GraduationCandidate sealed marker");

    struct PendingDiscovery;
    impl graduation_seal::Sealed for PendingDiscovery {}
    impl GraduationCandidate for PendingDiscovery {}

    fn only_candidates<T: GraduationCandidate>(_: &T) {
        println!("  ✓ GraduationCandidate bound satisfied");
    }
    only_candidates(&PendingDiscovery);
    println!("  ✓ Sealed marker: third-party types cannot implement without graduation_seal");

    println!("\n=== All assertions passed — interop module surface is witnessed ===");
    println!("  Covered: Pm4pyShape (7 tags), FilterShape, SummaryShape,");
    println!("           ConformanceTriple (is_grounded + claimed_count),");
    println!("           ArtifactGrounding (lawful + 2 refusals),");
    println!("           InteropRefusal (5 named laws + Display),");
    println!("           check_filter_shape (ok + DimensionShapeMismatch),");
    println!("           OcelToXesProjection + XesToOcedProjection (Project impls),");
    println!("           OcelShape/XesShape/OcedShape (zero-sized markers),");
    println!("           FilterShapeConst<IS_OC> + assert_filter_oc_compatible,");
    println!("           GraduationCandidate sealed trait.");
    println!("  Structure only — no PM4Py objects, no discovery, no replay.");
    println!(
        "  Graduate to wasm4pm for: filter execution, summary tallying, conformance measurement."
    );
}

// Required: Project impl for OcelToXesProjection needs the Project trait in scope.
use wasm4pm_compat::loss::Project;
