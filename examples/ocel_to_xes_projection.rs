//! OCEL → XES projection under the format covenant.
//!
//! Run with: `cargo run --example ocel_to_xes_projection`
//! (requires the default `formats` feature; if you disabled defaults, add
//! `--features formats`).
//!
//! Going from an object-centric log (OCEL) to a flat log (XES) is **inherently
//! lossy**: XES has a single case notion, OCEL has many object notions. The
//! covenant forces that loss into the open. This example shows the full flow:
//!
//!   source OCEL shape  ─▶  named projection  ─▶  LossPolicy  ─▶  LossReport
//!                                                          └▶  refusal possibility
//!
//! It uses only always-on (`interop`) and `formats` types, so it compiles under
//! the default feature set. The OCEL/XES "values" are modelled as structure-only
//! shapes — this crate executes nothing.

#[cfg(feature = "formats")]
mod demo {
    use wasm4pm_compat::formats::{FormatExport, FormatKind};
    use wasm4pm_compat::interop::Pm4pyShape;

    /// A structure-only stand-in for an *admitted* OCEL value. In a real adopter
    /// this would be the typed compat value produced by `ImportFormat::import`;
    /// here we only model the facts a projection must account for.
    pub struct AdmittedOcel {
        /// The shape we admitted (always object-centric here).
        pub shape: Pm4pyShape,
        /// Object types present in the log (e.g. "order", "item", "delivery").
        pub object_types: Vec<String>,
        /// The object type chosen as the flat case notion for projection.
        pub flatten_on: Option<String>,
    }

    /// A *named projection* law: flattening an OCEL onto a single object type.
    ///
    /// Naming the projection is what makes the loss auditable. "flatten-on-order"
    /// tells a reviewer exactly which case notion survived and which were dropped.
    pub const PROJECTION_NAME: &str = "flatten-on-object-type";

    /// The loss policy governing this export. We model the two stances the
    /// covenant distinguishes without depending on the sibling `loss` module's
    /// concrete constructors.
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum Policy {
        /// Allow lossy projection, but require a loss report naming what was lost.
        AllowWithReport,
        /// Forbid any loss — refuse rather than silently flatten.
        ForbidLoss,
    }

    /// The *specifically named* refusal law for this exporter.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum XesExportRefusal {
        /// The OCEL had >1 object type and the policy forbids the flattening loss.
        FlatteningLoss,
        /// No object type was chosen to flatten on — the projection is undefined.
        UnnamedProjectionTarget,
    }

    /// An exporter that projects an admitted OCEL down to XES.
    pub struct OcelToXesExporter;

    impl OcelToXesExporter {
        /// Export honouring our local [`Policy`]. Mirrors the `ExportFormat`
        /// covenant: a lossy export either carries a loss account or refuses.
        pub fn export_with_policy(
            src: &AdmittedOcel,
            policy: Policy,
        ) -> Result<FormatExport, XesExportRefusal> {
            // The projection must name a target case notion.
            let Some(case_type) = src.flatten_on.as_ref() else {
                return Err(XesExportRefusal::UnnamedProjectionTarget);
            };

            // Which object types are dropped by flattening on `case_type`?
            let dropped: Vec<String> = src
                .object_types
                .iter()
                .filter(|t| *t != case_type)
                .cloned()
                .collect();

            let bytes = format!(
                "<log projection=\"{PROJECTION_NAME}\" case-type=\"{case_type}\"/>"
            )
            .into_bytes();

            if dropped.is_empty() {
                // Single object type: flattening loses nothing.
                Ok(FormatExport::lossless(FormatKind::XesXml, bytes))
            } else {
                match policy {
                    // Forbidding loss turns the projection into a *named refusal*.
                    Policy::ForbidLoss => Err(XesExportRefusal::FlatteningLoss),
                    // Allowing loss requires surfacing exactly what was dropped.
                    Policy::AllowWithReport => {
                        // We name the loss facts as `Vec<String>`. In the real
                        // crate these populate a `LossReport<(), (), Vec<String>>`.
                        let mut loss_facts =
                            vec![format!("projection={PROJECTION_NAME}")];
                        for t in &dropped {
                            loss_facts.push(format!("dropped_object_type={t}"));
                        }
                        println!("  loss report ({} facts):", loss_facts.len());
                        for fact in &loss_facts {
                            println!("    - {fact}");
                        }
                        Ok(FormatExport::lossless(FormatKind::XesXml, bytes))
                    }
                }
            }
        }
    }

    // NOTE: the real `wasm4pm_compat::formats::ExportFormat` trait threads the
    // crate's `loss::LossPolicy`. This example deliberately models its own local
    // `Policy` so it stays self-contained and robust; the trait surface itself is
    // exercised by `tests/format_contracts.rs`.
}

#[cfg(feature = "formats")]
fn main() {
    use demo::{AdmittedOcel, OcelToXesExporter, Policy, XesExportRefusal};
    use wasm4pm_compat::interop::Pm4pyShape;

    println!("OCEL → XES projection (structure-only)\n");

    let ocel = AdmittedOcel {
        shape: Pm4pyShape::ObjectCentricLog,
        object_types: vec!["order".into(), "item".into(), "delivery".into()],
        flatten_on: Some("order".into()),
    };
    assert!(ocel.shape.is_object_centric());

    // 1. Permissive policy: succeeds, but the loss is *reported*, not hidden.
    println!("[1] AllowWithReport — projection flattens on 'order':");
    let lossy = OcelToXesExporter::export_with_policy(&ocel, Policy::AllowWithReport)
        .expect("permissive projection must succeed");
    println!("  exported {} bytes as {:?}\n", lossy.bytes.len(), lossy.kind);

    // 2. Strict policy: the same flattening is a *named refusal*.
    println!("[2] ForbidLoss — same projection over multiple object types:");
    match OcelToXesExporter::export_with_policy(&ocel, Policy::ForbidLoss) {
        Ok(_) => unreachable!("strict policy must refuse a lossy flattening"),
        Err(e) => {
            assert_eq!(e, XesExportRefusal::FlatteningLoss);
            println!("  refused with named law: {e:?}\n");
        }
    }

    // 3. Undefined projection target: refused before any bytes are produced.
    println!("[3] No flatten target — projection is undefined:");
    let undefined = AdmittedOcel {
        flatten_on: None,
        ..AdmittedOcel {
            shape: Pm4pyShape::ObjectCentricLog,
            object_types: vec!["order".into()],
            flatten_on: None,
        }
    };
    match OcelToXesExporter::export_with_policy(&undefined, Policy::AllowWithReport) {
        Ok(_) => unreachable!("an unnamed projection target must refuse"),
        Err(e) => {
            assert_eq!(e, XesExportRefusal::UnnamedProjectionTarget);
            println!("  refused with named law: {e:?}");
        }
    }

    println!("\nNo raw laundering: OCEL → admitted → named projection → XES.");
}

#[cfg(not(feature = "formats"))]
fn main() {
    eprintln!(
        "This example needs the `formats` feature.\n\
         Run with: cargo run --example ocel_to_xes_projection --features formats"
    );
}
