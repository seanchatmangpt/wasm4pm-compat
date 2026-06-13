//! Integration test: full evidence lifecycle.
//!
//! Exercises the `Raw → Parsed → Admitted` path using the real [`Admit`] trait.
//! Verifies that each stage produces a distinct Rust type, and that the refusal
//! path produces a [`Refusal<R, W>`] carrying a specific named reason.

use wasm4pm_compat::admission::{Admission, Admit, Refusal};
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::{Admitted, Parsed, Raw, Refused};
use wasm4pm_compat::witness::Ocel20;

// ── A minimal Admit implementation for testing ──────────────────────────────

/// A toy OCEL admission boundary: refuses empty strings.
enum NonEmptyOcelLog {}

impl Admit for NonEmptyOcelLog {
    type Raw = String;
    type Admitted = String;
    type Reason = &'static str;
    type Witness = Ocel20;

    fn admit(
        raw: Evidence<String, Raw, Ocel20>,
    ) -> Result<Admission<String, Ocel20>, Refusal<&'static str, Ocel20>> {
        if raw.value.is_empty() {
            Err(Refusal::new("DanglingEventObjectLink"))
        } else {
            Ok(Admission::new(raw.value))
        }
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

/// The `Raw` type alias and `into_parsed` produce distinct stage types.
#[test]
fn raw_and_parsed_are_distinct_types() {
    let raw: Evidence<String, Raw, Ocel20> = Evidence::raw("ocel-payload".to_owned());
    // `into_parsed` advances the stage — the resulting type is different.
    let parsed: Evidence<String, Parsed, Ocel20> = raw.into_parsed();
    assert_eq!(parsed.value, "ocel-payload");
}

/// The `Admitted` stage is only reachable via an `Admit` implementation.
#[test]
fn admitted_stage_only_via_admit_impl() {
    let raw: Evidence<String, Raw, Ocel20> = Evidence::raw("non-empty-log".to_owned());
    let admission = NonEmptyOcelLog::admit(raw).unwrap();
    let admitted: Evidence<String, Admitted, Ocel20> = admission.into_evidence();
    assert_eq!(admitted.into_inner(), "non-empty-log");
}

/// The happy path: `Raw → (Admit) → Admitted → Exportable → Receipted`.
#[test]
fn full_raw_to_receipted_lifecycle() {
    let raw = Evidence::<String, Raw, Ocel20>::raw("full-lifecycle".to_owned());
    let admitted = NonEmptyOcelLog::admit(raw)
        .expect("non-empty log must be admitted")
        .into_evidence();
    let exportable = admitted.into_exportable();
    let receipted = exportable.into_receipted();
    assert_eq!(receipted.value, "full-lifecycle");
}

/// Refusal carries the specific named reason — never a bare "InvalidInput".
#[test]
fn refusal_carries_named_reason() {
    let raw = Evidence::<String, Raw, Ocel20>::raw(String::new()); // empty → refused
    let refusal = NonEmptyOcelLog::admit(raw).unwrap_err();
    assert_eq!(refusal.reason, "DanglingEventObjectLink");
}

/// Refusing at the `Raw` stage skips `Parsed` entirely.
#[test]
fn fast_reject_raw_to_refused() {
    let raw: Evidence<&str, Raw, Ocel20> = Evidence::raw("");
    let refused: Evidence<&str, Refused, Ocel20> = raw.refuse();
    assert_eq!(*refused.as_refused_value(), "");
    assert_eq!(refused.into_refused_value(), "");
}

/// `Parsed → Refused` path: well-formed but structurally inadmissible.
#[test]
fn parsed_to_refused_carries_original_value() {
    let raw: Evidence<Vec<u8>, Raw, Ocel20> = Evidence::raw(vec![0x00]);
    let parsed: Evidence<Vec<u8>, Parsed, Ocel20> = raw.into_parsed();
    let refused: Evidence<Vec<u8>, Refused, Ocel20> = parsed.into_refused();
    assert_eq!(*refused.as_refused_value(), vec![0x00]);
}

/// `Admitted → Projected → Receipted` is a distinct valid lifecycle branch.
#[test]
fn admitted_to_projected_to_receipted() {
    let raw = Evidence::<String, Raw, Ocel20>::raw("projected".to_owned());
    let admitted = NonEmptyOcelLog::admit(raw).unwrap().into_evidence();
    let projected = admitted.into_projected();
    let receipted = projected.into_receipted();
    assert_eq!(receipted.value, "projected");
}
