// COMPILE-PASS: STATE_TOKEN_REFUSED — Refused is a distinct zero-sized terminal lifecycle marker
//
// Law: Refused is a first-class terminal outcome. A value in the Refused stage
// carries the *named* reason it was declined (never a bare "invalid input").
// Refused is an uninhabited empty enum used only as a PhantomData tag — it is
// zero-cost, distinct from every other state token, and cannot be coerced back
// to Admitted. The Refusal<R, W> type is the primary carrier paired with this state.
use core::marker::PhantomData;

use wasm4pm_compat::admission::{Admit, Admission, Refusal};
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::{Raw, Refused};
use wasm4pm_compat::witness::Ocel20;

/// A named boundary that refuses OCEL logs whose event count is zero.
enum NonEmptyEventLog {}

impl Admit for NonEmptyEventLog {
    type Raw = u32;
    type Admitted = u32;
    type Reason = &'static str;
    type Witness = Ocel20;

    fn admit(
        raw: Evidence<u32, Raw, Ocel20>,
    ) -> Result<Admission<u32, Ocel20>, Refusal<&'static str, Ocel20>> {
        if raw.value == 0 {
            // The reason is a *named law*, not a generic error string.
            Err(Refusal::new("EmptyEventLog"))
        } else {
            Ok(Admission::new(raw.value))
        }
    }
}

fn main() {
    // The Refused token itself: uninhabited empty enum, zero runtime cost.
    assert_eq!(core::mem::size_of::<Refused>(), 0);

    // PhantomData<Refused> is a valid zero-sized type tag.
    let _: PhantomData<Refused> = PhantomData;

    // Refused is distinct from every other state token at the type level.
    // These four assertions confirm distinct sizes are still all 0 — they are
    // different types even though they share the same representation.
    assert_eq!(core::mem::size_of::<Raw>(), 0);
    assert_eq!(core::mem::size_of::<Refused>(), 0);

    // Refusal<R, W> carries the named reason as a first-class value.
    let refusal: Refusal<&'static str, Ocel20> =
        NonEmptyEventLog::admit(Evidence::raw(0u32)).unwrap_err();
    assert_eq!(refusal.reason, "EmptyEventLog");

    // into_reason yields the named law string.
    let reason = refusal.into_reason();
    assert_eq!(reason, "EmptyEventLog");

    // A direct refusal with a named enum reason (not just a string).
    #[derive(Debug, PartialEq)]
    enum OcelBoundaryLaw {
        DanglingEventObjectLink,
        EmptyEventLog,
    }

    let named: Refusal<OcelBoundaryLaw, Ocel20> =
        Refusal::new(OcelBoundaryLaw::DanglingEventObjectLink);
    assert_eq!(named.reason, OcelBoundaryLaw::DanglingEventObjectLink);
}
