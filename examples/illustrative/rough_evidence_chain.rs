//! A "rough" end-to-end evidence lifecycle simulator.
//!
//! This example demonstrates how a value (representing an OCEL log) moves
//! through the strictly-ordered lifecycle stages:
//!
//! 1. **Raw**: Untrusted input, just arrived.
//! 2. **Parsed**: Structurally well-formed (decoder accepted it).
//! 3. **Admitted**: Judged against a named witness (authority).
//! 4. **Projected**: After a named, accounted lossy transformation.
//! 5. **Receipted**: Sealed in a provenance-bearing receipt.
//!
//! The type system prevents illegal transitions (e.g., you cannot project
//! raw evidence without admitting it first).

use wasm4pm_compat::admission::{Admission, Admit, Refusal};
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::Raw;
use wasm4pm_compat::witness::Ocel20;

/// A dummy OCEL log shape.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct OcelLog {
    events: Vec<String>,
    objects: Vec<String>,
}

/// A toy admission boundary: refuse logs that have no events.
enum NonEmptyLog {}

impl Admit for NonEmptyLog {
    type Raw = OcelLog;
    type Admitted = OcelLog;
    type Reason = &'static str;
    type Witness = Ocel20;

    fn admit(
        raw: Evidence<OcelLog, Raw, Ocel20>,
    ) -> Result<Admission<OcelLog, Ocel20>, Refusal<&'static str, Ocel20>> {
        if !raw.value.events.is_empty() {
            // Success: return an Admission token.
            Ok(Admission::new(raw.value))
        } else {
            // Failure: return a named Refusal.
            Err(Refusal::new("EmptyEventLog"))
        }
    }
}

fn main() {
    println!("--- Evidence Lifecycle Simulator ---");

    // 1. Start with Raw evidence.
    let log = OcelLog {
        events: vec!["payment_received".to_string()],
        objects: vec!["invoice_123".to_string()],
    };
    let raw_ev = Evidence::<_, Raw, Ocel20>::raw(log);
    println!("Step 1 (Raw): {:?}", raw_ev);

    // 2. Advance to Parsed.
    // In a real scenario, this happens after a format decoder (JSON/XML) succeeds.
    let parsed_ev = raw_ev.into_parsed();
    println!("Step 2 (Parsed): {:?}", parsed_ev);

    // 3. Advance to Admitted.
    // This requires passing through the Admit gate.
    // Note: We move the Parsed evidence back to a "Raw" state for admission
    // because Admit::admit expects Evidence<..., Raw, ...>.
    // In many workflows, Raw and Parsed are treated as the "untrusted" side of the boundary.
    let raw_for_admission = Evidence::<_, Raw, Ocel20>::raw(parsed_ev.value);
    let admission_result = NonEmptyLog::admit(raw_for_admission);

    let admitted_ev = match admission_result {
        Ok(admission) => admission.into_evidence(),
        Err(refusal) => {
            eprintln!("Admission refused: {}", refusal);
            return;
        }
    };
    println!("Step 3 (Admitted): {:?}", admitted_ev);

    // 4. Advance to Projected.
    // A named lossy transformation (e.g., removing sensitive fields).
    let projected_ev = admitted_ev.into_projected();
    println!("Step 4 (Projected): {:?}", projected_ev);

    // 5. Advance to Receipted.
    // The final stage: evidence is sealed and ready for graduation.
    let receipted_ev = projected_ev.into_receipted();
    println!("Step 5 (Receipted): {:?}", receipted_ev);

    println!("\nLifecycle complete!");

    // --- TYPED-DRIVEN SAFETY ---
    //
    // The following code would fail to compile if uncommented because
    // 'into_projected()' is only defined for Evidence<..., Admitted, ...>.
    // This is because Admitted implements the Projectible trait, but Raw does not.
    //
    // let bad_jump = raw_ev.into_projected();
    //
    // Similarly, you cannot jump from Raw directly to Receipted because
    // 'into_receipted()' is only defined for Admitted, Projected, and Exportable states.
    // The EvidenceState tokens (Raw, Parsed, Admitted, Projected, Receipted)
    // enforce this strictly-ordered lifecycle at compile time.
    //
    // let illegal = raw_ev.into_receipted();
}
