// COMPILE-FAIL: Receipt chain law — ReceiptChain (dynamic Vec-backed) cannot be passed
// where ReceiptChainConst<N> (fixed const-generic array) is required.
// Law: ReceiptChain (dynamic) and ReceiptChainConst<N> (const-generic, fixed arity)
// are distinct types. A dynamic chain cannot replace a statically-sized chain.
use wasm4pm_compat::receipt::{ReceiptChain, ReceiptChainConst};

fn requires_chain_const(_c: ReceiptChainConst<1>) {}

fn main() {
    let dynamic_chain: ReceiptChain = todo!();
    // This must fail: ReceiptChain is not ReceiptChainConst<1>.
    requires_chain_const(dynamic_chain);
}
