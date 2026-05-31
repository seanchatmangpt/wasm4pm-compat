// COMPILE-PASS: nightly_foundry::petri_law sub-module exposes its law surface
// (no engine logic, structure-only types) — zero nightly_foundry coverage today.
//
// Law: Murata (1989) IEEE Proc. 77(4) §2 — bipartite arc matrices W⁻, W⁺;
// token marking M: P → ℕ; enabling condition ∀p: M[p] ≥ W⁻(p,t);
// firing rule M'[p] = M[p] − W⁻(p,t) + W⁺(t,p).
//
// generic_const_exprs: P*T used as array-length const expression.
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use wasm4pm_compat::nightly_foundry::petri_law::{Marking, PostMatrix, PreMatrix};

fn main() {
    // Zero marking: EMPTY const is constructible.
    const M0: Marking<3> = Marking::EMPTY;
    assert_eq!(M0.total_tokens(), 0);

    // Marking with tokens.
    let m = Marking([2u32, 1u32, 0u32]);
    assert_eq!(m.total_tokens(), 3);
    assert_eq!(m.at(0), Some(2));
    assert_eq!(m.at(3), None);

    // 3 places, 2 transitions.
    // p0 → t0 → p1, p1 → t1 → p2.
    let mut pre = PreMatrix::<3, 2>::ZERO;
    pre.weights[0 * 2 + 0] = 1; // W⁻(p0, t0) = 1
    pre.weights[1 * 2 + 1] = 1; // W⁻(p1, t1) = 1

    let mut post = PostMatrix::<3, 2>::ZERO;
    post.weights[0 * 3 + 1] = 1; // W⁺(t0, p1) = 1
    post.weights[1 * 3 + 2] = 1; // W⁺(t1, p2) = 1

    // Initial marking: one token in p0.
    let m_init = Marking([1u32, 0u32, 0u32]);

    // t0 is enabled: M[p0]=1 ≥ W⁻(p0,t0)=1.
    assert!(pre.is_enabled(0, &m_init));
    // t1 is NOT enabled: M[p1]=0 < W⁻(p1,t1)=1.
    assert!(!pre.is_enabled(1, &m_init));

    // Fire t0: token moves from p0 to p1.
    let m_after_t0 = post.fire(0, m_init, &pre);
    assert_eq!(m_after_t0, Marking([0u32, 1u32, 0u32]));

    // Now t1 is enabled.
    assert!(pre.is_enabled(1, &m_after_t0));

    // Fire t1: token moves from p1 to p2.
    let m_final = post.fire(1, m_after_t0, &pre);
    assert_eq!(m_final, Marking([0u32, 0u32, 1u32]));

    // Default is ZERO/EMPTY.
    let pre_default = PreMatrix::<2, 2>::default();
    assert_eq!(pre_default.w(0, 0), 0);
}
