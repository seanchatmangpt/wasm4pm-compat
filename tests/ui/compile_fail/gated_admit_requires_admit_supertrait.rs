// COMPILE-FAIL: Rejects a `GatedAdmit` impl for a type that does not also
// implement `Admit` — proves `GatedAdmit` is enforced at the type level as a
// wrapper over `Admit`, never a substitute for it.
//
// Law: `pub trait GatedAdmit: crate::admission::Admit` — `GatedAdmit`'s
// supertrait bound means a type cannot implement `GatedAdmit` without also
// implementing `Admit`. This fixture implements only `GatedAdmit`'s own items
// (`EnvelopeWitness`, `envelope`) and omits `Admit` entirely.
use wasm4pm_compat::authority::{AuthorityEnvelope, Capability, GatedAdmit};
use wasm4pm_compat::receipt::Digest;
use wasm4pm_compat::witness::Ocel20;

struct MissingAdmit {
    envelope: AuthorityEnvelope<Ocel20>,
}

// This must fail: `Admit` is not implemented for `MissingAdmit`, so
// `GatedAdmit: crate::admission::Admit` is not satisfied.
impl GatedAdmit for MissingAdmit {
    type EnvelopeWitness = Ocel20;

    fn envelope(&self) -> &AuthorityEnvelope<Self::EnvelopeWitness> {
        &self.envelope
    }
}

fn main() {
    let _ = MissingAdmit {
        envelope: AuthorityEnvelope::new(
            Capability::<Ocel20>::new("x", Digest::new("d")),
            vec![],
            "scope",
        ),
    };
}
