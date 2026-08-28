use wasm4pm_compat::authority::Capability;
use wasm4pm_compat::receipt::Digest;
use wasm4pm_compat::witness::Ocel20;

#[test]
fn named_capability_with_digest_is_structurally_pinned() {
    let capability = Capability::<Ocel20>::new("ocel.import", Digest::new("blake3:abc"));
    assert!(capability.is_pinned());
    assert_eq!(capability.name, "ocel.import");
    assert_eq!(capability.digest.0, "blake3:abc");
}
