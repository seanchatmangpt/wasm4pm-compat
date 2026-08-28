use wasm4pm_compat::authority::{AuthorityConstraint, AuthorityEnvelope, AuthorityRefusal, Capability};
use wasm4pm_compat::receipt::Digest;
use wasm4pm_compat::witness::Ocel20;

#[test]
fn authority_validation_accumulates_independent_constraint_failures_in_order() {
    let capability = Capability::<Ocel20>::new("ocel.import", Digest::new(""));
    let envelope = AuthorityEnvelope::new(
        capability,
        vec![
            AuthorityConstraint::RequiresDigestPin,
            AuthorityConstraint::RequiresBoundedScope,
            AuthorityConstraint::RequiresDataMinimization,
            AuthorityConstraint::RequiresFairnessAttestation,
        ],
        "",
    );
    assert_eq!(
        envelope.validate(),
        Err(vec![
            AuthorityRefusal::MissingDigestPin,
            AuthorityRefusal::UnboundedScope,
            AuthorityRefusal::MissingDataMinimizationNote,
            AuthorityRefusal::MissingFairnessAttestation,
        ])
    );
}
