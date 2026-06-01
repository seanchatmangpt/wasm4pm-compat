// Law: ReceiptChainConstWellShapedLaw — ReceiptChainConst<N> satisfies WellShaped bound; the const-arity chain passes a uniform shape-check surface
// COMPILE-PASS: ReceiptChainConst WellShaped — proves WellShaped impl on const-generic chain

use wasm4pm_compat::receipt::{Digest, ReceiptChainConst, ReceiptEnvelope, ReplayHint, WellShaped};

fn check<T: WellShaped>(t: &T) -> bool {
    t.well_shaped()
}

fn main() {
    let a = ReceiptEnvelope::new("root", "w", Digest::new("d0"), ReplayHint::new("h0"));
    let b = ReceiptEnvelope::new("step", "w", Digest::new("d1"), ReplayHint::new("h1"));
    let chain = ReceiptChainConst::try_new("run", [a, b]).unwrap();
    assert!(check(&chain));
    assert_eq!(chain.arity(), 2);
}
