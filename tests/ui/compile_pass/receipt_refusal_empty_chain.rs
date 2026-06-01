// Law: ReceiptChainEmptyLaw — ReceiptChain::try_new with an empty vec is refused as ReceiptRefusal::EmptyChain; a chain with no links cannot exist
// COMPILE-PASS: ReceiptChain::try_new empty chain refused — proves EmptyChain law

use wasm4pm_compat::receipt::{ReceiptChain, ReceiptRefusal};

fn main() {
    let result = ReceiptChain::try_new("run-x", vec![]);
    assert_eq!(result, Err(ReceiptRefusal::EmptyChain));
}
