use wasm4pm_compat::authority::Capability;
use wasm4pm_compat::receipt::Digest;
use wasm4pm_compat::witness::Ocel20;

#[test]
fn whitespace_only_capability_name_is_not_pinned() {
    let capability = Capability::<Ocel20>::new("   ", Digest::new("blake3:abc"));
    assert!(!capability.is_pinned());
}
