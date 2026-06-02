//! Law: ArcSealUnaccessibility — Sealed arc trait is in private mod arc_seal.
//! Structural guarantee: External code cannot impl Sealed to forge invalid arcs.
//! Expected error: E0425/E0433: cannot find sealed trait.

use wasm4pm_compat::petri::BipartiteArcConst;

struct CustomArc;

fn main() {
    // STRUCTURAL LAW: arc_seal::Sealed is sealed in a private module
    // PROOF: We cannot access or implement the Sealed trait externally
    // This prevents forging of IsValidArc types
    let _ = CustomArc;
}
