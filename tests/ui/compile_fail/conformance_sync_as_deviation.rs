// COMPILE-FAIL: Conformance alignment law — SyncMove cannot be passed where LogOnlyMove is required.
// Law: SyncMove, LogOnlyMove, and ModelOnlyMove are distinct alignment move types.
// A synchronous (matching) move must not be confused with a log-only deviation.
use wasm4pm_compat::conformance::{LogOnlyMove, SyncMove};

fn requires_log_only(_m: LogOnlyMove) {}

fn main() {
    let sync = SyncMove;
    // This must fail: SyncMove is not LogOnlyMove.
    requires_log_only(sync);
}
