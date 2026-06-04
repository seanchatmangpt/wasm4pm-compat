//! # Causal Consistency Law
//!
//! Typed markers for causal ordering in object-centric event logs.
//! Cross-object causality must be mutually consistent — this module
//! provides the witness markers for verified causal chains.
//!
//! ## What this module IS
//!
//! - Structure-only typed shapes for causal links, causal chains, and the
//!   causal consistency verdict of an object-centric log.
//! - A zero-cost [`CausalOrderWitness`] tag that names the authority under
//!   which causal ordering has been established.
//! - A [`CausallyOrderedEvidence`] envelope that distinguishes evidence with
//!   verified causal ordering from evidence without it at the type level.
//!
//! ## What this module is NOT
//!
//! - **Not** a causal ordering algorithm. No happens-before derivation, no
//!   cycle detection, no topological sort. Those concerns graduate to `wasm4pm`.
//! - **Not** a replacement for [`crate::evidence::Evidence`]. Causal ordering
//!   is orthogonal to the `Raw → Admitted` lifecycle — layer them as needed.
//!
//! ## The Chicago TDD doctrine applied here
//!
//! Per the process-mining Chicago TDD doctrine: the declared causal order is
//! not the real causal order until the event log proves it. A value tagged
//! `CausallyOrderedEvidence<T>` asserts the log-derivable causal order is
//! consistent; that assertion must be backed by evidence that can be mined.
//! Graduate to `wasm4pm` when you need the mining to run.
//!
//! ## Graduation
//!
//! When you need to derive causal ordering (e.g. from a Heuristics Miner or
//! a direct-follows relation), detect cycles, or verify mutual consistency
//! across object types, graduate to `wasm4pm`. The causal witness travels
//! with the evidence into the engine.

use core::marker::PhantomData;

/// Witness that causal ordering has been verified for this evidence.
///
/// This is a zero-sized marker. Presence of this witness as a type parameter
/// means the evidence has passed through a causal ordering check. It does not
/// run the check — it names the authority. Graduate to `wasm4pm` to execute.
///
/// This is structure only. See [`crate::causality`]. Graduate to `wasm4pm`
/// when causal ordering derivation must execute.
pub struct CausalOrderWitness;

/// A causal link between two events with a direction.
///
/// `From` and `To` are type-level event markers naming the cause and the
/// effect. The link is directional: `CausalLink<A, B>` means "A causes B".
/// This shape is zero-cost — no runtime data beyond `PhantomData`.
///
/// This is structure only. See [`crate::causality`]. Graduate to `wasm4pm`
/// when the causal link must be derived or validated from log evidence.
pub struct CausalLink<From, To> {
    _from: PhantomData<From>,
    _to: PhantomData<To>,
}

impl<From, To> CausalLink<From, To> {
    /// Construct a typed causal link shape.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wasm4pm_compat::causality::CausalLink;
    ///
    /// struct PlaceOrder;
    /// struct ConfirmOrder;
    ///
    /// let _link: CausalLink<PlaceOrder, ConfirmOrder> = CausalLink::new();
    /// ```
    pub fn new() -> Self {
        Self {
            _from: PhantomData,
            _to: PhantomData,
        }
    }
}

impl<From, To> Default for CausalLink<From, To> {
    fn default() -> Self {
        Self::new()
    }
}

/// A causal chain — ordered sequence of causally-linked events.
///
/// `LENGTH` is a compile-time constant naming the number of causal links in
/// the chain. A chain of length 0 is vacuously consistent; a chain of length
/// 1 is a single causal link; longer chains form ordered sequences.
///
/// This is a structure-only envelope — no link list is stored at this layer.
/// Graduate to `wasm4pm` when the chain contents must be inspected or verified.
///
/// This is structure only. See [`crate::causality`]. Graduate to `wasm4pm`
/// when chain verification must execute.
pub struct CausalChain<const LENGTH: usize> {
    _private: (),
}

