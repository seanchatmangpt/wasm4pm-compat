//! Example: WF-net construction with typed arcs and soundness typestate
//!
//! Demonstrates how to build a minimal workflow net (WF-net) using the typed
//! arc law (bipartite pre- and post-incidence arcs), the `WfNetConst` soundness
//! typestate (`Unknown` → `Claimed`), and the `SoundnessProof` requirement that
//! makes `Claimed` → `Witnessed` non-forgeable from outside this crate.
//!
//! Run: cargo run --example petri_net_construction

#![allow(dead_code)]

use wasm4pm_compat::law::SoundnessState;
use wasm4pm_compat::petri::{
    Arc, Marking, PetriNet, Place, PlaceNodeMarker, PlaceToTransitionArc, Transition,
    TransitionNodeMarker, TransitionToPlaceArc, WfNet, WfNetConst,
};

// ---------------------------------------------------------------------------
// Node-kind marker types used as type parameters on the typed arc types.
//
// PlaceNodeMarker and TransitionNodeMarker are zero-sized (PhantomData-backed)
// marker structs. Using them as type params on PlaceToTransitionArc /
// TransitionToPlaceArc encodes the bipartite arc direction into the Rust type
// system — a PlaceToTransitionArc<P, T, _> cannot satisfy a slot that expects
// a TransitionToPlaceArc<T, P, _>.
// ---------------------------------------------------------------------------

fn demonstrate_typed_arc_markers() {
    println!("=== Typed arc-direction markers ===\n");

    // PlaceNodeMarker implements IsPlaceNode (sealed trait).
    // TransitionNodeMarker implements IsTransitionNode (sealed trait).
    // Neither can be implemented by user types — the seals are module-private.
    let _place_kind: PlaceNodeMarker = PlaceNodeMarker;
    let _transition_kind: TransitionNodeMarker = TransitionNodeMarker;

    println!("  PlaceNodeMarker       — sealed IsPlaceNode impl");
    println!("  TransitionNodeMarker  — sealed IsTransitionNode impl");
    println!("  (neither is constructible by user types outside petri)\n");
}

// ---------------------------------------------------------------------------
// PlaceToTransitionArc and TransitionToPlaceArc are the two typed arc structs.
//
// Their type parameters carry namespace markers (P and T are phantom types).
// The struct *type* — not a direction flag — enforces bipartite arc law:
// Murata (1989) §2, F ⊆ (P×T) ∪ (T×P).
// ---------------------------------------------------------------------------

fn demonstrate_typed_arcs() {
    println!("=== PlaceToTransitionArc and TransitionToPlaceArc ===\n");

    // P0 and T0 are user-defined phantom namespace markers. Their only role is
    // to let the compiler distinguish arcs that belong to different net shapes.
    struct P0;
    struct T0;

    // Pre-incidence arc: place P0 → transition T0. Weight = 1.
    let pre: PlaceToTransitionArc<P0, T0, u32> = PlaceToTransitionArc::new(1);
    println!("  PlaceToTransitionArc<P0, T0, u32>::new(1)");
    println!("  weight = {}", pre.weight());

    // Post-incidence arc: transition T0 → place P0. Weight = 1.
    let post: TransitionToPlaceArc<T0, P0, u32> = TransitionToPlaceArc::new(1);
    println!("  TransitionToPlaceArc<T0, P0, u32>::new(1)");
    println!("  weight = {}", post.weight());

    println!();
    println!("  Law (Murata 1989 §2): arcs are strictly bipartite.");
    println!("  PlaceToPlaceArc and TransitionToTransitionArc do not exist.");
    println!("  Attempting to construct one is a compile error — no type exists.\n");
}

// ---------------------------------------------------------------------------
// Place, Transition, Arc, and PetriNet are the runtime-valued node and net
// types. They carry string ids and (optionally) object-type annotations.
// ---------------------------------------------------------------------------

fn build_petri_net() -> PetriNet {
    println!("=== PetriNet construction ===\n");

    // A minimal WF-net shape for an order-fulfillment process:
    //   source → approve → sink
    //
    //   source --(p→t)-→ approve --(t→p)-→ sink
    let source = Place::new("source");
    let sink = Place::new("sink");
    let approve = Transition::new("t_approve", "approve");

    let arc_in = Arc::place_to_transition("source", "t_approve");
    let arc_out = Arc::transition_to_place("t_approve", "sink");

    let initial = Marking::new([("source".to_string(), 1)]);

    let net = PetriNet::new([source, sink], [approve], [arc_in, arc_out], initial);

    println!(
        "  Places       : {:?}",
        net.places().iter().map(Place::id).collect::<Vec<_>>()
    );
    println!(
        "  Transitions  : {:?}",
        net.transitions()
            .iter()
            .map(Transition::id)
            .collect::<Vec<_>>()
    );
    println!("  Arc count    : {}", net.arcs().len());
    println!(
        "  Initial mark : tokens on source = {}",
        net.initial_marking().tokens_on("source")
    );

    match net.validate() {
        Ok(()) => println!("  Structural validation: OK"),
        Err(e) => println!("  Structural validation: REFUSED — {e}"),
    }
    println!();

    net
}

