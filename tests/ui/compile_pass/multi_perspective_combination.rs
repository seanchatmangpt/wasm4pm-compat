//! Compile-pass fixture: multi-perspective evidence combination compiles.
//!
//! Proves that combining two perspective markers into a
//! `PerspectiveCombination` and wrapping them in `MultiPerspectiveEvidence`
//! is representable and compiles without error.

use wasm4pm_compat::multiperspective::{
    ControlFlowPerspective, DataPerspective, MultiPerspectiveEvidence, PerspectiveCombination,
};

fn main() {
    // Control-flow + Data perspective combination
    let ev: MultiPerspectiveEvidence<
        &str,
        PerspectiveCombination<ControlFlowPerspective, DataPerspective>,
    > = MultiPerspectiveEvidence::new("place_order");

    assert_eq!(ev.inner, "place_order");
}
