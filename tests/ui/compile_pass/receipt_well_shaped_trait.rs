// Law: WellShapedTraitLaw — all receipt types (ReceiptShape, ReceiptEnvelope, ReceiptChain, GraduationReceipt) satisfy the WellShaped bound; well_shaped() is a uniform shape-check surface
// COMPILE-PASS: WellShaped trait — proves all receipt types satisfy WellShaped bound

use wasm4pm_compat::receipt::{
    Digest, GraduationReceipt, ReceiptChain, ReceiptEnvelope, ReceiptShape, ReplayHint, WellShaped,
};

fn check_well_shaped<T: WellShaped>(val: &T) -> bool {
    val.well_shaped()
}

fn main() {
    let shape = ReceiptShape::new("w", Digest::new("d"), ReplayHint::new("h"));
    assert!(check_well_shaped(&shape));

    let env = ReceiptEnvelope::new("s", "w", Digest::new("d"), ReplayHint::new("h"));
    assert!(check_well_shaped(&env));

    let link = ReceiptEnvelope::new("s", "w", Digest::new("d"), ReplayHint::new("h"));
    let chain = ReceiptChain::try_new("id", vec![link]).unwrap();
    assert!(check_well_shaped(&chain));

    let gr_env = ReceiptEnvelope::new("s", "w", Digest::new("d"), ReplayHint::new("h"));
    let gr = GraduationReceipt::new(gr_env, "needs_replay");
    assert!(check_well_shaped(&gr));
}
