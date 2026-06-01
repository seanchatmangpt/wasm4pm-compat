// COMPILE-FAIL: TemporalOrder type confusion — Online/Offline context non-interchangeable.
//
// Law: TemporalContextLaw — ContextualEvidence<T, OnlineMonitoringContext> and
// ContextualEvidence<T, OfflineAnalysisContext> are distinct types.
// A function requiring offline analysis evidence must reject online monitoring evidence.
use wasm4pm_compat::streaming::{ContextualEvidence, OfflineAnalysisContext, OnlineMonitoringContext};

fn requires_offline(_ev: ContextualEvidence<u64, OfflineAnalysisContext>) {}

fn main() {
    let online_ev: ContextualEvidence<u64, OnlineMonitoringContext> =
        ContextualEvidence::online(42u64);
    // This must fail: ContextualEvidence<_, OnlineMonitoringContext> is not
    // ContextualEvidence<_, OfflineAnalysisContext>.
    // The Context type parameter enforces collection-context at the type level.
    requires_offline(online_ev);
}
