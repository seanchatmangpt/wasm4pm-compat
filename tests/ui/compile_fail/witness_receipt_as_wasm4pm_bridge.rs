// COMPILE-FAIL: Witness confusion law — Evidence<T, Admitted, ReceiptFamily> cannot be used
// where Evidence<T, Admitted, Wasm4pmBridge> is required.
// Law: ReceiptFamily and Wasm4pmBridge are distinct witnesses. Receipt evidence
// has not been graduated to the wasm4pm bridge.
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::Admitted;
use wasm4pm_compat::witness::{ReceiptFamily, Wasm4pmBridge};

fn requires_bridge_evidence(_: Evidence<String, Admitted, Wasm4pmBridge>) {}

fn main() {
    let receipt_ev: Evidence<String, Admitted, ReceiptFamily> = todo!();
    // This must fail: ReceiptFamily witness is not Wasm4pmBridge.
    requires_bridge_evidence(receipt_ev);
}