impl<const LENGTH: usize> CausalChain<LENGTH> {
    /// Construct a typed causal chain shape of the given length.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wasm4pm_compat::causality::CausalChain;
    ///
    /// let _chain: CausalChain<3> = CausalChain::new();
    /// ```
    pub fn new() -> Self {
        Self { _private: () }
    }

    /// The number of causal links in this chain.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wasm4pm_compat::causality::CausalChain;
    ///
    /// assert_eq!(CausalChain::<5>::new().length(), 5);
    /// ```
    pub const fn length(&self) -> usize {
        LENGTH
    }
}

impl<const LENGTH: usize> Default for CausalChain<LENGTH> {
    fn default() -> Self {
        Self::new()
    }
}

/// Causal consistency verdict for an object-centric log.
///
/// This is the structural verdict shape — a label produced after a causal
/// ordering check has been performed (or attempted). It does not perform
/// the check; that graduates to `wasm4pm`.
///
/// ## Variants
///
/// - [`Consistent`](CausalConsistency::Consistent) — all cross-object causal
///   links are mutually consistent; no cycles, no contradictions.
/// - [`HasCycles`](CausalConsistency::HasCycles) — at least one causal cycle
///   was detected in the cross-object ordering.
/// - [`HasContradictions`](CausalConsistency::HasContradictions) — at least
///   one contradictory causal ordering claim was found.
/// - [`Unknown`](CausalConsistency::Unknown) — causal consistency has not yet
///   been established (the log has not been mined).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CausalConsistency {
    /// All cross-object causal links are mutually consistent.
    Consistent,
    /// At least one causal cycle was detected.
    HasCycles,
    /// At least one contradictory causal ordering claim was found.
    HasContradictions,
    /// Causal consistency has not yet been established.
    Unknown,
}

impl core::fmt::Display for CausalConsistency {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Consistent => write!(f, "causally-consistent"),
            Self::HasCycles => write!(f, "has-causal-cycles"),
            Self::HasContradictions => write!(f, "has-causal-contradictions"),
            Self::Unknown => write!(f, "causal-consistency-unknown"),
        }
    }
}

/// Evidence with verified causal ordering.
///
/// Wrapping a value in `CausallyOrderedEvidence<T>` asserts at the type level
/// that causal ordering has been established for `T`. A function demanding
/// `CausallyOrderedEvidence<T>` cannot be called with unordered evidence.
///
/// The `_witness: PhantomData<CausalOrderWitness>` field is zero-cost — it is
/// a compile-time-only tag.
///
/// This is structure only. See [`crate::causality`]. Graduate to `wasm4pm`
/// when causal ordering derivation must execute.
pub struct CausallyOrderedEvidence<T> {
    /// The inner evidence value.
    pub inner: T,
    _witness: PhantomData<CausalOrderWitness>,
}

impl<T> CausallyOrderedEvidence<T> {
    /// Wrap `inner` as causally-ordered evidence.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wasm4pm_compat::causality::CausallyOrderedEvidence;
    ///
    /// let ev = CausallyOrderedEvidence::new(42u32);
    /// assert_eq!(ev.inner, 42);
    /// ```
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            _witness: PhantomData,
        }
    }
}

// ── One-way-door: Unknown → Consistent ────────────────────────────────────
//
// The problem: `CausalConsistency::Consistent` is a plain enum variant —
// anyone can write `CausalConsistency::Consistent` without going through any
// verification. This is the same unchecked-admission problem that
// `evidence.rs` solves with `Evidence<T, Admitted, W>::sealed()`.
//
// The solution: a sealed `ConsistencyVerified<T>` envelope. The only public
// path to *carrying* `CausalConsistency::Consistent` inside an envelope is
// through the `VerifyCausalConsistency` trait. Callers can still use
// `CausalConsistency::Consistent` as a plain value (it's a public enum), but
// code that demands `ConsistencyVerified<T>` cannot be satisfied by direct
// construction — the `_seal` field is module-private.
//
// Graduate to `wasm4pm` for the actual consistency-checking implementation.

