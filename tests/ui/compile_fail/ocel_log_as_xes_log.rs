// COMPILE-FAIL: OCEL/XES confusion law — OcelLog cannot be passed where XesLog is required.
// Law: OcelLog and XesLog are structurally distinct types. Object-centric logs
// cannot silently replace case-centric logs. The boundary law is enforced by the type system.
use wasm4pm_compat::ocel::OcelLog;
use wasm4pm_compat::xes::XesLog;

fn requires_xes_log(_log: XesLog) {}

fn _test(ocel: OcelLog) {
    // This must fail: OcelLog is not XesLog.
    requires_xes_log(ocel);
}

fn main() {}
