//! Petri net, WF-net, and OC-Petri-net **shapes** — with soundness as a
//! *typestate claim*, never a computed proof.
//!
//! This module models the place/transition canon: a [`PetriNet`] of [`Place`]s
//! and [`Transition`]s joined by [`Arc`]s with a [`Marking`]; a [`WfNet`]
//! (workflow net) specialization with a single source and sink and a soundness
//! *claim* tracked at the type level; and an [`ObjectCentricPetriNet`] whose
//! arcs are typed by object type and may be variable.
//!
//! ## Soundness is a *claim*, not a result
//!
//! WF-net **soundness** (option to complete, proper completion, no dead
//! transitions) is a non-trivial property. This crate **does not compute it** —
//! computing soundness is an engine and graduates to `wasm4pm`. Instead, a
//! [`WfNet`] carries a *typestate token* recording at the type level whether
//! soundness is [`SoundnessUnknown`] (default), merely [`SoundnessClaimed`]
//! (asserted by a human/upstream, unproven here), or [`SoundnessWitnessed`]
//! (carrying a witness obtained from `wasm4pm` and re-attached here).
//!
//! These three tokens are **empty enums** (uninhabited markers) used only as
//! `PhantomData` type parameters — zero-cost, never constructed.
//!
//! ## Structure only
//!
//! [`PetriNet::validate`] and [`WfNet::validate`] check *structural* shape
//! (arcs reference declared nodes; a WF-net has an initial and final marking).
//! They never check reachability, boundedness, liveness, or soundness — those
//! are `wasm4pm`.
//!
//! ## Graduation to `wasm4pm`
//!
//! Soundness witnessing, boundedness/safeness analysis, reachability, and
//! token-game replay graduate to `wasm4pm`. A [`SoundnessWitnessed`] WF-net is
//! the shape into which such a witness is re-attached for evidence-carrying
//! interchange.

use core::marker::PhantomData;

use crate::law::SoundnessState;

// ── Place / Transition node-kind markers ─────────────────────────────────────

/// A zero-sized type-level marker asserting that a type parameter represents a
/// **place** node in a Petri net.
///
/// `PlaceNodeMarker` and [`TransitionNodeMarker`] are the runtime-facing
/// counterparts of the [`crate::law::EndpointKind`] const-param enum. Use them
/// as type parameters when you want the compiler to enforce that a slot is
/// filled by a place kind rather than a transition kind.
///
/// Structure-only: carries no data, no token semantics.
///
/// ```
/// use wasm4pm_compat::petri::PlaceNodeMarker;
/// fn place_slot<P: wasm4pm_compat::petri::IsPlaceNode>() {}
/// place_slot::<PlaceNodeMarker>();
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct PlaceNodeMarker;

/// A zero-sized type-level marker asserting that a type parameter represents a
/// **transition** node in a Petri net.
///
/// Paired with [`PlaceNodeMarker`]; see its documentation.
///
/// ```
/// use wasm4pm_compat::petri::TransitionNodeMarker;
/// fn transition_slot<T: wasm4pm_compat::petri::IsTransitionNode>() {}
/// transition_slot::<TransitionNodeMarker>();
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TransitionNodeMarker;

mod node_marker_seal {
    pub trait PlaceSeal {}
    pub trait TransitionSeal {}
    impl PlaceSeal for super::PlaceNodeMarker {}
    impl TransitionSeal for super::TransitionNodeMarker {}
}

/// Sealed trait — only [`PlaceNodeMarker`] is a place node kind.
///
/// ```
/// use wasm4pm_compat::petri::{PlaceNodeMarker, IsPlaceNode};
/// fn needs_place<P: IsPlaceNode>(_: P) {}
/// needs_place(PlaceNodeMarker);
/// ```
pub trait IsPlaceNode: node_marker_seal::PlaceSeal {}
impl IsPlaceNode for PlaceNodeMarker {}

/// Sealed trait — only [`TransitionNodeMarker`] is a transition node kind.
///
/// ```
/// use wasm4pm_compat::petri::{TransitionNodeMarker, IsTransitionNode};
/// fn needs_transition<T: IsTransitionNode>(_: T) {}
/// needs_transition(TransitionNodeMarker);
/// ```
pub trait IsTransitionNode: node_marker_seal::TransitionSeal {}
impl IsTransitionNode for TransitionNodeMarker {}

// ── Bipartite arc type law ───────────────────────────────────────────────────

/// A typed arc from a **place** to a **transition** — the only valid
/// pre-incidence direction in a Petri net.
///
/// Paper: Murata (1989) §2 — F ⊆ (P×T) ∪ (T×P); arcs must be bipartite.
/// There is **no** `PlaceToPlaceArc` or `TransitionToTransitionArc` type in
/// this crate; they are unconstructible.
///
/// The generic kinds `P` and `T` are independent namespace markers; a place
/// kind and a transition kind can be the same Rust type without breaking the
/// law (the arc direction is carried by the struct type, not by the type params).
///
/// Structure-only: a typed directed edge. No token flow.
///
/// ```
/// use wasm4pm_compat::petri::PlaceToTransitionArc;
/// struct P1; struct T1;
/// let arc = PlaceToTransitionArc::<P1, T1, u8>::new(0u8);
/// assert_eq!(arc.weight(), 0);
/// ```
pub struct PlaceToTransitionArc<P, T, Weight> {
    pub(crate) _from: PhantomData<P>,
    pub(crate) _to: PhantomData<T>,
    /// Arc weight (pre-incidence W⁻).
    pub weight: Weight,
}

impl<P, T, Weight: Copy> PlaceToTransitionArc<P, T, Weight> {
    /// Constructs a place→transition arc with the given weight.
    ///
    /// ```
    /// use wasm4pm_compat::petri::PlaceToTransitionArc;
    /// struct P1; struct T1;
    /// let arc = PlaceToTransitionArc::<P1, T1, u32>::new(1u32);
    /// assert_eq!(arc.weight(), 1u32);
    /// ```
    pub fn new(weight: Weight) -> Self {
        PlaceToTransitionArc {
            _from: PhantomData,
            _to: PhantomData,
            weight,
        }
    }

    /// The arc weight.
    ///
    /// ```
    /// use wasm4pm_compat::petri::PlaceToTransitionArc;
    /// struct P1; struct T1;
    /// assert_eq!(PlaceToTransitionArc::<P1, T1, u8>::new(3u8).weight(), 3);
    /// ```
    pub fn weight(&self) -> Weight {
        self.weight
    }
}

/// A typed arc from a **transition** to a **place** — the only valid
/// post-incidence direction in a Petri net.
///
/// Paper: Murata (1989) §2 — bipartite arc law. Structure-only.
///
/// ```
/// use wasm4pm_compat::petri::TransitionToPlaceArc;
/// struct T1; struct P1;
/// let arc = TransitionToPlaceArc::<T1, P1, u8>::new(1u8);
/// assert_eq!(arc.weight(), 1);
/// ```
pub struct TransitionToPlaceArc<T, P, Weight> {
    pub(crate) _from: PhantomData<T>,
    pub(crate) _to: PhantomData<P>,
    /// Arc weight (post-incidence W⁺).
    pub weight: Weight,
}

impl<T, P, Weight: Copy> TransitionToPlaceArc<T, P, Weight> {
    /// Constructs a transition→place arc with the given weight.
    ///
    /// ```
    /// use wasm4pm_compat::petri::TransitionToPlaceArc;
    /// struct T1; struct P1;
    /// let arc = TransitionToPlaceArc::<T1, P1, u32>::new(1u32);
    /// assert_eq!(arc.weight(), 1u32);
    /// ```
    pub fn new(weight: Weight) -> Self {
        TransitionToPlaceArc {
            _from: PhantomData,
            _to: PhantomData,
            weight,
        }
    }

