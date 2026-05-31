//! Integration tests for the admission/refusal boundary and typed ids.
//!
//! These prove the *spine* laws hold:
//! - raw evidence can be freely constructed,
//! - a sample [`Admit`] impl returns [`Admission`] on good input and a *named*
//!   [`Refusal`] on bad input,
//! - typed ids are not interchangeable (enforced at compile time).

use core::marker::PhantomData;

use wasm4pm_compat::admission::{Admit, Admission, Refusal};
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::ids::{EventId, ObjectId};
use wasm4pm_compat::state::Raw;
use wasm4pm_compat::witness::{Ocel20, Witness, WitnessFamily};

/// A namespace marker for ids in these tests.
enum TestNs {}

/// A named refusal law for the toy OCEL boundary below.
#[derive(Debug, PartialEq, Eq)]
enum OcelRefusal {
    /// An event references an object that does not exist in the log.
    DanglingEventObjectLink,
}

/// Toy OCEL admission: the raw value is `true` iff every event-object link
/// resolves to a real object. We refuse with a *specific named law*, never a
/// bare "invalid input".
enum LinkedOcel {}

impl Admit for LinkedOcel {
    type Raw = bool;
    type Admitted = bool;
    type Reason = OcelRefusal;
    type Witness = Ocel20;

    fn admit(
        raw: Evidence<bool, Raw, Ocel20>,
    ) -> Result<Admission<bool, Ocel20>, Refusal<OcelRefusal, Ocel20>> {
        if raw.value {
            Ok(Admission::new(true))
        } else {
            Err(Refusal::new(OcelRefusal::DanglingEventObjectLink))
        }
    }
}

#[test]
fn raw_evidence_can_be_constructed_freely() {
    // (a) Untrusted input enters the boundary as `Raw`.
    let raw = Evidence::<_, _, Ocel20>::raw(true);
    assert!(raw.value);
}

#[test]
fn admit_returns_admission_on_good_input() {
    // (b) Good input crosses the boundary.
    let raw = Evidence::raw(true);
    let admission = LinkedOcel::admit(raw).expect("well-linked OCEL must admit");
    let admitted = admission.into_evidence();
    assert!(admitted.into_inner());
}

#[test]
fn admit_returns_named_refusal_on_bad_input() {
    // (b) Bad input is refused with a SPECIFIC named law — not "InvalidInput".
    let raw = Evidence::raw(false);
    let refusal = LinkedOcel::admit(raw).expect_err("dangling link must refuse");
    assert_eq!(refusal.reason, OcelRefusal::DanglingEventObjectLink);
}

#[test]
fn typed_ids_are_distinct_at_the_type_level() {
    // (c) Construct distinct EventId and ObjectId. They wrap the same u64 but
    // are different types, so they cannot be swapped. We demonstrate that they
    // are constructed independently and read back correctly.
    let event: EventId<TestNs> = EventId::new(7);
    let object: ObjectId<TestNs> = ObjectId::new(7);

    assert_eq!(event.raw(), 7);
    assert_eq!(object.raw(), 7);

    // A helper that ONLY accepts an EventId proves non-interchangeability:
    // passing `object` here would fail to compile. We assert it for events.
    fn require_event_id(id: EventId<TestNs>) -> u64 {
        id.raw()
    }
    assert_eq!(require_event_id(event), 7);

    // The negative case is enforced by the compiler; documented here:
    // `require_event_id(object); // ERROR: expected EventId, found ObjectId`
    let _ = PhantomData::<TestNs>;
}

#[test]
fn witness_metadata_is_legible() {
    assert_eq!(Ocel20::KEY, "ocel-2.0");
    assert_eq!(Ocel20::TITLE, "OCEL 2.0");
    assert_eq!(Ocel20::YEAR, Some(2023));
    assert_eq!(Ocel20::FAMILY, WitnessFamily::Standard);
}
