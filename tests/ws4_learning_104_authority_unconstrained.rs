use wasm4pm_compat::authority::{AuthorityEnvelope, AuthorityRefusal, Capability};
use wasm4pm_compat::receipt::Digest;
use wasm4pm_compat::witness::Ocel20;

#[test]
fn unconstrained_authority_envelope_is_refused() {
    let capability = Capability::<Ocel20>::new("ocel.import", Digest::new("blake3:abc"));
    let envelope = AuthorityEnvelope::new(capability, vec![], "account:123");
    assert_eq!(
        envelope.validate(),
        Err(vec![AuthorityRefusal::UnconstrainedEnvelope])
    );
}
