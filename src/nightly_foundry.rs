//! Nightly foundry — zero-cost type-law surfaces derived from process-mining papers.
//!
//! Gated by `#[cfg(wasm4pm_compat_nightly)]`; never compiled on stable.
//! Enable with:
//!   `RUSTFLAGS="--cfg wasm4pm_compat_nightly" cargo +nightly check --all-features`
//!
//! ## Four surfaces, four nightly features, four paper mappings
//!
//! | Surface | Feature | Paper |
//! |---------|---------|-------|
//! | [`petri_law`] | `generic_const_exprs` | Murata (1989) §2 incidence matrices W⁻, W⁺ |
//! | [`powl_law`]  | `adt_const_params`    | Kourani (2505.07052) §3 POWL fragment kinds |
//! | [`evidence_law`] | `min_specialization` | Blue River Dam — admitted vs raw label |
//! | [`token_law`] | `portable_simd`       | Murata §2 enabling condition ∀p: M[p] ≥ W⁻[p][t] |
//!
//! ## Zero-cost guarantee
//!
//! Every type in this module is `#[repr(transparent)]` over a fixed-size array
//! or a `u32`, or is a zero-sized marker.  There is no heap allocation, no
//! runtime dispatch, and no branch in the hot path.  The nightly features move
//! paper-derived invariants into the *type system*, not into runtime machinery.

// ─────────────────────────────────────────────────────────────────────────────
// Surface 1: Bipartite Petri-net arc matrices  (generic_const_exprs)
// Paper: Murata (1989) IEEE Proc. 77(4) "Petri Nets: Properties, Analysis …"
//   §2: N = (P, T, F) is bipartite; arcs in F ⊆ (P×T) ∪ (T×P).
//   W⁻: P×T→ℕ pre-incidence, W⁺: T×P→ℕ post-incidence.
//   Enabling: ∀p: M[p] ≥ W⁻(p,t).  Firing: M'[p] = M[p]−W⁻(p,t)+W⁺(t,p).
//
// `generic_const_exprs` lets us write `[u8; P * T]` as a struct field —
// the flat arc matrix is zero-cost and bipartite-direction-safe at the type level:
//   PreMatrix<P, T>  ≠  PostMatrix<P, T>  (same count, opposite semantics).
// ─────────────────────────────────────────────────────────────────────────────

/// **Compile-pass law**: `Marking<P>::EMPTY` is a const-generic compile-time constant.
///
/// ```
/// use wasm4pm_compat::nightly_foundry::petri_law::Marking;
/// const M0: Marking<3> = Marking::EMPTY;
/// assert_eq!(M0.total_tokens(), 0);
/// let m1 = Marking([1u32, 2u32, 0u32]);
/// assert_eq!(m1.total_tokens(), 3);
/// ```
///
/// **Compile-pass law**: pre-matrix enabling check and firing are sound.
///
/// ```
/// use wasm4pm_compat::nightly_foundry::petri_law::{Marking, PreMatrix, PostMatrix};
/// // 2 places, 1 transition. p0 → t0 → p1.
/// let mut pre = PreMatrix::<2, 1>::ZERO;
/// pre.weights[0] = 1; // W⁻(p0,t0) = 1
/// let mut post = PostMatrix::<2, 1>::ZERO;
/// post.weights[1] = 1; // W⁺(t0,p1) = 1
/// let m = Marking([1u32, 0u32]);
/// assert!(pre.is_enabled(0, &m));
/// let m2 = post.fire(0, m, &pre);
/// assert_eq!(m2, Marking([0u32, 1u32]));
/// ```
pub mod petri_law {
    /// Token marking of exactly `P` places — M: P → ℕ.
    ///
    /// Paper: Murata (1989) §2 Def. 2 — M₀ ∈ ℕᴾ.
    /// Zero-cost: `#[repr(transparent)]` over `[u32; P]`.
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Marking<const P: usize>(pub [u32; P]);

    impl<const P: usize> Marking<P> {
        /// Zero marking: no tokens anywhere.
        pub const EMPTY: Self = Self([0u32; P]);

        /// Total token count across all places.
        #[inline]
        pub fn total_tokens(&self) -> u32 {
            let mut s = 0u32;
            let mut i = 0;
            while i < P {
                s += self.0[i];
                i += 1;
            }
            s
        }

