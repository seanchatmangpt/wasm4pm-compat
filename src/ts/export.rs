use super::brand::*;
use super::law_projection::*;
use specta::ts::{BigIntExportBehavior, ExportConfiguration};

/// Generates a single TypeScript declaration string containing the entire branded law projection surface.
pub fn export_ts_bindings() -> String {
    let config = ExportConfiguration::default().bigint(BigIntExportBehavior::BigInt);

    #[cfg(feature = "wasm")]
    let wasm_proj = format!(
        "\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}",
        specta::ts::export::<crate::wasm::boundary::WasmWitness>(&config).unwrap(),
        specta::ts::export::<crate::wasm::boundary::WasmStateTag>(&config).unwrap(),
        specta::ts::export::<crate::wasm::boundary::WasmAdmissionResult>(&config).unwrap(),
        specta::ts::export::<crate::wasm::boundary::WasmGraduationCandidate>(&config).unwrap(),
        specta::ts::export::<crate::wasm::boundary::WasmProcessEvidence>(&config).unwrap()
    );
    #[cfg(not(feature = "wasm"))]
    let wasm_proj = String::new();

    format!(
        "/* Generated compile-time TypeScript type definitions (WASM Boundary Law) */\n\n\
         {}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}{}",
        specta::ts::export::<EvidenceTs<String, String, String>>(&config).unwrap(),
        specta::ts::export::<EvidenceState>(&config).unwrap(),
        specta::ts::export::<WitnessKey>(&config).unwrap(),
        specta::ts::export::<AdmissionTs<String, String>>(&config).unwrap(),
        specta::ts::export::<RefusalTs<String, String>>(&config).unwrap(),
        specta::ts::export::<LossPolicyTs>(&config).unwrap(),
        specta::ts::export::<LossReportTs<String, String, String>>(&config).unwrap(),
        specta::ts::export::<ReceiptShapeTs>(&config).unwrap(),
        specta::ts::export::<GraduationCandidateTs>(&config).unwrap(),
        specta::ts::export::<OcelBrand>(&config).unwrap(),
        specta::ts::export::<XesBrand>(&config).unwrap(),
        specta::ts::export::<WfNetBrand>(&config).unwrap(),
        wasm_proj
    )
}
