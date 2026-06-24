#![feature(generic_const_exprs, adt_const_params)]
#![allow(incomplete_features)]

use wasm4pm_compat::law::SoundnessState;
use wasm4pm_compat::petri::{
    WfNetConst, FreeChoiceMarker, BranchingProcess, Condition, Event, UnfoldingPrefix,
};

// Function restricted to free-choice nets (polynomial soundness path)
fn verify_free_choice_soundness<const S: SoundnessState>(
    _net: WfNetConst<S, { FreeChoiceMarker::FreeChoice }>,
) {}

// Concrete type marker for net
struct NetA;

fn process_prefix(_prefix: UnfoldingPrefix<NetA>) {}

fn main() {
    // 1. Free-choice Net verification
    // Construct unknown general net (defaults to General)
    let unknown_gen = WfNetConst::<{ SoundnessState::Unknown }>::new();
    
    // Transition it to FreeChoice
    let unknown_fc = unknown_gen.into_free_choice();
    
    // Propagate the type parameters through a claim advancement
    let claimed_fc = unknown_fc.claim_sound();
    
    // Pass the free-choice net to the restricted polynomial soundness checker
    verify_free_choice_soundness(claimed_fc);

    // 2. Petri Net Unfolding verification
    let cond = Condition::new("c1", "p1");
    let ev = Event::new("e1", "t1");
    
    let bp = BranchingProcess::<NetA>::new(
        vec![cond],
        vec![ev],
        vec![],
    );
    
    let prefix = UnfoldingPrefix::<NetA>::new(bp, vec!["e1".to_string()]);
    process_prefix(prefix);
}
