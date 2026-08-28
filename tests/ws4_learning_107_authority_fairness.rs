use wasm4pm_compat::authority::{AuthorityConstraint, AuthorityEnvelope, AuthorityRefusal, Capability};
use wasm4pm_compat::receipt::Digest;
use wasm4pm_compat::witness::Ocel20;

#[test]
fn fairness_constraint_refuses_blank_attestation_reference() {
    let capability = Capability::<Ocel20>::new("ocel.import", Digest::new("blake3:abc"));
    let envelope = AuthorityEnvelope::new(
        capability,
        vec![AuthorityConstraint::RequiresFairnessAttestation],
        "account:123",
    )
    .with_fairness_attestation("   ");
    assert_eq!(
        envelope.validate(),
        Err(vec![AuthorityRefusal::MissingFairnessAttestation])
    );
}
