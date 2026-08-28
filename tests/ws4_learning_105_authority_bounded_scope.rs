use wasm4pm_compat::authority::{AuthorityConstraint, AuthorityEnvelope, AuthorityRefusal, Capability};
use wasm4pm_compat::receipt::Digest;
use wasm4pm_compat::witness::Ocel20;

#[test]
fn bounded_scope_constraint_refuses_whitespace_scope() {
    let capability = Capability::<Ocel20>::new("ocel.import", Digest::new("blake3:abc"));
    let envelope = AuthorityEnvelope::new(
        capability,
        vec![AuthorityConstraint::RequiresBoundedScope],
        "   ",
    );
    assert_eq!(envelope.validate(), Err(vec![AuthorityRefusal::UnboundedScope]));
}
