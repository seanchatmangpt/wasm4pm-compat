// COMPILE-FAIL: EvidenceState sealed law — a user-defined type cannot implement EvidenceState.
// Law: EvidenceState is sealed via a private::Sealed supertrait. Only the seven canonical
// lifecycle stage tokens implement it. A downstream type cannot invent its own stage.
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::EvidenceState;

// A user-defined "stage" that does NOT implement private::Sealed or EvidenceState.
struct MyCustomStage;

fn requires_evidence_state<S: EvidenceState>(_e: Evidence<u32, S, ()>) {}

fn main() {
    // MyCustomStage does not implement EvidenceState.
    requires_evidence_state(Evidence::<u32, MyCustomStage, ()> {
        value: 0u32,
        state: core::marker::PhantomData,
        witness: core::marker::PhantomData,
    });
}
