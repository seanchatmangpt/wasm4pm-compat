use wasm4pm_compat::authority::{AuthorityConstraint, AuthorityEnvelope, Capability};
use wasm4pm_compat::receipt::Digest;
use wasm4pm_compat::witness::Ocel20;

#[test]
fn fully_scoped_authority_envelope_is_structurally_admitted() {
    let capability = Capability::<Ocel20>::new("ocel.import", Digest::new("blake3:abc"));
    let envelope = AuthorityEnvelope::new(
        capability,
        vec![
            AuthorityConstraint::RequiresWitness,
            AuthorityConstraint::RequiresDigestPin,
            AuthorityConstraint::RequiresBoundedScope,
            AuthorityConstraint::RequiresDataMinimization,
            AuthorityConstraint::RequiresFairnessAttestation,
        ],
        "account:123/region:us-east-1",
    )
    .with_data_minimization("project only admitted OCEL fields")
    .with_fairness_attestation("receipt:fairness-42");
    assert_eq!(envelope.validate(), Ok(()));
}
