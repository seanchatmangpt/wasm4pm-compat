// COMPILE-FAIL: XES structural law — XesTrace cannot be passed where XesLog is required.
// Law: XesTrace (one case, one sequence of events) and XesLog (the full log with extensions)
// are distinct structural types. A single trace is not a log.
use wasm4pm_compat::xes::{XesLog, XesTrace};

fn requires_xes_log(_l: XesLog) {}

fn main() {
    let trace = XesTrace::new("case-1", []);
    // This must fail: XesTrace is not XesLog.
    requires_xes_log(trace);
}
