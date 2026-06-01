//! Law: WfNetForgeabilityHole --- WfNet::attest_witnessed() is deprecated; use WfNetConst.
//! This fixture ensures the deprecated path is flagged by the compiler.
#![deny(deprecated)]
use wasm4pm_compat::petri::WfNet;

fn main() {
    let net: WfNet<_> = todo!();
    let _witnessed = net.attest_witnessed(); // ERROR: deprecated
}
