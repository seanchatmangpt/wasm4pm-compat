//! Law: ArcSealUnaccessibility — Sealed arc trait is in private mod arc_seal.
//! Structural guarantee: External code cannot impl IsValidArc to forge invalid arcs
//! because arc_seal::Sealed is in a private module — no external impl is possible.
//! Expected error: E0603: module `arc_seal` is private.

use wasm4pm_compat::petri::arc_seal;

struct CustomArc;

fn main() {
    // STRUCTURAL LAW: arc_seal is a private module — cannot access Sealed trait
    // PROOF: Attempting to access arc_seal module fails at compile time with E0603
    let _ = CustomArc;
}
