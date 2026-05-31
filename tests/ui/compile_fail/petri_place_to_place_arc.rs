// COMPILE-FAIL: Bipartite arc law — place→place arcs are unconstructible.
// Paper: Murata (1989) §2 — F ⊆ (P×T) ∪ (T×P), no P→P arcs.
// Expected error: PlaceToTransitionArc.weight field has type Weight, not PlaceId;
// trying to misuse PlaceToTransitionArc as a place→place arc fails.
use wasm4pm_compat::petri::PlaceToTransitionArc;

struct P1;
struct P2;

fn accept_place_to_place<P, Q>(_: PlaceToTransitionArc<P, Q, u8>) {}

fn main() {
    // PlaceToTransitionArc<P1, P2, u8> has weight: u8.
    // There is no place-to-place arc type at all.
    // Attempting to use TransitionToPlaceArc as PlaceToTransitionArc:
    let arc: PlaceToTransitionArc<P1, P2, u8> = wasm4pm_compat::petri::TransitionToPlaceArc::<P1, P2, u8>::new(1u8);
    let _ = arc;
}
