pub use crate::models::{Arc, PetriNet, PetriNetRefusal, Place, Transition};
use crate::law::SoundnessState;
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
        Marking { tokens: tokens.into_iter().collect() }
    }

    pub fn empty() -> Self { Marking::default() }

    pub fn is_empty(&self) -> bool { self.tokens.is_empty() }

    pub fn tokens(&self) -> &[(String, usize)] { &self.tokens }
}

// ── PetriRefusal ─────────────────────────────────────────────────────────────

/// Named refusal variants for Petri net validation laws.
///
/// Every variant names a specific law from van der Aalst's workflow net theory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PetriRefusal {
    /// No final marking provided — a workflow net requires a defined sink state.
    MissingFinalMarking,
    /// An arc is typed but its object type is not declared in the net's type set.
    ObjectTypeNotPreserved,
}

impl fmt::Display for PetriRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PetriRefusal::MissingFinalMarking => write!(f, "MissingFinalMarking"),
            PetriRefusal::ObjectTypeNotPreserved => write!(f, "ObjectTypeNotPreserved"),
        }
    }
}

impl std::error::Error for PetriRefusal {}

// ── WfNet typestate ───────────────────────────────────────────────────────────

/// Typestate marker: soundness has been asserted (not verified by replay).
pub struct SoundnessClaimed;

/// Typestate marker: soundness has been witnessed by token replay.
pub struct SoundnessWitnessed;

/// Default typestate: no soundness claim has been made yet.
///
/// Users cannot construct `WfNet<Unchecked>` by naming the type parameter —
/// they use `WfNet::new()` which returns `WfNet<Unchecked>` by inference.
pub struct Unchecked;

/// A workflow net — a Petri net with a designated final marking.
///
/// The typestate parameter `S` tracks soundness evidence:
/// - `WfNet<Unchecked>` — no soundness claim made
/// - `WfNet<SoundnessClaimed>` — caller asserted soundness via `claim_sound()`
/// - `WfNet<SoundnessWitnessed>` — soundness verified by token replay
pub struct WfNet<S = Unchecked> {
    net: PetriNet,
    final_marking: Marking,
    _s: PhantomData<S>,
}

impl WfNet<Unchecked> {
    pub fn new(net: PetriNet, final_marking: Marking) -> Self {
        WfNet { net, final_marking, _s: PhantomData }
    }

    #[must_use]
    pub fn claim_sound(self) -> WfNet<SoundnessClaimed> {
        WfNet { net: self.net, final_marking: self.final_marking, _s: PhantomData }
    }
}

impl<S> WfNet<S> {
    #[must_use]
    pub fn validate(&self) -> Result<(), PetriRefusal> {
        if self.final_marking.is_empty() {
            return Err(PetriRefusal::MissingFinalMarking);
        }
        Ok(())
    }

    #[must_use]
    pub fn final_marking(&self) -> Option<&Marking> {
        if self.final_marking.is_empty() { None } else { Some(&self.final_marking) }
    }

    pub fn net(&self) -> &PetriNet { &self.net }
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
        ObjectCentricPetriNet { net, object_types: object_types.into_iter().collect() }
    }

    pub fn net(&self) -> &PetriNet { &self.net }
    pub fn object_types(&self) -> &[String] { &self.object_types }

    #[must_use]
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
    pub fn id(&self) -> &str { &self.id }
}

impl Transition {
    pub fn id(&self) -> &str { &self.id }
}

impl PetriNet {
    pub fn places(&self) -> &[Place] { &self.places }
    pub fn transitions(&self) -> &[Transition] { &self.transitions }
    pub fn arcs(&self) -> &[Arc] { &self.arcs }
    pub fn initial_marking(&self) -> RuntimeMarking<'_> { RuntimeMarking { net: self } }
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
        self.tokens.iter()
            .find(|(id, _)| id == place_id)
            .map(|(_, n)| *n)
            .unwrap_or(0)
    }
}

// ── ObjectChange builder ──────────────────────────────────────────────────────

impl crate::ocel::ObjectChange {
    /// Builder: add a timestamp to this change (no-op stub — ObjectChange is
    /// currently structure-only; `at_ns` is reserved for future OCEL 2.0 typing).
    #[must_use]
    pub fn at_ns(self, _ns: u64) -> Self { self }
}

// ── Node-kind markers and typed arc structs ───────────────────────────────────

/// Sealed marker: this type represents a place node.
pub struct PlaceNodeMarker;

/// Sealed marker: this type represents a transition node.
pub struct TransitionNodeMarker;

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
        PlaceToTransitionArc { w: weight, _p: PhantomData }
    }
    pub fn weight(&self) -> W { self.w }
}

/// A post-incidence (transition → place) arc carrying a typed weight.
pub struct TransitionToPlaceArc<T, P, W> {
    w: W,
    _p: PhantomData<(T, P)>,
}

impl<T, P, W: Copy> TransitionToPlaceArc<T, P, W> {
    pub fn new(weight: W) -> Self {
        TransitionToPlaceArc { w: weight, _p: PhantomData }
    }
    pub fn weight(&self) -> W { self.w }
}

// ── WfNetConst — const-generic soundness typestate ────────────────────────────

mod wfnet_seal {
    pub(super) struct WfNetSeal;
}

/// Proof token that soundness has been verified (crate-internal only).
pub struct SoundnessProof(wfnet_seal::WfNetSeal);

impl SoundnessProof {
    pub(crate) fn new() -> Self { SoundnessProof(wfnet_seal::WfNetSeal) }
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
    pub fn new() -> Self { WfNetConst { _seal: () } }

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
    pub fn witness_soundness(self, _proof: SoundnessProof) -> WfNetConst<{ SoundnessState::Witnessed }> {
        WfNetConst { _seal: () }
    }
}

impl<const S: SoundnessState> WfNetConst<S> {
    pub fn soundness_state(&self) -> SoundnessState { S }
}

// ── WfNetQuery ────────────────────────────────────────────────────────────────

/// Query interface for WF-net structural properties (reserved for future use).
pub struct WfNetQuery;

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
