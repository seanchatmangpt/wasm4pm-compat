use wasm4pm_compat::authority::{
    AuthorityConstraint, AuthorityEnvelope, AuthorityRefusal, Capability,
};
use wasm4pm_compat::receipt::Digest;
use wasm4pm_compat::witness::Ocel20;

#[test]
fn data_minimization_constraint_refuses_blank_note() {
    let capability = Capability::<Ocel20>::new("ocel.import", Digest::new("blake3:abc"));
    let envelope = AuthorityEnvelope::new(
        capability,
        vec![AuthorityConstraint::RequiresDataMinimization],
        "account:123",
    )
    .with_data_minimization("   ");
    assert_eq!(
        envelope.validate(),
        Err(vec![AuthorityRefusal::MissingDataMinimizationNote])
    );
}