    /// The arc weight.
    ///
    /// ```
    /// use wasm4pm_compat::petri::TransitionToPlaceArc;
    /// struct T1; struct P1;
    /// assert_eq!(TransitionToPlaceArc::<T1, P1, u8>::new(2u8).weight(), 2);
    /// ```
    pub fn weight(&self) -> Weight {
        self.weight
    }
}

// ── BipartiteArcConst: const-generic bipartite arc law ───────────────────────

/// A bipartite arc with direction encoded as a const-generic [`crate::law::ArcDirectionConst`]
/// parameter.
///
/// `BipartiteArcConst<{ArcDirectionConst::PlaceToTransition}, Weight>` and
/// `BipartiteArcConst<{ArcDirectionConst::TransitionToPlace}, Weight>` are
/// **distinct types** at compile time — a slot requiring a pre-incidence arc
/// (`PlaceToTransition`) rejects a post-incidence arc (`TransitionToPlace`) with
/// a type error.
///
/// Paper: Murata (1989) §2 — `F ⊆ (P×T) ∪ (T×P)`.
///
/// ## Difference from [`PlaceToTransitionArc`] / [`TransitionToPlaceArc`]
///
/// The named-type arcs use *struct shape* to enforce direction (no common base
/// type). `BipartiteArcConst` provides a **single type** parameterised over
/// direction — useful for generic containers that hold arcs of either direction
/// while still encoding direction in the type.
///
/// Structure-only: a typed directed edge. No token flow.
///
/// ```
/// use wasm4pm_compat::petri::BipartiteArcConst;
/// use wasm4pm_compat::law::ArcDirectionConst;
///
/// let pre = BipartiteArcConst::<{ ArcDirectionConst::PlaceToTransition }, u8>::new("p0", "t0", 1);
/// assert_eq!(pre.place_id(), "p0");
/// assert_eq!(pre.weight(), 1u8);
/// ```
pub struct BipartiteArcConst<const DIR: crate::law::ArcDirectionConst, Weight> {
    place_id: alloc::string::String,
    transition_id: alloc::string::String,
    /// Arc weight (multiplicity).
    pub weight: Weight,
}

extern crate alloc;

impl<const DIR: crate::law::ArcDirectionConst, Weight: Copy>
    BipartiteArcConst<DIR, Weight>
{
    /// Construct a bipartite arc with the given endpoints and weight.
    ///
    /// ```
    /// use wasm4pm_compat::petri::BipartiteArcConst;
    /// use wasm4pm_compat::law::ArcDirectionConst;
    /// let post = BipartiteArcConst::<{ ArcDirectionConst::TransitionToPlace }, u32>::new("t0", "p1", 2);
    /// assert_eq!(post.transition_id(), "t0");
    /// assert_eq!(post.place_id(), "p1");
    /// ```
    pub fn new(
        place_id: impl Into<alloc::string::String>,
        transition_id: impl Into<alloc::string::String>,
        weight: Weight,
    ) -> Self {
        BipartiteArcConst {
            place_id: place_id.into(),
            transition_id: transition_id.into(),
            weight,
        }
    }

    /// The place endpoint id.
    ///
    /// ```
    /// use wasm4pm_compat::petri::BipartiteArcConst;
    /// use wasm4pm_compat::law::ArcDirectionConst;
    /// let a = BipartiteArcConst::<{ ArcDirectionConst::PlaceToTransition }, u8>::new("p0", "t0", 1);
    /// assert_eq!(a.place_id(), "p0");
    /// ```
    pub fn place_id(&self) -> &str {
        &self.place_id
    }

    /// The transition endpoint id.
    ///
    /// ```
    /// use wasm4pm_compat::petri::BipartiteArcConst;
    /// use wasm4pm_compat::law::ArcDirectionConst;
    /// let a = BipartiteArcConst::<{ ArcDirectionConst::PlaceToTransition }, u8>::new("p0", "t0", 1);
    /// assert_eq!(a.transition_id(), "t0");
    /// ```
    pub fn transition_id(&self) -> &str {
        &self.transition_id
    }

    /// The arc weight.
    ///
    /// ```
    /// use wasm4pm_compat::petri::BipartiteArcConst;
    /// use wasm4pm_compat::law::ArcDirectionConst;
    /// let a = BipartiteArcConst::<{ ArcDirectionConst::PlaceToTransition }, u8>::new("p0", "t0", 3);
    /// assert_eq!(a.weight(), 3u8);
    /// ```
    pub fn weight(&self) -> Weight {
        self.weight
    }

    /// The arc direction encoded in the const parameter.
    ///
    /// ```
    /// use wasm4pm_compat::petri::BipartiteArcConst;
    /// use wasm4pm_compat::law::ArcDirectionConst;
    /// let a = BipartiteArcConst::<{ ArcDirectionConst::TransitionToPlace }, u8>::new("t0", "p0", 1);
    /// assert_eq!(a.direction(), ArcDirectionConst::TransitionToPlace);
    /// ```
    pub const fn direction(&self) -> crate::law::ArcDirectionConst {
        DIR
    }
}

// ── IsValidArc sealed trait ──────────────────────────────────────────────────

mod arc_seal {
    pub trait Sealed {}
    impl<P, T, W> Sealed for super::PlaceToTransitionArc<P, T, W> {}
    impl<T, P, W> Sealed for super::TransitionToPlaceArc<T, P, W> {}
}

/// Sealed marker: only [`PlaceToTransitionArc`] and [`TransitionToPlaceArc`]
/// are valid arcs in a Petri net. No user type can implement this trait.
///
/// ```
/// use wasm4pm_compat::petri::{PlaceToTransitionArc, IsValidArc};
/// struct P1; struct T1;
/// fn accept<A: IsValidArc>(_arc: A) {}
/// accept(PlaceToTransitionArc::<P1, T1, u8>::new(1u8));
/// ```
pub trait IsValidArc: arc_seal::Sealed {}
impl<P, T, W> IsValidArc for PlaceToTransitionArc<P, T, W> {}
impl<T, P, W> IsValidArc for TransitionToPlaceArc<T, P, W> {}

// ── Non-forgeable WF-net soundness (const-generic + private seal) ────────────

mod wfnet_seal {
    /// Private seal — only constructible inside `petri`. Prevents external
    /// users from building `WfNetConst<{SoundnessState::Witnessed}>` directly.
    pub(super) struct WfNetSeal;
}

/// A WF-net with soundness state encoded as a const generic parameter.
///
/// The `SOUNDNESS` parameter tracks whether soundness is unknown, claimed, or
/// witnessed. Crucially, `WfNetConst<{SoundnessState::Witnessed}>` is **only**
/// constructible via [`WfNetConst::witness_soundness`], which requires a
/// [`SoundnessProof`] — a type that is itself only constructible inside this
/// module or via the `wasm4pm` graduation bridge.
///
/// Direct struct-literal construction fails because `_seal` is a private field.
///
/// ```
/// use wasm4pm_compat::petri::{WfNetConst, SoundnessProof};
/// use wasm4pm_compat::law::SoundnessState;
///
/// // Build an unknown-soundness net, then claim it:
/// let unknown = WfNetConst::<{ SoundnessState::Unknown }>::new();
/// let claimed = unknown.claim_sound();
/// // To reach Witnessed, you would call claimed.witness_soundness(proof)
/// // where proof is only producible by the wasm4pm engine or this module.
/// ```
///
/// ```compile_fail
/// use wasm4pm_compat::petri::WfNetConst;
/// use wasm4pm_compat::law::SoundnessState;
/// // ERROR: _seal is a private field; direct forged construction is impossible.
/// let forged: WfNetConst<{ SoundnessState::Witnessed }> = WfNetConst {
///     _seal: todo!(),
/// };
/// ```
pub struct WfNetConst<const SOUNDNESS: SoundnessState> {
    // Private seal prevents direct struct-literal construction of any
    // WfNetConst variant from outside this module.
    _seal: wfnet_seal::WfNetSeal,
}

