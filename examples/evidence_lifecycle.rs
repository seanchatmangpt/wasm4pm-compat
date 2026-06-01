//! Example: Evidence lifecycle — Raw → Parsed → Admitted → Receipted
//!
//! Demonstrates the `Evidence<T, State, W>` one-way lifecycle enforced at the
//! type level. Each transition changes the `State` type parameter, so functions
//! that demand admitted evidence cannot be called with raw evidence — the
//! compiler rejects the substitution entirely.
//!
//! Run: cargo run --example evidence_lifecycle

#![allow(dead_code)]

use wasm4pm_compat::admission::{Admission, Admit, Refusal};
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::{Admitted, Parsed, Raw, Receipted, Refused};
use wasm4pm_compat::witness::Ocel20;

// ---------------------------------------------------------------------------
// A concrete Admit implementation for this example.
//
// `OcelLogAdmit` enforces a single named law: an OCEL event log string must
// be non-empty. The reason type `OcelRefusalReason` is a specific named enum
// variant, never a bare "InvalidInput" (that is forbidden by the boundary law).
// ---------------------------------------------------------------------------

/// Named refusal reasons for the OCEL 2.0 admission boundary.
///
/// Every refusal must carry a *specific* named law. Catch-all strings or
/// `InvalidInput` variants are defects under the boundary contract.
#[derive(Debug, PartialEq)]
enum OcelRefusalReason {
    /// Law: an OCEL log must have at least one event entry.
    EmptyEventLog,
    /// Law: an OCEL log string must not begin with structural garbage.
    MalformedHeader,
}

impl core::fmt::Display for OcelRefusalReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            OcelRefusalReason::EmptyEventLog => write!(f, "EmptyEventLog"),
            OcelRefusalReason::MalformedHeader => write!(f, "MalformedHeader"),
        }
    }
}

/// The admission boundary: judges a `String` against OCEL 2.0 structural laws.
///
/// This is the **only** sanctioned path from `Raw` to `Admitted` for this
/// value type. There is no free conversion elsewhere in the type system.
enum OcelLogAdmit {}

impl Admit for OcelLogAdmit {
    type Raw = String;
    type Admitted = String;
    type Reason = OcelRefusalReason;
    type Witness = Ocel20;

    fn admit(
        raw: Evidence<String, Raw, Ocel20>,
    ) -> Result<Admission<String, Ocel20>, Refusal<OcelRefusalReason, Ocel20>> {
        // Law: EmptyEventLog — a log with no content cannot be admitted.
        if raw.value.is_empty() {
            return Err(Refusal::new(OcelRefusalReason::EmptyEventLog));
        }
        // Law: MalformedHeader — structural garbage at byte 0 is refused.
        if raw.value.starts_with('\0') {
            return Err(Refusal::new(OcelRefusalReason::MalformedHeader));
        }
        // All named laws satisfied — mint the admission.
        Ok(Admission::new(raw.value))
    }
}

// ---------------------------------------------------------------------------
// Helper: consume only admitted evidence.
//
// This function cannot be called with Raw or Parsed evidence. The `State`
// type parameter `Admitted` in the signature is the entire enforcement
// mechanism — no runtime check is required or performed.
// ---------------------------------------------------------------------------

fn process_admitted(ev: &Evidence<String, Admitted, Ocel20>) {
    println!("  [admitted] value = {:?}", ev.value);
}

// ---------------------------------------------------------------------------
// Main: walk the full lifecycle with commentary at each step.
// ---------------------------------------------------------------------------

