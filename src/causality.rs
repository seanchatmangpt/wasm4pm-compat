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