        /// Token count at place `p`. Returns `None` if `p >= P`.
        #[inline]
        pub fn at(&self, p: usize) -> Option<u32> {
            self.0.get(p).copied()
        }
    }

    impl<const P: usize> Default for Marking<P> {
        fn default() -> Self { Self::EMPTY }
    }

    /// Pre-incidence matrix W⁻: P×T→ℕ, stored flat row-major as `[u8; P * T]`.
    ///
    /// Paper: Murata (1989) §2 — W⁻(p,t) = arc weight from place p to transition t.
    /// Enabling condition: ∀p: M[p] ≥ W⁻(p,t).
    ///
    /// **Requires `generic_const_exprs`**: `P * T` is a const expression in a
    /// where-bound and in the array-length field.  Zero-cost flat array, no heap.
    pub struct PreMatrix<const P: usize, const T: usize>
    where
        [(); P * T]: Sized,
    {
        /// Row-major weights; index `p * T + t`.
        pub weights: [u8; P * T],
    }

    impl<const P: usize, const T: usize> PreMatrix<P, T>
    where
        [(); P * T]: Sized,
    {
        /// Zero arc-weight matrix.
        pub const ZERO: Self = Self { weights: [0u8; P * T] };

        /// Arc weight W⁻(p, t).
        #[inline]
        pub fn w(&self, p: usize, t: usize) -> u8 { self.weights[p * T + t] }

        /// Is transition `t` enabled in marking `m`?
        ///
        /// Paper: Murata §2 Rule 1 — t is enabled iff ∀p: M[p] ≥ W⁻(p,t).
        #[inline]
        pub fn is_enabled(&self, t: usize, m: &Marking<P>) -> bool {
            (0..P).all(|p| m.0[p] >= self.weights[p * T + t] as u32)
        }
    }

    impl<const P: usize, const T: usize> Default for PreMatrix<P, T>
    where
        [(); P * T]: Sized,
    {
        fn default() -> Self { Self::ZERO }
    }

    /// Post-incidence matrix W⁺: T×P→ℕ, stored flat row-major as `[u8; T * P]`.
    ///
    /// Paper: Murata §2 — W⁺(t,p) = arc weight from transition t to place p.
    ///
    /// Note: `PostMatrix<P,T>` and `PreMatrix<P,T>` are **distinct types** even
    /// though `P*T == T*P` arithmetically.  The bipartite direction is in the type.
    pub struct PostMatrix<const P: usize, const T: usize>
    where
        [(); T * P]: Sized,
    {
        /// Row-major weights; index `t * P + p`.
        pub weights: [u8; T * P],
    }

    impl<const P: usize, const T: usize> PostMatrix<P, T>
    where
        [(); T * P]: Sized,
    {
        /// Zero arc-weight matrix.
        pub const ZERO: Self = Self { weights: [0u8; T * P] };

        /// Arc weight W⁺(t, p).
        #[inline]
        pub fn w(&self, t: usize, p: usize) -> u8 { self.weights[t * P + p] }

        /// Fire transition `t` on `m`, returning the new marking.
        ///
        /// Paper: Murata §2 Rule 2 — M'[p] = M[p] − W⁻(p,t) + W⁺(t,p).
        /// **Caller must first verify `pre.is_enabled(t, &m)`.**
        #[inline]
        pub fn fire(&self, t: usize, m: Marking<P>, pre: &PreMatrix<P, T>) -> Marking<P>
        where
            [(); P * T]: Sized,
        {
            let mut next = m;
            let mut p = 0;
            while p < P {
                next.0[p] = next.0[p]
                    - pre.weights[p * T + t] as u32
                    + self.weights[t * P + p] as u32;
                p += 1;
            }
            next
        }
    }