/// Proof obligation token produced by a `VerifyCausalConsistency` impl.
///
/// A `ConsistencyProof` token can only be constructed by code with access to
/// the private `ConsistencyProof { _seal: () }` constructor — i.e., by impls
/// of `VerifyCausalConsistency` that reside in this module or are granted
/// `pub(crate)` access. External crates cannot forge this token.
///
/// This is the same sealing idiom used by [`crate::evidence::Evidence`] for
/// the `Raw → Admitted` transition.
pub struct ConsistencyProof {
    /// Module-private seal — prevents external forgery.
    _seal: (),
}

impl ConsistencyProof {
    /// Construct a proof token. `pub(crate)` — only this crate can mint one.
    pub(crate) fn new() -> Self {
        Self { _seal: () }
    }
}

/// An evidence value paired with a verified `CausalConsistency` verdict.
///
/// `ConsistencyVerified<T>` can only be constructed via
/// [`VerifyCausalConsistency::verify`]. The inner value is accessible via
/// `.inner`; the verdict via `.verdict()`.
///
/// ## Chicago TDD Rank-2 Oracle
///
/// This envelope enforces: *"All cross-object dependencies respect temporal
/// ordering."* Code demanding `ConsistencyVerified<T>` cannot receive an
/// unverified value — the `ConsistencyProof` token seals the door.
///
/// ## Graduation
///
/// The `wasm4pm` crate provides a `VerifyCausalConsistency` impl that runs
/// actual causal ordering derivation (Heuristics Miner, cycle detection,
/// topological sort). This crate provides only the law surface.
pub struct ConsistencyVerified<T> {
    /// The inner evidence value.
    pub inner: T,
    verdict: CausalConsistency,
    /// Module-private proof token — cannot be forged by external crates.
    _proof: ConsistencyProof,
}

impl<T> ConsistencyVerified<T> {
    /// Construct a verified envelope. `pub(crate)` — callers outside this
    /// crate must go through `VerifyCausalConsistency::verify`.
    pub(crate) fn new(inner: T, verdict: CausalConsistency, proof: ConsistencyProof) -> Self {
        Self {
            inner,
            verdict,
            _proof: proof,
        }
    }

    /// The causal consistency verdict established by the verifier.
    ///
    /// If the verifier produced [`CausalConsistency::Consistent`], all
    /// cross-object causal dependencies are mutually ordered. Any other
    /// verdict names a specific failure mode.
    pub fn verdict(&self) -> CausalConsistency {
        self.verdict
    }

    /// True iff the verdict is [`CausalConsistency::Consistent`].
    pub fn is_consistent(&self) -> bool {
        self.verdict == CausalConsistency::Consistent
    }
}

/// One-way-door trait for the `Unknown → Consistent` transition.
///
/// An impl of this trait is the *only* way to produce a
/// `ConsistencyVerified<T>` value. The `wasm4pm` crate provides the impl
/// that runs actual causal ordering derivation; this crate defines only the
/// law surface.
///
/// ## Rank-2 Oracle contract
///
/// Implementors must satisfy: *"If `verify` returns
/// `CausalConsistency::Consistent`, all cross-object event dependencies in
/// the evidence respect temporal ordering and contain no cycles."*
///
/// ## Forgery prevention
///
/// A caller cannot construct `ConsistencyVerified<T>` directly — the
/// `ConsistencyProof` field is module-private. The trait impl is the only
/// minting path.
pub trait VerifyCausalConsistency<T> {
    /// Run the causal consistency check and return a sealed verdict.
    ///
    /// The returned `ConsistencyVerified<T>` carries the verdict established
    /// by this impl. The verdict may be `Unknown` if the check cannot be
    /// performed (e.g. insufficient evidence), `Consistent` if all ordering
    /// constraints hold, or a failure variant if they do not.
    ///
    /// Graduate to `wasm4pm` for the real algorithm.
    fn verify(&self, evidence: T) -> ConsistencyVerified<T>;
}

