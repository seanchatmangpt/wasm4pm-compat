use crate::law::SoundnessState;
pub use crate::models::{Arc, ArcDirection, PetriNet, PetriNetRefusal, Place, Transition};
use std::fmt;
use std::marker::PhantomData;

// ── Marking ──────────────────────────────────────────────────────────────────

/// A token distribution over places — the runtime state of a Petri net.
#[derive(Debug, Clone, Default)]
pub struct Marking {
    tokens: Vec<(String, usize)>,
}

impl Marking {
    pub fn new(tokens: impl IntoIterator<Item = (String, usize)>) -> Self {
        Marking {
            tokens: tokens.into_iter().collect(),
        }
    }

    pub fn empty() -> Self {
        Marking::default()
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub fn tokens(&self) -> &[(String, usize)] {
        &self.tokens
    }
}

// ── PetriRefusal ─────────────────────────────────────────────────────────────

/// Named refusal variants for Petri net validation laws.
///
/// Every variant names a specific law from van der Aalst's workflow net theory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PetriRefusal {
    MissingInitialMarking,
    MissingFinalMarking,
    DeadTransition,
    UnsafeNet,
    UnboundedNet,
    ObjectTypeNotPreserved,
    InvalidVariableArc,
    SoundnessNotWitnessed,
    InvalidCancellationRegion,
    InvalidInstanceBounds,
}

impl fmt::Display for PetriRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl std::error::Error for PetriRefusal {}

// ── WfNet typestate ───────────────────────────────────────────────────────────

/// Typestate marker: soundness has been asserted (not verified by replay).
pub struct SoundnessClaimed;

/// Typestate marker: soundness has been witnessed by token replay.
pub struct SoundnessWitnessed;

/// Default typestate: no soundness claim has been made yet.
pub struct SoundnessUnknown;

/// Alias for compatibility.
pub type Unchecked = SoundnessUnknown;

/// A workflow net — a Petri net with a designated final marking.
///
/// The typestate parameter `S` tracks soundness evidence:
/// - `WfNet<SoundnessUnknown>` — no soundness claim made
/// - `WfNet<SoundnessClaimed>` — caller asserted soundness via `claim_sound()`
/// - `WfNet<SoundnessWitnessed>` — soundness verified by token replay
pub struct WfNet<S = SoundnessUnknown> {
    net: PetriNet,
    final_marking: Marking,
    _s: PhantomData<S>,
}

/// Phantom-typed proof carrier for workflow net soundness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WfNetSoundnessProofOf<Net> {
    _seal: (),
    _marker: std::marker::PhantomData<Net>,
}

impl<Net> WfNetSoundnessProofOf<Net> {
    #[allow(dead_code)]
    pub(crate) fn new() -> Self {
        WfNetSoundnessProofOf {
            _seal: (),
            _marker: std::marker::PhantomData,
        }
    }
}

impl WfNet<SoundnessUnknown> {
    pub fn new(net: PetriNet, final_marking: Marking) -> Self {
        WfNet {
            net,
            final_marking,
            _s: PhantomData,
        }
    }

    #[must_use]
    pub fn claim_sound(self) -> WfNet<SoundnessClaimed> {
        WfNet {
            net: self.net,
            final_marking: self.final_marking,
            _s: PhantomData,
        }
    }
}

impl<S> WfNet<S> {
    pub fn validate(&self) -> Result<(), PetriRefusal> {
        if self.final_marking.is_empty() {
            return Err(PetriRefusal::MissingFinalMarking);
        }
        Ok(())
    }

    #[must_use]
    pub fn final_marking(&self) -> Option<&Marking> {
        if self.final_marking.is_empty() {
            None
        } else {
            Some(&self.final_marking)
        }
    }

    pub fn net(&self) -> &PetriNet {
        &self.net
    }
}

// ── ObjectCentricPetriNet ─────────────────────────────────────────────────────

