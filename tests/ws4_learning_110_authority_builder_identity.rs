use wasm4pm_compat::authority::{AuthorityConstraint, AuthorityEnvelope, Capability};
use wasm4pm_compat::receipt::Digest;
use wasm4pm_compat::witness::Ocel20;

#[test]
fn authority_builder_preserves_scope_and_attestation_identity() {
    let capability = Capability::<Ocel20>::new("ocel.import", Digest::new("blake3:abc"));
    let envelope = AuthorityEnvelope::new(
        capability,
        vec![
            AuthorityConstraint::RequiresDataMinimization,
            AuthorityConstraint::RequiresFairnessAttestation,
        ],
        "case:42",
    )
    .with_data_minimization("drop private extension fields")
    .with_fairness_attestation("receipt:fairness:42");

    assert_eq!(envelope.scope, "case:42");
    assert_eq!(envelope.data_minimization_note, "drop private extension fields");
    assert_eq!(envelope.fairness_attestation_ref, "receipt:fairness:42");
    assert_eq!(envelope.validate(), Ok(()));
}