// ---------------------------------------------------------------------------
// WfNet<SoundnessUnknown> wraps a PetriNet and adds a final marking.
//
// The soundness state is a phantom type parameter: WfNet<SoundnessUnknown>,
// WfNet<SoundnessClaimed>, WfNet<SoundnessWitnessed>. Advancing the state
// calls .claim_sound() — a type-level re-tagging only, no computation.
// ---------------------------------------------------------------------------

fn demonstrate_wfnet_typestate(net: PetriNet) {
    println!("=== WfNet<S> soundness typestate ===\n");

    let final_marking = Marking::new([("sink".to_string(), 1)]);

    // WfNet::new produces WfNet<SoundnessUnknown> — the default typestate.
    let unknown_wf = WfNet::new(net, final_marking);

    match unknown_wf.validate() {
        Ok(()) => println!("  WfNet<SoundnessUnknown> — structural validation: OK"),
        Err(e) => println!("  WfNet<SoundnessUnknown> — refused: {e}"),
    }

    // claim_sound() is a type-level re-tag: Unknown → Claimed.
    // It does not verify soundness. It records that an upstream source asserts
    // soundness; the claim must be discharged by graduating to wasm4pm.
    let claimed_wf = unknown_wf.claim_sound();
    println!("  .claim_sound() → WfNet<SoundnessClaimed>");
    println!("  (type-level re-tag only; no soundness analysis performed)\n");

    // To reach SoundnessWitnessed via WfNet, you would call .attest_witnessed().
    // That method is migrated because it is freely forgeable — it requires no
    // SoundnessProof token. See WfNetConst below for the non-forgeable path.
    let _ = claimed_wf;
}

// ---------------------------------------------------------------------------
// WfNetConst<{SoundnessState}> is the non-forgeable const-generic alternative.
//
// The SOUNDNESS const param is a SoundnessState enum value embedded in the
// type. Unknown and Claimed are reachable by anyone. Witnessed is only
// reachable via witness_soundness(proof), and SoundnessProof is only
// constructible inside the petri module or via the wasm4pm graduation bridge.
// ---------------------------------------------------------------------------

fn demonstrate_wfnet_const() {
    println!("=== WfNetConst<{{SoundnessState}}> non-forgeable typestate ===\n");

    // Unknown: the freely-constructible initial state.
    let unknown: WfNetConst<{ SoundnessState::Unknown }> =
        WfNetConst::<{ SoundnessState::Unknown }>::new();
    println!("  WfNetConst::<{{ SoundnessState::Unknown }}>::new()");
    println!("  soundness_state() = {:?}", unknown.soundness_state());

    // Claimed: advance from Unknown — a type-level re-tag, zero cost.
    let claimed: WfNetConst<{ SoundnessState::Claimed }> = unknown.claim_sound();
    println!("  .claim_sound() → WfNetConst<{{ SoundnessState::Claimed }}>");
    println!("  soundness_state() = {:?}", claimed.soundness_state());

    println!();
    println!("  To reach SoundnessState::Witnessed, call claimed.witness_soundness(proof).");
    println!("  SoundnessProof is only constructible by the petri module or wasm4pm bridge.");
    println!("  Attempting to forge it outside petri is a compile error:");
    println!("    WfNetConst {{ _seal: todo!() }}  →  private field, does not compile.");
    println!();

    // Illustrate what witness_soundness requires — its signature in petri.rs:
    //   pub fn witness_soundness(self, _proof: SoundnessProof) -> WfNetConst<{Witnessed}>
    //
    // SoundnessProof wraps the private wfnet_seal::WfNetSeal type. User code
    // cannot construct WfNetSeal, so it cannot construct SoundnessProof, so it
    // cannot call witness_soundness. This is the non-forgeability guarantee.
    println!("  SoundnessProof anatomy:");
    println!("    pub struct SoundnessProof(wfnet_seal::WfNetSeal)  // WfNetSeal is private");
    println!("    pub(crate) fn new() -> Self {{ ... }}               // pub(crate) only");
    println!("  External callers: cannot construct SoundnessProof — compile error.\n");
}

fn main() {
    println!("=== petri_net_construction example ===\n");
    println!("This example demonstrates the typed arc law, place/transition");
    println!("construction, WfNet<S> soundness typestate, and WfNetConst<S>");
    println!("non-forgeable soundness witnessing.\n");

    demonstrate_typed_arc_markers();
    demonstrate_typed_arcs();

    let net = build_petri_net();
    demonstrate_wfnet_typestate(net);
    demonstrate_wfnet_const();

    println!("=== Example complete ===");
}
