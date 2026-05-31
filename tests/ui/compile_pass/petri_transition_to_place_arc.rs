// COMPILE-PASS: Bipartite arc law — T→P arc is the lawful post-incidence direction.
use wasm4pm_compat::petri::{TransitionToPlaceArc, IsValidArc};

struct T1;
struct P1;

fn accept<A: IsValidArc>(_: A) {}

fn main() {
    let arc = TransitionToPlaceArc::<T1, P1, u32>::new(2u32);
    assert_eq!(arc.weight(), 2);
    accept(arc);  // T→P arc implements IsValidArc
}
