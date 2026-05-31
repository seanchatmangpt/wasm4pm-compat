// COMPILE-FAIL: DfgRefusal::DiscoveryRequired — a Raw DFG evidence cannot be
// consumed as an admitted (discovered) DFG.
//
// Law: DfgRefusal::DiscoveryRequired — a DFG that must be discovered from a log
// is an engine concern that graduates to wasm4pm. Before discovery, the DFG
// evidence is in the Raw state. The one-way door law (Evidence typestate) means
// Raw evidence cannot be extracted with into_inner() — that path is only open
// to Admitted evidence, i.e., evidence that has been admitted through a named
// law (e.g. the DFG shape law). Calling into_inner() on a Raw DFG evidence
// conflates the "needs discovery" state with the "has been admitted" state.
//
// Expected error: no method named `into_inner` found for
//   Evidence<Dfg, Raw, Pm4pyApiGrammar>
use wasm4pm_compat::dfg::Dfg;
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::witness::Pm4pyApiGrammar;

fn main() {
    // Raw DFG evidence: not yet admitted through any named law.
    // Before discovery (DfgRefusal::DiscoveryRequired), the DFG is structurally
    // unknown — it is Raw, not Admitted.
    let raw_dfg: Evidence<Dfg, _, Pm4pyApiGrammar> = Evidence::raw(Dfg::default());

    // Attempting to extract the DFG as if it were already discovered (Admitted)
    // is a compile error: into_inner() is only available on Admitted evidence.
    // This guards the DfgRefusal::DiscoveryRequired boundary at the type level.
    let _discovered_dfg = raw_dfg.into_inner();
}
