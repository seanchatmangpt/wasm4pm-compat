//! Law: NodeMarkerSealPrivacy — node_marker_seal module is private and inaccessible.
//! Structural guarantee: Sealed marker traits prevent impl-leakage attacks.
//! Expected error: E0433: cannot find module `node_marker_seal` in scope or E0425: cannot find function/type.
use wasm4pm_compat::petri::node_marker_seal;

fn main() {
    // STRUCTURAL LAW: node_marker_seal is a private module
    // PROOF: Direct access to node_marker_seal should fail
    let _ = node_marker_seal::PlaceSeal;
}
