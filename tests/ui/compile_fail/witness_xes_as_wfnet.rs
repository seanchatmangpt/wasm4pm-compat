// COMPILE-FAIL: Witness confusion law — Evidence<T, Admitted, Xes1849> cannot be used
// where Evidence<T, Admitted, WfNetSoundnessPaper> is required.
// Law: The witness type parameter W makes XES-admitted and WF-net-admitted
// evidence structurally distinct with no coercion path between them.
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::Admitted;
use wasm4pm_compat::witness::{WfNetSoundnessPaper, Xes1849};

fn requires_wfnet_evidence(_: Evidence<String, Admitted, WfNetSoundnessPaper>) {}

fn _test(xes_ev: Evidence<String, Admitted, Xes1849>) {
    // This must fail: Xes1849 witness is not WfNetSoundnessPaper.
    requires_wfnet_evidence(xes_ev);
}

fn main() {}