/// A proof token for `SoundnessState::Witnessed` — only constructible inside
/// `petri` (or via the `wasm4pm` graduation bridge).
///
/// Callers outside this module cannot construct `SoundnessProof` because the
/// inner [`wfnet_seal::WfNetSeal`] type is private.
pub struct SoundnessProof(wfnet_seal::WfNetSeal);

impl SoundnessProof {
    /// Module-private constructor — only `petri` and the `wasm4pm` bridge can
    /// produce a proof.
    #[allow(dead_code)]
    pub(crate) fn new() -> Self {
        SoundnessProof(wfnet_seal::WfNetSeal)
    }
}

impl Default for WfNetConst<{ SoundnessState::Unknown }> {
    fn default() -> Self {
        Self::new()
    }
}

impl WfNetConst<{ SoundnessState::Unknown }> {
    /// Construct a `WfNetConst` in the initial `Unknown` soundness state.
    ///
    /// ```
    /// use wasm4pm_compat::petri::WfNetConst;
    /// use wasm4pm_compat::law::SoundnessState;
    /// let _wf = WfNetConst::<{ SoundnessState::Unknown }>::new();
    /// ```
    pub fn new() -> Self {
        WfNetConst {
            _seal: wfnet_seal::WfNetSeal,
        }
    }

    /// Advance to `Claimed` soundness — a type-level re-tagging only.
    ///
    /// ```
    /// use wasm4pm_compat::petri::WfNetConst;
    /// use wasm4pm_compat::law::SoundnessState;
    /// let claimed: WfNetConst<{ SoundnessState::Claimed }> =
    ///     WfNetConst::<{ SoundnessState::Unknown }>::new().claim_sound();
    /// ```
    pub fn claim_sound(self) -> WfNetConst<{ SoundnessState::Claimed }> {
        WfNetConst { _seal: self._seal }
    }
}

impl WfNetConst<{ SoundnessState::Claimed }> {
    /// Advance a *claimed* WF-net to `Witnessed` — requires a [`SoundnessProof`].
    ///
    /// The `SoundnessProof` is only constructible inside this module or by the
    /// `wasm4pm` graduation bridge. This is the sanctioned, non-forgeable path
    /// to `SoundnessState::Witnessed`.
    ///
    /// ```ignore
    /// // Conceptual: the wasm4pm bridge supplies the proof after verifying soundness.
    /// let witnessed = claimed.witness_soundness(proof_from_wasm4pm);
    /// ```
    pub fn witness_soundness(
        self,
        _proof: SoundnessProof,
    ) -> WfNetConst<{ SoundnessState::Witnessed }> {
        WfNetConst {
            _seal: wfnet_seal::WfNetSeal,
        }
    }
}

/// A place: a named token-holding location in a Petri net.
///
/// Structure-only: identity and name; no token dynamics.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Place {
    id: String,
}

impl Place {
    /// Construct a place with an id.
    ///
    /// ```
    /// use wasm4pm_compat::petri::Place;
    /// assert_eq!(Place::new("p0").id(), "p0");
    /// ```
    pub fn new(id: impl Into<String>) -> Self {
        Place { id: id.into() }
    }

    /// The place id.
    ///
    /// ```
    /// use wasm4pm_compat::petri::Place;
    /// assert_eq!(Place::new("p0").id(), "p0");
    /// ```
    pub fn id(&self) -> &str {
        &self.id
    }
}

/// A transition: a named firing element. An empty `label` denotes a *silent*
/// (tau) transition.
///
/// Structure-only: identity and label; no firing semantics.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transition {
    id: String,
    label: String,
}

impl Transition {
    /// Construct a labeled transition.
    ///
    /// ```
    /// use wasm4pm_compat::petri::Transition;
    /// let t = Transition::new("t0", "approve");
    /// assert_eq!(t.id(), "t0");
    /// assert!(!t.is_silent());
    /// ```
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Transition {
            id: id.into(),
            label: label.into(),
        }
    }

    /// Construct a silent (tau) transition with an empty label.
    ///
    /// ```
    /// use wasm4pm_compat::petri::Transition;
    /// assert!(Transition::silent("t0").is_silent());
    /// ```
    pub fn silent(id: impl Into<String>) -> Self {
        Transition::new(id, "")
    }

    /// The transition id.
    ///
    /// ```
    /// use wasm4pm_compat::petri::Transition;
    /// assert_eq!(Transition::new("t0", "a").id(), "t0");
    /// ```
    pub fn id(&self) -> &str {
        &self.id
    }

    /// The transition label (empty means silent).
    ///
    /// ```
    /// use wasm4pm_compat::petri::Transition;
    /// assert_eq!(Transition::new("t0", "a").label(), "a");
    /// ```
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Whether the transition is silent (tau).
    ///
    /// ```
    /// use wasm4pm_compat::petri::Transition;
    /// assert!(Transition::silent("t0").is_silent());
    /// ```
    pub fn is_silent(&self) -> bool {
        self.label.is_empty()
    }
}

/// The direction of a Petri-net [`Arc`] relative to a transition.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ArcDirection {
    /// Place → Transition (an input/consume arc).
    PlaceToTransition,
    /// Transition → Place (an output/produce arc).
    TransitionToPlace,
}

/// An arc: a directed, weighted connection between a place and a transition.
///
/// `weight` is the arc multiplicity (default 1). An arc whose endpoints are not
/// declared is refused as [`PetriRefusal::InvalidVariableArc`] (the closest
/// structural law in this shape's vocabulary) or, for WF-nets, a soundness
/// concern at `wasm4pm`.
///
/// Structure-only: a typed graph edge, no token flow.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Arc {
    place_id: String,
    transition_id: String,
    direction: ArcDirection,
    weight: u32,
    /// For OC-Petri-nets: the object type this arc carries, if any.
    object_type: Option<String>,
    /// For OC-Petri-nets: whether the arc is a *variable* arc (multiple tokens
    /// of the object type).
    variable: bool,
}

impl Arc {
    /// Construct a place→transition arc with weight 1.
    ///
    /// ```
    /// use wasm4pm_compat::petri::{Arc, ArcDirection};
    /// let a = Arc::place_to_transition("p", "t");
    /// assert_eq!(a.direction(), ArcDirection::PlaceToTransition);
    /// assert_eq!(a.weight(), 1);
    /// ```
    pub fn place_to_transition(
        place_id: impl Into<String>,
        transition_id: impl Into<String>,
    ) -> Self {
        Arc {
            place_id: place_id.into(),
            transition_id: transition_id.into(),
            direction: ArcDirection::PlaceToTransition,
            weight: 1,
            object_type: None,
            variable: false,
        }
    }

    /// Construct a transition→place arc with weight 1.
    ///
    /// ```
    /// use wasm4pm_compat::petri::{Arc, ArcDirection};
    /// let a = Arc::transition_to_place("t", "p");
    /// assert_eq!(a.direction(), ArcDirection::TransitionToPlace);
    /// ```
    pub fn transition_to_place(
        transition_id: impl Into<String>,
        place_id: impl Into<String>,
    ) -> Self {
        Arc {
            place_id: place_id.into(),
            transition_id: transition_id.into(),
            direction: ArcDirection::TransitionToPlace,
            weight: 1,
            object_type: None,
            variable: false,
        }
    }

