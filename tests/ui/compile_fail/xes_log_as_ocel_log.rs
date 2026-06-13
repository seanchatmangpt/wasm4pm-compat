// COMPILE-FAIL: OCEL/XES confusion law — XesLog cannot be passed where OcelLog is required.
// Law: XesLog and OcelLog are structurally distinct types. Case-centric logs
// cannot silently replace object-centric logs. The boundary law is enforced by the type system.
use wasm4pm_compat::ocel::OcelLog;
use wasm4pm_compat::xes::XesLog;

fn requires_ocel_log(_log: OcelLog) {}

fn _test(xes: XesLog) {
    // This must fail: XesLog is not OcelLog.
    requires_ocel_log(xes);
}

fn main() {}