/// A Petri net extended with object-type annotations on arcs,
/// per the OCPN model from van der Aalst et al.
#[derive(Debug, Clone)]
pub struct ObjectCentricPetriNet {
    net: PetriNet,
    object_types: Vec<String>,
}

impl ObjectCentricPetriNet {
    pub fn new(net: PetriNet, object_types: impl IntoIterator<Item = String>) -> Self {
        ObjectCentricPetriNet {
            net,
            object_types: object_types.into_iter().collect(),
        }
    }

    pub fn net(&self) -> &PetriNet {
        &self.net
    }
    pub fn object_types(&self) -> &[String] {
        &self.object_types
    }

    pub fn validate(&self) -> Result<(), PetriRefusal> {
        let type_set: std::collections::HashSet<&str> =
            self.object_types.iter().map(|s| s.as_str()).collect();
        for arc in &self.net.arcs {
            if let Some((ref ot, _)) = arc.object_type {
                if !type_set.contains(ot.as_str()) {
                    return Err(PetriRefusal::ObjectTypeNotPreserved);
                }
            }
        }
        Ok(())
    }
}

// ── PetriNet/Place/Transition accessor methods ────────────────────────────────

impl Place {
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl Transition {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn label(&self) -> &str {
        &self.label
    }
    pub fn is_silent(&self) -> bool {
        self.is_invisible.unwrap_or(false)
    }
    pub fn silent(id: &str) -> Self {
        Transition {
            id: id.to_owned(),
            label: String::new(),
            is_invisible: Some(true),
        }
    }
}

impl PetriNet {
    pub fn places(&self) -> &[Place] {
        &self.places
    }
    pub fn transitions(&self) -> &[Transition] {
        &self.transitions
    }
    pub fn arcs(&self) -> &[Arc] {
        &self.arcs
    }
    pub fn initial_marking(&self) -> RuntimeMarking<'_> {
        RuntimeMarking { net: self }
    }
}

/// Read-only view of the initial marking backed by the PetriNet's PackedKeyTable.
pub struct RuntimeMarking<'a> {
    net: &'a PetriNet,
}

impl<'a> RuntimeMarking<'a> {
    /// Returns the token count on a named place, or 0 if not marked.
    pub fn tokens_on(&self, place_id: &str) -> usize {
        use crate::dense_kernel::fnv1a_64;
        let hash = fnv1a_64(place_id.as_bytes());
        *self.net.initial_marking.get(hash).unwrap_or(&0)
    }
}

// ── Marking::tokens_on ───────────────────────────────────────────────────────

impl Marking {
    /// Returns the token count on a named place, or 0 if absent.
    pub fn tokens_on(&self, place_id: &str) -> usize {
        self.tokens
            .iter()
            .find(|(id, _)| id == place_id)
            .map(|(_, n)| *n)
            .unwrap_or(0)
    }
}

// ── BipartiteArcConst ─────────────────────────────────────────────────────────

use crate::law::ArcDirectionConst;

pub struct BipartiteArcConst<const DIR: ArcDirectionConst, W> {
    place_id: String,
    transition_id: String,
    weight: W,
}

impl<const DIR: ArcDirectionConst, W: Copy> BipartiteArcConst<DIR, W> {
    pub fn new(place_id: &str, transition_id: &str, weight: W) -> Self {
        BipartiteArcConst {
            place_id: place_id.to_owned(),
            transition_id: transition_id.to_owned(),
            weight,
        }
    }
    pub fn place_id(&self) -> &str {
        &self.place_id
    }
    pub fn transition_id(&self) -> &str {
        &self.transition_id
    }
    pub fn weight(&self) -> W {
        self.weight
    }
    pub fn direction(&self) -> ArcDirectionConst {
        DIR
    }
}

// ── InitialFinalMarkingPair ───────────────────────────────────────────────────

pub struct InitialFinalMarkingPair {
    pub initial: Marking,
    pub final_marking: Marking,
}