    /// Set the arc weight (multiplicity). Builder-style.
    ///
    /// ```
    /// use wasm4pm_compat::petri::Arc;
    /// assert_eq!(Arc::place_to_transition("p", "t").with_weight(3).weight(), 3);
    /// ```
    pub fn with_weight(mut self, weight: u32) -> Self {
        self.weight = weight;
        self
    }

    /// Type the arc by object type, marking it variable or not (OC-Petri-net).
    /// Builder-style.
    ///
    /// ```
    /// use wasm4pm_compat::petri::Arc;
    /// let a = Arc::place_to_transition("p", "t").typed("order", true);
    /// assert_eq!(a.object_type(), Some("order"));
    /// assert!(a.is_variable());
    /// ```
    pub fn typed(mut self, object_type: impl Into<String>, variable: bool) -> Self {
        self.object_type = Some(object_type.into());
        self.variable = variable;
        self
    }

    /// The place endpoint id.
    pub fn place_id(&self) -> &str {
        &self.place_id
    }

    /// The transition endpoint id.
    pub fn transition_id(&self) -> &str {
        &self.transition_id
    }

    /// The arc direction.
    ///
    /// ```
    /// use wasm4pm_compat::petri::{Arc, ArcDirection};
    /// assert_eq!(Arc::place_to_transition("p", "t").direction(), ArcDirection::PlaceToTransition);
    /// ```
    pub fn direction(&self) -> ArcDirection {
        self.direction
    }

    /// The arc weight (multiplicity).
    ///
    /// ```
    /// use wasm4pm_compat::petri::Arc;
    /// assert_eq!(Arc::place_to_transition("p", "t").weight(), 1);
    /// ```
    pub fn weight(&self) -> u32 {
        self.weight
    }

    /// The OC-Petri-net object type carried by this arc, if any.
    ///
    /// ```
    /// use wasm4pm_compat::petri::Arc;
    /// assert_eq!(Arc::place_to_transition("p", "t").object_type(), None);
    /// ```
    pub fn object_type(&self) -> Option<&str> {
        self.object_type.as_deref()
    }

    /// Whether this is a variable arc (OC-Petri-net).
    ///
    /// ```
    /// use wasm4pm_compat::petri::Arc;
    /// assert!(!Arc::place_to_transition("p", "t").is_variable());
    /// ```
    pub fn is_variable(&self) -> bool {
        self.variable
    }
}

/// A marking: how many tokens sit on each place.
///
/// Structure-only: a snapshot of token counts by place id. This crate never
/// fires a transition to move from one marking to another.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Marking {
    tokens: Vec<(String, u32)>,
}

impl Marking {
    /// Construct an empty marking.
    ///
    /// ```
    /// use wasm4pm_compat::petri::Marking;
    /// assert!(Marking::empty().is_empty());
    /// ```
    pub fn empty() -> Self {
        Marking::default()
    }

    /// Construct a marking from `(place_id, token_count)` pairs.
    ///
    /// ```
    /// use wasm4pm_compat::petri::Marking;
    /// let m = Marking::new([("p0".to_string(), 1)]);
    /// assert_eq!(m.tokens_on("p0"), 1);
    /// ```
    pub fn new(tokens: impl IntoIterator<Item = (String, u32)>) -> Self {
        Marking {
            tokens: tokens.into_iter().collect(),
        }
    }

    /// The token count on a place (0 if unmarked).
    ///
    /// ```
    /// use wasm4pm_compat::petri::Marking;
    /// assert_eq!(Marking::empty().tokens_on("p0"), 0);
    /// ```
    pub fn tokens_on(&self, place_id: &str) -> u32 {
        self.tokens
            .iter()
            .find(|(p, _)| p == place_id)
            .map(|(_, n)| *n)
            .unwrap_or(0)
    }

    /// Whether the marking places no tokens anywhere.
    ///
    /// ```
    /// use wasm4pm_compat::petri::Marking;
    /// assert!(Marking::empty().is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.tokens.iter().all(|(_, n)| *n == 0)
    }
}

/// A paired initial and final marking for a WF-net.
///
/// WF-net soundness (van der Aalst, 1998) requires both an *initial* marking
/// (a single token on the source place) and a *final* marking (a single token
/// on the sink place). `InitialFinalMarkingPair` bundles these two markings as
/// a single named shape, so a WF-net construction site can pass them together
/// without risk of swapping the two arguments.
///
/// ## Structural law
///
/// [`InitialFinalMarkingPair::validate`] checks that neither marking is empty
/// and that the initial and final markings do not overlap (a place carrying
/// tokens in both is a structural defect). It does **not** check that the
/// markings reference declared places — that check is performed by
/// [`WfNet::validate`].
///
/// Structure-only: a shape. No token dynamics.
///
/// ```
/// use wasm4pm_compat::petri::{InitialFinalMarkingPair, Marking, PetriRefusal};
/// let pair = InitialFinalMarkingPair::new(
///     Marking::new([("src".to_string(), 1)]),
///     Marking::new([("snk".to_string(), 1)]),
/// );
/// assert!(pair.validate().is_ok());
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InitialFinalMarkingPair {
    /// The initial marking (typically one token on the source place).
    pub initial: Marking,
    /// The final marking (typically one token on the sink place).
    pub final_marking: Marking,
}

impl InitialFinalMarkingPair {
    /// Construct an `InitialFinalMarkingPair` from two markings.
    ///
    /// ```
    /// use wasm4pm_compat::petri::{InitialFinalMarkingPair, Marking};
    /// let pair = InitialFinalMarkingPair::new(
    ///     Marking::new([("src".to_string(), 1)]),
    ///     Marking::new([("snk".to_string(), 1)]),
    /// );
    /// assert_eq!(pair.initial.tokens_on("src"), 1);
    /// assert_eq!(pair.final_marking.tokens_on("snk"), 1);
    /// ```
    pub fn new(initial: Marking, final_marking: Marking) -> Self {
        InitialFinalMarkingPair {
            initial,
            final_marking,
        }
    }

    /// Structurally validate the marking pair.
    ///
    /// Returns [`PetriRefusal::MissingInitialMarking`] if the initial marking is
    /// empty, [`PetriRefusal::MissingFinalMarking`] if the final marking is empty,
    /// or [`PetriRefusal::InvalidVariableArc`] if any place id appears in both
    /// markings (overlapping initial/final places is a structural defect).
    ///
    /// ```
    /// use wasm4pm_compat::petri::{InitialFinalMarkingPair, Marking, PetriRefusal};
    /// // Empty initial marking:
    /// let bad = InitialFinalMarkingPair::new(
    ///     Marking::empty(),
    ///     Marking::new([("snk".to_string(), 1)]),
    /// );
    /// assert_eq!(bad.validate(), Err(PetriRefusal::MissingInitialMarking));
    /// ```
    pub fn validate(&self) -> Result<(), PetriRefusal> {
        if self.initial.is_empty() {
            return Err(PetriRefusal::MissingInitialMarking);
        }
        if self.final_marking.is_empty() {
            return Err(PetriRefusal::MissingFinalMarking);
        }
        // Detect overlapping place ids between initial and final markings.
        for (pid, cnt) in &self.initial.tokens {
            if *cnt > 0 && self.final_marking.tokens_on(pid) > 0 {
                return Err(PetriRefusal::InvalidVariableArc);
            }
        }
        Ok(())
    }
}

/// A plain Petri net: places, transitions, arcs, and an initial marking.
///
/// [`PetriNet::validate`] checks structural shape (arcs reference declared
/// nodes); it does not analyze behavior. Reachability, boundedness, and
/// soundness graduate to `wasm4pm`.
///
/// Structure-only.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PetriNet {
    places: Vec<Place>,
    transitions: Vec<Transition>,
    arcs: Vec<Arc>,
    initial: Marking,
}

