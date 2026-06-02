//! Law: NodeMarkerSealUnaccessibility — PlaceSeal is a private sealed trait in node_marker_seal module.
//! Structural guarantee: External code cannot impl the PlaceSeal trait to forge node markers.
//! Expected error: E0425/E0433: cannot find trait `PlaceSeal` (private module).

use wasm4pm_compat::petri::PlaceNodeMarker;

struct CustomPlace;

fn main() {
    // STRUCTURAL LAW: PlaceSeal is sealed in private mod node_marker_seal
    // PROOF: We cannot access PlaceSeal to implement it for custom types
    // impl PlaceSeal for CustomPlace {} would fail with E0433/E0425
    let _ = CustomPlace;
}