impl InitialFinalMarkingPair {
    pub fn new(initial: Marking, final_marking: Marking) -> Self {
        InitialFinalMarkingPair {
            initial,
            final_marking,
        }
    }
    pub fn validate(&self) -> Result<(), PetriRefusal> {
        for (p_init, t_init) in self.initial.tokens() {
            if *t_init > 0 && self.final_marking.tokens_on(p_init) > 0 {
                return Err(PetriRefusal::UnsafeNet);
            }
        }
        Ok(())
    }
}

// ── Node-kind markers and typed arc structs ───────────────────────────────────

/// Sealed marker: this type represents a place node.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct PlaceNodeMarker;

pub trait IsPlaceNode: Default + Clone + Copy {}
impl IsPlaceNode for PlaceNodeMarker {}

/// Sealed marker: this type represents a transition node.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct TransitionNodeMarker;

pub trait IsTransitionNode: Default + Clone + Copy {}
impl IsTransitionNode for TransitionNodeMarker {}

pub trait IsValidArc {}

/// A pre-incidence (place → transition) arc carrying a typed weight.
///
/// The phantom types `P` and `T` name the place and transition namespaces;
/// the weight type `W` names the arc multiplicity type (usually `u32`).
pub struct PlaceToTransitionArc<P, T, W> {
    w: W,
    _p: PhantomData<(P, T)>,
}

impl<P, T, W: Copy> PlaceToTransitionArc<P, T, W> {
    pub fn new(weight: W) -> Self {
        PlaceToTransitionArc {
            w: weight,
            _p: PhantomData,
        }
    }
    pub fn weight(&self) -> W {
        self.w
    }
}

impl<P, T, W> IsValidArc for PlaceToTransitionArc<P, T, W> {}

/// A post-incidence (transition → place) arc carrying a typed weight.
pub struct TransitionToPlaceArc<T, P, W> {
    w: W,
    _p: PhantomData<(T, P)>,
}

impl<T, P, W: Copy> TransitionToPlaceArc<T, P, W> {
    pub fn new(weight: W) -> Self {
        TransitionToPlaceArc {
            w: weight,
            _p: PhantomData,
        }
    }
    pub fn weight(&self) -> W {
        self.w
    }
}

impl<T, P, W> IsValidArc for TransitionToPlaceArc<T, P, W> {}

pub struct SeparableWfNetMarker;

pub struct SeparableWfNet<const S: SoundnessState> {
    pub net: WfNetConst<S>,
}

impl<const S: SoundnessState> SeparableWfNet<S> {
    pub fn declare_separable(net: WfNetConst<S>) -> Self {
        SeparableWfNet { net }
    }
}

// ── WfNetConst — const-generic soundness typestate ────────────────────────────

mod wfnet_seal {
    pub(super) struct WfNetSeal;
}

/// Proof token that soundness has been verified (crate-internal only).
pub struct SoundnessProof(wfnet_seal::WfNetSeal);

#[allow(dead_code)] // reserved: issued by soundness-witnessing logic not yet in this crate
impl SoundnessProof {
    pub(crate) fn new() -> Self {
        SoundnessProof(wfnet_seal::WfNetSeal)
    }
}

/// A workflow net whose soundness state is embedded as a const generic parameter.
///
/// `SoundnessState::Unknown` is freely constructible.
/// `SoundnessState::Claimed` is reachable via `.claim_sound()`.
/// `SoundnessState::Witnessed` requires a `SoundnessProof` token — non-forgeable.
pub struct WfNetConst<const SOUNDNESS: SoundnessState> {
    _seal: (),
}

impl WfNetConst<{ SoundnessState::Unknown }> {
    pub fn new() -> Self {
        WfNetConst { _seal: () }
    }

    /// Advance: Unknown → Claimed (type-level re-tag, zero cost).
    #[must_use]
    pub fn claim_sound(self) -> WfNetConst<{ SoundnessState::Claimed }> {
        WfNetConst { _seal: () }
    }
}

