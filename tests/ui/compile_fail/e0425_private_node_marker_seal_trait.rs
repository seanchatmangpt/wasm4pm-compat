//! Law: NodeMarkerSealUnaccessibility — node_marker_seal module is private.
//! Structural guarantee: External code cannot impl PlaceSeal to forge node markers
//! because node_marker_seal is a private module — no external access is possible.
//! Expected error: E0603: module `node_marker_seal` is private.

use wasm4pm_compat::petri::node_marker_seal;

struct CustomPlace;

fn main() {
    // STRUCTURAL LAW: node_marker_seal is a private module — cannot access PlaceSeal
    // PROOF: Attempting to access node_marker_seal module fails at compile time with E0603
    let _ = CustomPlace;
}
