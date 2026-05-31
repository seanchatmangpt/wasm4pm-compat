// COMPILE-PASS: ReceiptChain::try_new empty chain refused — proves EmptyChain law

use wasm4pm_compat::receipt::{ReceiptChain, ReceiptRefusal};

fn main() {
    let result = ReceiptChain::try_new("run-x", vec![]);
    assert_eq!(result, Err(ReceiptRefusal::EmptyChain));
}