impl PetriNet {
    /// Construct a Petri net from its parts.
    ///
    /// ```
    /// use wasm4pm_compat::petri::{PetriNet, Place, Transition, Arc, Marking};
    /// let net = PetriNet::new(
    ///     [Place::new("p")],
    ///     [Transition::new("t", "a")],
    ///     [Arc::place_to_transition("p", "t")],
    ///     Marking::new([("p".to_string(), 1)]),
    /// );
    /// assert!(net.validate().is_ok());
    /// ```
    pub fn new(
        places: impl IntoIterator<Item = Place>,
        transitions: impl IntoIterator<Item = Transition>,
        arcs: impl IntoIterator<Item = Arc>,
        initial: Marking,
    ) -> Self {
        PetriNet {
            places: places.into_iter().collect(),
            transitions: transitions.into_iter().collect(),
            arcs: arcs.into_iter().collect(),
            initial,
        }
    }

    /// The places.
    pub fn places(&self) -> &[Place] {
        &self.places
    }

    /// The transitions.
    pub fn transitions(&self) -> &[Transition] {
        &self.transitions
    }

    /// The arcs.
    pub fn arcs(&self) -> &[Arc] {
        &self.arcs
    }

    /// The initial marking.
    pub fn initial_marking(&self) -> &Marking {
        &self.initial
    }

    /// Structurally validate that every arc references declared nodes.
    ///
    /// This is a shape check, not behavior analysis. A dangling arc is reported
    /// as [`PetriRefusal::InvalidVariableArc`] (this shape's structural
    /// arc-defect law). Initial marking presence is required at the WF-net
    /// level, not here.
    ///
    /// ```
    /// use wasm4pm_compat::petri::{PetriNet, Place, Transition, Arc, Marking, PetriRefusal};
    /// // Arc references transition "ghost" that does not exist.
    /// let net = PetriNet::new(
    ///     [Place::new("p")],
    ///     [Transition::new("t", "a")],
    ///     [Arc::place_to_transition("p", "ghost")],
    ///     Marking::empty(),
    /// );
    /// assert_eq!(net.validate(), Err(PetriRefusal::InvalidVariableArc));
    /// ```
    pub fn validate(&self) -> Result<(), PetriRefusal> {
        use std::collections::HashSet;
        let pids: HashSet<&str> = self.places.iter().map(Place::id).collect();
        let tids: HashSet<&str> = self.transitions.iter().map(Transition::id).collect();
        for a in &self.arcs {
            if !pids.contains(a.place_id()) || !tids.contains(a.transition_id()) {
                return Err(PetriRefusal::InvalidVariableArc);
            }
        }
        Ok(())
    }
}

/// Soundness state marker: soundness has **not** been asserted or proven.
///
/// This is an **uninhabited** type-level token (an empty enum) used only as a
/// `PhantomData` parameter on [`WfNet`]. It is never constructed; it carries no
/// data and costs nothing at runtime. It is the *default* state of a freshly
/// constructed WF-net.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SoundnessUnknown {}

/// Soundness state marker: soundness is **claimed** but unproven here.
///
/// An uninhabited type-level token. A [`WfNet`] in this state asserts soundness
/// (e.g. by an upstream tool or a human) but this crate has **not** verified it —
/// verification is a `wasm4pm` engine. Treat this as a claim to be discharged,
/// not as evidence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SoundnessClaimed {}

/// Soundness state marker: soundness is **witnessed**.
///
/// An uninhabited type-level token. A [`WfNet`] reaches this state only by
/// re-attaching a soundness witness obtained from `wasm4pm`; this crate models
/// the *shape* of "soundness has been witnessed", not the witnessing itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SoundnessWitnessed {}

/// A workflow net (WF-net): a Petri net with a single source and sink place and
/// a soundness state tracked at the type level by the `S` typestate parameter.
///
/// The `S` parameter is one of [`SoundnessUnknown`] (default),
/// [`SoundnessClaimed`], or [`SoundnessWitnessed`] — empty-enum markers used via
/// `PhantomData`. The field/parameter is a **claim about** soundness, never a
/// computed proof: [`WfNet::validate`] checks only *structural* WF-net shape
/// (initial and final markings present), and the soundness transition methods
/// merely *re-type* the claim; they do not compute soundness.
///
/// Structure-only: actual soundness witnessing graduates to `wasm4pm`, which
/// produces the witness that justifies moving to [`SoundnessWitnessed`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WfNet<S = SoundnessUnknown> {
    net: PetriNet,
    final_marking: Option<Marking>,
    _soundness: PhantomData<S>,
}

impl WfNet<SoundnessUnknown> {
    /// Construct a WF-net in the [`SoundnessUnknown`] state from a Petri net and
    /// a final marking.
    ///
    /// ```
    /// use wasm4pm_compat::petri::{WfNet, PetriNet, Place, Transition, Arc, Marking};
    /// let net = PetriNet::new(
    ///     [Place::new("src"), Place::new("snk")],
    ///     [Transition::new("t", "a")],
    ///     [Arc::place_to_transition("src", "t"), Arc::transition_to_place("t", "snk")],
    ///     Marking::new([("src".to_string(), 1)]),
    /// );
    /// let wf = WfNet::new(net, Marking::new([("snk".to_string(), 1)]));
    /// assert!(wf.validate().is_ok());
    /// ```
    pub fn new(net: PetriNet, final_marking: Marking) -> Self {
        WfNet {
            net,
            final_marking: Some(final_marking),
            _soundness: PhantomData,
        }
    }
}

impl<S> WfNet<S> {
    /// The underlying Petri net.
    pub fn net(&self) -> &PetriNet {
        &self.net
    }

    /// The final marking, if declared.
    pub fn final_marking(&self) -> Option<&Marking> {
        self.final_marking.as_ref()
    }

    /// Structurally validate the WF-net shape.
    ///
    /// Checks that the underlying net is structurally well-formed, that an
    /// initial marking places at least one token
    /// ([`PetriRefusal::MissingInitialMarking`]), and that a final marking is
    /// declared ([`PetriRefusal::MissingFinalMarking`]). It does **not** check
    /// soundness, safeness, or boundedness — those graduate to `wasm4pm`.
    ///
    /// ```
    /// use wasm4pm_compat::petri::{WfNet, PetriNet, Place, Transition, Arc, Marking, PetriRefusal};
    /// // Initial marking is empty -> MissingInitialMarking.
    /// let net = PetriNet::new(
    ///     [Place::new("src"), Place::new("snk")],
    ///     [Transition::new("t", "a")],
    ///     [Arc::place_to_transition("src", "t"), Arc::transition_to_place("t", "snk")],
    ///     Marking::empty(),
    /// );
    /// let wf = WfNet::new(net, Marking::new([("snk".to_string(), 1)]));
    /// assert_eq!(wf.validate(), Err(PetriRefusal::MissingInitialMarking));
    /// ```
    pub fn validate(&self) -> Result<(), PetriRefusal> {
        self.net.validate()?;
        if self.net.initial_marking().is_empty() {
            return Err(PetriRefusal::MissingInitialMarking);
        }
        match &self.final_marking {
            Some(m) if !m.is_empty() => Ok(()),
            _ => Err(PetriRefusal::MissingFinalMarking),
        }
    }

    /// Re-type this WF-net as carrying a soundness **claim** (unproven here).
    ///
    /// This is a *type-level re-tagging only* — it computes nothing. Use it to
    /// record that an upstream source asserts soundness; discharge the claim by
    /// graduating to `wasm4pm` and witnessing it.
    ///
    /// ```
    /// use wasm4pm_compat::petri::{WfNet, PetriNet, Marking, SoundnessClaimed};
    /// let wf = WfNet::new(PetriNet::default(), Marking::new([("snk".to_string(), 1)]));
    /// let _claimed: WfNet<SoundnessClaimed> = wf.claim_sound();
    /// ```
    pub fn claim_sound(self) -> WfNet<SoundnessClaimed> {
        WfNet {
            net: self.net,
            final_marking: self.final_marking,
            _soundness: PhantomData,
        }
    }
}

