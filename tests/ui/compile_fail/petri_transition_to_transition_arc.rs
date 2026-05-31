// COMPILE-FAIL: Bipartite arc law — transition→transition arcs are unconstructible.
// Paper: Murata (1989) §2 — F ⊆ (P×T) ∪ (T×P), no T→T arcs.
// Expected error: TransitionToPlaceArc<T1, T2, u8> cannot be assigned to
// PlaceToTransitionArc<T1, T2, u8> — distinct types.
use wasm4pm_compat::petri::{PlaceToTransitionArc, TransitionToPlaceArc};

struct T1;
struct T2;

fn main() {
    // Trying to treat a T→P arc as a P→T arc:
    let arc: PlaceToTransitionArc<T1, T2, u8> = TransitionToPlaceArc::<T1, T2, u8>::new(1u8);
    let _ = arc;
}