    impl<const P: usize, const T: usize> Default for PostMatrix<P, T>
    where
        [(); T * P]: Sized,
    {
        fn default() -> Self { Self::ZERO }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Surface 2: Typed POWL nodes  (adt_const_params)
// Paper: Kourani (arXiv:2505.07052) §3 — POWL recursive grammar:
//   POWL ::= A | X(M₁,M₂) | L(M₁,M₂) | P(M⁺, ≺) | τ
//
// `adt_const_params` + `ConstParamTy` let an enum variant become a const
// generic: `TypedNode<{ PowlKind::Atom }>` vs `TypedNode<{ PowlKind::Partial }>`.
// The compiler rejects calling atom-only APIs on a partial-order node.
// Zero-cost: KIND is fully erased at runtime; the struct is just a `u32` id.
// ─────────────────────────────────────────────────────────────────────────────

/// **Compile-fail law**: an `Atom` node must NOT expose the partial-order API.
///
/// This doctest is gated by the module's `#[cfg(wasm4pm_compat_nightly)]`, so it
/// only runs on nightly with `--cfg wasm4pm_compat_nightly`.  The `compile_fail`
/// annotation verifies that the type system refuses the call.
///
/// ```compile_fail
/// use wasm4pm_compat::nightly_foundry::powl_law::TypedNode;
/// let atom = TypedNode::atom(1u32);
/// // E0599: no method `are_concurrent` found for `TypedNode<{PowlKind::Atom}>`
/// let _ = atom.are_concurrent(&[], 1, 2);
/// ```
///
/// **Compile-fail law**: `Atom` and `Silent` are distinct types; assignment must fail.
///
/// ```compile_fail
/// use wasm4pm_compat::nightly_foundry::powl_law::{TypedNode, PowlKind};
/// // E0308: mismatched types — `TypedNode<{Atom}>` ≠ `TypedNode<{Silent}>`
/// let _: TypedNode<{ PowlKind::Silent }> = TypedNode::atom(0u32);
/// ```
///
/// **Compile-pass law**: a well-formed atom node is admitted.
///
/// ```
/// use wasm4pm_compat::nightly_foundry::powl_law::TypedNode;
/// let a = TypedNode::atom(42u32);
/// assert!(a.is_observable());
/// assert_eq!(a.id(), 42);
/// ```
pub mod powl_law {
    use core::marker::ConstParamTy;

    /// POWL fragment kind — used as a const generic parameter.
    ///
    /// Paper: Kourani (2505.07052) §3.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, ConstParamTy)]
    pub enum PowlKind {
        /// Atom: single activity node (leaf), observable.
        Atom,
        /// Exclusive choice (xor): exactly one branch fires.
        Xor,
        /// Loop: `do`-body with optional `redo`.
        Loop,
        /// Partial order: DAG of children with precedence edges.
        Partial,
        /// Silent step: tau, no observable activity.
        Silent,
    }

    /// A POWL node with its fragment kind encoded at the type level.
    ///
    /// `KIND` is erased at runtime — the value is just a `u32` id.
    /// Fragment-specific APIs are only available on the correct variant.
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct TypedNode<const KIND: PowlKind>(pub u32);

    // ── Atom ──────────────────────────────────────────────────────────────────
    impl TypedNode<{ PowlKind::Atom }> {
        #[inline] pub const fn atom(id: u32) -> Self { Self(id) }
        /// Atoms are always observable (carry an activity label).
        #[inline] pub const fn is_observable(&self) -> bool { true }
    }

    // ── Silent ────────────────────────────────────────────────────────────────
    impl TypedNode<{ PowlKind::Silent }> {
        #[inline] pub const fn silent(id: u32) -> Self { Self(id) }
        /// Silent steps are never observable.
        #[inline] pub const fn is_observable(&self) -> bool { false }
    }

    // ── Partial order ─────────────────────────────────────────────────────────

    /// Precedence edge a ≺ b within a POWL partial-order node.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct OrderEdge { pub before: u32, pub after: u32 }

    impl TypedNode<{ PowlKind::Partial }> {
        #[inline] pub const fn partial(id: u32) -> Self { Self(id) }

        /// Are `a` and `b` concurrent (neither precedes the other)?
        /// Paper: Kourani §3 — concurrency = absence of precedence in both directions.
        #[inline]
        pub fn are_concurrent(&self, edges: &[OrderEdge], a: u32, b: u32) -> bool {
            let ab = edges.iter().any(|e| e.before == a && e.after == b);
            let ba = edges.iter().any(|e| e.before == b && e.after == a);
            !ab && !ba
        }
    }

    // ── Xor ───────────────────────────────────────────────────────────────────
    impl TypedNode<{ PowlKind::Xor }> {
        #[inline] pub const fn xor(id: u32) -> Self { Self(id) }
        /// Minimum branch count for a well-formed choice node (≥ 2).
        #[inline] pub const fn min_branches() -> usize { 2 }
    }

    // ── Loop ──────────────────────────────────────────────────────────────────
    impl TypedNode<{ PowlKind::Loop }> {
        #[inline] pub const fn loop_node(id: u32) -> Self { Self(id) }
    }

    // ── Universal id accessor (macro avoids repeated `where` complexity) ──────
    macro_rules! impl_id {
        ($kind:expr) => {
            impl TypedNode<{ $kind }> {
                #[inline] pub const fn id(&self) -> u32 { self.0 }
            }
        };
    }
    impl_id!(PowlKind::Atom);
    impl_id!(PowlKind::Silent);
    impl_id!(PowlKind::Partial);
    impl_id!(PowlKind::Xor);
    impl_id!(PowlKind::Loop);
}

// ─────────────────────────────────────────────────────────────────────────────
// Surface 3: Evidence-kind label via specialization  (min_specialization)
// Doctrine: Blue River Dam — `Admitted` and `Raw` are first-class, distinct states.
//
// The blanket impl gives every T the label "raw".
// The specialised impl overrides that for T: AdmittedMarker to "admitted".
// Resolution is at compile time: no vtable, no branch, no heap.
// ─────────────────────────────────────────────────────────────────────────────

pub mod evidence_law {
    /// Compile-time evidence-kind label — `"raw"` or `"admitted"`.
    ///
    /// Uses `min_specialization` to override the blanket `"raw"` impl
    /// with `"admitted"` for any `Admitted<T>` wrapper — resolved at compile
    /// time with no vtable and no branch.
    pub trait EvidenceKind {
        fn kind_label(&self) -> &'static str;
    }

    // Blanket: every T that is not Admitted<_> is "raw".
    impl<T> EvidenceKind for T {
        default fn kind_label(&self) -> &'static str { "raw" }
    }

    /// Newtype wrapper that marks a value as having crossed a named boundary.
    /// Zero-cost: `#[repr(transparent)]` — same ABI as `T`.
    #[repr(transparent)]
    pub struct Admitted<T>(pub T);

    // Specialization on the concrete type constructor `Admitted<T>`.
    // `min_specialization` allows this because `Admitted<T>` is strictly
    // more specific than the blanket `T` — it narrows on the type constructor,
    // not on an arbitrary trait bound.
    impl<T> EvidenceKind for Admitted<T> {
        fn kind_label(&self) -> &'static str { "admitted" }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Surface 4: SIMD token-enabling check  (portable_simd)
// Paper: Murata (1989) §2 Rule 1 — t is enabled iff ∀p: M[p] ≥ W⁻(p,t).
//
// With portable_simd we check 4 or 8 places at once via u32x4 / u32x8.
// For small Petri subnets this is the entire enabling condition in one
// SIMD lane comparison + mask reduction — zero branches, no heap.
// ─────────────────────────────────────────────────────────────────────────────

pub mod token_law {
    use core::simd::{cmp::SimdPartialOrd, u32x4, u32x8};

    /// Check enabling for a 4-place subnet — single SIMD vector comparison.
    ///
    /// Returns `true` iff ∀p ∈ {0..4}: `marking[p] >= pre_weights[p]`.
    #[inline]
    pub fn transition_enabled_4(marking: [u32; 4], pre_weights: [u32; 4]) -> bool {
        u32x4::from_array(marking).simd_ge(u32x4::from_array(pre_weights)).all()
    }

    /// Check enabling for an 8-place subnet.
    #[inline]
    pub fn transition_enabled_8(marking: [u32; 8], pre_weights: [u32; 8]) -> bool {
        u32x8::from_array(marking).simd_ge(u32x8::from_array(pre_weights)).all()
    }

    /// Fire a transition on a 4-place marking via SIMD arithmetic.
    ///
    /// Paper: Murata §2 Rule 2 — M'[p] = M[p] − W⁻[p] + W⁺[p].
    /// **Requires `transition_enabled_4` was true.** No runtime check.
    #[inline]
    pub fn fire_4(marking: [u32; 4], pre: [u32; 4], post: [u32; 4]) -> [u32; 4] {
        (u32x4::from_array(marking) - u32x4::from_array(pre)
            + u32x4::from_array(post))
        .to_array()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests — run under nightly with --cfg wasm4pm_compat_nightly
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // petri_law ────────────────────────────────────────────────────────────────

    #[test]
    fn marking_empty_is_zero() {
        assert_eq!(petri_law::Marking::<4>::EMPTY.total_tokens(), 0);
    }

    #[test]
    fn pre_matrix_enables_and_blocks() {
        // 2 places, 2 transitions.
        // W⁻(p0,t0)=1, W⁻(p1,t1)=1; all others zero.
        let mut pre = petri_law::PreMatrix::<2, 2>::ZERO;
        pre.weights[0] = 1; // W⁻(p0,t0): p=0,t=0 → index p*T+t = 0*2+0 = 0
        pre.weights[3] = 1; // W⁻(p1,t1): p=1,t=1 → index p*T+t = 1*2+1 = 3

        let m = petri_law::Marking([1u32, 0u32]);
        assert!(pre.is_enabled(0, &m));  // t0 enabled: M[p0]=1 ≥ 1
        assert!(!pre.is_enabled(1, &m)); // t1 blocked: M[p1]=0 < 1
    }

    #[test]
    fn firing_token_moves_correctly() {
        // 2 places, 1 transition. p0 → t0 → p1.
        let mut pre = petri_law::PreMatrix::<2, 1>::ZERO;
        pre.weights[0] = 1; // W⁻(p0,t0) = 1
        let mut post = petri_law::PostMatrix::<2, 1>::ZERO;
        post.weights[1] = 1; // W⁺(t0,p1) = 1

        let m = petri_law::Marking([1u32, 0u32]);
        assert!(pre.is_enabled(0, &m));
        let m2 = post.fire(0, m, &pre);
        assert_eq!(m2, petri_law::Marking([0u32, 1u32]));
    }

    // powl_law ────────────────────────────────────────────────────────────────

    #[test]
    fn atom_observable_silent_not() {
        assert!(powl_law::TypedNode::atom(1).is_observable());
        assert!(!powl_law::TypedNode::silent(2).is_observable());
    }

    #[test]
    fn partial_concurrency_correct() {
        let p = powl_law::TypedNode::partial(0);
        let edges = [powl_law::OrderEdge { before: 1, after: 2 }];
        assert!(!p.are_concurrent(&edges, 1, 2)); // 1 ≺ 2: not concurrent
        assert!(p.are_concurrent(&edges, 1, 3));  // no edge: concurrent
    }

    #[test]
    fn xor_min_branches_is_two() {
        assert_eq!(powl_law::TypedNode::<{ powl_law::PowlKind::Xor }>::min_branches(), 2);
    }

    // evidence_law ────────────────────────────────────────────────────────────

    #[test]
    fn raw_u32_labels_raw() {
        use evidence_law::EvidenceKind;
        assert_eq!(42u32.kind_label(), "raw");
    }

    #[test]
    fn admitted_wrapper_labels_admitted() {
        use evidence_law::{Admitted, EvidenceKind};
        assert_eq!(Admitted(42u32).kind_label(), "admitted");
    }

    // token_law ───────────────────────────────────────────────────────────────

    #[test]
    fn simd_enabled_all_met() {
        assert!(token_law::transition_enabled_4([5, 3, 1, 0], [1, 1, 1, 0]));
    }

    #[test]
    fn simd_enabled_one_unmet() {
        assert!(!token_law::transition_enabled_4([5, 0, 1, 0], [1, 1, 1, 0]));
    }

    #[test]
    fn simd_fire_moves_tokens() {
        let m = token_law::fire_4([2, 0, 0, 0], [1, 0, 0, 0], [0, 1, 0, 0]);
        assert_eq!(m, [1, 1, 0, 0]);
    }

    #[test]
    fn simd_enabled_8_all_met() {
        assert!(token_law::transition_enabled_8(
            [9, 8, 7, 6, 5, 4, 3, 2],
            [1, 1, 1, 1, 1, 1, 1, 1],
        ));
    }
}