fn main() {
    println!("=== Evidence<String, _, Ocel20> lifecycle ===\n");

    // ------------------------------------------------------------------
    // Stage 1 — Raw
    //
    // Law satisfied: any value may enter the boundary as Raw. This is the
    // only freely-available constructor. No check is performed here; the
    // value is merely tagged as "untrusted input that has entered the crate
    // boundary".
    // ------------------------------------------------------------------
    let raw: Evidence<String, Raw, Ocel20> =
        Evidence::raw("ocel-log: {events: [{id: e1, type: place_order}]}".to_string());
    println!("Stage 1 — Raw");
    println!("  type: Evidence<String, Raw, Ocel20>");
    println!("  value = {:?}", raw.value);
    println!("  (no law checked yet — value is untrusted)\n");

    // ------------------------------------------------------------------
    // Stage 2 — Parsed
    //
    // Law satisfied: RawToParsed — the format decoder accepted the shape.
    // Parsing proves the value is *well-formed*, not that it is admissible.
    // The `State` type parameter changes from `Raw` to `Parsed`; the
    // compiler will not allow using this as `Raw` evidence anymore.
    // ------------------------------------------------------------------
    let parsed: Evidence<String, Parsed, Ocel20> = raw.into_parsed();
    println!("Stage 2 — Parsed  (Raw → Parsed)");
    println!("  type: Evidence<String, Parsed, Ocel20>");
    println!("  value = {:?}", parsed.value);
    println!("  (shape is well-formed; structural admission not yet checked)\n");

    // ------------------------------------------------------------------
    // Stage 3a — Admitted (happy path)
    //
    // Law satisfied: OcelLogAdmit — all named OCEL 2.0 structural laws pass.
    // The Admit trait is the **only** sanctioned Raw → Admitted path. We
    // re-enter from Raw here because Admit takes Raw evidence; the Parsed
    // stage is used when a pre-admission structural check is needed first
    // (see Stage 3b below).
    //
    // After this call the `State` type parameter is `Admitted`. Functions
    // like `process_admitted` accept this value; they would not compile with
    // a `Raw` or `Parsed` argument.
    // ------------------------------------------------------------------
    let fresh_raw: Evidence<String, Raw, Ocel20> =
        Evidence::raw("ocel-log: {events: [{id: e1, type: place_order}]}".to_string());

    let admitted: Evidence<String, Admitted, Ocel20> = match OcelLogAdmit::admit(fresh_raw) {
        Ok(admission) => admission.into_evidence(),
        Err(refusal) => {
            eprintln!("Unexpected refusal: {}", refusal.reason);
            return;
        }
    };

    println!("Stage 3a — Admitted  (Raw → [Admit gate] → Admitted)");
    println!("  type: Evidence<String, Admitted, Ocel20>");
    process_admitted(&admitted);
    println!("  (type system now enforces: only admitted evidence may proceed)\n");

    // ------------------------------------------------------------------
    // Stage 3b — Refused (sad path — named law broken)
    //
    // Law broken: EmptyEventLog — the Admit impl returns a named Refusal.
    // The `Refusal<OcelRefusalReason, Ocel20>` carries a specific enum
    // variant; bare strings or catch-all "InvalidInput" are forbidden.
    // Refused evidence is terminal: it cannot be silently coerced back to
    // Admitted.
    // ------------------------------------------------------------------
    let empty_raw: Evidence<String, Raw, Ocel20> = Evidence::raw(String::new());

    match OcelLogAdmit::admit(empty_raw) {
        Ok(_) => eprintln!("Unexpected admission of empty log"),
        Err(refusal) => {
            println!("Stage 3b — Refused  (Raw → [Admit gate] → Refused)");
            println!("  type: Refusal<OcelRefusalReason, Ocel20>");
            println!("  named law broken: {:?}", refusal.reason);
            assert_eq!(refusal.reason, OcelRefusalReason::EmptyEventLog);
            println!("  (terminal: Refused cannot be coerced back to Admitted)\n");
        }
    }

    // ------------------------------------------------------------------
    // Stage 4 — Receipted
    //
    // Law satisfied: AdmittedToReceipted — the admitted value is sealed
    // inside a provenance-bearing receipt stage. This is the strongest
    // structural stage and the natural hand-off point when graduating to
    // a `wasm4pm` engine that will verify the receipt.
    //
    // Only `Admitted` evidence may call `into_receipted()`; the method is
    // not available on `Raw`, `Parsed`, or `Refused` evidence.
    // ------------------------------------------------------------------
    let receipted: Evidence<String, Receipted, Ocel20> = admitted.into_receipted();
    println!("Stage 4 — Receipted  (Admitted → Receipted)");
    println!("  type: Evidence<String, Receipted, Ocel20>");
    println!("  value = {:?}", receipted.value);
    println!("  (ready for hand-off to wasm4pm engine for provenance verification)\n");

    println!("=== Lifecycle complete ===");
    println!();
    println!("Summary of type changes:");
    println!("  Evidence<String, Raw,      Ocel20>  — untrusted input");
    println!("  Evidence<String, Parsed,   Ocel20>  — well-formed shape");
    println!("  Evidence<String, Admitted, Ocel20>  — passed named boundary laws");
    println!("  Evidence<String, Receipted,Ocel20>  — sealed for engine hand-off");
    println!();
    println!("Each transition is zero-cost: State and W are PhantomData tags.");
    println!("Illegal transitions (e.g. Raw → Receipted) do not compile.");
}
