//! Law: WfNetSealPrivacy — wfnet_seal module is private and inaccessible.
//! Structural guarantee: The WfNetSeal type is only constructible inside petri module.
//! Expected error: E0433: cannot find module `wfnet_seal` in scope or E0425: cannot find type.
use wasm4pm_compat::petri::wfnet_seal;

fn main() {
    // STRUCTURAL LAW: wfnet_seal is a private module
    // PROOF: Direct access to wfnet_seal should fail
    let _ = wfnet_seal::WfNetSeal;
}