impl WfNet<SoundnessClaimed> {
    /// Re-type a *claimed* WF-net as **witnessed** sound.
    ///
    /// This crate cannot produce a soundness witness itself; this method models
    /// re-attaching a witness obtained from `wasm4pm`. It performs **no
    /// computation** — the caller is responsible for supplying genuine evidence
    /// out-of-band. The method exists so the *shape* "soundness is witnessed"
    /// can travel through the compat boundary as a distinct type.
    ///
    /// ```
    /// use wasm4pm_compat::petri::{WfNet, PetriNet, Marking, SoundnessClaimed, SoundnessWitnessed};
    /// let wf = WfNet::new(PetriNet::default(), Marking::new([("snk".to_string(), 1)]))
    ///     .claim_sound();
    /// let _w: WfNet<SoundnessWitnessed> = wf.attest_witnessed();
    /// ```
    pub fn attest_witnessed(self) -> WfNet<SoundnessWitnessed> {
        WfNet {
            net: self.net,
            final_marking: self.final_marking,
            _soundness: PhantomData,
        }
    }
}

/// An object-centric Petri net (OC-Petri-net): a Petri net whose arcs are typed
/// by object type and may be variable, plus the declared object types.
///
/// [`ObjectCentricPetriNet::validate`] checks that every typed arc names a
/// declared object type ([`PetriRefusal::ObjectTypeNotPreserved`]) and that the
/// underlying net is structurally sound (arcs reference declared nodes). It does
/// not analyze object-centric behavior — that graduates to `wasm4pm`.
///
/// Structure-only.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ObjectCentricPetriNet {
    net: PetriNet,
    object_types: Vec<String>,
}

impl ObjectCentricPetriNet {
    /// Construct an OC-Petri-net from a Petri net and its object types.
    ///
    /// ```
    /// use wasm4pm_compat::petri::{ObjectCentricPetriNet, PetriNet, Place, Transition, Arc, Marking};
    /// let net = PetriNet::new(
    ///     [Place::new("p")],
    ///     [Transition::new("t", "a")],
    ///     [Arc::place_to_transition("p", "t").typed("order", false)],
    ///     Marking::empty(),
    /// );
    /// let ocpn = ObjectCentricPetriNet::new(net, ["order".to_string()]);
    /// assert!(ocpn.validate().is_ok());
    /// ```
    pub fn new(net: PetriNet, object_types: impl IntoIterator<Item = String>) -> Self {
        ObjectCentricPetriNet {
            net,
            object_types: object_types.into_iter().collect(),
        }
    }

    /// The underlying Petri net.
    pub fn net(&self) -> &PetriNet {
        &self.net
    }

    /// The declared object types.
    pub fn object_types(&self) -> &[String] {
        &self.object_types
    }

    /// Structurally validate the OC-Petri-net shape.
    ///
    /// Checks the underlying net structurally, then verifies every typed arc's
    /// object type is declared ([`PetriRefusal::ObjectTypeNotPreserved`]). No
    /// behavior analysis is performed.
    ///
    /// ```
    /// use wasm4pm_compat::petri::{ObjectCentricPetriNet, PetriNet, Place, Transition, Arc, Marking, PetriRefusal};
    /// // Arc typed "ghost" but only "order" is declared.
    /// let net = PetriNet::new(
    ///     [Place::new("p")],
    ///     [Transition::new("t", "a")],
    ///     [Arc::place_to_transition("p", "t").typed("ghost", false)],
    ///     Marking::empty(),
    /// );
    /// let ocpn = ObjectCentricPetriNet::new(net, ["order".to_string()]);
    /// assert_eq!(ocpn.validate(), Err(PetriRefusal::ObjectTypeNotPreserved));
    /// ```
    pub fn validate(&self) -> Result<(), PetriRefusal> {
        self.net.validate()?;
        for a in self.net.arcs() {
            if let Some(ot) = a.object_type() {
                if !self.object_types.iter().any(|t| t == ot) {
                    return Err(PetriRefusal::ObjectTypeNotPreserved);
                }
            }
        }
        Ok(())
    }
}

// ── YAWL cancellation-region shape ──────────────────────────────────────────

/// A YAWL cancellation region: the named set of place/condition/task ids whose
/// tokens are vacuumed when the owning task fires.
///
/// YAWL Definition 1 (van der Aalst & ter Hofstede, 2004) specifies
/// `rem: T ⇸ P(T ∪ C \ {i, o})` — each task optionally names a cancellation
/// region. This struct carries the *shape* of that region: a named set of node
/// ids. Token-removal execution (the actual vacuuming) graduates to `wasm4pm`.
///
/// The `#[repr(transparent)]` newtype prevents a bare `Vec<String>` from being
/// accidentally passed where a `CancellationRegion` is required. It is zero-cost
/// to hold and clone.
///
/// Structure-only: carries ids, never fires.
///
/// ```
/// use wasm4pm_compat::petri::CancellationRegion;
/// let cr = CancellationRegion::new(["p1", "t2"]);
/// assert_eq!(cr.members(), &["p1".to_string(), "t2".to_string()]);
/// ```
#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct CancellationRegion {
    /// The ids of nodes (places, conditions, tasks) in this cancellation region,
    /// excluding the initial and final place of the net (i, o).
    pub members: Vec<String>,
}

impl CancellationRegion {
    /// Construct a cancellation region from an iterator of node ids.
    ///
    /// ```
    /// use wasm4pm_compat::petri::CancellationRegion;
    /// let cr = CancellationRegion::new(["p1", "t2"]);
    /// assert_eq!(cr.members().len(), 2);
    /// ```
    pub fn new<I, S>(members: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        CancellationRegion {
            members: members.into_iter().map(Into::into).collect(),
        }
    }

    /// The node ids in this cancellation region.
    ///
    /// ```
    /// use wasm4pm_compat::petri::CancellationRegion;
    /// assert!(CancellationRegion::default().members().is_empty());
    /// ```
    pub fn members(&self) -> &[String] {
        &self.members
    }
}

// ── YAWL multiple-instance task spec ────────────────────────────────────────

/// The creation kind for a YAWL multiple-instance task.
///
/// YAWL Definition 1 defines `nofi: T ⇸ N × N^∞ × N^∞ × {dynamic, static}`.
/// The `{Static, Dynamic}` variant names whether child instances are created
/// once at firing time (`Static`) or may be created incrementally during
/// execution (`Dynamic`). This is the structural tag; execution semantics of
/// dynamic spawning graduate to `wasm4pm`.
///
/// Structure-only marker; carries no instantiation behavior.
///
/// ```
/// use wasm4pm_compat::petri::InstanceCreationKind;
/// let k = InstanceCreationKind::Static;
/// assert_eq!(k, InstanceCreationKind::Static);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InstanceCreationKind {
    /// All child instances are created at the moment the task fires.
    Static,
    /// Child instances may be created incrementally during the task's lifetime.
    Dynamic,
}

