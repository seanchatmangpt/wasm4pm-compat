// COMPILE-FAIL: Refusal type requires a specific named law reason — bare InvalidInput rejected
//
// Law: The Admit trait's return signature is
//   Result<Admission<Self::Admitted, Self::Witness>, Refusal<Self::Reason, Self::Witness>>
// The witness on the Refusal MUST be Self::Witness. Returning a Refusal<_, OtherWitness>
// — i.e. one that answers to a different authority than the boundary declares —
// is a compile-time type error. A refusal that names the wrong authority is not a
// lawful refusal for this boundary.
//
// Expected error: mismatched types — Refusal<_, Ocel20> is not
// Refusal<_, Xes1849> (the declared witness for this boundary).
use wasm4pm_compat::admission::{Admit, Admission, Refusal};
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::Raw;
use wasm4pm_compat::witness::{Ocel20, Xes1849};

/// A boundary that claims to judge against Xes1849.
enum XesBoundary {}

impl Admit for XesBoundary {
    type Raw = bool;
    type Admitted = bool;
    type Reason = &'static str;
    type Witness = Xes1849;

    fn admit(
        raw: Evidence<bool, Raw, Xes1849>,
    ) -> Result<Admission<bool, Xes1849>, Refusal<&'static str, Xes1849>> {
        if raw.value {
            Ok(Admission::new(true))
        } else {
            // VIOLATION: This refusal names Ocel20 as its authority, but the
            // boundary declares Xes1849. A refusal for the wrong witness is not
            // a lawful refusal — the type system must reject this.
            Err(Refusal::<_, Ocel20>::new("InvalidInput"))
        }
    }
}

fn main() {}
