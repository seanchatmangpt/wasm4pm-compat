// COMPILE-PASS: WfNet2PowlWitness construction with context label.
//
// Law: Kourani, Park & van der Aalst (2026) Theorem 4.3 — a separable WF-net
// can be converted to a POWL 2.0 model preserving the process language.
// WfNet2PowlWitness records that conversion; its context names the source.
use wasm4pm_compat::powl::WfNet2PowlWitness;

fn main() {
    let w = WfNet2PowlWitness::new_internal("wfnet-001");
    assert_eq!(w.context, "wfnet-001");
}