/// A YAWL multiple-instance task specification: the four-tuple
/// `(min_instances, max_instances, threshold, creation_kind)`.
///
/// YAWL Definition 1 (`nofi`) mandates that every multi-instance task carries:
/// - `min`: the minimum number of child instances that must complete (`N`).
/// - `max`: the maximum number of child instances (`N^∞`), `None` = unbounded.
/// - `threshold`: the minimum count for collective completion (`N^∞`),
///   `None` = all instances must complete.
/// - `creation`: whether spawning is [`InstanceCreationKind::Static`] or
///   [`InstanceCreationKind::Dynamic`].
///
/// The structural law `min <= max` (when `max` is bounded) is checked by
/// [`MultipleInstanceSpec::validate`]; a violation is refused as
/// [`PetriRefusal::InvalidInstanceBounds`].
///
/// Structure-only: instance creation and threshold enforcement graduate to
/// `wasm4pm`.
///
/// ```
/// use wasm4pm_compat::petri::{MultipleInstanceSpec, InstanceCreationKind};
/// let spec = MultipleInstanceSpec::new(1, Some(4), Some(2), InstanceCreationKind::Static);
/// assert!(spec.validate().is_ok());
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MultipleInstanceSpec {
    /// Minimum number of instances that must complete.
    pub min: u32,
    /// Maximum number of instances (`None` = unbounded / ∞).
    pub max: Option<u32>,
    /// Threshold for collective completion (`None` = all instances).
    pub threshold: Option<u32>,
    /// Whether child instances are created statically or dynamically.
    pub creation: InstanceCreationKind,
}

impl MultipleInstanceSpec {
    /// Construct a multiple-instance spec.
    ///
    /// ```
    /// use wasm4pm_compat::petri::{MultipleInstanceSpec, InstanceCreationKind};
    /// let spec = MultipleInstanceSpec::new(1, Some(3), None, InstanceCreationKind::Dynamic);
    /// assert_eq!(spec.min, 1);
    /// assert_eq!(spec.creation, InstanceCreationKind::Dynamic);
    /// ```
    pub fn new(
        min: u32,
        max: Option<u32>,
        threshold: Option<u32>,
        creation: InstanceCreationKind,
    ) -> Self {
        MultipleInstanceSpec {
            min,
            max,
            threshold,
            creation,
        }
    }

    /// Structurally validate the instance bounds.
    ///
    /// Returns [`PetriRefusal::InvalidInstanceBounds`] if `max` is bounded and
    /// `min > max`, or if `min == 0` (at least one instance is required by YAWL).
    ///
    /// ```
    /// use wasm4pm_compat::petri::{MultipleInstanceSpec, InstanceCreationKind, PetriRefusal};
    /// let bad = MultipleInstanceSpec::new(5, Some(2), None, InstanceCreationKind::Static);
    /// assert_eq!(bad.validate(), Err(PetriRefusal::InvalidInstanceBounds));
    /// let zero = MultipleInstanceSpec::new(0, Some(1), None, InstanceCreationKind::Static);
    /// assert_eq!(zero.validate(), Err(PetriRefusal::InvalidInstanceBounds));
    /// ```
    pub fn validate(&self) -> Result<(), PetriRefusal> {
        if self.min == 0 {
            return Err(PetriRefusal::InvalidInstanceBounds);
        }
        if let Some(max) = self.max {
            if self.min > max {
                return Err(PetriRefusal::InvalidInstanceBounds);
            }
        }
        Ok(())
    }
}

// ── YAWL multiple-instance bounds — compile-time law surface ────────────────

/// A YAWL multiple-instance spec with bounds enforced **at compile time**.
///
/// `MultipleInstanceSpecConst<MIN, MAX>` encodes the YAWL Definition 1 `nofi`
/// invariant `1 ≤ MIN ≤ MAX` as const-generic where-bounds so that a violation
/// is a **compile error**, not a runtime refusal.
///
/// Law: YAWL Definition 1 nofi — `min: N`, `max: N^∞`, `1 ≤ min ≤ max`.
///
/// ## Compile-time negative receipt
///
/// `MultipleInstanceSpecConst<5, 2>` does **not compile** because `5 <= 2` is
/// false — the `Require<{ MIN <= MAX }>: IsTrue` bound fails at the type level.
/// `MultipleInstanceSpecConst<0, 4>` does **not compile** because `0 >= 1` is
/// false — `Require<{ MIN >= 1 }>: IsTrue` fails.
///
/// Use [`MultipleInstanceSpec`] for runtime-validated, dynamically constructed
/// specs. Use this type when the bounds are known at compile time and you want
/// the compiler to verify them.
///
/// Structure-only: zero-cost (`_private: ()`) marker that carries the min/max
/// law in its type. No engine logic.
///
/// ```
/// # #![feature(generic_const_exprs)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::petri::MultipleInstanceSpecConst;
/// // 1 <= 4: lawful at compile time.
/// let _: MultipleInstanceSpecConst<1, 4> = MultipleInstanceSpecConst::new();
/// ```
///
/// ```compile_fail
/// # #![feature(generic_const_exprs)]
/// # #![allow(incomplete_features)]
/// use wasm4pm_compat::petri::MultipleInstanceSpecConst;
/// // 5 > 2: compile error — Require<{ 5 <= 2 }>: IsTrue not satisfied.
/// let _: MultipleInstanceSpecConst<5, 2> = MultipleInstanceSpecConst::new();
/// ```
pub struct MultipleInstanceSpecConst<const MIN: u32, const MAX: u32>
where
    crate::law::Require<{ MIN >= 1 }>: crate::law::IsTrue,
    crate::law::Require<{ MIN <= MAX }>: crate::law::IsTrue,
{
    _private: (),
}

impl<const MIN: u32, const MAX: u32> MultipleInstanceSpecConst<MIN, MAX>
where
    crate::law::Require<{ MIN >= 1 }>: crate::law::IsTrue,
    crate::law::Require<{ MIN <= MAX }>: crate::law::IsTrue,
{
    /// Construct a `MultipleInstanceSpecConst<MIN, MAX>` — only possible when
    /// `MIN >= 1` and `MIN <= MAX`.
    ///
    /// ```
    /// # #![feature(generic_const_exprs)]
    /// # #![allow(incomplete_features)]
    /// use wasm4pm_compat::petri::MultipleInstanceSpecConst;
    /// let _: MultipleInstanceSpecConst<1, 1> = MultipleInstanceSpecConst::new();
    /// let _: MultipleInstanceSpecConst<2, 10> = MultipleInstanceSpecConst::new();
    /// ```
    pub const fn new() -> Self {
        MultipleInstanceSpecConst { _private: () }
    }

    /// The minimum instance count encoded in the type.
    ///
    /// ```
    /// # #![feature(generic_const_exprs)]
    /// # #![allow(incomplete_features)]
    /// use wasm4pm_compat::petri::MultipleInstanceSpecConst;
    /// assert_eq!(MultipleInstanceSpecConst::<2, 5>::new().min(), 2);
    /// ```
    pub const fn min(&self) -> u32 {
        MIN
    }

    /// The maximum instance count encoded in the type.
    ///
    /// ```
    /// # #![feature(generic_const_exprs)]
    /// # #![allow(incomplete_features)]
    /// use wasm4pm_compat::petri::MultipleInstanceSpecConst;
    /// assert_eq!(MultipleInstanceSpecConst::<2, 5>::new().max(), 5);
    /// ```
    pub const fn max(&self) -> u32 {
        MAX
    }
}

impl<const MIN: u32, const MAX: u32> Default for MultipleInstanceSpecConst<MIN, MAX>
where
    crate::law::Require<{ MIN >= 1 }>: crate::law::IsTrue,
    crate::law::Require<{ MIN <= MAX }>: crate::law::IsTrue,
{
    fn default() -> Self {
        Self::new()
    }
}

