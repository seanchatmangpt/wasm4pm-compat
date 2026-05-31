// COMPILE-PASS: ReceiptEnvelope all four fields — proves that subject, witness,
// digest, and replay_hint are all accessible public fields on a constructed
// ReceiptEnvelope, and that the type carries (not computes) each value.
//
// Law: Blue River Dam covenant — ReceiptEnvelope is structure-only. All four
// fields are public and directly inspectable. The envelope never computes a
// digest or executes a replay — it carries values produced elsewhere.
use wasm4pm_compat::receipt::{Digest, ReceiptEnvelope, ReplayHint};

fn main() {
    let env = ReceiptEnvelope::new(
        "artifact-ocel-log-007",
        "OCEL-2.0-admission",
        Digest::new("blake3:0011223344556677"),
        ReplayHint::new("wasm4pm://intake/ocel-log-007"),
    );

    // All four public fields are accessible by name.
    assert_eq!(env.subject, "artifact-ocel-log-007");
    assert_eq!(env.witness, "OCEL-2.0-admission");
    assert_eq!(env.digest.0, "blake3:0011223344556677");
    assert_eq!(env.replay_hint.0, "wasm4pm://intake/ocel-log-007");

    // is_well_shaped passes when all four are non-empty.
    assert!(env.is_well_shaped());

    // Clone preserves all four fields.
    let cloned = env.clone();
    assert_eq!(cloned.subject, env.subject);
    assert_eq!(cloned.witness, env.witness);
    assert_eq!(cloned.digest, env.digest);
    assert_eq!(cloned.replay_hint, env.replay_hint);

    // PartialEq on ReceiptEnvelope compares all four fields structurally.
    assert_eq!(env, cloned);
}
