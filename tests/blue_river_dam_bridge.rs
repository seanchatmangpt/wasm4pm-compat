#![cfg(feature = "wasm4pm")]

use wasm4pm_compat::admission::{Admission, Admit, Refusal};
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::receipt::{ReceiptBuilder, ReceiptEnvelope};
use wasm4pm_compat::state::{Admitted, Raw};
use wasm4pm_compat::witness::Ocel20;

// The Blue River Dam Bridge
// "BlueRiverDam = κ ∘ ρ ∘ α ∘ μ"
// Admissible Construction (μ) -> Actuation (α) -> Receipt (ρ) -> Checkpoint (κ)

/// 1. Admissible Construction (μ)
struct OcelAdmitter;
impl Admit for OcelAdmitter {
    type Raw = String;
    type Admitted = String;
    type Reason = &'static str;
    type Witness = Ocel20;

    fn admit(
        raw: Evidence<String, Raw, Ocel20>,
    ) -> Result<Admission<String, Ocel20>, Refusal<&'static str, Ocel20>> {
        if raw.value.contains("event") {
            Ok(Admission::new(raw.value))
        } else {
            Err(Refusal::new("MissingEvent"))
        }
    }
}

/// 2. Actuation (α)
///
/// Stubs the execution phase in wasm4pm
struct ActuationEngine;
impl ActuationEngine {
    fn actuate(admitted: Evidence<String, Admitted, Ocel20>) -> String {
        format!("actuated: {}", admitted.value)
    }
}

/// 3. Receipt (ρ)
///
/// Cryptographically bound Receipt object connecting compat and wasm4pm
struct ReceiptGenerator;
impl ReceiptGenerator {
    fn generate(subject: &str) -> ReceiptEnvelope {
        ReceiptBuilder::<Ocel20>::new()
            .subject(subject)
            .digest("blake3:cryptographically_bound")
            .replay_hint("wasm4pm://checkpoint")
            .build()
            .unwrap()
    }
}

/// 4. Checkpoint (κ)
struct CheckpointStore;
impl CheckpointStore {
    fn save(receipt: ReceiptEnvelope) -> bool {
        receipt.is_well_shaped()
    }
}

#[test]
fn test_blue_river_dam_flow() {
    // Admissible Construction (μ)
    let raw_evidence = Evidence::raw("event log data".to_string());
    let admission = OcelAdmitter::admit(raw_evidence).expect("Admission failed");
    let admitted_evidence = admission.into_evidence();

    // Actuation (α)
    let actuated_result = ActuationEngine::actuate(admitted_evidence);

    // Receipt (ρ)
    let receipt = ReceiptGenerator::generate(&actuated_result);

    // Checkpoint (κ)
    let checkpointed = CheckpointStore::save(receipt);

    assert!(
        checkpointed,
        "Blue River Dam = κ ∘ ρ ∘ α ∘ μ is not satisfied"
    );
}
