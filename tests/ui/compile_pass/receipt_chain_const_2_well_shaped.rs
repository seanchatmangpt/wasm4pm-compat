// COMPILE-PASS: ReceiptChainConst<2> WellShaped trait dispatch — proves the
// WellShaped trait is implemented for the const-generic 2-link chain and that
// iter() visits exactly two envelopes.
//
// Law: Blue River Dam covenant — ReceiptChainConst<N> encodes arity in the
// type. A 2-link chain is a different type from a 1-link or 3-link chain. The
// WellShaped trait provides a uniform shape-check surface across all receipt
// types.
use wasm4pm_compat::receipt::{Digest, ReceiptChainConst, ReceiptEnvelope, ReplayHint, WellShaped};

fn assert_well_shaped(r: &dyn WellShaped) {
    assert!(r.well_shaped());
}

fn main() {
    let a = ReceiptEnvelope::new(
        "stage-seeded",
        "discovery-run",
        Digest::new("blake3:seed000"),
        ReplayHint::new("rerun:plan#seeded"),
    );
    let b = ReceiptEnvelope::new(
        "stage-bred",
        "conformance-check",
        Digest::new("blake3:bred111"),
        ReplayHint::new("rerun:plan#bred"),
    );

    let chain: ReceiptChainConst<2> = ReceiptChainConst::try_new("run-seeded-bred", [a, b]).unwrap();

    // Arity is exactly 2.
    assert_eq!(chain.arity(), 2);

    // WellShaped trait dispatches correctly via dyn.
    assert_well_shaped(&chain);

    // iter() visits both links in order.
    let count = chain.iter().count();
    assert_eq!(count, 2);

    // chain_id is accessible as a public field.
    assert_eq!(chain.chain_id, "run-seeded-bred");
}
