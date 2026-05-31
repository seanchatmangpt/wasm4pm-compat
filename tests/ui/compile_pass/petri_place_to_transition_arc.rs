// COMPILE-PASS: Bipartite arc law — P→T arc is the lawful pre-incidence direction.
use wasm4pm_compat::petri::{PlaceToTransitionArc, IsValidArc};

struct P1;
struct T1;

fn accept<A: IsValidArc>(_: A) {}

fn main() {
    let arc = PlaceToTransitionArc::<P1, T1, u8>::new(1u8);
    assert_eq!(arc.weight(), 1);
    accept(arc);  // P→T arc implements IsValidArc
}
