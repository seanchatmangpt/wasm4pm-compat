// COMPILE-PASS: `GatedAdmit` composes with `Admit` rather than replacing it —
// proves both `check_gate()` and `Admit::admit()` are callable on the same
// implementor, matching `authority.rs`'s own doc claim: "This trait never
// replaces `crate::admission::Admit`; it wraps it."
//
// Law: `pub trait GatedAdmit: crate::admission::Admit` is satisfiable by a
// type implementing both traits' items; the gate and the admission remain
// two separate, composable calls (this fixture does not itself enforce
// calling check_gate before admit — that composition is caller
// responsibility, as documented).

use wasm4pm_compat::admission::{Admission, Admit, Refusal};
use wasm4pm_compat::authority::{AuthorityConstraint, AuthorityEnvelope, Capability, GatedAdmit};
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::receipt::Digest;
use wasm4pm_compat::state::Raw;
use wasm4pm_compat::witness::Ocel20;

/// A named law (never bare "InvalidInput") for this boundary's refusal.
#[derive(Debug, PartialEq, Eq)]
enum ExampleBoundaryLaw {
    Empty,
}

struct ExampleBoundary {
    envelope: AuthorityEnvelope<Ocel20>,
}

impl Admit for ExampleBoundary {
    type Raw = bool;
    type Admitted = bool;
    type Reason = ExampleBoundaryLaw;
    type Witness = Ocel20;

    fn admit(
        raw: Evidence<bool, Raw, Ocel20>,
    ) -> Result<Admission<bool, Ocel20>, Refusal<ExampleBoundaryLaw, Ocel20>> {
        if raw.value {
            Ok(Admission::new(true))
        } else {
            Err(Refusal::new(ExampleBoundaryLaw::Empty))
        }
    }
}

impl GatedAdmit for ExampleBoundary {
    type EnvelopeWitness = Ocel20;

    fn envelope(&self) -> &AuthorityEnvelope<Self::EnvelopeWitness> {
        &self.envelope
    }
}

fn main() {
    let boundary = ExampleBoundary {
        envelope: AuthorityEnvelope::new(
            Capability::<Ocel20>::new("terraform.apply", Digest::new("blake3:abc")),
            vec![AuthorityConstraint::RequiresDigestPin],
            "acct:1",
        ),
    };

    // Both calls are available on the same implementor.
    assert!(boundary.check_gate().is_ok());
    assert!(ExampleBoundary::admit(Evidence::raw(true)).is_ok());
}