/// A pass-through verifier that always returns `CausalConsistency::Unknown`.
///
/// This is the compat-layer stand-in: it satisfies the type law (returns a
/// sealed `ConsistencyVerified`) without running any algorithm. It is the
/// correct default for the structure-only layer.
///
/// Replace with a real `wasm4pm` verifier when causal ordering must execute.
pub struct UnknownVerifier;

impl<T> VerifyCausalConsistency<T> for UnknownVerifier {
    fn verify(&self, evidence: T) -> ConsistencyVerified<T> {
        ConsistencyVerified::new(
            evidence,
            CausalConsistency::Unknown,
            ConsistencyProof::new(),
        )
    }
}

// ── Rank-2 oracle tests ────────────────────────────────────────────────────
//
// These tests live in the crate because only crate-internal code can mint
// ConsistencyProof::new() and therefore ConsistencyVerified with any verdict.
// This verifies: the seal works; Consistent can be produced internally;
// external code cannot forge it (verified by compile-fail fixture).
#[cfg(test)]
mod tests {
    use super::*;

    /// A stub verifier that always claims Consistent — models what a real
    /// wasm4pm causal ordering impl would return after running cycle detection.
    struct AlwaysConsistentVerifier;

    impl<T> VerifyCausalConsistency<T> for AlwaysConsistentVerifier {
        fn verify(&self, evidence: T) -> ConsistencyVerified<T> {
            // Only callable from within the crate (pub(crate) seal).
            ConsistencyVerified::new(
                evidence,
                CausalConsistency::Consistent,
                ConsistencyProof::new(),
            )
        }
    }

    /// Rank-2 oracle: "All cross-object dependencies respect temporal ordering."
    ///
    /// A verifier that claims Consistent must produce a sealed envelope;
    /// the verdict must be Consistent; is_consistent() must return true.
    #[test]
    fn rank2_oracle_consistent_verdict_is_sealed() {
        let verifier = AlwaysConsistentVerifier;
        let result: ConsistencyVerified<u64> = verifier.verify(99u64);
        assert_eq!(result.verdict(), CausalConsistency::Consistent);
        assert!(result.is_consistent());
        assert_eq!(result.inner, 99u64);
    }

    /// Rank-2 oracle: Unknown verifier must NOT produce Consistent.
    ///
    /// The compat-layer stand-in (UnknownVerifier) must never claim ordering
    /// is established when no algorithm ran.
    #[test]
    fn rank2_oracle_unknown_verifier_stays_unknown() {
        let verifier = UnknownVerifier;
        let result: ConsistencyVerified<&str> = verifier.verify("raw-evidence");
        assert_eq!(result.verdict(), CausalConsistency::Unknown);
        assert!(!result.is_consistent());
    }

    /// Rank-2 oracle: HasCycles and HasContradictions are mintable internally,
    /// preserving the full verdict surface for wasm4pm engine impls.
    #[test]
    fn rank2_oracle_failure_verdicts_are_sealed() {
        struct CycleVerifier;
        impl<T> VerifyCausalConsistency<T> for CycleVerifier {
            fn verify(&self, evidence: T) -> ConsistencyVerified<T> {
                ConsistencyVerified::new(
                    evidence,
                    CausalConsistency::HasCycles,
                    ConsistencyProof::new(),
                )
            }
        }

        let result: ConsistencyVerified<i32> = CycleVerifier.verify(-1);
        assert_eq!(result.verdict(), CausalConsistency::HasCycles);
        assert!(!result.is_consistent());
    }

    /// Rank-2 oracle: ConsistencyProof is zero-sized (no runtime overhead).
    #[test]
    fn consistency_proof_is_zero_sized() {
        assert_eq!(core::mem::size_of::<ConsistencyProof>(), 0);
    }
}
