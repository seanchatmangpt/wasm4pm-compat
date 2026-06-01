//! Law: WfNetForgeabilityHole --- WfNet::attest_witnessed() is private to prevent forgery.
//! This fixture ensures that the deprecated attest_witnessed method is private and E0624 is triggered.

use wasm4pm_compat::petri::WfNet;

fn main() {
    let net: WfNet<_> = todo!();
    let _witnessed = net.attest_witnessed(); // ERROR: E0624
}
