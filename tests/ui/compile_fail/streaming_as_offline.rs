// COMPILE-FAIL: Streaming context confusion — online evidence used as offline.
//
// Law: StreamingContextLaw — ContextualEvidence with OnlineMonitoringContext
// cannot be coerced into OfflineAnalysisContext. The context tag is non-coercible.
// This is the same law as temporal_order_confusion but approached via type coercion
// rather than function argument passing.
use wasm4pm_compat::streaming::{ContextualEvidence, OfflineAnalysisContext, OnlineMonitoringContext};

fn main() {
    let online: ContextualEvidence<String, OnlineMonitoringContext> =
        ContextualEvidence::online("stream-payload".to_string());
    // This must fail: cannot assign OnlineMonitoringContext evidence to an
    // OfflineAnalysisContext binding — the context tags are different types with no coercion.
    let _offline: ContextualEvidence<String, OfflineAnalysisContext> = online;
}
