// COMPILE-PASS: WfNet2PowlWitness — the conversion witness for WF-net to
// POWL 2.0 can be constructed and carries its context label.
//
// Law: Kourani, Park & van der Aalst (2026) Theorem 4.3 — separable WF-net
// can be converted to POWL 2.0 preserving language. The WfNet2PowlWitness
// records that the conversion happened under the separability precondition.
use wasm4pm_compat::powl::WfNet2PowlWitness;

fn main() {
    let witness = WfNet2PowlWitness::new_internal("wfnet-order-process");
    assert_eq!(witness.context, "wfnet-order-process");

    // The witness carries its context string for provenance.
    let w2 = WfNet2PowlWitness::new_internal("case-process-net");
    assert_eq!(w2.context, "case-process-net");
}
