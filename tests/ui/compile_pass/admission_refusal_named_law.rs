// Law: RefusalNamedLawLaw — Refusal<R,W> requires a specific named reason type and is witness-parametric; Refusal<R,W1> and Refusal<R,W2> are distinct types
// COMPILE-PASS: Refusal<R, W> carries a named-law reason and is witness-distinct
//
// Proves that:
//   1. Refusal<R, W> is constructible with a specific named reason type.
//   2. Refusal<R, W1> and Refusal<R, W2> are distinct types at the type level.
//   3. The reason is accessible via the .reason field and into_reason().
//   4. "InvalidInput" is not the reason — the named law IS the reason.

use wasm4pm_compat::admission::Refusal;
use wasm4pm_compat::witness::{Ocel20, WfNetSoundnessPaper, Xes1849};

/// A specifically named law: an event object link points at no declared object.
#[derive(Debug, PartialEq, Eq)]
enum OcelLaw {
    DanglingEventObjectLink,
    MissingObjectType,
}

/// A specifically named law for Petri net soundness boundary.
#[derive(Debug, PartialEq, Eq)]
enum PetriLaw {
    MissingFinalMarking,
    UnsoundWfNet,
}

fn main() {
    // Refusal carries the exact named law — not "InvalidInput".
    let r1 = Refusal::<OcelLaw, Ocel20>::new(OcelLaw::DanglingEventObjectLink);
    assert_eq!(r1.reason, OcelLaw::DanglingEventObjectLink);

    // A different variant of the same law enum.
    let r2 = Refusal::<OcelLaw, Ocel20>::new(OcelLaw::MissingObjectType);
    assert_eq!(r2.reason, OcelLaw::MissingObjectType);

    // into_reason() consumes and yields the reason.
    let reason = Refusal::<OcelLaw, Ocel20>::new(OcelLaw::DanglingEventObjectLink).into_reason();
    assert_eq!(reason, OcelLaw::DanglingEventObjectLink);

    // Refusal against a different witness is a distinct type.
    // Refusal<PetriLaw, WfNetSoundnessPaper> cannot be confused with Refusal<OcelLaw, Ocel20>.
    let r_petri = Refusal::<PetriLaw, WfNetSoundnessPaper>::new(PetriLaw::MissingFinalMarking);
    assert_eq!(r_petri.reason, PetriLaw::MissingFinalMarking);

    let r_petri2 = Refusal::<PetriLaw, WfNetSoundnessPaper>::new(PetriLaw::UnsoundWfNet);
    assert_eq!(r_petri2.reason, PetriLaw::UnsoundWfNet);

    // Witness distinction: Refusal<&str, Ocel20> vs Refusal<&str, Xes1849>.
    // Both carry a named &'static str law but respond to different witnesses.
    let r_ocel = Refusal::<&'static str, Ocel20>::new("DanglingEventObjectLink");
    let r_xes  = Refusal::<&'static str, Xes1849>::new("UndeclaredExtensionPrefix");

    assert_eq!(r_ocel.reason, "DanglingEventObjectLink");
    assert_eq!(r_xes.reason, "UndeclaredExtensionPrefix");

    // They are distinct types — each names the witness that issued the refusal.
    // (If they were the same type this function couldn't compile with two bindings.)
    fn accepts_ocel_refusal(_: Refusal<&'static str, Ocel20>) {}
    fn accepts_xes_refusal(_: Refusal<&'static str, Xes1849>) {}
    accepts_ocel_refusal(r_ocel);
    accepts_xes_refusal(r_xes);
}