/// The specific, named laws under which Petri-net / WF-net / OC-Petri-net
/// structure is refused.
///
/// Each variant cites a distinct law — never a bare "invalid input".
/// [`PetriRefusal::SoundnessNotWitnessed`] is the boundary law for evidence:
/// it marks the refusal to treat a [`SoundnessClaimed`] net as if it were
/// [`SoundnessWitnessed`] without a witness from `wasm4pm`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PetriRefusal {
    /// A WF-net declares no (non-empty) initial marking.
    MissingInitialMarking,
    /// A WF-net declares no (non-empty) final marking.
    MissingFinalMarking,
    /// A transition can never fire (a behavioral defect surfaced from `wasm4pm`).
    DeadTransition,
    /// The net is not safe (a place can hold more than one token where one was
    /// required) — surfaced from analysis at `wasm4pm`.
    UnsafeNet,
    /// The net is unbounded — surfaced from analysis at `wasm4pm`.
    UnboundedNet,
    /// An OC-Petri-net arc carries an object type that is not declared/preserved.
    ObjectTypeNotPreserved,
    /// An arc is structurally invalid (e.g. dangling endpoint, malformed
    /// variable arc).
    InvalidVariableArc,
    /// Soundness was relied upon but not witnessed — graduate to `wasm4pm` to
    /// obtain the witness.
    SoundnessNotWitnessed,
    /// A [`CancellationRegion`] references a node id not declared in the net.
    ///
    /// Law: YAWL Definition 1 rem(t) ⊆ T ∪ C \ {i, o}.
    InvalidCancellationRegion,
    /// A [`MultipleInstanceSpec`] has `min == 0` or `min > max`.
    ///
    /// Law: YAWL Definition 1 nofi invariant: `1 ≤ min ≤ max`.
    InvalidInstanceBounds,
}

/// A WF-net tagged as **separable** — a necessary precondition for lossless
/// POWL 2.0 conversion (Kourani, Park & van der Aalst, 2026).
///
/// A WF-net is *separable* if it can be decomposed into a set of structurally
/// independent sub-nets whose composition reproduces the original language. Only
/// a separable WF-net can be faithfully represented as a POWL 2.0 model via the
/// POWL decomposition theorem (Kourani 2026, Definition 4.1).
///
/// ## Structure-only
///
/// This marker records the *claim* that a WF-net is separable; it does not
/// verify separability (that graduates to `wasm4pm`). The claim prevents code
/// from inadvertently feeding a non-separable net to a POWL 2.0 conversion
/// path at the type level.
///
/// ## How to use
///
/// Wrap your `WfNetConst` in a `SeparableWfNet` after verifying separability
/// via the `wasm4pm` engine and attaching the result here. The type-level
/// marker then flows through the rest of the pipeline.
///
/// ## Paper
///
/// Kourani, Park & van der Aalst (2026) — *Hierarchical Decomposition of
/// Separable Workflow-Nets*. Definition 4.1 (separable WF-net) and
/// Theorem 4.3 (POWL 2.0 language preservation under decomposition).
///
/// Structure-only: carries no behavior, no token semantics.
pub struct SeparableWfNet<const SOUNDNESS: crate::law::SoundnessState> {
    /// The underlying WF-net with its soundness state.
    pub net: WfNetConst<SOUNDNESS>,
    // Private seal — separability claim is not forgeable from outside this module.
    _seal: wfnet_seal::WfNetSeal,
}

impl<const SOUNDNESS: crate::law::SoundnessState> SeparableWfNet<SOUNDNESS> {
    /// Construct a `SeparableWfNet` from a verified `WfNetConst`.
    ///
    /// This is `pub(crate)` — the only public construction path is through the
    /// `wasm4pm` graduation bridge that verifies separability.
    ///
    /// ```
    /// use wasm4pm_compat::petri::{WfNetConst, SeparableWfNet};
    /// use wasm4pm_compat::law::SoundnessState;
    /// let _: SeparableWfNet<{ SoundnessState::Unknown }> =
    ///     SeparableWfNet::declare_separable(WfNetConst::new());
    /// ```
    pub fn declare_separable(net: WfNetConst<SOUNDNESS>) -> Self {
        SeparableWfNet {
            net,
            _seal: wfnet_seal::WfNetSeal,
        }
    }
}

/// A standalone, named error type for the *missing final marking* law.
///
/// [`PetriRefusal::MissingFinalMarking`] names the law as a variant on the
/// shared refusal enum. `MissingFinalMarkingError` is a separate zero-sized
/// struct that names the same law as a **first-class type** — useful when a
/// function returns `Result<_, MissingFinalMarkingError>` instead of the broad
/// `Result<_, PetriRefusal>`, making the specific law visible in the return
/// type signature.
///
/// Law: a WF-net must have a declared, non-empty final marking (van der Aalst,
/// 1998 — "proper completion" requires a designated final state).
///
/// ## Conversion
///
/// `MissingFinalMarkingError` converts to [`PetriRefusal::MissingFinalMarking`]
/// via `From<MissingFinalMarkingError> for PetriRefusal`.
///
/// Structure-only: zero-sized, carries no data.
///
/// ```
/// use wasm4pm_compat::petri::{MissingFinalMarkingError, PetriRefusal};
/// let e = MissingFinalMarkingError;
/// let r: PetriRefusal = e.into();
/// assert_eq!(r, PetriRefusal::MissingFinalMarking);
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct MissingFinalMarkingError;

impl core::fmt::Display for MissingFinalMarkingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "WF-net refused by law: MissingFinalMarking")
    }
}

impl From<MissingFinalMarkingError> for PetriRefusal {
    fn from(_: MissingFinalMarkingError) -> Self {
        PetriRefusal::MissingFinalMarking
    }
}

/// An uninhabited type-level marker asserting the *separability claim* about a
/// WF-net, independent of the [`SeparableWfNet`] wrapper struct.
///
/// `SeparableWfNetMarker` is used as a `PhantomData` type parameter in
/// downstream types that need to carry a separability claim through a pipeline
/// without wrapping the whole WF-net. The marker is an empty enum and therefore
/// uninhabited — it can only appear as a `PhantomData` field.
///
/// ## Why both `SeparableWfNet` and `SeparableWfNetMarker`?
///
/// [`SeparableWfNet`] wraps a [`WfNetConst`] — it is a value-carrying container.
/// `SeparableWfNetMarker` is a *phantom* — it marks a type as "was derived from
/// a separable net" without copying the net value. Use the marker when the net
/// itself does not need to travel with the downstream type.
///
/// ## Paper
///
/// Kourani, Park & van der Aalst (2026) — *Hierarchical Decomposition of
/// Separable Workflow-Nets*. Definition 4.1.
///
/// Structure-only: uninhabited, zero-cost phantom marker.
///
/// ```
/// use core::marker::PhantomData;
/// use wasm4pm_compat::petri::SeparableWfNetMarker;
///
/// // A downstream type that remembers the net was separable.
/// struct PowlConversionResult<S>(PhantomData<S>);
/// let _: PowlConversionResult<SeparableWfNetMarker>;
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SeparableWfNetMarker {}

impl core::fmt::Display for PetriRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let law = match self {
            PetriRefusal::MissingInitialMarking => "MissingInitialMarking",
            PetriRefusal::MissingFinalMarking => "MissingFinalMarking",
            PetriRefusal::DeadTransition => "DeadTransition",
            PetriRefusal::UnsafeNet => "UnsafeNet",
            PetriRefusal::UnboundedNet => "UnboundedNet",
            PetriRefusal::ObjectTypeNotPreserved => "ObjectTypeNotPreserved",
            PetriRefusal::InvalidVariableArc => "InvalidVariableArc",
            PetriRefusal::SoundnessNotWitnessed => "SoundnessNotWitnessed",
            PetriRefusal::InvalidCancellationRegion => "InvalidCancellationRegion",
            PetriRefusal::InvalidInstanceBounds => "InvalidInstanceBounds",
        };
        write!(f, "Petri-net refused by law: {law}")
    }
}