impl WfNetConst<{ SoundnessState::Claimed }> {
    /// Advance: Claimed → Witnessed, guarded by a `SoundnessProof` token.
    ///
    /// `SoundnessProof` is only constructible inside the petri module, making
    /// this transition non-forgeable from external code.
    #[must_use]
    pub fn witness_soundness(
        self,
        _proof: SoundnessProof,
    ) -> WfNetConst<{ SoundnessState::Witnessed }> {
        WfNetConst { _seal: () }
    }
}

impl<const S: SoundnessState> WfNetConst<S> {
    pub fn soundness_state(&self) -> SoundnessState {
        S
    }
}

impl Default for WfNetConst<{ SoundnessState::Unknown }> {
    fn default() -> Self {
        WfNetConst { _seal: () }
    }
}

// ── WfNetQuery ────────────────────────────────────────────────────────────────

/// Query interface for WF-net structural properties (reserved for future use).
pub trait WfNetQuery {
    /// The soundness state of this WF-net as a runtime value.
    fn soundness_state(&self) -> SoundnessState;
}

impl<const S: SoundnessState> WfNetQuery for WfNetConst<S> {
    fn soundness_state(&self) -> SoundnessState {
        S
    }
}

/// `rem: T ⇸ P(T ∪ C \ {i, o})` — each task optionally names a cancellation
/// region. This struct carries the *shape* of that region: a named set of node
/// ids. Token-removal execution (the actual vacuuming) graduates to `wasm4pm`.
///
/// The `#[repr(transparent)]` newtype prevents a bare `Vec<String>` from being
/// accidentally passed where a `CancellationRegion` is required. It is zero-cost
/// to hold and clone.
///
/// Structure-only: carries ids, never fires.
#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct CancellationRegion {
    /// The ids of nodes (places, conditions, tasks) in this cancellation region,
    /// excluding the initial and final place of the net (i, o).
    pub members: Vec<String>,
}

impl CancellationRegion {
    /// Construct a cancellation region from an iterator of node ids.
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
    pub fn members(&self) -> &[String] {
        &self.members
    }
}

// ── YAWL multiple-instance bounds ───────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InstanceCreationKind {
    /// All child instances are created at the moment the task fires.
    Static,
    /// Child instances may be created incrementally during the task's lifetime.
    Dynamic,
}

/// A YAWL multiple-instance task specification: the four-tuple
/// `(min_instances, max_instances, threshold, creation_kind)`.
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
    #[must_use = "check the shape-check result"]
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
    pub const fn new() -> Self {
        MultipleInstanceSpecConst { _private: () }
    }

    /// The minimum instance count encoded in the type.
    pub const fn min(&self) -> u32 {
        MIN
    }

    /// The maximum instance count encoded in the type.
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

/// A standalone, named error type for the *missing final marking* law.
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

pub struct BoundedWfNet<const PLACES: usize, const TRANSITIONS: usize>
where
    [(); PLACES + TRANSITIONS]:,
{
    pub places: [String; PLACES],
    pub transitions: [String; TRANSITIONS],
}

impl<const PLACES: usize, const TRANSITIONS: usize> BoundedWfNet<PLACES, TRANSITIONS>
where
    [(); PLACES + TRANSITIONS]:,
    crate::law::Require<{ PLACES + TRANSITIONS <= 4096 }>: crate::law::IsTrue,
{
    pub fn new(places: [String; PLACES], transitions: [String; TRANSITIONS]) -> Self {
        Self {
            places,
            transitions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::law::SoundnessState;

    #[test]
    fn soundness_proof_mints_and_advances_to_witnessed() {
        // SoundnessProof::new() is pub(crate) — this test is the only caller,
        // which is the point: it proves the sealed capability works end-to-end.
        let proof = SoundnessProof::new();
        let witnessed = WfNetConst::<{ SoundnessState::Unknown }>::new()
            .claim_sound()
            .witness_soundness(proof);
        assert_eq!(witnessed.soundness_state(), SoundnessState::Witnessed);
    }
}
