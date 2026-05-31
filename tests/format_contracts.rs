//! Format covenant contract tests (gated on the `formats` feature).
//!
//! Run with: `cargo test --test format_contracts --features formats` (or with
//! default features, since `formats` is on by default).
//!
//! These tests exercise the *owned* concrete covenant surface — [`FormatEnvelope`],
//! [`FormatExport`], [`FormatKind`], [`RoundTripClaim`] — directly. The
//! [`ExportFormat`] impl is fully *runnable*: it consumes the real
//! [`wasm4pm_compat::loss::LossPolicy`] and produces a loss-accountable export or a
//! named refusal. The [`ImportFormat`] impl is proved *implementable* in shape; its
//! `Admission`/`Refusal`-minting body is deferred (`todo!`) and not invoked, since
//! a concrete admitter belongs to an adopter, not to this structure-only contract
//! test.

#![cfg(feature = "formats")]

use wasm4pm_compat::formats::{
    ExportFormat, FormatEnvelope, FormatExport, FormatKind, ImportFormat, RoundTripClaim,
};
use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

#[test]
fn envelope_carries_raw_bytes_and_tag() {
    let env = FormatEnvelope::<()>::new(FormatKind::OcelJson, b"{\"ocel\":1}".to_vec());
    assert_eq!(env.kind, FormatKind::OcelJson);
    assert!(env.kind.is_object_centric());
    assert_eq!(env.len(), 10);
    assert!(!env.is_empty());
}

#[test]
fn empty_envelope_is_detectable_for_refusal() {
    let env = FormatEnvelope::<()>::new(FormatKind::XesXml, Vec::new());
    assert!(env.is_empty(), "an empty envelope must be refusable at import");
    assert!(!env.kind.is_object_centric());
}

#[test]
fn lossless_export_carries_no_loss_report() {
    let e = FormatExport::lossless(FormatKind::XesXml, b"<log/>".to_vec());
    assert!(!e.is_lossy());
    assert!(e.loss.is_none());
    assert_eq!(e.kind, FormatKind::XesXml);
}

#[test]
fn format_kind_tags_are_stable_and_object_centric_flags_correct() {
    assert_eq!(FormatKind::OcelJson.tag(), "ocel_json");
    assert_eq!(FormatKind::PetriPnml.tag(), "petri_pnml");
    assert!(FormatKind::OcelSqlite.is_object_centric());
    assert!(!FormatKind::BpmnXml.is_object_centric());
    assert!(!FormatKind::PowlJson.is_object_centric());
}

#[test]
fn round_trip_claim_is_named_and_lossiness_aware() {
    let exact = RoundTripClaim::exact(FormatKind::OcelJson, "p2p-tiny");
    assert!(exact.is_named());
    assert!(!exact.allows_lossy);
    assert_eq!(exact.fixture, "p2p-tiny");

    let lossy = RoundTripClaim::lossy_tolerant(FormatKind::XesXml, "running-example");
    assert!(lossy.is_named());
    assert!(lossy.allows_lossy);

    // An unnamed claim cannot be discharged by any test.
    let unnamed = RoundTripClaim::exact(FormatKind::PowlJson, "   ");
    assert!(!unnamed.is_named());
}

// ── Trait implementability (shape only; bodies never run) ────────────────────

/// A stand-in importer proving `ImportFormat` is implementable. The body is
/// deferred because constructing `Admission`/`Refusal` is the spine's job; this
/// test only contracts that the associated types and signature line up.
struct DemoImporter;

impl ImportFormat for DemoImporter {
    type Admitted = ();
    type Reason = ();
    type Witness = ();

    fn import(
        _env: FormatEnvelope<Self::Witness>,
    ) -> Result<
        wasm4pm_compat::admission::Admission<Self::Admitted, Self::Witness>,
        wasm4pm_compat::admission::Refusal<Self::Reason, Self::Witness>,
    > {
        // Never invoked in this contract test — see module docs.
        todo!("admission/refusal construction lives in the spine `admission` module")
    }
}

/// A stand-in exporter proving `ExportFormat` is implementable AND runnable: it
/// consumes the real `LossPolicy` and produces a loss-accountable `FormatExport`.
struct DemoExporter;

/// The exporter's source: an admitted OCEL modelled by its object types and the
/// case type chosen for flattening.
struct AdmittedOcel {
    object_types: Vec<&'static str>,
    case_type: &'static str,
}

/// The exporter's *specifically named* refusal law.
#[derive(Debug, PartialEq, Eq)]
enum XesExportRefusal {
    /// Policy forbade the flattening loss.
    FlatteningLoss,
}

impl ExportFormat for DemoExporter {
    type Source = AdmittedOcel;
    type Reason = XesExportRefusal;

    fn export(
        src: &Self::Source,
        policy: LossPolicy,
    ) -> Result<FormatExport, Self::Reason> {
        let dropped: Vec<String> = src
            .object_types
            .iter()
            .filter(|t| **t != src.case_type)
            .map(|t| format!("dropped_object_type={t}"))
            .collect();

        let bytes = format!("<log case-type=\"{}\"/>", src.case_type).into_bytes();

        if dropped.is_empty() {
            // Single object type: lossless flattening.
            Ok(FormatExport::lossless(FormatKind::XesXml, bytes))
        } else if policy == LossPolicy::RefuseLoss {
            // Forbidden loss becomes a named refusal — never a silent flatten.
            Err(XesExportRefusal::FlatteningLoss)
        } else {
            let report = LossReport::<(), (), Vec<String>>::new(
                ProjectionName("ocel-flatten-to-xes:by-case"),
                policy,
                dropped,
            );
            Ok(FormatExport::lossy(FormatKind::XesXml, bytes, report))
        }
    }
}

#[test]
fn import_trait_is_implementable() {
    // We only *name* the importer to assert the impl type-checks; its body
    // (which mints `Admission`/`Refusal`) is deferred — see module docs.
    let _ = core::marker::PhantomData::<DemoImporter>;
}

#[test]
fn export_under_policy_is_loss_accountable() {
    let multi = AdmittedOcel {
        object_types: vec!["order", "item", "invoice"],
        case_type: "order",
    };

    // RefuseLoss: the flattening loss is refused with a named law.
    match DemoExporter::export(&multi, LossPolicy::RefuseLoss) {
        Err(XesExportRefusal::FlatteningLoss) => {}
        other => panic!("expected FlatteningLoss refusal, got {other:?}"),
    }

    // AllowLossWithReport: the export succeeds and carries the loss report.
    let reported = DemoExporter::export(&multi, LossPolicy::AllowLossWithReport)
        .expect("reporting policy must succeed");
    assert!(reported.is_lossy());
    let report = reported.loss.expect("lossy export carries a report");
    assert_eq!(report.lost.len(), 2); // "item" and "invoice" dropped
    assert_eq!(report.projection.as_str(), "ocel-flatten-to-xes:by-case");

    // A single-object-type OCEL flattens losslessly.
    let single = AdmittedOcel {
        object_types: vec!["order"],
        case_type: "order",
    };
    let lossless = DemoExporter::export(&single, LossPolicy::RefuseLoss)
        .expect("single-type flatten is lossless");
    assert!(!lossless.is_lossy());
}
