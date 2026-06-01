// COMPILE-FAIL: Witness confusion law — Evidence<T, Admitted, Pm4pyApiGrammar> cannot be used
// where Evidence<T, Admitted, PmaxConsumerGrammar> is required.
// Law: Pm4pyApiGrammar and PmaxConsumerGrammar are distinct API grammar witnesses.
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::Admitted;
use wasm4pm_compat::witness::{Pm4pyApiGrammar, PmaxConsumerGrammar};

fn requires_pmax_evidence(_: Evidence<String, Admitted, PmaxConsumerGrammar>) {}

fn main() {
    let pm4py_ev: Evidence<String, Admitted, Pm4pyApiGrammar> = todo!();
    // This must fail: Pm4pyApiGrammar witness is not PmaxConsumerGrammar.
    requires_pmax_evidence(pm4py_ev);
}
