use wasm4pm_compat::receipt::{Blake3Hash, Digest};

#[test]
fn blake3_alias_carries_identity_without_validating_hash_length() {
    let hash: Blake3Hash = Digest::new("not-a-validated-64-char-hash");
    assert_eq!(hash.as_inner(), "not-a-validated-64-char-hash");
}
