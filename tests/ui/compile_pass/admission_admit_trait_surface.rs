// COMPILE-PASS: Admit trait can be satisfied — only sanctioned Raw→Admitted path
//
// Proves that:
//   1. Admit trait is implementable for a concrete boundary type.
//   2. The impl returns Result<Admission<T,W>, Refusal<R,W>> — the exact contract shape.
//   3. The admit() call on a Raw Evidence is the only public Raw→Admitted route.
//   4. A named reason type is used in the Refusal arm — not "InvalidInput".

use wasm4pm_compat::admission::{Admit, Admission, Refusal};
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::{Admitted, Raw};
use wasm4pm_compat::witness::Ocel20;

/// Named law enum for this boundary.
#[derive(Debug, PartialEq, Eq)]
enum OcelBoundaryLaw {
    /// Event log carries no events at all.
    EmptyEventLog,
    /// Every event must reference at least one object.
    DanglingEventObjectLink,
}

/// A boundary that admits OCEL event logs requiring at least one event.
/// Refusal carries a specifically named law.
enum NonEmptyOcelBoundary {}

impl Admit for NonEmptyOcelBoundary {
    type Raw = u32;        // simplified: count of events in the log
    type Admitted = u32;
    type Reason = OcelBoundaryLaw;
    type Witness = Ocel20;

    fn admit(
        raw: Evidence<u32, Raw, Ocel20>,
    ) -> Result<Admission<u32, Ocel20>, Refusal<OcelBoundaryLaw, Ocel20>> {
        if raw.value == 0 {
            Err(Refusal::new(OcelBoundaryLaw::EmptyEventLog))
        } else {
            Ok(Admission::new(raw.value))
        }
    }
}

fn main() {
    // Lawful path: raw evidence admitted through the Admit trait.
    let raw_ok: Evidence<u32, Raw, Ocel20> = Evidence::raw(5u32);
    let admission = NonEmptyOcelBoundary::admit(raw_ok).expect("5 events must be admitted");

    // Admission carries the admitted value.
    assert_eq!(admission.value, 5u32);

    // Bridge to Admitted Evidence — the only public Raw→Admitted route.
    let admitted: Evidence<u32, Admitted, Ocel20> = admission.into_evidence();
    assert_eq!(admitted.into_inner(), 5u32);

    // Refusal path: the named law is the reason, not a bare error string.
    let raw_bad: Evidence<u32, Raw, Ocel20> = Evidence::raw(0u32);
    let refused = NonEmptyOcelBoundary::admit(raw_bad).unwrap_err();
    assert_eq!(refused.reason, OcelBoundaryLaw::EmptyEventLog);

    // A second impl proves the trait works with a different named reason.
    enum DanglingBoundary {}

    impl Admit for DanglingBoundary {
        type Raw = bool;
        type Admitted = bool;
        type Reason = OcelBoundaryLaw;
        type Witness = Ocel20;

        fn admit(
            raw: Evidence<bool, Raw, Ocel20>,
        ) -> Result<Admission<bool, Ocel20>, Refusal<OcelBoundaryLaw, Ocel20>> {
            if raw.value {
                Ok(Admission::new(true))
            } else {
                Err(Refusal::new(OcelBoundaryLaw::DanglingEventObjectLink))
            }
        }
    }

    let link_ok = DanglingBoundary::admit(Evidence::raw(true)).unwrap();
    assert_eq!(link_ok.value, true);

    let link_bad = DanglingBoundary::admit(Evidence::raw(false)).unwrap_err();
    assert_eq!(link_bad.reason, OcelBoundaryLaw::DanglingEventObjectLink);
}
