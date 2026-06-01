#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
// COMPILE-FAIL: Petri net matrix law — PreMatrix<P,T> cannot be passed where PostMatrix<P,T> is required.
// Law: PreMatrix (pre-incidence W⁻) and PostMatrix (post-incidence W⁺) are distinct types
// even when P*T == T*P. The bipartite direction is encoded in the type constructor.
use wasm4pm_compat::nightly_foundry::petri_law::{PostMatrix, PreMatrix};

fn requires_post<const P: usize, const T: usize>(_m: PostMatrix<P, T>)
where
    [(); T * P]: Sized,
{
}

fn main() {
    let pre: PreMatrix<2, 3> = PreMatrix::ZERO;
    // This must fail: PreMatrix<2,3> is not PostMatrix<2,3>.
    requires_post(pre);
}
